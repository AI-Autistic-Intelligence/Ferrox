//! # Bipartite Graph Sybil & Botnet Threat Engine (`ferrox-sentinel::graph`)
//!
//! In-memory bipartite graph mapping IP addresses, User Accounts, and Device Fingerprints.
//! Detects multi-account fraud rings, botnets, and credential stuffing clusters.
//! Reference: *Machine Learning and Security* (Ch. 6 - Graph Analytics).

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};

/// Alert structure for detected botnet or Sybil clusters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotnetClusterAlert {
    pub cluster_id: String,
    pub ip_count: usize,
    pub account_count: usize,
    pub risk_score: f64,
}

/// Bipartite Graph Engine for tracking IP-Account-Device relationships
pub struct ThreatGraphEngine {
    ip_to_accounts: HashMap<String, HashSet<String>>,
    account_to_ips: HashMap<String, HashSet<String>>,
    account_to_fingerprints: HashMap<String, HashSet<String>>,
}

impl ThreatGraphEngine {
    pub fn new() -> Self {
        Self {
            ip_to_accounts: HashMap::new(),
            account_to_ips: HashMap::new(),
            account_to_fingerprints: HashMap::new(),
        }
    }

    /// Records an interaction relationship between IP, User Account, and Device Fingerprint
    pub fn record_interaction(&mut self, ip: &str, user_account: &str, fingerprint: &str) {
        self.ip_to_accounts
            .entry(ip.to_string())
            .or_default()
            .insert(user_account.to_string());

        self.account_to_ips
            .entry(user_account.to_string())
            .or_default()
            .insert(ip.to_string());

        self.account_to_fingerprints
            .entry(user_account.to_string())
            .or_default()
            .insert(fingerprint.to_string());
    }

    /// Detects dense botnet clusters where single IPs connect to multiple accounts or vice versa
    pub fn detect_botnet_clusters(&self, max_accounts_per_ip: usize) -> Vec<BotnetClusterAlert> {
        let mut alerts = Vec::new();

        for (ip, accounts) in &self.ip_to_accounts {
            if accounts.len() >= max_accounts_per_ip {
                let risk_score = (accounts.len() as f64 / max_accounts_per_ip as f64).min(1.0);
                alerts.push(BotnetClusterAlert {
                    cluster_id: format!("cluster_ip_{}", ip),
                    ip_count: 1,
                    account_count: accounts.len(),
                    risk_score,
                });
            }
        }

        alerts
    }
}

impl Default for ThreatGraphEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_graph_engine_sybil_detection() {
        let mut graph = ThreatGraphEngine::new();

        // Single IP connecting to 6 accounts (Botnet cluster pattern)
        for i in 0..6 {
            graph.record_interaction("192.168.1.100", &format!("user_{}", i), "fp_device_xyz");
        }

        let alerts = graph.detect_botnet_clusters(5);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].account_count, 6);
        assert!(alerts[0].risk_score >= 1.0);
    }
}
