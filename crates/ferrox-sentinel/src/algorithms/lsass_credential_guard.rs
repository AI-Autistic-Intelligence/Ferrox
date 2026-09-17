//! # LSASS Process Handle & Credential Dumping Guard (`lsass_credential_guard.rs`)
//!
//! Intercepts process handle opening and duplication telemetry targeting LSASS/Security Authority processes
//! (`ObRegisterCallbacks` concept, *Evading EDR*, No Starch Press).

use serde::{Deserialize, Serialize};

/// Process Handle Access Telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessHandleAccessTelemetry {
    pub source_pid: u32,
    pub target_process_name: String,
    pub requested_access_mask: u32, // e.g. PROCESS_VM_READ (0x0010), PROCESS_DUP_HANDLE (0x0040)
    pub is_signed_binary: bool,
}

/// Credential Dumping Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialDumpAlert {
    pub source_pid: u32,
    pub target_process: String,
    pub threat_severity: f64,
    pub rationale: String,
}

/// LSASS Process Handle & Credential Dumping Guard Engine
pub struct LsassCredentialGuardEngine;

impl LsassCredentialGuardEngine {
    /// Inspects process handle access requests targeting security authority processes
    pub fn inspect_handle_access(telemetry: &ProcessHandleAccessTelemetry) -> Option<CredentialDumpAlert> {
        let is_target_lsass = telemetry.target_process_name.to_lowercase().contains("lsass")
            || telemetry.target_process_name.to_lowercase().contains("winlogon")
            || telemetry.target_process_name.to_lowercase().contains("services.exe");

        if is_target_lsass {
            // Check for PROCESS_VM_READ (0x0010) or PROCESS_CREATE_PROCESS (0x0080) from unsigned processes
            let vm_read = (telemetry.requested_access_mask & 0x0010) != 0;
            let dup_handle = (telemetry.requested_access_mask & 0x0040) != 0;

            if (vm_read || dup_handle) && !telemetry.is_signed_binary {
                return Some(CredentialDumpAlert {
                    source_pid: telemetry.source_pid,
                    target_process: telemetry.target_process_name.clone(),
                    threat_severity: 0.99,
                    rationale: format!(
                        "Unsigned Process (PID {}) requested VM_READ/DUP_HANDLE access on {}",
                        telemetry.source_pid, telemetry.target_process_name
                    ),
                });
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsass_credential_dump_detection() {
        let telemetry = ProcessHandleAccessTelemetry {
            source_pid: 4096,
            target_process_name: "lsass.exe".to_string(),
            requested_access_mask: 0x0010 | 0x0040, // VM_READ + DUP_HANDLE
            is_signed_binary: false,               // Unsigned malicious process!
        };

        let alert = LsassCredentialGuardEngine::inspect_handle_access(&telemetry);
        assert!(alert.is_some());
        let a = alert.unwrap();
        assert_eq!(a.threat_severity, 0.99);
        assert!(a.rationale.contains("lsass.exe"));
    }
}
