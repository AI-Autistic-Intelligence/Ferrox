//! # SBOM & Supply Chain Dependency Integrity Guard (`sbom_supply_chain_verifier.rs`)
//!
//! Software Bill of Materials cryptographic hash & dependency tree validator 
//! (*Intelligent Continuous Security*, O'Reilly - Ch. 7).

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Component SBOM Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SbomComponentRecord {
    pub name: String,
    pub version: String,
    pub expected_sha256: String,
    pub is_revoked: bool,
}

/// Supply Chain Integrity Report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyChainIntegrityReport {
    pub is_valid: bool,
    pub tampered_components: Vec<String>,
    pub revoked_components: Vec<String>,
    pub total_verified: usize,
}

/// SBOM Supply Chain Verifier Engine
pub struct SbomSupplyChainVerifierEngine;

impl SbomSupplyChainVerifierEngine {
    /// Verifies actual binary component bytes against registered SBOM expected SHA-256 hashes
    pub fn verify_component(
        component: &SbomComponentRecord,
        actual_bytes: &[u8],
    ) -> (bool, String) {
        let mut hasher = Sha256::new();
        hasher.update(actual_bytes);
        let actual_hash: String = hasher.finalize().iter().map(|b| format!("{:02x}", b)).collect();

        let matches = actual_hash.eq_ignore_ascii_case(&component.expected_sha256);
        (matches, actual_hash)
    }

    /// Conducts a comprehensive supply chain audit across an entire manifest of components
    pub fn audit_supply_chain(
        manifest: &[SbomComponentRecord],
        actual_components: &[(&str, &[u8])],
    ) -> SupplyChainIntegrityReport {
        let mut tampered = Vec::new();
        let mut revoked = Vec::new();
        let mut total_verified = 0;

        for record in manifest {
            if record.is_revoked {
                revoked.push(format!("{}@{}", record.name, record.version));
            }

            if let Some((_, bytes)) = actual_components.iter().find(|(name, _)| *name == record.name) {
                let (matches, _) = Self::verify_component(record, bytes);
                if !matches {
                    tampered.push(format!("{}@{} (Hash mismatch)", record.name, record.version));
                } else {
                    total_verified += 1;
                }
            } else {
                tampered.push(format!("{}@{} (Missing binary component)", record.name, record.version));
            }
        }

        let is_valid = tampered.is_empty() && revoked.is_empty();

        SupplyChainIntegrityReport {
            is_valid,
            tampered_components: tampered,
            revoked_components: revoked,
            total_verified,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbom_integrity_audit() {
        let rec = SbomComponentRecord {
            name: "ferrox-security".to_string(),
            version: "0.5.0".to_string(),
            expected_sha256: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".to_string(), // "hello"
            is_revoked: false,
        };

        let actual_bytes = b"hello";
        let (ok, _) = SbomSupplyChainVerifierEngine::verify_component(&rec, actual_bytes);
        assert!(ok);
    }
}
