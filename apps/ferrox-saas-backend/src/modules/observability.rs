use axum::{
    body::Body,
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use std::time::Instant;

pub async fn trace_request_lifecycle_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let correlation_id = req
        .headers()
        .get("x-correlation-id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("trace_{}", uuid::Uuid::new_v4().simple()));

    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let mut response = next.run(req).await;

    let duration_ms = start.elapsed().as_millis();
    let status = response.status().as_u16();

    tracing::info!(
        "[W3C Trace: {}] {} {} -> Status {} ({:?}ms)",
        correlation_id, method, path, status, duration_ms
    );

    if let Ok(hdr) = HeaderValue::from_str(&correlation_id) {
        response.headers_mut().insert("x-correlation-id", hdr);
    }

    response
}
