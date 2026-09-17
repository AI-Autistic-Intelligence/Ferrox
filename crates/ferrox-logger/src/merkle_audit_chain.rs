//! # Merkle Tree Audit Chain (`merkle_audit_chain.rs`)
//!
//! Provides a cryptographically tamper-proof Merkle Tree Audit Chain for legal CISO compliance reports
//! and incident logging in Ferrox applications.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A single immutable audit log record in the Merkle Audit Chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogRecord {
    pub record_id: String,
    pub event_type: String,
    pub node_id: String,
    pub payload_hash: String,
    pub timestamp: DateTime<Utc>,
}

/// Cryptographic Merkle Audit Ledger Chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MerkleAuditChain {
    pub chain_id: String,
    pub records: Vec<AuditLogRecord>,
    pub merkle_root_sha256: String,
    pub last_updated_at: DateTime<Utc>,
}

impl MerkleAuditChain {
    pub fn new(chain_id: &str) -> Self {
        Self {
            chain_id: chain_id.to_string(),
            records: Vec::new(),
            merkle_root_sha256: Self::compute_root(&[]),
            last_updated_at: Utc::now(),
        }
    }

    /// Appends a new audit record to the chain and recomputes the Merkle Root.
    pub fn append_record(&mut self, event_type: &str, node_id: &str, payload_json: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload_json.as_bytes());
        let payload_hash = format!("{:x}", hasher.finalize());

        let record_id = format!("rec_{:x}", rand::random::<u128>());
        let record = AuditLogRecord {
            record_id: record_id.clone(),
            event_type: event_type.to_string(),
            node_id: node_id.to_string(),
            payload_hash,
            timestamp: Utc::now(),
        };

        self.records.push(record);
        self.merkle_root_sha256 = Self::compute_root(&self.records);
        self.last_updated_at = Utc::now();

        record_id
    }

    /// Computes the binary SHA-256 Merkle Root over all records in the chain.
    pub fn compute_root(records: &[AuditLogRecord]) -> String {
        if records.is_empty() {
            let mut hasher = Sha256::new();
            hasher.update(b"EMPTY_MERKLE_ROOT");
            return format!("{:x}", hasher.finalize());
        }

        let mut leaf_hashes: Vec<Vec<u8>> = records
            .iter()
            .map(|r| {
                let mut h = Sha256::new();
                h.update(r.record_id.as_bytes());
                h.update(b":");
                h.update(r.event_type.as_bytes());
                h.update(b":");
                h.update(r.node_id.as_bytes());
                h.update(b":");
                h.update(r.payload_hash.as_bytes());
                h.finalize().to_vec()
            })
            .collect();

        while leaf_hashes.len() > 1 {
            let mut next_level = Vec::new();
            for chunk in leaf_hashes.chunks(2) {
                let mut h = Sha256::new();
                h.update(&chunk[0]);
                if chunk.len() > 1 {
                    h.update(&chunk[1]);
                } else {
                    h.update(&chunk[0]);
                }
                next_level.push(h.finalize().to_vec());
            }
            leaf_hashes = next_level;
        }

        format!("{:x}", sha2::Sha256::digest(&leaf_hashes[0]))
    }

    /// Verifies the cryptographic integrity of the Merkle Audit Chain.
    pub fn verify_integrity(&self) -> bool {
        let expected = Self::compute_root(&self.records);
        expected == self.merkle_root_sha256
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_audit_chain_tamper_detection() {
        let mut chain = MerkleAuditChain::new("chain_ciso_audit_01");
        assert!(chain.verify_integrity());

        chain.append_record("GUARD_TAMPER", "node_eu_01", r#"{"status": "bypassed"}"#);
        chain.append_record("PENTEST_PROBE", "node_us_01", r#"{"score": 100}"#);

        assert_eq!(chain.records.len(), 2);
        assert!(chain.verify_integrity());

        // Tamper with record
        chain.records[0].event_type = "TAMPERED_EVENT".to_string();
        assert!(!chain.verify_integrity());
    }
}
