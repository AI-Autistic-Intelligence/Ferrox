use stripe::{Client, Webhook, WebhookEvent};
use yalc_errors::AppError;

#[derive(Clone)]
pub struct StripeClient {
    pub client: Client,
    pub webhook_secret: String,
}

impl StripeClient {
    pub fn new(secret_key: &str, webhook_secret: &str) -> Self {
        Self {
            client: Client::new(secret_key),
            webhook_secret: webhook_secret.to_string(),
        }
    }

    /// Verifies the cryptographic signature of an incoming Stripe webhook
    /// and parses it into a strongly typed WebhookEvent.
    pub fn verify_webhook(&self, payload: &str, stripe_signature_header: &str) -> Result<WebhookEvent, AppError> {
        Webhook::construct_event(payload, stripe_signature_header, &self.webhook_secret)
            .map_err(|e| AppError::Unauthorized(format!("Invalid Stripe Webhook Signature: {}", e)))
    }
}
