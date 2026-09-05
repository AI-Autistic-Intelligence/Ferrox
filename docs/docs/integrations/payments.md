---
sidebar_position: 2
---

# 💳 Payment Gateways (Stripe & Google Pay)

Integrating e-commerce payments requires secure checkout session handling and cryptographic signature verification of incoming payment webhooks.

Ferrox provides `integrations/ferrox-payments-stripe` and `integrations/ferrox-payments-google`.

---

## 1. Stripe Integration

### Creating Stripe Checkout Sessions

```rust
use ferrox_payments_stripe::StripeClient;
use ferrox_errors::AppError;

let stripe = StripeClient::new("sk_test_51...your_secret_key");

// Create checkout session for product
let checkout_url = stripe.create_checkout_session(
    "price_1N...",
    "https://mycompany.com/success",
    "https://mycompany.com/cancel"
).await?;
```

### Verifying Stripe Webhook Signatures

To prevent spoofed payment events, verify the `Stripe-Signature` header using your Webhook Secret:

```rust
use axum::{extract::HeaderMap, http::StatusCode};
use ferrox_payments_stripe::verify_webhook_signature;

pub async fn stripe_webhook_handler(
    headers: HeaderMap,
    body: String,
) -> Result<StatusCode, AppError> {
    let sig = headers.get("Stripe-Signature")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing signature header".into()))?;

    let webhook_secret = "whsec_...";
    let valid = verify_webhook_signature(&body, sig, webhook_secret)?;

    if valid {
        println!("✅ Payment Webhook verified successfully!");
        Ok(StatusCode::OK)
    } else {
        Err(AppError::Unauthorized("Invalid signature".into()))
    }
}
```

---

## 2. Google Pay & In-App Purchases

`integrations/ferrox-payments-google` provides token verification for Google Pay payment tokens and Google Play Android in-app purchase receipts.
