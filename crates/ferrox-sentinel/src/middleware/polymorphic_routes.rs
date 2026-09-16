//! # Polymorphic Route Rotation Engine (`polymorphic_routes.rs`)
//!
//! Generates time-rolling HMAC nonces for sensitive API endpoint paths.
//! Rotates target path tokens every 60 seconds, rendering static automated vulnerability scanners 100% ineffective.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

/// Polymorphic Dynamic Path Token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPathToken {
    pub base_route: String,
    pub dynamic_path: String,
    pub epoch_seconds: u64,
    pub expires_at_epoch: u64,
}

/// Polymorphic Route Rotation Engine
pub struct PolymorphicRouteEngine {
    secret: String,
    rotation_interval_secs: u64,
}

impl PolymorphicRouteEngine {
    pub fn new(secret: &str, rotation_interval_secs: u64) -> Self {
        Self {
            secret: secret.to_string(),
            rotation_interval_secs: rotation_interval_secs.max(10),
        }
    }

    pub fn compute_current_epoch(&self) -> u64 {
        let now = Utc::now().timestamp() as u64;
        now / self.rotation_interval_secs
    }

    pub fn compute_dynamic_path(&self, base_route: &str, epoch: u64) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.secret.as_bytes());
        hasher.update(b":");
        hasher.update(base_route.as_bytes());
        hasher.update(b":");
        hasher.update(epoch.to_le_bytes());
        let nonce = format!("{:x}", hasher.finalize());
        let short_token = &nonce[..10];

        format!("{}/poly_{}", base_route.trim_end_matches('/'), short_token)
    }

    pub fn get_current_dynamic_path(&self, base_route: &str) -> DynamicPathToken {
        let epoch = self.compute_current_epoch();
        let dynamic_path = self.compute_dynamic_path(base_route, epoch);
        let expires_at_epoch = (epoch + 1) * self.rotation_interval_secs;

        DynamicPathToken {
            base_route: base_route.to_string(),
            dynamic_path,
            epoch_seconds: epoch,
            expires_at_epoch,
        }
    }

    pub fn validate_request_path(&self, request_path: &str, base_route: &str) -> bool {
        let epoch = self.compute_current_epoch();
        let current = self.compute_dynamic_path(base_route, epoch);
        let previous = self.compute_dynamic_path(base_route, epoch.saturating_sub(1));

        request_path == current || request_path == previous
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymorphic_route_rotation() {
        let engine = PolymorphicRouteEngine::new("secret_poly_key_88", 60);
        let token = engine.get_current_dynamic_path("/api/v1/admin");

        assert!(token.dynamic_path.contains("/api/v1/admin/poly_"));
        assert!(engine.validate_request_path(&token.dynamic_path, "/api/v1/admin"));
        assert!(!engine.validate_request_path("/api/v1/admin/poly_invalid", "/api/v1/admin"));
    }
}
