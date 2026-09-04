use aws_sdk_secretsmanager::Client;
use yalc_errors::AppError;

pub struct CloudHelper;

impl CloudHelper {
    /// Loads a secret string from AWS Secrets Manager (or compatible API)
    pub async fn get_aws_secret(secret_id: &str) -> Result<String, AppError> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);

        let response = client
            .get_secret_value()
            .secret_id(secret_id)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        if let Some(secret_string) = response.secret_string() {
            return Ok(secret_string.to_string());
        }

        Err(AppError::InternalServerError("Secret is binary, expected string".into()))
    }
}
