use axum::{routing::get, Router, Json};
use serde::Serialize;
use ferrox_utils::date::now_utc;

#[derive(Serialize)]
pub struct SystemStatus {
    pub status: String,
    pub timestamp: String,
    pub framework: String,
}

#[derive(Serialize)]
pub struct HealthProbeResponse {
    pub status: &'static str,
    pub timestamp: String,
    pub service: &'static str,
    pub checks: Option<ReadinessChecks>,
}

#[derive(Serialize)]
pub struct ReadinessChecks {
    pub security: &'static str,
    pub memory: &'static str,
    pub transport: &'static str,
}

pub async fn health_check() -> Json<SystemStatus> {
    Json(SystemStatus {
        status: "operational".to_string(),
        timestamp: now_utc().to_rfc3339(),
        framework: "Ferrox Showcase".to_string(),
    })
}

pub async fn liveness() -> Json<HealthProbeResponse> {
    Json(HealthProbeResponse {
        status: "UP",
        timestamp: now_utc().to_rfc3339(),
        service: "ferrox-showcase",
        checks: None,
    })
}

pub async fn readiness() -> Json<HealthProbeResponse> {
    Json(HealthProbeResponse {
        status: "READY",
        timestamp: now_utc().to_rfc3339(),
        service: "ferrox-showcase",
        checks: Some(ReadinessChecks {
            security: "PASETO_ACTIVE",
            memory: "HEALTHY",
            transport: "AXUM_HTTP_READY",
        }),
    })
}

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/health/live", get(liveness))
        .route("/health/ready", get(readiness))
}
