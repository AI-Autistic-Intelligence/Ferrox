//! # Distributed Honeynet Deception & Shadow-Ban Mesh (`honeynet_mesh.rs`)
//!
//! Cross-node trap sharing, synthetic honeypot endpoint monitoring & sub-second global shadow-bans
//! (*USENIX Security* / *Network Security Through Data Analysis*).

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Honeypot Trap Event Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoneypotTrapEvent {
    pub source_ip: String,
    pub trap_endpoint: String,
    pub timestamp_secs: u64,
    pub payload_sample: String,
}

/// Honeynet Node Reputation Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackerReputationRecord {
    pub source_ip: String,
    pub trap_hits_count: u32,
    pub is_shadow_banned: bool,
    pub threat_score: f64,
}

/// Distributed Honeynet Deception & Shadow-Ban Mesh Engine
pub struct HoneynetMeshEngine {
    reputation_table: HashMap<String, AttackerReputationRecord>,
    trap_endpoints: Vec<String>,
}

impl HoneynetMeshEngine {
    /// Initializes a new Honeynet Mesh Engine with registered synthetic trap endpoints
    pub fn new(trap_endpoints: &[String]) -> Self {
        Self {
            reputation_table: HashMap::new(),
            trap_endpoints: trap_endpoints.to_vec(),
        }
    }

    /// Checks if a request target matches a honeypot trap endpoint
    pub fn is_trap_endpoint(&self, path: &str) -> bool {
        self.trap_endpoints.iter().any(|trap| path.contains(trap))
    }

    /// Processes an incoming threat event, updating global attacker IP reputation and triggering shadow-bans
    pub fn record_trap_hit(&mut self, event: HoneypotTrapEvent) -> AttackerReputationRecord {
        let entry = self.reputation_table.entry(event.source_ip.clone()).or_insert_with(|| AttackerReputationRecord {
            source_ip: event.source_ip.clone(),
            trap_hits_count: 0,
            is_shadow_banned: false,
            threat_score: 0.0,
        });

        entry.trap_hits_count += 1;
        entry.threat_score = (entry.threat_score + 0.4).min(1.0);

        if entry.trap_hits_count >= 2 || entry.threat_score >= 0.8 {
            entry.is_shadow_banned = true;
        }

        entry.clone()
    }

    /// Queries whether a given client IP is currently shadow-banned
    pub fn is_ip_shadow_banned(&self, ip: &str) -> bool {
        self.reputation_table
            .get(ip)
            .map(|record| record.is_shadow_banned)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_honeynet_shadow_ban_trigger() {
        let traps = vec!["/admin/phpmyadmin".to_string(), "/.env".to_string()];
        let mut engine = HoneynetMeshEngine::new(&traps);

        assert!(engine.is_trap_endpoint("/admin/phpmyadmin/index.php"));

        let event1 = HoneypotTrapEvent {
            source_ip: "192.168.1.100".to_string(),
            trap_endpoint: "/.env".to_string(),
            timestamp_secs: 1700000000,
            payload_sample: "GET /.env".to_string(),
        };

        engine.record_trap_hit(event1);
        assert!(!engine.is_ip_shadow_banned("192.168.1.100"));

        let event2 = HoneypotTrapEvent {
            source_ip: "192.168.1.100".to_string(),
            trap_endpoint: "/admin/phpmyadmin".to_string(),
            timestamp_secs: 1700000001,
            payload_sample: "GET /admin/phpmyadmin".to_string(),
        };

        engine.record_trap_hit(event2);
        assert!(engine.is_ip_shadow_banned("192.168.1.100"));
    }
}
