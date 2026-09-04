---
sidebar_position: 13
---

# Auto-Caching & Zero-Trust Webhooks

`Ferrox` includes high-performance caching for HTTP APIs and a secure background Webhook dispatcher.

## 1. Automatic Redis Cache Interceptor

Instead of manually checking Redis inside your controllers, you can attach the `cache_interceptor` to any Axum Router.

```rust
use axum::{Router, middleware::from_fn_with_state};
use ferrox_interceptors::cache::{cache_interceptor, CacheConfig};
use std::sync::Arc;

let config = CacheConfig {
    redis: Arc::new(redis_client),
    ttl_seconds: 60,
};

let app = Router::new()
    // Routes ...
    .layer(from_fn_with_state(config, cache_interceptor));
```

**How it works:**
1. When a `GET` request arrives, the interceptor checks Redis.
2. If it's a **Cache Hit**, the response is returned instantly (<1ms), entirely bypassing the controller logic and SQL/Mongo database.
3. If it's a **Cache Miss**, the controller is executed. The result is then intercepted and stored in Redis with the configured TTL automatically.

## 2. Zero-Trust Webhooks

When dispatching webhooks to your users' servers, it is crucial to guarantee authenticity so the receiver can mathematically verify the payload originated from your application and hasn't been tampered with.

`ferrox-webhooks` solves this by automatically signing the payload using HMAC SHA-256.

```rust
use ferrox_webhooks::WebhookSender;
use serde::Serialize;

#[derive(Serialize)]
struct PaymentSuccess {
    user_id: String,
    amount: u32,
}

// 1. Initialize with your cryptographic secret
let webhook = WebhookSender::new("super_secret_key_123");

let payload = PaymentSuccess {
    user_id: "usr_1".to_string(),
    amount: 5000,
};

// 2. Dispatch in background (Fire-and-Forget)
// The payload is JSON stringified, signed with HMAC-SHA256, and sent 
// asynchronously without blocking your current HTTP response!
webhook.dispatch("https://client.example.com/webhooks", payload);
```

The receiver will find the cryptographic signature in the `x-ferrox-signature` HTTP header.
