use redis::AsyncCommands;
use ferrox_errors::AppError;

#[derive(Clone)]
pub struct RateLimiter {
    redis_client: redis::Client,
}

impl RateLimiter {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let redis_client = redis::Client::open(redis_url)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        Ok(Self { redis_client })
    }

    /// Basic implementation of a fixed window rate limiter using Redis
    pub async fn check_limit(&self, identifier: &str, limit: i64, window_secs: u64) -> Result<bool, AppError> {
        let mut con = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        let key = format!("rate_limit:{}", identifier);
        
        // Atomically increment and set expiry if it's a new key
        let (count, _): (i64, ()) = redis::pipe()
            .atomic()
            .incr(&key, 1)
            .expire(&key, window_secs.try_into().unwrap_or(60))
            .ignore()
            .query_async(&mut con)
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        Ok(count <= limit)
    }
}
