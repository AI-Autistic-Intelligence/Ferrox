use async_trait::async_trait;
use yalc_errors::AppError;

/// An abstraction for any Payment Gateway (Stripe, PayPal, Adyen, etc.)
#[async_trait]
pub trait PaymentProvider: Send + Sync {
    /// Charge a customer. Returns the external transaction ID.
    async fn charge(&self, amount: i64, currency: &str, source_id: &str) -> Result<String, AppError>;
    
    /// Refund a previously executed transaction.
    async fn refund(&self, transaction_id: &str) -> Result<(), AppError>;
}

/// An abstraction for any Notification delivery service (SendGrid, Slack, Twilio, etc.)
#[async_trait]
pub trait NotificationProvider: Send + Sync {
    /// Send an email
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), AppError>;
    
    /// Send a Chat Message (Slack/Teams/Discord)
    async fn send_chat(&self, channel_id: &str, message: &str) -> Result<(), AppError>;
}

pub fn setup() {
    println!("yalc-integrations initialized: Third-Party API abstractions ready.");
}
