use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: String,
    pub name: String,
    pub subject: String,
    pub html_body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMailRequest {
    pub recipient: String,
    pub template_id: String,
    pub variables: serde_json::Value,
}

pub fn router() -> Router {
    Router::new()
        .route("/templates", get(list_email_templates))
        .route("/send", post(send_email_handler))
}

async fn list_email_templates() -> Json<Value> {
    let templates = vec![
        EmailTemplate {
            id: "tpl_welcome".to_string(),
            name: "Welcome Onboarding Email".to_string(),
            subject: "Benvenuto su Ferrox SaaS Framework! 🚀".to_string(),
            html_body: "<h1>Ciao {{name}}!</h1><p>Grazie per esserti iscritto. Clicca qui per completare l'onboarding.</p>".to_string(),
        },
        EmailTemplate {
            id: "tpl_reset_password".to_string(),
            name: "Password Reset Prompt".to_string(),
            subject: "Ripristina la tua password Ferrox".to_string(),
            html_body: "<p>Hai richiesto il reset della password. Usa questo codice: <strong>{{code}}</strong></p>".to_string(),
        }
    ];

    Json(json!({ "success": true, "templates": templates }))
}

async fn send_email_handler(Json(payload): Json<SendMailRequest>) -> Json<Value> {
    tracing::info!("📧 Sending transactional mail to {} using template {}", payload.recipient, payload.template_id);
    Json(json!({
        "success": true,
        "message_id": format!("msg_{}", uuid::Uuid::new_v4().simple()),
        "status": "queued_smtp"
    }))
}
