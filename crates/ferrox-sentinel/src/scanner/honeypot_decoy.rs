//! # Deception Honeypot & Decoy Traps (`honeypot_decoy.rs`)
//!
//! Deploys synthetic decoy endpoints and game state honeypots (Active Defense Literature).
//! Any automated vulnerability scanner or malicious bot targeting a decoy endpoint is instantly trapped and shadow-banned.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use chrono::{DateTime, Utc};

/// Honeypot Decoy Trap Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecoyTrapHit {
    pub trap_id: String,
    pub decoy_path: String,
    pub offending_ip: String,
    pub user_agent: String,
    pub timestamp: DateTime<Utc>,
    pub action_taken: String,
}

/// Registry of Synthetic Decoy Routes
pub struct HoneypotTrapRegistry {
    decoy_paths: HashSet<String>,
}

impl Default for HoneypotTrapRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl HoneypotTrapRegistry {
    pub fn new() -> Self {
        let mut decoy_paths = HashSet::new();
        decoy_paths.insert("/api/v1/admin/debug_dump".to_string());
        decoy_paths.insert("/api/v1/cheat/peek_deck".to_string());
        decoy_paths.insert("/api/v1/cheat/unlock_pozzetto".to_string());
        decoy_paths.insert("/env.bak".to_string());
        decoy_paths.insert("/.git/config".to_string());
        decoy_paths.insert("/phpmyadmin".to_string());

        Self { decoy_paths }
    }

    pub fn is_decoy(&self, path: &str) -> bool {
        self.decoy_paths.contains(path)
    }

    pub fn evaluate_request(&self, path: &str, client_ip: &str, user_agent: &str) -> Option<DecoyTrapHit> {
        if self.is_decoy(path) {
            Some(DecoyTrapHit {
                trap_id: format!("decoy_trap_{:x}", rand::random::<u128>()),
                decoy_path: path.to_string(),
                offending_ip: client_ip.to_string(),
                user_agent: user_agent.to_string(),
                timestamp: Utc::now(),
                action_taken: "IMMEDIATE_ECOSYSTEM_SHADOW_BAN".to_string(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_honeypot_decoy_trap_detection() {
        let registry = HoneypotTrapRegistry::new();
        assert!(registry.is_decoy("/api/v1/admin/debug_dump"));
        assert!(registry.is_decoy("/api/v1/cheat/peek_deck"));
        assert!(!registry.is_decoy("/api/v1/auth/login"));

        let hit = registry.evaluate_request("/api/v1/cheat/peek_deck", "192.168.1.100", "SqlmapScanner/1.0");
        assert!(hit.is_some());
        let event = hit.unwrap();
        assert_eq!(event.offending_ip, "192.168.1.100");
        assert_eq!(event.action_taken, "IMMEDIATE_ECOSYSTEM_SHADOW_BAN");
    }
}
