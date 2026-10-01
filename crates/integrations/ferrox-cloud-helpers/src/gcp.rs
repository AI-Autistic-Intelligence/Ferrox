use google_cloud_storage::client::{Client as GcsClient, ClientConfig as GcsConfig};
use google_cloud_storage::http::objects::upload::{UploadObjectRequest, UploadType};
use google_cloud_auth::project::Config;
use google_cloud_auth::token::DefaultTokenSourceProvider;
use google_cloud_token::TokenSourceProvider;
use reqwest::Client;
use ferrox_errors::AppError;
use serde_json::Value;

pub struct GcpCloudHelper {
    pub storage: GcsClient,
    pub http_client: Client,
    pub token_provider: DefaultTokenSourceProvider,
}

impl GcpCloudHelper {
    /// Initializes GCP clients using the Application Default Credentials (ADC) / Workload Identity
    pub async fn new() -> Result<Self, AppError> {
        let config = Config {
            audience: None,
            scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
            sub: None,
        };
        let token_provider = DefaultTokenSourceProvider::new(config).await
            .map_err(|e| AppError::InternalError(format!("GCP Auth Error: {}", e)))?;

        let gcs_config = GcsConfig::default().with_auth().await
            .map_err(|e| AppError::InternalError(format!("GCS Auth Error: {}", e)))?;

        Ok(Self {
            storage: GcsClient::new(gcs_config),
            http_client: Client::new(),
            token_provider,
        })
    }

    /// Fetches a payload from GCP Secret Manager via REST API
    pub async fn get_secret(&self, project_id: &str, secret_id: &str, version: &str) -> Result<String, AppError> {
        let token_source = self.token_provider.token_source();
        let token = token_source.token().await
            .map_err(|e| AppError::InternalError(format!("Failed to get GCP Token: {}", e)))?;

        let url = format!(
            "https://secretmanager.googleapis.com/v1/projects/{}/secrets/{}/versions/{}:access",
            project_id, secret_id, version
        );

        let res = self.http_client.get(&url)
            .bearer_auth(token)
            .send()
            .await
            .map_err(|e| AppError::InternalError(format!("HTTP Request Failed: {}", e)))?;

        let json: Value = res.json().await
            .map_err(|e| AppError::InternalError(format!("Invalid JSON response: {}", e)))?;

        let b64_payload = json["payload"]["data"].as_str()
            .ok_or_else(|| AppError::InternalError("Missing payload.data in GCP response".to_string()))?;

        use base64::{Engine as _, engine::general_purpose::STANDARD};
        let decoded = STANDARD.decode(b64_payload)
            .map_err(|_| AppError::InternalError("Base64 decode failed".to_string()))?;

        String::from_utf8(decoded).map_err(|_| AppError::InternalError("Invalid UTF-8".to_string()))
    }

    /// Uploads an object to Google Cloud Storage (GCS)
    pub async fn upload_to_gcs(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<(), AppError> {
        let upload_type = UploadType::Simple(google_cloud_storage::http::objects::upload::Media::new(key.to_string()));
        self.storage.upload_object(&UploadObjectRequest {
            bucket: bucket.to_string(),
            ..Default::default()
        }, data, &upload_type).await
            .map_err(|e| AppError::InternalError(format!("GCS Upload Error: {}", e)))?;
        Ok(())
    }
}
