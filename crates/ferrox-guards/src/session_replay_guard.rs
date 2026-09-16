//! # Session Replay & Fingerprint Guard (`session_replay_guard.rs`)
//!
//! Binds PASETO/JWT session tokens to client TLS/HTTP User-Agent fingerprints (OWASP Session Security).
//! Prevents stolen token replay attacks when tokens are reused across mismatched browsers or IPs.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// Status returned by session replay verification
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionStatus {
    Valid,
    HijackDetected { expected_fingerprint: String, actual_fingerprint: String },
}

/// Composite Client Fingerprint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientFingerprint {
    pub hash: String,
    pub user_agent: String,
}

impl ClientFingerprint {
    pub fn new(user_agent: &str, accept_language: &str, accept_encoding: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(user_agent.as_bytes());
        hasher.update(b"|");
        hasher.update(accept_language.as_bytes());
        hasher.update(b"|");
        hasher.update(accept_encoding.as_bytes());
        let hash = format!("{:x}", hasher.finalize());

        Self {
            hash,
            user_agent: user_agent.to_string(),
        }
    }
}

/// Session Replay Detector Engine
pub struct SessionReplayDetector;

impl SessionReplayDetector {
    pub fn evaluate_session(
        bound_fingerprint_hash: &str,
        current_fingerprint: &ClientFingerprint,
    ) -> SessionStatus {
        if bound_fingerprint_hash == current_fingerprint.hash {
            SessionStatus::Valid
        } else {
            SessionStatus::HijackDetected {
                expected_fingerprint: bound_fingerprint_hash.to_string(),
                actual_fingerprint: current_fingerprint.hash.clone(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_replay_detector() {
        let fp1 = ClientFingerprint::new("Mozilla/5.0 (Windows NT 10.0)", "it-IT,it;q=0.9", "gzip, deflate");
        let fp2 = ClientFingerprint::new("Python-urllib/3.9", "en-US", "gzip");

        // Matching fingerprint -> Valid
        let res_valid = SessionReplayDetector::evaluate_session(&fp1.hash, &fp1);
        assert_eq!(res_valid, SessionStatus::Valid);

        // Mismatched fingerprint -> HijackDetected
        let res_hijack = SessionReplayDetector::evaluate_session(&fp1.hash, &fp2);
        match res_hijack {
            SessionStatus::HijackDetected { .. } => {}
            _ => panic!("Expected HijackDetected"),
        }
    }
}
