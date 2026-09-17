use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationPayload {
    pub target_segment_id: Option<String>,
    pub title: String,
    pub body: String,
    pub action_url: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route("/list", get(list_notifications_handler))
        .route("/broadcast", post(broadcast_notification_handler))
        .route("/webhooks/register", post(register_webhook_handler))
}

async fn list_notifications_handler() -> Json<Value> {
    Json(json!({
        "success": true,
        "notifications": [
            {
                "id": "notif_01",
                "title": "Nuovo aggiornamento Ferrox v0.1.2",
                "body": "Il tuo backend è stato aggiornato con il tracciamento Sentry W3C.",
                "read": false,
                "created_at": "2026-09-10T14:10:00Z"
            }
        ]
    }))
}

async fn broadcast_notification_handler(Json(payload): Json<NotificationPayload>) -> Json<Value> {
    tracing::info!("🔔 Broadcasting notification '{}' to segment {:?}", payload.title, payload.target_segment_id);
    Json(json!({
        "success": true,
        "dispatched_count": 150,
        "channels": ["in_app", "web_push", "websocket"]
    }))
}

async fn register_webhook_handler(Json(payload): Json<Value>) -> Json<Value> {
    let url = payload["url"].as_str().unwrap_or("https://my-webhook-listener.com");
    let secret = "whsec_HMAC_SHA256_ferrox_secret_key";
    Json(json!({
        "success": true,
        "webhook_id": "whk_88190c",
        "target_url": url,
        "hmac_secret": secret
    }))
}
