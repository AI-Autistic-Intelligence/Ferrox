use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Zero-Trust Device & Client Fingerprint Binder for PASETO v4 Session Tokens
pub struct ClientFingerprint;

impl ClientFingerprint {
    /// Computes a deterministic 64-bit fingerprint hash for a client HTTP session
    pub fn compute_fingerprint(user_agent: &str, accept_language: &str, subnet_ip: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        user_agent.hash(&mut hasher);
        accept_language.hash(&mut hasher);

        // Normalize IP subnet (e.g. 192.168.1.xxx -> 192.168.1.0)
        let subnet = if let Some(idx) = subnet_ip.rfind('.') {
            &subnet_ip[..idx]
        } else {
            subnet_ip
        };
        subnet.hash(&mut hasher);

        hasher.finish()
    }

    /// Verifies if a token binding fingerprint matches current client request
    pub fn verify_binding(bound_fingerprint: u64, current_user_agent: &str, current_lang: &str, current_ip: &str) -> bool {
        let current_fp = Self::compute_fingerprint(current_user_agent, current_lang, current_ip);
        bound_fingerprint == current_fp
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_fingerprint_binding() {
        let fp = ClientFingerprint::compute_fingerprint("Mozilla/5.0", "it-IT", "192.168.1.50");
        assert!(ClientFingerprint::verify_binding(fp, "Mozilla/5.0", "it-IT", "192.168.1.99")); // Same subnet
        assert!(!ClientFingerprint::verify_binding(fp, "MaliciousBot/1.0", "it-IT", "192.168.1.50"));
    }
}
