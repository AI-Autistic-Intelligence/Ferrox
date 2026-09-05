---
sidebar_position: 3
---

# 🧅 Middlewares

Middlewares are functions that have access to the request object, the response object, and the `next` middleware function in the application’s request-response cycle.

In Ferrox, Middlewares are executed **before** the request reaches your Guards or Controllers. They are the perfect place to implement things like:
- Execution tracing and logging
- Request ID injection
- API Gateway Token Translation (See [Zero-Trust Security](../security/jwt))

## High-Level Example

Here is how you can write a simple middleware that measures how long a request takes to process.

```rust
use axum::{
    http::Request,
    middleware::{self, Next},
    response::Response,
    routing::get,
    Router,
};
use std::time::Instant;

// 1. Define the Middleware function
async fn timing_middleware<B>(
    req: Request<B>,
    next: Next,
) -> Response {
    let start = Instant::now();

    // Pass control to the next layer (Guards -> Controller)
    let response = next.run(req).await;

    let latency = start.elapsed();
    println!("Request took {} ms", latency.as_millis());

    response
}

// 2. Attach it to your router
pub fn app_router() -> Router {
    Router::new()
        .route("/fast-route", get(|| async { "Hello" }))
        .layer(middleware::from_fn(timing_middleware)) // <--- Attached globally
}
```

## Low-Level Internal Details

Ferrox uses the `tower::Service` abstraction under the hood for its middleware layer. 

Unlike Extractors (which parse data and can only fail or succeed), a Middleware wraps the entire request-response execution. This means a Middleware can:
1. **Mutate the Request**: You can inject HTTP headers into `req.headers_mut()` before calling `next.run(req)`. This is exactly how Ferrox propagates the `X-User-Id` downstream in a microservice architecture.
2. **Mutate the Response**: You can read the `response` after the controller finishes, and append custom HTTP headers (like CORS or Rate Limiting counters) before the client receives it.
3. **Short-circuit**: If a Middleware returns a `Response` directly without calling `next.run()`, the request is aborted early (useful for IP blocklisting).
