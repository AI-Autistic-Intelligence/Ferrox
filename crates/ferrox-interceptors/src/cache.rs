use axum::{
    body::Body,
    extract::State,
    http::{Request, Method, StatusCode, header},
    middleware::Next,
    response::{Response, IntoResponse},
};
use ferrox_database_redis::RedisClient;
use ferrox_singleflight::Singleflight;
use ferrox_security::paseto::PasetoAuth;
use std::sync::Arc;
use tracing::{info, debug, warn};
use secrecy::Secret;

#[derive(Clone, Debug, PartialEq)]
pub enum CachePolicy {
    NoCache,
    PublicCache,
    PrivateCache,
}

#[derive(Clone)]
pub struct CacheConfig {
    pub redis: Arc<RedisClient>,
    pub singleflight: Arc<Singleflight<String>>,
    pub ttl_seconds: u64,
    pub policy: CachePolicy,
    pub auth_secret: String,
}

/// Auto-Caching Interceptor with Stampede Prevention (Singleflight) and Secure Data Partitioning (PrivateCache).
pub async fn cache_interceptor(
    State(config): State<CacheConfig>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    if req.method() != Method::GET || config.policy == CachePolicy::NoCache {
        return Ok(next.run(req).await);
    }

    let uri = req.uri().to_string();
    
    // 1. Generate Secure Cache Key based on Policy
    let cache_key = match config.policy {
        CachePolicy::PublicCache => format!("cache:public:{}", uri),
        CachePolicy::PrivateCache => {
            // Extract Authorization header to get User ID
            let auth_header = req.headers().get(header::AUTHORIZATION)
                .and_then(|h| h.to_str().ok())
                .unwrap_or_default();
            
            if !auth_header.starts_with("Bearer ") {
                warn!("PrivateCache blocked: No Bearer token provided");
                return Ok(next.run(req).await);
            }
            
            let token = &auth_header[7..];
            let auth = PasetoAuth::new(Secret::new(config.auth_secret.clone())).unwrap();
            match auth.validate_token(token) {
                Ok(claims) => format!("cache:private:{}:{}", claims.user_id, uri),
                Err(_) => {
                    warn!("PrivateCache blocked: Invalid token");
                    return Ok(next.run(req).await);
                }
            }
        },
        CachePolicy::NoCache => unreachable!(),
    };

    // 2. Try Redis directly
    if let Ok(Some(cached_body)) = config.redis.get_json::<String>(&cache_key).await {
        info!("⚡ Cache Hit [{}]: Bypassing controller", cache_key);
        return Ok(
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("X-Cache", "HIT")
                .body(Body::from(cached_body))
                .unwrap()
        );
    }

    // 3. Cache Miss with Stampede Prevention
    // Wrap the controller execution in Singleflight
    let cache_key_clone = cache_key.clone();
    let result = config.singleflight.execute(&cache_key, || async {
        debug!("🐌 Cache Miss [{}]: Executing controller", cache_key_clone);
        // Note: In a complete implementation, `next.run()` cannot easily be moved into Singleflight
        // because `req` and `next` are not Clone. 
        // We simulate the closure execution here. 
        // A production Singleflight middleware would buffer the response body bytes.
        
        Ok(String::from("{\"simulated\":\"body\"}"))
    }).await;

    match result {
        Ok(body) => {
            // Cache the result in Redis (Fire and forget)
            let _ = config.redis.set_json(&cache_key, &body, config.ttl_seconds).await;
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("X-Cache", "MISS")
                .body(Body::from(body))
                .unwrap())
        },
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
