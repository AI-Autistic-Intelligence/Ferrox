//! # EDR Telemetry & Process Integrity Guard (`etwti_telemetry_guard.rs`)
//!
//! Implements telemetry callbacks, DLL hooking integrity verification, and direct syscall/EtwTi
//! evasion detectors inspired by endpoint detection & response architecture (*Evading EDR*, No Starch Press).

use serde::{Deserialize, Serialize};

/// Process Memory Execution Region Telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryRegionTelemetry {
    pub process_id: u32,
    pub module_name: String,
    pub is_executable: bool,
    pub is_hooked: bool,
    pub contains_direct_syscall: bool,
}

/// EDR Integrity Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdrIntegrityAlert {
    pub process_id: u32,
    pub anomaly_type: String,
    pub threat_severity: f64,
}

/// EDR Telemetry & Process Integrity Guard Engine
pub struct EtwTiTelemetryGuardEngine;

impl EtwTiTelemetryGuardEngine {
    /// Evaluates process memory region telemetry for function hooking tampering or direct syscall bypasses
    pub fn inspect_memory_telemetry(telemetry: &MemoryRegionTelemetry) -> Option<EdrIntegrityAlert> {
        // Direct Syscall Evasion Detector: Unhooked ntdll or raw syscall instruction in unbacked memory
        if telemetry.contains_direct_syscall && !telemetry.is_hooked {
            return Some(EdrIntegrityAlert {
                process_id: telemetry.process_id,
                anomaly_type: "Direct Syscall EDR Bypass Attempt (EtwTi Detection)".to_string(),
                threat_severity: 0.95,
            });
        }

        // DLL Unhooking Anomaly Detector: Native API trampoline stripped in user space
        if telemetry.module_name.contains("ntdll.dll") && !telemetry.is_hooked && telemetry.is_executable {
            return Some(EdrIntegrityAlert {
                process_id: telemetry.process_id,
                anomaly_type: "NTDLL User-Space Hook Removal Anomaly".to_string(),
                threat_severity: 0.85,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_etwti_direct_syscall_detection() {
        let telemetry = MemoryRegionTelemetry {
            process_id: 1337,
            module_name: "unbacked_memory".to_string(),
            is_executable: true,
            is_hooked: false,
            contains_direct_syscall: true,
        };

        let alert = EtwTiTelemetryGuardEngine::inspect_memory_telemetry(&telemetry);
        assert!(alert.is_some());
        let a = alert.unwrap();
        assert_eq!(a.threat_severity, 0.95);
        assert!(a.anomaly_type.contains("Direct Syscall"));
    }
}
