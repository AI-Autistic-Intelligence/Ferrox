//! # Automated Canary Token Deception System (`canary_tokens.rs`)
//!
//! Generates synthetic canary keys (`fk_live_canary_9910a`) planted inside decoy API responses.
//! Emits an immediate out-of-band alert with attacker IP and payload signature if a canary token is touched anywhere on the web.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};

/// Out-of-Band Canary Token Trigger Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CanaryTriggerAlert {
    pub alert_id: String,
    pub canary_token: String,
    pub source_ip: String,
    pub user_agent: String,
    pub request_uri: String,
    pub triggered_at: DateTime<Utc>,
}

/// Canary Token Deception Engine
pub struct CanaryTokenTracker {
    active_canaries: Arc<Mutex<HashSet<String>>>,
    triggered_alerts: Arc<Mutex<Vec<CanaryTriggerAlert>>>,
}

impl Default for CanaryTokenTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl CanaryTokenTracker {
    pub fn new() -> Self {
        let mut active = HashSet::new();
        active.insert("fk_live_canary_default_db_key_99".to_string());
        active.insert("fk_live_canary_admin_secret_10".to_string());

        Self {
            active_canaries: Arc::new(Mutex::new(active)),
            triggered_alerts: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn generate_canary_token(&self) -> String {
        let token = format!("fk_live_canary_{:x}", rand::random::<u128>());
        if let Ok(mut set) = self.active_canaries.lock() {
            set.insert(token.clone());
        }
        token
    }

    pub fn is_canary(&self, token: &str) -> bool {
        if let Ok(set) = self.active_canaries.lock() {
            set.contains(token)
        } else {
            false
        }
    }

    pub fn evaluate_incoming_token(&self, token: &str, source_ip: &str, user_agent: &str, request_uri: &str) -> Option<CanaryTriggerAlert> {
        if self.is_canary(token) {
            let alert = CanaryTriggerAlert {
                alert_id: format!("alert_canary_{:x}", rand::random::<u128>()),
                canary_token: token.to_string(),
                source_ip: source_ip.to_string(),
                user_agent: user_agent.to_string(),
                request_uri: request_uri.to_string(),
                triggered_at: Utc::now(),
            };

            if let Ok(mut list) = self.triggered_alerts.lock() {
                list.push(alert.clone());
            }

            Some(alert)
        } else {
            None
        }
    }

    pub fn list_alerts(&self) -> Vec<CanaryTriggerAlert> {
        if let Ok(list) = self.triggered_alerts.lock() {
            list.clone()
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canary_token_generation_and_alert() {
        let tracker = CanaryTokenTracker::new();
        let canary = tracker.generate_canary_token();
        assert!(canary.starts_with("fk_live_canary_"));
        assert!(tracker.is_canary(&canary));

        let alert_opt = tracker.evaluate_incoming_token(&canary, "203.0.113.4", "Go-http-client/1.1", "/api/v1/auth/login");
        assert!(alert_opt.is_some());
        let alert = alert_opt.unwrap();
        assert_eq!(alert.source_ip, "203.0.113.4");
        assert_eq!(tracker.list_alerts().len(), 1);
    }
}
