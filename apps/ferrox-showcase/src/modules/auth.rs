use axum::{routing::get, Router, Json, middleware};
use std::sync::Arc;
use serde::Serialize;
use ferrox_security::{PasetoAuth, auth_middleware::require_auth};

#[derive(Serialize)]
pub struct UserProfile {
    pub id: String,
    pub username: String,
    pub role: String,
    pub message: String,
}

/// Protected route that requires a valid PASETO token
pub async fn get_profile() -> Json<UserProfile> {
    // In a real app, you would extract the user_id from request extensions
    // injected by the `require_auth` middleware.
    Json(UserProfile {
        id: "usr_123".into(),
        username: "ferrox_admin".into(),
        role: "admin".into(),
        message: "You have accessed a secure route powered by Ferrox PASETO Middleware!".into(),
    })
}

pub fn router(paseto: Arc<PasetoAuth>) -> Router {
    Router::new()
        .route("/profile", get(get_profile))
        .route_layer(middleware::from_fn_with_state(paseto, require_auth))
}
