//! # Ferrox Interceptors (`ferrox-interceptors`)
//!
//! `ferrox-interceptors` provides request/response lifecycle hooks and middleware interceptors (e.g. `CacheInterceptor`, `TimingInterceptor`)
//! that inspect, transform, or cache HTTP responses.
//!
//! ## Key Features
//! - 🔄 **`Interceptor` Trait**: Modify requests before route execution and transform responses afterwards.
//! - ⚡ **`CacheInterceptor`**: Automated response caching based on HTTP headers and cache keys.

use axum::{
    body::Body,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use tracing::info;

pub mod cache;

/// Interceptor Middleware that logs the execution time of a request
pub async fn logging_interceptor(
    req: Request<Body>,
    next: Next,
) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    let start = Instant::now();
    
    // Execute the next middleware/handler in the chain
    let res = next.run(req).await;
    
    let duration = start.elapsed();
    let status = res.status();
    
    info!(
        "[{}] {} {} - {:?}",
        status.as_u16(),
        method,
        uri,
        duration
    );
    
    res
}

pub fn setup() {
    println!("ferrox-interceptors initialized: Global middleware intercepts active.");
}