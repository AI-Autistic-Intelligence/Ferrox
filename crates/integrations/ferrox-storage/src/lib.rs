use aws_sdk_s3::{presigning::PresigningConfig, Client};
use std::time::Duration;
use ferrox_errors::AppError;

#[derive(Clone)]
pub struct S3StorageClient {
    client: Client,
    bucket_name: String,
}

impl S3StorageClient {
    /// Connects to AWS S3, MinIO, or compatible storage using environment variables
    /// (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY, AWS_REGION, AWS_ENDPOINT_URL)
    pub async fn connect(bucket_name: &str) -> Result<Self, AppError> {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        
        Ok(Self {
            client,
            bucket_name: bucket_name.to_string(),
        })
    }

    /// Generates a presigned URL for secure client-side uploads (expires in 15 mins)
    pub async fn generate_presigned_upload_url(&self, object_key: &str) -> Result<String, AppError> {
        let expires_in = Duration::from_secs(900); // 15 minutes
        let presigning_config = PresigningConfig::expires_in(expires_in)
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        let presigned_req = self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(object_key)
            .presigned(presigning_config)
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        Ok(presigned_req.uri().to_string())
    }
}
