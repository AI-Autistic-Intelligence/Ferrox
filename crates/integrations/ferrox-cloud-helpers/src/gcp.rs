use google_cloud_storage::client::{Client as GcsClient, ClientConfig as GcsConfig};
use google_cloud_storage::http::objects::upload::{UploadObjectRequest, UploadType};
use google_cloud_secretmanager::client::{Client as SecretManagerClient, ClientConfig as SmConfig};
use google_cloud_default::WithAuthExt;
use ferrox_errors::FerroxError;

pub struct GcpCloudHelper {
    pub storage: GcsClient,
    pub secrets: SecretManagerClient,
}

impl GcpCloudHelper {
    /// Initializes GCP clients using the Application Default Credentials (ADC) / Workload Identity
    pub async fn new() -> Result<Self, FerroxError> {
        let gcs_config = GcsConfig::default().with_auth().await
            .map_err(|e| FerroxError::IntegrationError(format!("GCS Auth Error: {}", e)))?;
        let sm_config = SmConfig::default().with_auth().await
            .map_err(|e| FerroxError::IntegrationError(format!("SM Auth Error: {}", e)))?;

        Ok(Self {
            storage: GcsClient::new(gcs_config),
            secrets: SecretManagerClient::new(sm_config).await.map_err(|e| FerroxError::IntegrationError(e.to_string()))?,
        })
    }

    /// Fetches a payload from GCP Secret Manager
    pub async fn get_secret(&self, project_id: &str, secret_id: &str, version: &str) -> Result<String, FerroxError> {
        let name = format!("projects/{}/secrets/{}/versions/{}", project_id, secret_id, version);
        
        let request = google_cloud_secretmanager::grpc::apiv1::secretmanager::AccessSecretVersionRequest {
            name,
        };
        
        let response = self.secrets.access_secret_version(request, None).await
            .map_err(|e| FerroxError::IntegrationError(format!("GCP Secret Error: {}", e)))?;

        let payload = response.payload.ok_or_else(|| FerroxError::IntegrationError("Payload empty".to_string()))?;
        String::from_utf8(payload.data).map_err(|_| FerroxError::IntegrationError("Invalid UTF-8 in secret".to_string()))
    }

    /// Uploads an object to Google Cloud Storage (GCS)
    pub async fn upload_to_gcs(&self, bucket: &str, key: &str, data: Vec<u8>) -> Result<(), FerroxError> {
        let upload_type = UploadType::Simple(google_cloud_storage::http::objects::upload::Media::new(key.to_string()));
        self.storage.upload_object(&UploadObjectRequest {
            bucket: bucket.to_string(),
            ..Default::default()
        }, data, &upload_type).await
            .map_err(|e| FerroxError::IntegrationError(format!("GCS Upload Error: {}", e)))?;
        Ok(())
    }
}
