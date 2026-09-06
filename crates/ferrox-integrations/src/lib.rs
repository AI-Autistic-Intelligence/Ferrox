//! # Ferrox Integrations (`ferrox-integrations`)
//!
//! `ferrox-integrations` defines unified trait abstractions for external third-party service integrations, including `PaymentProvider`,
//! `NotificationProvider`, `MailerProvider`, and `StorageProvider`.
//!
//! ## Key Features
//! - 🔌 **Standardized Provider Traits**: Common interfaces across Stripe, Slack, SendGrid, and AWS.
//! - 🧪 **Test Mocking**: Swap external API calls with mock implementations during automated testing.

use async_trait::async_trait;
use ferrox_errors::AppError;

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
    println!("ferrox-integrations initialized: Third-Party API abstractions ready.");
}