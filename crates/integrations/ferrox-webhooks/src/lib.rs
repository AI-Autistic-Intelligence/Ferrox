use ferrox_errors::AppError;
use hmac::{Hmac, Mac};
use reqwest::Client;
use serde::Serialize;
use sha2::Sha256;
use tracing::{error, info};

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct WebhookSender {
    client: Client,
    secret_key: String,
}

impl WebhookSender {
    pub fn new(secret_key: &str) -> Self {
        Self {
            client: Client::new(),
            secret_key: secret_key.to_string(),
        }
    }

    /// Generates HMAC SHA-256 signature for the given JSON payload
    pub fn sign_payload(&self, payload: &str) -> Result<String, AppError> {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .map_err(|e| AppError::InternalError(format!("HMAC Error: {}", e)))?;
        mac.update(payload.as_bytes());
        let result = mac.finalize();
        Ok(hex::encode(result.into_bytes()))
    }

    /// Dispatches a webhook to a target URL in the background (fire-and-forget).
    /// Generates a Zero-Trust cryptographic signature and injects it into headers.
    pub fn dispatch<T: Serialize + Send + 'static>(&self, url: &str, payload: T) {
        let sender = self.clone();
        let url_owned = url.to_string();

        tokio::spawn(async move {
            let json_payload = match serde_json::to_string(&payload) {
                Ok(json) => json,
                Err(e) => {
                    error!("Failed to serialize webhook payload: {}", e);
                    return;
                }
            };

            let signature = match sender.sign_payload(&json_payload) {
                Ok(sig) => sig,
                Err(e) => {
                    error!("Failed to sign webhook payload: {}", e);
                    return;
                }
            };

            match sender.client.post(&url_owned)
                .header("Content-Type", "application/json")
                .header("x-ferrox-signature", signature)
                .body(json_payload)
                .send()
                .await
            {
                Ok(response) => {
                    if response.status().is_success() {
                        info!("✅ Webhook successfully delivered to {}", url_owned);
                    } else {
                        error!("❌ Webhook to {} returned status: {}", url_owned, response.status());
                        // Future: Dispatch to Retry Queue (ferrox-jobs)
                    }
                }
                Err(e) => {
                    error!("❌ Webhook delivery to {} failed completely: {}", url_owned, e);
                    // Future: Dispatch to Retry Queue
                }
            }
        });
    }
}
