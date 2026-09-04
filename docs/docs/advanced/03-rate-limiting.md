---
sidebar_position: 3
---

# Rate Limiting

The `yalc-rate-limiter` uses Redis to track IPs and tokens, applying distributed rate limits across all running pods in the Kubernetes cluster.

## Axum Middleware

You can attach the rate limiter directly to an Axum Router as a layer.

```rust
use yalc_rate_limiter::RateLimitLayer;

let app = Router::new()
    .route("/api/v1/data", get(handler))
    .layer(RateLimitLayer::new(100, std::time::Duration::from_secs(60)));
```
