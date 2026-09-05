---
sidebar_position: 4
---

# 🕵️ OpenTelemetry & Distributed Tracing

In a distributed microservice network, a single user HTTP request might pass through an API Gateway, an Authentication service, an Inventory service, and a Payments database.

**Distributed Tracing** tracks the lifecycle of a request across all service boundaries by injecting and propagating unique trace identifiers (`traceparent` and `X-Correlation-ID` headers).

`ferrox-tracing` integrates OpenTelemetry and Jaeger/Zipkin exporters.

---

## 1. Request Correlation ID Middleware

`ferrox-tracing` provides middleware that automatically extracts or generates a unique correlation ID for every incoming request:

```rust
use axum::{
    body::Body,
    http::{Request, HeaderValue},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub async fn correlation_id_middleware(
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let correlation_id = req.headers()
        .get("X-Correlation-ID")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Re-inject correlation header into request
    req.headers_mut().insert(
        "X-Correlation-ID",
        HeaderValue::from_str(&correlation_id).unwrap(),
    );

    let mut response = next.run(req).await;

    // Attach correlation ID to response headers
    response.headers_mut().insert(
        "X-Correlation-ID",
        HeaderValue::from_str(&correlation_id).unwrap(),
    );

    response
}
```

---

## 2. Propagating Correlation IDs across Microservice Calls

When calling downstream microservices via `reqwest`, attach the `X-Correlation-ID` header:

```rust
pub async fn call_downstream_service(correlation_id: &str) {
    let client = reqwest::Client::new();
    let res = client.get("http://payments-service/charge")
        .header("X-Correlation-ID", correlation_id)
        .send()
        .await;
}
```

---

## 3. Visualizing Traces in Jaeger / Grafana Tempo

Distributed traces are collected by OpenTelemetry collectors and rendered in Jaeger UI, visualizing call stacks, duration waterfalls, and exact bottlenecks.
