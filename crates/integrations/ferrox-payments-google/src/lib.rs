use reqwest::Client;
use serde::{Deserialize, Serialize};
use ferrox_errors::AppError;

#[derive(Clone)]
pub struct GooglePlayClient {
    package_name: String,
    http_client: Client,
    // In a real implementation, you would load Google Service Account credentials 
    // and handle OAuth2 Bearer token generation here.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GooglePurchaseVerification {
    pub purchase_state: i32,
    pub consumption_state: i32,
    pub developer_payload: Option<String>,
}

impl GooglePlayClient {
    pub fn new(package_name: &str) -> Self {
        Self {
            package_name: package_name.to_string(),
            http_client: Client::new(),
        }
    }

    /// Verifies an in-app purchase token with Google Play Developer API
    pub async fn verify_purchase(&self, product_id: &str, token: &str, access_token: &str) -> Result<GooglePurchaseVerification, AppError> {
        let url = format!(
            "https://androidpublisher.googleapis.com/androidpublisher/v3/applications/{}/purchases/products/{}/tokens/{}",
            self.package_name, product_id, token
        );

        let response = self.http_client
            .get(&url)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::Unauthorized(
                format!("Google Play API Error: {}", error_text)
            ));
        }

        let verification: GooglePurchaseVerification = response
            .json()
            .await
            .map_err(|e| AppError::InternalServerError(Box::new(e)))?;

        Ok(verification)
    }
}
