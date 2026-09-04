use reqwest::Client;
use serde::Serialize;
use yalc_errors::AppError;

#[derive(Clone)]
pub struct SlackClient {
    webhook_url: String,
    http_client: Client,
}

#[derive(Serialize)]
struct SlackMessage {
    text: String,
}

impl SlackClient {
    pub fn new(webhook_url: &str) -> Self {
        Self {
            webhook_url: webhook_url.to_string(),
            http_client: Client::new(),
        }
    }

    /// Sends a simple text message to the configured Slack webhook
    pub async fn send_alert(&self, message: &str) -> Result<(), AppError> {
        let payload = SlackMessage {
            text: message.to_string(),
        };

        let response = self.http_client
            .post(&self.webhook_url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::InternalServerError(
                format!("Slack API Error: {}", error_text).into()
            ));
        }

        Ok(())
    }
}
