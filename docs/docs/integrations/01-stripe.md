---
sidebar_position: 1
---

# Stripe

Rust-YALC integrates natively with Stripe for B2B billing and subscription management via `integrations/yalc-stripe`.

## Core Features
- Automatic checkout session creation
- Webhook signature verification and handling
- Subscription lifecycle synchronization

## Example

```rust
use yalc_stripe::StripeClient;

let stripe = StripeClient::new("sk_test_123");
let session_url = stripe.create_checkout("price_123", "cus_456").await?;
```
