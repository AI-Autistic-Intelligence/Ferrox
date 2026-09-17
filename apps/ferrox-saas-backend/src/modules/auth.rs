use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::persistence::{UserRepository, UserEntity};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub totp_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub token: Option<String>,
    pub requires_2fa: bool,
    pub user: Option<UserEntity>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TotpSetupResponse {
    pub secret: String,
    pub qr_code_url: String,
}

pub struct PasetoAuth {
    secret: String,
}

impl PasetoAuth {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }

    pub fn generate_token(&self, user_id: &str, role: &str, _ttl_sec: u64) -> Result<String, String> {
        let claims = json!({
            "sub": user_id,
            "role": role,
            "iss": "ferrox-saas-backend",
            "exp": chrono::Utc::now().timestamp() + 86400
        });
        let payload = serde_json::to_string(&claims).map_err(|e| e.to_string())?;
        let token = format!("v4.local.{}", base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, payload.as_bytes()));
        Ok(token)
    }
}

pub struct AuthState {
    pub paseto: Arc<PasetoAuth>,
    pub user_repo: Arc<dyn UserRepository>,
}

pub fn router(paseto: Arc<PasetoAuth>, user_repo: Arc<dyn UserRepository>) -> Router {
    let state = Arc::new(AuthState { paseto, user_repo });

    Router::new()
        .route("/login", post(login_handler))
        .route("/register", post(register_handler))
        .route("/oauth/google", get(oauth_google_handler))
        .route("/oauth/github", get(oauth_github_handler))
        .route("/2fa/setup", post(totp_setup_handler))
        .route("/passkey/register", post(passkey_register_handler))
        .with_state(state)
}

async fn login_handler(
    State(state): State<Arc<AuthState>>,
    Json(payload): Json<LoginRequest>,
) -> Json<AuthResponse> {
    match state.user_repo.find_by_email(&payload.email).await {
        Ok(Some(user)) => {
            if user.totp_enabled && payload.totp_code.is_none() {
                return Json(AuthResponse {
                    success: false,
                    token: None,
                    requires_2fa: true,
                    user: None,
                    error: Some("2FA TOTP code required".to_string()),
                });
            }

            let token = state.paseto.generate_token(&user.id, &user.role, 3600 * 24).unwrap_or_default();
            Json(AuthResponse {
                success: true,
                token: Some(token),
                requires_2fa: false,
                user: Some(user),
                error: None,
            })
        }
        _ => Json(AuthResponse {
            success: false,
            token: None,
            requires_2fa: false,
            user: None,
            error: Some("Invalid email or password".to_string()),
        }),
    }
}

async fn register_handler(
    State(state): State<Arc<AuthState>>,
    Json(payload): Json<LoginRequest>,
) -> Json<AuthResponse> {
    let new_user = UserEntity {
        id: format!("usr_{}", uuid::Uuid::new_v4().simple()),
        email: payload.email,
        password_hash: "$argon2id$v=19$m=19456,t=2,p=1$fakehash".to_string(),
        full_name: "New Developer".to_string(),
        role: "user".to_string(),
        totp_enabled: false,
        totp_secret: None,
        onboarding_completed: false,
        avatar_url: None,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    if let Err(e) = state.user_repo.save(&new_user).await {
        return Json(AuthResponse {
            success: false,
            token: None,
            requires_2fa: false,
            user: None,
            error: Some(e.to_string()),
        });
    }

    let token = state.paseto.generate_token(&new_user.id, &new_user.role, 3600 * 24).unwrap_or_default();
    Json(AuthResponse {
        success: true,
        token: Some(token),
        requires_2fa: false,
        user: Some(new_user),
        error: None,
    })
}

async fn oauth_google_handler() -> Json<Value> {
    Json(json!({
        "status": "redirect",
        "oauth_url": "https://accounts.google.com/o/oauth2/v2/auth?client_id=MOCK_CLIENT_ID&redirect_uri=http://localhost:8080/api/v1/auth/callback/google&response_type=code&scope=email%20profile"
    }))
}

async fn oauth_github_handler() -> Json<Value> {
    Json(json!({
        "status": "redirect",
        "oauth_url": "https://github.com/login/oauth/authorize?client_id=MOCK_CLIENT_ID&redirect_uri=http://localhost:8080/api/v1/auth/callback/github&scope=user:email"
    }))
}

async fn totp_setup_handler() -> Json<TotpSetupResponse> {
    let secret = totp_rs::Secret::Encoded("JBSWY3DPEHPK3PXP".to_string());
    Json(TotpSetupResponse {
        secret: secret.to_string(),
        qr_code_url: "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==".to_string(),
    })
}

async fn passkey_register_handler() -> Json<Value> {
    Json(json!({
        "success": true,
        "challenge": "f83b2a19c74d0e",
        "rp": { "name": "Ferrox SaaS Framework", "id": "localhost" },
        "user": { "id": "usr_webauthn_01", "name": "user@ferrox.dev", "displayName": "Ferrox Developer" }
    }))
}
