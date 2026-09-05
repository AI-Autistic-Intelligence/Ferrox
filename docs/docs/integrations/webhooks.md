---
sidebar_position: 5
---

# 🔗 Outgoing Webhook Dispatcher

When your platform allows external developers to subscribe to API events (e.g. `order.created`, `invoice.paid`), sending reliable outgoing webhooks requires cryptographic signing (HMAC-SHA256) and automated retry logic with exponential backoff.

`integrations/ferrox-webhooks` manages outgoing webhook dispatching.

---

## 1. Dispatching Signed Outgoing Webhooks

```rust
use ferrox_webhooks::WebhookDispatcher;
use ferrox_errors::AppError;

let dispatcher = WebhookDispatcher::new();

let target_url = "https://partner.com/webhooks/receiver";
let secret = "whsec_partner_secret_key";
let payload = serde_json::json!({
    "event": "order.created",
    "order_id": 42
});

// Signs payload with HMAC-SHA256 and dispatches HTTP POST with X-Ferrox-Signature header
dispatcher.dispatch(target_url, secret, &payload).await?;
```

---

## 2. Webhook Signature Header (`X-Ferrox-Signature`)

`WebhookDispatcher` automatically generates an HMAC-SHA256 hex signature attached as an HTTP header:

```text
X-Ferrox-Signature: t=1788602400,v1=9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08
```

---

## 3. Retries with Exponential Backoff

If the recipient server returns `503 Service Unavailable` or times out, `WebhookDispatcher` retries transmission automatically across 3 backoff intervals (1s, 5s, 25s).
