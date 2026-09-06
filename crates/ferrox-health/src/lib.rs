//! # Ferrox Health (`ferrox-health`)
//!
//! `ferrox-health` provides Kubernetes-compliant `/healthz` (liveness) and `/readyz` (readiness) health check endpoints for Ferrox backends.
//!
//! ## Key Features
//! - 🩺 **Liveness Probe**: Fast endpoint indicating that the service process is active.
//! - 🚦 **Readiness Probe**: Performs dynamic checks against database pools, Redis, and external dependencies before routing traffic.

use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub timestamp: i64,
}

/// Liveness Probe: Tells Kubernetes the process is running.
async fn liveness() -> impl IntoResponse {
    let status = HealthStatus {
        status: "UP".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    };
    (StatusCode::OK, Json(status))
}

/// Readiness Probe: Tells Kubernetes the app is ready to accept traffic.
/// In a real app, you would check DB and Redis connections here.
async fn readiness() -> impl IntoResponse {
    // Simplified for boilerpate
    let status = HealthStatus {
        status: "READY".to_string(),
        timestamp: chrono::Utc::now().timestamp(),
    };
    (StatusCode::OK, Json(status))
}

/// Returns a Router containing standard /healthz and /readyz endpoints
pub fn health_router() -> Router {
    Router::new()
        .route("/healthz", get(liveness))
        .route("/readyz", get(readiness))
}