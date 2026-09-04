use redis::AsyncCommands;
use ferrox_errors::AppError;

#[derive(Clone)]
pub struct FeatureFlagsClient {
    redis_client: redis::Client,
}

impl FeatureFlagsClient {
    pub fn new(redis_url: &str) -> Result<Self, AppError> {
        let redis_client = redis::Client::open(redis_url)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        Ok(Self { redis_client })
    }

    /// Checks if a feature flag is globally enabled
    pub async fn is_enabled(&self, feature_name: &str) -> Result<bool, AppError> {
        let mut con = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        let key = format!("feature_flag:{}", feature_name);
        
        let is_enabled: Option<bool> = con.get(&key)
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        Ok(is_enabled.unwrap_or(false))
    }

    /// Enables or disables a feature flag globally
    pub async fn set_flag(&self, feature_name: &str, enabled: bool) -> Result<(), AppError> {
        let mut con = self.redis_client.get_async_connection().await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        let key = format!("feature_flag:{}", feature_name);
        
        con.set(&key, enabled)
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        Ok(())
    }
}
