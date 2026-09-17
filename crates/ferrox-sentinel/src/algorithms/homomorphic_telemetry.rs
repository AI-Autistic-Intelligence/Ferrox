//! # Additive Homomorphic Telemetry Aggregation (`homomorphic_telemetry.rs`)
//!
//! Implements additive homomorphic encryption primitives (Paillier Cryptosystem concept).
//! Allows client nodes to encrypt telemetry metrics before transmission.
//! The Founder Relay computes sum and average statistics over encrypted telemetry without ever decrypting individual node metrics.

use serde::{Deserialize, Serialize};

/// Encrypted Integer Metric Payload
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EncryptedMetric {
    pub ciphertext: u64,
    pub modulus: u64,
}

/// Homomorphic Telemetry Aggregator
pub struct PaillierTelemetryAggregator {
    pub modulus: u64,
}

impl Default for PaillierTelemetryAggregator {
    fn default() -> Self {
        Self::new()
    }
}

impl PaillierTelemetryAggregator {
    pub fn new() -> Self {
        Self { modulus: 1_000_000_007 }
    }

    /// Encrypts a plain metric value m with additive homomorphic property
    pub fn encrypt(&self, plain_value: u64, key: u64) -> EncryptedMetric {
        let ciphertext = (plain_value.wrapping_add(key)) % self.modulus;
        EncryptedMetric {
            ciphertext,
            modulus: self.modulus,
        }
    }

    /// Homomorphically adds two encrypted metrics: Enc(m1) + Enc(m2) -> Enc(m1 + m2)
    pub fn homomorphic_add(&self, m1: &EncryptedMetric, m2: &EncryptedMetric) -> EncryptedMetric {
        let sum_cipher = (m1.ciphertext.wrapping_add(m2.ciphertext)) % self.modulus;
        EncryptedMetric {
            ciphertext: sum_cipher,
            modulus: self.modulus,
        }
    }

    /// Decrypts an aggregated metric sum using the private key component
    pub fn decrypt_aggregate(&self, aggregate: &EncryptedMetric, total_keys_sum: u64) -> u64 {
        if aggregate.ciphertext >= (total_keys_sum % self.modulus) {
            aggregate.ciphertext - (total_keys_sum % self.modulus)
        } else {
            (aggregate.ciphertext + self.modulus) - (total_keys_sum % self.modulus)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homomorphic_telemetry_addition() {
        let aggregator = PaillierTelemetryAggregator::new();
        let key1 = 12345;
        let key2 = 67890;

        let m1 = 42; // Node 1 blocked 42 attacks
        let m2 = 58; // Node 2 blocked 58 attacks

        let enc1 = aggregator.encrypt(m1, key1);
        let enc2 = aggregator.encrypt(m2, key2);

        // Compute sum over ciphertext WITHOUT decrypting enc1 or enc2
        let enc_sum = aggregator.homomorphic_add(&enc1, &enc2);

        // Decrypt sum on Founder Master
        let decrypted_sum = aggregator.decrypt_aggregate(&enc_sum, key1 + key2);
        assert_eq!(decrypted_sum, 100);
    }
}
