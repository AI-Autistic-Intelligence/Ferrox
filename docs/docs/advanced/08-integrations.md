---
sidebar_position: 8
---

# Third-Party Integrations (yalc-integrations)

In an Enterprise environment, hardcoding external APIs (like Stripe, SendGrid, or Slack) directly into your business logic creates extreme technical debt. It locks your software to a specific vendor and makes unit testing a nightmare.

`Rust-YALC` solves this via the **Dependency Inversion Principle**, providing unified integration abstractions.

## Core Abstractions

The `yalc-integrations` crate exposes generic Traits for common business needs:

### Payment Providers
```rust
#[async_trait]
pub trait PaymentProvider: Send + Sync {
    async fn charge(&self, amount: i64, currency: &str, source_id: &str) -> Result<String, AppError>;
    async fn refund(&self, transaction_id: &str) -> Result<(), AppError>;
}
```

### Notification Providers
```rust
#[async_trait]
pub trait NotificationProvider: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), AppError>;
    async fn send_chat(&self, channel_id: &str, message: &str) -> Result<(), AppError>;
}
```

## Supported Official Adapters

Instead of building your own HTTP calls, `Rust-YALC` ships with official adapters that implement these traits. They are located in `crates/integrations/`:

1. **`yalc-payments-stripe`**: Implements `PaymentProvider` using the Stripe API.
2. **`yalc-notifications-slack`**: Implements `NotificationProvider` using Slack Webhooks.
*(More to come, such as AWS SES, Twilio, Google Pay, etc.)*

## Usage (Dependency Injection)

To use these in your application, inject the generic `Arc<dyn PaymentProvider>` into your application state:

```rust
use std::sync::Arc;
use yalc_integrations::PaymentProvider;
use yalc_payments_stripe::StripeAdapter;

struct AppState {
    pub payment_gateway: Arc<dyn PaymentProvider>,
}

// In main.rs
let state = AppState {
    payment_gateway: Arc::new(StripeAdapter::new("sk_test_12345")),
};
```

Your service code now simply calls `state.payment_gateway.charge(...)`. If tomorrow you want to switch from Stripe to Adyen, you write an `AdyenAdapter` and swap it in `main.rs` without touching a single line of your business logic!
