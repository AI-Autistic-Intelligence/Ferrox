---
sidebar_position: 2
---

# 🛡️ Rate Limiting & Throttling

To protect APIs from abuse, credential stuffing attacks, and Denial of Service (DoS), Ferrox provides `ferrox-rate-limiter`—a Redis-backed rate limiting engine implementing atomic fixed-window and token-bucket algorithms.

---

## 1. How Rate Limiting Works

`RateLimiter` increments a counter in Redis for a specific identifier (such as client IP address or authenticated User ID). If the count exceeds the max allowed limit within a time window, the request is rejected with `429 Too Many Requests`.

```
Incoming Request ---> [ Check Redis Rate Limit Key ]
                               |
               +---------------+---------------+
               |                               |
        Count <= Limit                  Count > Limit
               |                               |
               v                               v
    Proceed to Controller           Return 429 Too Many Requests
```

---

## 2. Using `RateLimiter`

```rust
use ferrox_rate_limiter::RateLimiter;
use ferrox_errors::AppError;

let limiter = RateLimiter::new("redis://127.0.0.1:6379")?;

// Allow maximum 100 requests per 60 seconds per IP
let client_ip = "192.168.1.50";
let allowed = limiter.check_limit(client_ip, 100, 60).await?;

if !allowed {
    return Err(AppError::ValidationError("Rate limit exceeded. Please wait.".into()));
}
```

---

## 3. Rate Limiter Axum Middleware

You can attach rate limiting as global or per-route middleware:

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn rate_limit_middleware(
    limiter: RateLimiter,
    req: Request<Body>,
    next: Next,
) -> Response {
    let ip = req.headers()
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown_ip");

    match limiter.check_limit(ip, 60, 60).await {
        Ok(true) => next.run(req).await,
        Ok(false) => (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({ "status": 429, "message": "Too Many Requests" }))
        ).into_response(),
        Err(_) => next.run(req).await, // Fail open if Redis drops
    }
}
```

---

## 4. ✅ Best Practices

- **Differentiate public vs authenticated endpoints**: Apply tighter rate limits (e.g. 5 req/min) on `/api/v1/auth/login` to stop brute-force attacks, while allowing higher limits (e.g. 1000 req/min) for authenticated API users.
