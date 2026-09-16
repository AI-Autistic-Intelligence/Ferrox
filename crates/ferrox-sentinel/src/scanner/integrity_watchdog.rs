//! # Memory Integrity Watchdog (`integrity_watchdog.rs`)
//!
//! Computes runtime SHA-256 cryptographic digests over critical security configuration structs and memory states.
//! Detects unauthorized memory injection, debugging hooks, or state mutation attempts.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};

/// Integrity Audit Snapshot Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrityReport {
    pub expected_hash: String,
    pub actual_hash: String,
    pub is_intact: bool,
    pub audited_at: DateTime<Utc>,
}

/// Runtime Memory Integrity Watchdog Engine
pub struct MemoryIntegrityWatchdog {
    baseline_hash: String,
}

impl MemoryIntegrityWatchdog {
    pub fn new(initial_bytes: &[u8]) -> Self {
        let baseline_hash = Self::compute_hash(initial_bytes);
        Self { baseline_hash }
    }

    pub fn compute_hash(bytes: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_integrity(&self, current_bytes: &[u8]) -> IntegrityReport {
        let actual_hash = Self::compute_hash(current_bytes);
        let is_intact = self.baseline_hash == actual_hash;

        IntegrityReport {
            expected_hash: self.baseline_hash.clone(),
            actual_hash,
            is_intact,
            audited_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_integrity_watchdog() {
        let state_bytes = b"ferrox_sentinel_rules_v1_active";
        let watchdog = MemoryIntegrityWatchdog::new(state_bytes);

        // Verification with unaltered state -> Intact
        let report_clean = watchdog.verify_integrity(state_bytes);
        assert!(report_clean.is_intact);

        // Verification with mutated memory -> Tampering detected!
        let mutated_bytes = b"ferrox_sentinel_rules_v1_DISABLED";
        let report_tampered = watchdog.verify_integrity(mutated_bytes);
        assert!(!report_tampered.is_intact);
    }
}
