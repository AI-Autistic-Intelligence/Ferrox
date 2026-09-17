//! # SOAR Auto-Remediation & Adaptive VPS Gateway Engine (`soar_vps_enforcer.rs`)
//!
//! Synthesizes continuous security metrics, automated SOAR playbooks, and Founder VPS Gateway enforcement
//! (*Intelligent Continuous Security*, O'Reilly; *Ferrox Founder Security Suite*).

use serde::{Deserialize, Serialize};

/// SOAR Automated Action Vector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoarActionPlan {
    pub client_ip: String,
    pub generate_nftables_drop: bool,
    pub generate_ebpf_xdp_drop: bool,
    pub trigger_mtd_port_mutation: bool,
    pub revoke_zk_attestation: bool,
    pub action_rationale: String,
}

/// SOAR Auto-Remediation & Adaptive VPS Enforcer Engine
pub struct SoarVpsEnforcerEngine;

impl SoarVpsEnforcerEngine {
    /// Evaluates multi-vector threat indicators and constructs an automated SOAR execution plan
    pub fn evaluate_threat_remediation(
        client_ip: &str,
        composite_threat_score: f64,
        is_direct_syscall_evasion: bool,
        is_prompt_injection: bool,
        is_dns_tunneling: bool,
    ) -> SoarActionPlan {
        let mut generate_nftables_drop = false;
        let mut generate_ebpf_xdp_drop = false;
        let mut trigger_mtd_port_mutation = false;
        let mut revoke_zk_attestation = false;
        let mut rationale_parts = Vec::new();

        if composite_threat_score >= 0.85 || is_direct_syscall_evasion {
            generate_nftables_drop = true;
            generate_ebpf_xdp_drop = true;
            revoke_zk_attestation = true;
            rationale_parts.push("Critical Anomaly / Kernel Evasion Detected");
        }

        if is_dns_tunneling || is_prompt_injection {
            trigger_mtd_port_mutation = true;
            generate_nftables_drop = true;
            rationale_parts.push("Protocol Exfiltration / Prompt Injection Attack");
        }

        let action_rationale = if rationale_parts.is_empty() {
            "Benign Execution - No Remediation Required".to_string()
        } else {
            rationale_parts.join(" | ")
        };

        SoarActionPlan {
            client_ip: client_ip.to_string(),
            generate_nftables_drop,
            generate_ebpf_xdp_drop,
            trigger_mtd_port_mutation,
            revoke_zk_attestation,
            action_rationale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_soar_auto_remediation_execution() {
        let plan = SoarVpsEnforcerEngine::evaluate_threat_remediation(
            "198.51.100.44",
            0.92,
            true,  // Direct Syscall Evasion!
            false,
            true,  // DNS Tunneling!
        );

        assert!(plan.generate_nftables_drop);
        assert!(plan.generate_ebpf_xdp_drop);
        assert!(plan.trigger_mtd_port_mutation);
        assert!(plan.revoke_zk_attestation);
        assert!(plan.action_rationale.contains("Kernel Evasion"));
    }
}
