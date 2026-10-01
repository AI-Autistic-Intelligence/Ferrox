use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_secretsmanager::Client as SecretsClient;
use ferrox_errors::FerroxError;

pub struct AwsCloudHelper {
    pub s3: S3Client,
    pub secrets: SecretsClient,
}

impl AwsCloudHelper {
    /// Initializes AWS clients using the default credential chain (Env vars, IAM Instance Profile, EKS IRSA)
    pub async fn new() -> Self {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        Self {
            s3: S3Client::new(&config),
            secrets: SecretsClient::new(&config),
        }
    }

    /// Fetches a secret from AWS Secrets Manager
    pub async fn get_secret(&self, secret_id: &str) -> Result<String, FerroxError> {
        let resp = self.secrets.get_secret_value().secret_id(secret_id).send().await
            .map_err(|e| FerroxError::IntegrationError(format!("AWS Secret Error: {}", e)))?;
        
        resp.secret_string().map(|s| s.to_string())
            .ok_or_else(|| FerroxError::IntegrationError("Secret string is empty or binary".to_string()))
    }

    /// Uploads an object to S3
    pub async fn upload_to_s3(&self, bucket: &str, key: &str, body: Vec<u8>) -> Result<(), FerroxError> {
        self.s3.put_object()
            .bucket(bucket)
            .key(key)
            .body(body.into())
            .send()
            .await
            .map_err(|e| FerroxError::IntegrationError(format!("S3 Upload Error: {}", e)))?;
        Ok(())
    }
}
