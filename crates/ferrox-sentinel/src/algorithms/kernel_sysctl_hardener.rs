//! # Kernel Sysctl Hardening Engine (`kernel_sysctl_hardener.rs`)
//!
//! Generates Linux kernel sysctl hardening configuration (/etc/sysctl.d/99-ferrox-kernel-hardening.conf)
//! to enforce OS-level zero-day kernel protections against local privilege escalation and spoofing.

use serde::{Deserialize, Serialize};

/// Kernel Sysctl Parameter Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysctlEntry {
    pub key: String,
    pub value: String,
    pub rationale: String,
}

/// Linux Kernel Sysctl Hardening Profile
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KernelSysctlProfile {
    pub entries: Vec<SysctlEntry>,
    pub generated_sysctl_conf: String,
}

/// Kernel Sysctl Hardener Engine
pub struct KernelSysctlHardenerEngine;

impl KernelSysctlHardenerEngine {
    /// Generates baseline Linux kernel sysctl hardening configuration
    pub fn generate_sysctl_profile() -> KernelSysctlProfile {
        let entries = vec![
            SysctlEntry {
                key: "net.ipv4.tcp_syncookies".to_string(),
                value: "1".to_string(),
                rationale: "Mitigate SYN flood DDoS attacks via TCP SYN cookies".to_string(),
            },
            SysctlEntry {
                key: "kernel.kptr_restrict".to_string(),
                value: "2".to_string(),
                rationale: "Hide kernel pointers from unprivileged users to prevent kernel exploit targeting".to_string(),
            },
            SysctlEntry {
                key: "kernel.yama.ptrace_scope".to_string(),
                value: "3".to_string(),
                rationale: "Restrict process debugging/ptrace to root only (prevents process injection)".to_string(),
            },
            SysctlEntry {
                key: "fs.protected_fifos".to_string(),
                value: "2".to_string(),
                rationale: "Prevent symlink / FIFO spoofing attacks in world-writable directories".to_string(),
            },
            SysctlEntry {
                key: "fs.protected_regular".to_string(),
                value: "2".to_string(),
                rationale: "Prevent unauthorized writing to regular files in shared temporary folders".to_string(),
            },
            SysctlEntry {
                key: "net.ipv4.conf.all.rp_filter".to_string(),
                value: "1".to_string(),
                rationale: "Enforce strict Reverse Path Forwarding to block IP spoofing".to_string(),
            },
        ];

        let mut conf = String::new();
        conf.push_str("# Ferrox Enterprise Linux Kernel Hardening Profile\n");
        conf.push_str("# Path: /etc/sysctl.d/99-ferrox-kernel-hardening.conf\n\n");

        for entry in &entries {
            conf.push_str(&format!("# {}\n{} = {}\n\n", entry.rationale, entry.key, entry.value));
        }

        KernelSysctlProfile {
            entries,
            generated_sysctl_conf: conf,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kernel_sysctl_profile_generation() {
        let profile = KernelSysctlHardenerEngine::generate_sysctl_profile();
        assert!(profile.entries.len() >= 6);
        assert!(profile.generated_sysctl_conf.contains("kernel.kptr_restrict = 2"));
        assert!(profile.generated_sysctl_conf.contains("net.ipv4.tcp_syncookies = 1"));
        assert!(profile.generated_sysctl_conf.contains("kernel.yama.ptrace_scope = 3"));
    }
}
