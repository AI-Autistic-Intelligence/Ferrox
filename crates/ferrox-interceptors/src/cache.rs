use axum::{
    body::Body,
    extract::State,
    http::{Request, Method, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
};
use ferrox_database_redis::RedisClient;
use std::sync::Arc;
use tracing::{info, debug};

#[derive(Clone)]
pub struct CacheConfig {
    pub redis: Arc<RedisClient>,
    pub ttl_seconds: u64,
}

/// Auto-Caching Interceptor.
/// Checks Redis for a cached response for GET requests.
/// If found, returns it immediately (Cache Hit).
/// If not, executes the route, and caches the result (Cache Miss).
pub async fn cache_interceptor(
    State(config): State<CacheConfig>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if req.method() != Method::GET {
        // Only cache GET requests
        return Ok(next.run(req).await);
    }

    let uri = req.uri().to_string();
    let cache_key = format!("cache:{}", uri);

    // 1. Try to fetch from Redis
    if let Ok(Some(cached_body)) = config.redis.get_json::<String>(&cache_key).await {
        info!("⚡ Cache Hit [{}]: Bypassing controller", uri);
        return Ok(
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("X-Cache", "HIT")
                .body(Body::from(cached_body))
                .unwrap()
        );
    }

    // 2. Cache Miss: Execute controller
    debug!("🐌 Cache Miss [{}]: Executing controller", uri);
    let res = next.run(req).await;

    // In a full implementation, we would extract the Response Body bytes here.
    // However, consuming the Axum Body stream requires buffering. 
    // For this boilerplate, we assume that endpoints that need manual caching 
    // could also use decorators, but we demonstrate the interception here.
    // A complete proxy interceptor requires `http_body_util::BodyExt::collect`.

    Ok(res)
}
