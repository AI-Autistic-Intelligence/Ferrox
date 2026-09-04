use crate::{PasetoAuth, AuthPayload};
use ferrox_errors::AppError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DualTokens {
    pub access_token: String,
    pub refresh_token: String,
}

pub struct DualTokenManager {
    paseto: PasetoAuth,
}

impl DualTokenManager {
    pub fn new(secret_key: &str) -> Self {
        Self {
            paseto: PasetoAuth::new(secret_key),
        }
    }

    /// Generates both an Access Token (short-lived) and a Refresh Token (long-lived).
    pub fn generate_tokens(&self, payload: &AuthPayload) -> Result<DualTokens, AppError> {
        // Access Token: 15 minutes
        let access_token = self.paseto.generate_token(payload, time::Duration::minutes(15))?;
        
        // Refresh Token: Usually stored in DB/Redis with a longer expiration and UUID.
        // For boilerplate, we generate a cryptographically strong random hex.
        // In a real implementation, you would save this to Redis/DB to allow revocation.
        use rand::RngCore;
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        let refresh_token = hex::encode(key);

        Ok(DualTokens {
            access_token,
            refresh_token,
        })
    }

    /// Verifies the Access token. If it fails, the frontend should use the Refresh Token
    /// on a dedicated endpoint to get a new pair.
    pub fn verify_access_token(&self, token: &str) -> Result<String, AppError> {
        let claims = self.paseto.validate_token(token)?;
        Ok(claims.user_id)
    }
}
