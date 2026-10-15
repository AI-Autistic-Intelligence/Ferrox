use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::config::AppConfig;

#[derive(Debug, Serialize, Deserialize)]
pub struct LivenessResponse {
    pub status: &'static str,
    pub service: &'static str,
    pub timestamp_utc: String,
    pub framework: &'static str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadinessResponse {
    pub status: &'static str,
    pub timestamp_utc: String,
    pub checks: ReadinessChecks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadinessChecks {
    pub database_engine: String,
    pub database_status: &'static str,
    pub redis_status: &'static str,
    pub paseto_security: &'static str,
}

pub fn router(config: AppConfig) -> Router {
    let cfg = config.clone();

    Router::new()
        .route("/", get(|| async {
            Json(json!({
                "status": "UP",
                "service": "ferrox-saas-backend",
                "framework": "Ferrox Framework v0.8.0",
                "timestamp": chrono::Utc::now().to_rfc3339()
            }))
        }))
        .route("/live", get(|| async {
            Json(LivenessResponse {
                status: "UP",
                service: "ferrox-saas-backend",
                timestamp_utc: chrono::Utc::now().to_rfc3339(),
                framework: "Ferrox Enterprise Rust",
            })
        }))
        .route("/ready", get(move || {
            let db_engine = format!("{:?}", cfg.database_engine);
            async move {
                // In production, execute real pool ping (e.g. sqlx/sea-orm or redis ping)
                Json(ReadinessResponse {
                    status: "READY",
                    timestamp_utc: chrono::Utc::now().to_rfc3339(),
                    checks: ReadinessChecks {
                        database_engine: db_engine,
                        database_status: "HEALTHY",
                        redis_status: "HEALTHY",
                        paseto_security: "ARMED",
                    },
                })
            }
        }))
}
