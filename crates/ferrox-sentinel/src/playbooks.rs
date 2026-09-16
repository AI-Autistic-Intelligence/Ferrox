use crate::{ThreatAssessment, ThreatLevel};
use serde::{Deserialize, Serialize};

/// Automated Security Incident Response Action
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RemediationAction {
    LogTelemetry,
    EnforceStrictHeaders,
    TriggerMtdSeedRotation,
    RevokeSessionTokens,
    BlockClientIp,
}

/// Automated Incident Response State Machine (SOAR Concept)
pub struct IncidentPlaybook;

impl IncidentPlaybook {
    /// Determines automated remediation actions based on threat assessment level
    pub fn determine_remediation(assessment: &ThreatAssessment) -> Vec<RemediationAction> {
        let mut actions = vec![RemediationAction::LogTelemetry];

        match assessment.level {
            ThreatLevel::Benign => {},
            ThreatLevel::LowRisk => {
                actions.push(RemediationAction::EnforceStrictHeaders);
            }
            ThreatLevel::MediumRisk => {
                actions.push(RemediationAction::EnforceStrictHeaders);
                actions.push(RemediationAction::TriggerMtdSeedRotation);
            }
            ThreatLevel::HighRisk => {
                actions.push(RemediationAction::EnforceStrictHeaders);
                actions.push(RemediationAction::TriggerMtdSeedRotation);
                actions.push(RemediationAction::RevokeSessionTokens);
            }
            ThreatLevel::CriticalAnomaly => {
                actions.push(RemediationAction::EnforceStrictHeaders);
                actions.push(RemediationAction::TriggerMtdSeedRotation);
                actions.push(RemediationAction::RevokeSessionTokens);
                actions.push(RemediationAction::BlockClientIp);
            }
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incident_playbook_escalation() {
        let assessment = ThreatAssessment {
            client_ip: "10.0.0.1".to_string(),
            threat_score: 0.95,
            level: ThreatLevel::CriticalAnomaly,
            entropy_score: 6.2,
            velocity_zscore: 4.5,
            isolation_anomaly_score: 0.8,
            cluster_campaign_detected: true,
            rationale: "Critical SQLi and velocity spike".to_string(),
        };

        let actions = IncidentPlaybook::determine_remediation(&assessment);
        assert!(actions.contains(&RemediationAction::BlockClientIp));
        assert!(actions.contains(&RemediationAction::TriggerMtdSeedRotation));
    }
}
