use axum::{routing::get, Router, Json};
use serde::Serialize;
use ferrox_utils::date::now_utc;

#[derive(Serialize)]
pub struct SystemStatus {
    pub status: String,
    pub timestamp: String,
    pub framework: String,
}

pub async fn health_check() -> Json<SystemStatus> {
    Json(SystemStatus {
        status: "operational".to_string(),
        timestamp: now_utc().to_rfc3339(),
        framework: "Ferrox Showcase".to_string(),
    })
}

pub fn router() -> Router {
    Router::new().route("/health", get(health_check))
}
