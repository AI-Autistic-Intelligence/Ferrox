use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;
use crate::persistence::{UserRepository, SavedUserSegment, RequestLogEntry};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSegmentRequest {
    pub name: String,
    pub filter_query: String,
}

pub struct AdminState {
    pub user_repo: Arc<dyn UserRepository>,
    pub request_logs: Arc<tokio::sync::Mutex<Vec<RequestLogEntry>>>,
}

pub fn router(user_repo: Arc<dyn UserRepository>) -> Router {
    let state = Arc::new(AdminState {
        user_repo,
        request_logs: Arc::new(tokio::sync::Mutex::new(vec![
            RequestLogEntry {
                correlation_id: "trace_8f2910a".to_string(),
                method: "POST".to_string(),
                path: "/api/v1/auth/login".to_string(),
                status_code: 200,
                duration_ms: 12,
                timestamp: "2026-09-10T14:00:00Z".to_string(),
                user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64)".to_string(),
            },
            RequestLogEntry {
                correlation_id: "trace_1b920c4".to_string(),
                method: "GET".to_string(),
                path: "/api/v1/users/profile/usr_admin_01".to_string(),
                status_code: 200,
                duration_ms: 4,
                timestamp: "2026-09-10T14:01:15Z".to_string(),
                user_agent: "FerroxWasmClient/0.1".to_string(),
            }
        ])),
    });

    Router::new()
        .route("/metrics", get(get_admin_metrics))
        .route("/users/segments", get(get_user_segments))
        .route("/users/segments", post(create_user_segment))
        .route("/requests/monitor", get(get_request_lifecycle_monitor))
        .with_state(state)
}

async fn get_admin_metrics(State(state): State<Arc<AdminState>>) -> Json<Value> {
    let total_users = state.user_repo.count_users().await.unwrap_or(1);
    Json(json!({
        "success": true,
        "metrics": {
            "total_users": total_users,
            "active_sessions_24h": 42,
            "mrr_euros": 1980.0,
            "conversion_rate": 8.4,
            "server_uptime_seconds": 86400,
            "cpu_usage_percent": 3.2,
            "memory_usage_mb": 48.5
        }
    }))
}

async fn get_user_segments() -> Json<Value> {
    let mock_segments = vec![
        SavedUserSegment {
            id: "seg_01".to_string(),
            name: "Active Admins".to_string(),
            filter_query: "role:admin AND onboarding:true".to_string(),
            created_by: "usr_admin_01".to_string(),
        },
        SavedUserSegment {
            id: "seg_02".to_string(),
            name: "New Registrations (7 Days)".to_string(),
            filter_query: "created_at >= 7d".to_string(),
            created_by: "usr_admin_01".to_string(),
        }
    ];

    Json(json!({ "success": true, "segments": mock_segments }))
}

async fn create_user_segment(
    Json(payload): Json<CreateSegmentRequest>,
) -> Json<Value> {
    let segment = SavedUserSegment {
        id: format!("seg_{}", uuid::Uuid::new_v4().simple()),
        name: payload.name,
        filter_query: payload.filter_query,
        created_by: "usr_admin_01".to_string(),
    };
    Json(json!({ "success": true, "segment": segment }))
}

async fn get_request_lifecycle_monitor(State(state): State<Arc<AdminState>>) -> Json<Value> {
    let logs = state.request_logs.lock().await;
    Json(json!({
        "success": true,
        "service": "Ferrox SaaS Backend",
        "tracing_engine": "W3C Correlation ID + OpenTelemetry Sentry",
        "requests": *logs
    }))
}
