//! # Polymorphic API Route Mutation Engine (`polymorphic_routes.rs`)
//!
//! Ephemeral time-windowed path rotation for API endpoint obfuscation 
//! (*ACM SIGCOMM* / Web Security).

use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

/// Dynamic Polymorphic Route State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolymorphicRouteState {
    pub base_path: String,
    pub current_mutated_path: String,
    pub window_slot: u64,
    pub expires_in_secs: u64,
}

/// Polymorphic Route Verification Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteValidationResult {
    pub is_valid: bool,
    pub base_path: String,
    pub window_slot: u64,
}

/// Polymorphic API Route Mutation Engine
pub struct PolymorphicRouteEngine {
    secret_key: Vec<u8>,
    rotation_period_secs: u64,
}

impl PolymorphicRouteEngine {
    /// Creates a new Polymorphic Route Engine with a secret key and rotation period (e.g. 300s)
    pub fn new(secret_key: &[u8], rotation_period_secs: u64) -> Self {
        Self {
            secret_key: secret_key.to_vec(),
            rotation_period_secs,
        }
    }

    /// Computes the mutated path for a given base path and timestamp
    pub fn generate_mutated_path(&self, base_path: &str, timestamp_secs: u64) -> PolymorphicRouteState {
        let window_slot = timestamp_secs / self.rotation_period_secs;
        let mut hasher = Sha256::new();
        hasher.update(&self.secret_key);
        hasher.update(base_path.as_bytes());
        hasher.update(&window_slot.to_be_bytes());

        let result = hasher.finalize();
        let hex_hash: String = result[..8].iter().map(|b| format!("{:02x}", b)).collect();
        let mutated_path = format!("{}/_poly_{}", base_path, hex_hash);

        let expires_in_secs = self.rotation_period_secs - (timestamp_secs % self.rotation_period_secs);

        PolymorphicRouteState {
            base_path: base_path.to_string(),
            current_mutated_path: mutated_path,
            window_slot,
            expires_in_secs,
        }
    }

    /// Validates an incoming request path against current or previous window slots (allowing clock skew)
    pub fn validate_request(&self, request_path: &str, base_path: &str, timestamp_secs: u64) -> RouteValidationResult {
        let current_slot = timestamp_secs / self.rotation_period_secs;

        // Allow current slot and previous slot (clock skew tolerance)
        for slot in [current_slot, current_slot.saturating_sub(1)] {
            let mut hasher = Sha256::new();
            hasher.update(&self.secret_key);
            hasher.update(base_path.as_bytes());
            hasher.update(&slot.to_be_bytes());

            let result = hasher.finalize();
            let hex_hash: String = result[..8].iter().map(|b| format!("{:02x}", b)).collect();
            let expected_path = format!("{}/_poly_{}", base_path, hex_hash);

            if request_path == expected_path {
                return RouteValidationResult {
                    is_valid: true,
                    base_path: base_path.to_string(),
                    window_slot: slot,
                };
            }
        }

        RouteValidationResult {
            is_valid: false,
            base_path: base_path.to_string(),
            window_slot: current_slot,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymorphic_route_generation_and_validation() {
        let engine = PolymorphicRouteEngine::new(b"super_secret_key_123", 300);
        let base_path = "/api/v1/burraco/play";
        let timestamp = 1700000000;

        let route_state = engine.generate_mutated_path(base_path, timestamp);
        assert!(route_state.current_mutated_path.contains("_poly_"));

        // Validate correct path
        let val_ok = engine.validate_request(&route_state.current_mutated_path, base_path, timestamp);
        assert!(val_ok.is_valid);

        // Validate invalid path
        let val_bad = engine.validate_request("/api/v1/burraco/play/_poly_deadbeef", base_path, timestamp);
        assert!(!val_bad.is_valid);
    }
}
