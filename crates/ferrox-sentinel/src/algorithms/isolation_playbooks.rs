//! # Self-Executing Threat Isolation Playbooks (`isolation_playbooks.rs`)
//!
//! Automated active defense rules that dynamically throttle connection velocity, isolate suspicious user sessions,
//! enable strict squeezer sanitization, or lockdown nodes when critical threat scores (> 0.85) are sustained.

use serde::{Deserialize, Serialize};

/// Defensive Active Response Actions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybookAction {
    ThrottleVelocity { max_req_per_sec: u32 },
    IsolateUserSession { session_id: String },
    EnableStrictSqueezer,
    LockdownNode { reason: String },
}

/// Threat Isolation Playbook Engine
pub struct ThreatIsolationPlaybook;

impl ThreatIsolationPlaybook {
    pub fn evaluate_threat_score(threat_score: f64, session_id: Option<&str>) -> Vec<PlaybookAction> {
        let mut actions = Vec::new();

        if threat_score >= 0.95 {
            actions.push(PlaybookAction::LockdownNode {
                reason: format!("Sustained Critical Anomaly Score ({:.2})", threat_score),
            });
            actions.push(PlaybookAction::EnableStrictSqueezer);
            if let Some(sid) = session_id {
                actions.push(PlaybookAction::IsolateUserSession {
                    session_id: sid.to_string(),
                });
            }
        } else if threat_score >= 0.85 {
            actions.push(PlaybookAction::ThrottleVelocity { max_req_per_sec: 1 });
            actions.push(PlaybookAction::EnableStrictSqueezer);
            if let Some(sid) = session_id {
                actions.push(PlaybookAction::IsolateUserSession {
                    session_id: sid.to_string(),
                });
            }
        } else if threat_score >= 0.65 {
            actions.push(PlaybookAction::ThrottleVelocity { max_req_per_sec: 5 });
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_isolation_playbook_evaluation() {
        let low_actions = ThreatIsolationPlaybook::evaluate_threat_score(0.20, None);
        assert!(low_actions.is_empty());

        let high_actions = ThreatIsolationPlaybook::evaluate_threat_score(0.88, Some("sess_9901"));
        assert_eq!(high_actions.len(), 3);
        assert!(high_actions.contains(&PlaybookAction::ThrottleVelocity { max_req_per_sec: 1 }));
        assert!(high_actions.contains(&PlaybookAction::IsolateUserSession { session_id: "sess_9901".to_string() }));

        let critical_actions = ThreatIsolationPlaybook::evaluate_threat_score(0.98, None);
        assert!(critical_actions.iter().any(|a| matches!(a, PlaybookAction::LockdownNode { .. })));
    }
}
