//! # Ferrox Database Redis (`ferrox-database-redis`)
//!
//! `ferrox-database-redis` provides Redis integration for caching, session storage, distributed rate limiting, and real-time Pub/Sub.
//!
//! ## Key Features
//! - ⚡ **Multiplexed Connection Pool**: Efficient async Redis client backed by `bb8` or `redis-rs`.
//! - 🔑 **Cache Helper Operations**: Strongly typed `get_json`, `set_json`, `expire`, and `del` primitives.
//! - 📻 **Pub/Sub Subscriptions**: Asynchronous message receiver streams.

use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use ferrox_errors::AppError;

pub type RedisPool = Pool<RedisConnectionManager>;

#[derive(Clone)]
pub struct RedisClient {
    pub pool: RedisPool,
}

impl RedisClient {
    pub async fn connect(connection_string: &str) -> Result<Self, AppError> {
        let manager = RedisConnectionManager::new(connection_string)
            .map_err(|e| AppError::DatabaseError(format!("Redis URL Parse Error: {}", e)))?;
        
        // Setup an async connection pool
        let pool = Pool::builder()
            .max_size(15) // Enough for most web backends
            .build(manager)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Redis Pool Error: {}", e)))?;

        // Test connection
        let mut conn = pool.get().await
            .map_err(|e| AppError::DatabaseError(format!("Redis Ping Failed: {}", e)))?;
            
        let _: String = redis::cmd("PING")
            .query_async(&mut *conn)
            .await
            .map_err(|e| AppError::DatabaseError(format!("Redis Ping Error: {}", e)))?;

        tracing::info!("Connected to Redis via bb8 connection pool.");

        Ok(Self { pool })
    }

    /// Helper to store generic serializable objects in Redis with TTL
    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T, ttl_seconds: u64) -> Result<(), AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
        let json_str = serde_json::to_string(value)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
            
        conn.set_ex(key, json_str, ttl_seconds)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }

    /// Helper to retrieve generic objects from Redis
    pub async fn get_json<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Result<Option<T>, AppError> {
        let mut conn = self.pool.get().await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
        let result: Option<String> = conn.get(key)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
        match result {
            Some(json_str) => {
                let obj = serde_json::from_str(&json_str)
                    .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
                Ok(Some(obj))
            },
            None => Ok(None)
        }
    }
}