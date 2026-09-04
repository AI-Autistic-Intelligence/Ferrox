use axum::{
    routing::{get, post},
    Router, Json, middleware,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;
use secrecy::Secret;

// Import our framework components
use ferrox_security::{PasetoAuth, auth_middleware::{require_auth, require_role}};
use ferrox_errors::AppError;
use ferrox_utils::date::now_utc;

#[derive(Serialize)]
struct SystemStatus {
    status: String,
    timestamp: String,
    version: String,
}

#[derive(Serialize)]
struct UserData {
    message: String,
    secret_balance: u64,
}

async fn health_check() -> Json<SystemStatus> {
    Json(SystemStatus {
        status: "operational".to_string(),
        timestamp: now_utc().to_rfc3339(),
        version: "1.0.0".to_string(),
    })
}

/// Protected by require_auth middleware
async fn get_user_profile() -> Json<UserData> {
    // In a real app, this reads the `x-ferrox-user-id` header
    // injected by the API Gateway middleware.
    Json(UserData {
        message: "Welcome to the secure zone!".into(),
        secret_balance: 10_000,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    info!("🚀 Booting Ferrox Showcase App...");

    // 1. Boot Config & Security
    let secret = Secret::new("my_super_secret_key_that_is_32_bytes_long_12345678901234567890".to_string());
    let paseto_auth = Arc::new(PasetoAuth::new(secret).unwrap());

    // 2. Setup Routes
    // Public routes
    let public_routes = Router::new()
        .route("/health", get(health_check));

    // Protected routes (API Gateway Pattern)
    let protected_routes = Router::new()
        .route("/profile", get(get_user_profile))
        // Protect with PASETO JWT Middleware
        .route_layer(middleware::from_fn_with_state(
            paseto_auth.clone(),
            require_auth,
        ));

    // Combine
    let app = Router::new()
        .merge(public_routes)
        .merge(protected_routes);

    // 3. Start Server
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    info!("✅ Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await?;

    Ok(())
}
