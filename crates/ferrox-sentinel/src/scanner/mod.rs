//! # Static File Upload & Byte Entropy Heuristic Scanner (`ferrox-sentinel::scanner`)
//!
//! Inspects file upload buffers for high-entropy packed obfuscation, executable magic headers (PE/ELF),
//! embedded polyglot web shell payloads, deception honeypot traps, and memory integrity watchdogs.

use serde::{Deserialize, Serialize};

pub mod honeypot_decoy;
pub mod integrity_watchdog;
pub mod canary_tokens;
pub mod honeynet_mesh;
pub mod self_healing;
pub use honeynet_mesh::{BlacklistEntry, HoneynetMeshRegistry, HoneynetTrapEvent};
pub use self_healing::{SanitizedStateSnapshot, SelfHealingEngine, SelfHealingEvent};

/// Result emitted by static file upload heuristic inspection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub safe: bool,
    pub entropy: f64,
    pub mime_detected: String,
    pub threat_indicators: Vec<String>,
}

/// Heuristic scanner for file upload security audit
pub struct FileHeuristicScanner;

impl FileHeuristicScanner {
    /// Inspects raw file bytes and returns a comprehensive `ScanResult`
    pub fn scan_buffer(bytes: &[u8], filename: &str) -> ScanResult {
        let mut threat_indicators = Vec::new();
        let entropy = crate::features::entropy::calculate_shannon_entropy(bytes);

        // 1. High entropy packed malware detection (> 7.5 bits per byte)
        if entropy > 7.5 {
            threat_indicators.push(format!("High entropy payload ({:.2} bits/byte): potential packed binary or encrypted shellcode", entropy));
        }

        // 2. Executable Header Magic Signature Check
        let mime_detected = if bytes.starts_with(b"MZ") {
            threat_indicators.push("Executable Windows PE binary header (MZ) detected in upload".to_string());
            "application/x-dosexec".to_string()
        } else if bytes.starts_with(b"\x7fELF") {
            threat_indicators.push("Executable Linux ELF binary header detected in upload".to_string());
            "application/x-executable".to_string()
        } else if bytes.starts_with(b"\xca\xfe\xba\xbe") {
            threat_indicators.push("Java Class / Mach-O binary header detected in upload".to_string());
            "application/java-vm".to_string()
        } else if bytes.starts_with(b"\x89PNG\r\n\x1a\n") {
            "image/png".to_string()
        } else if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
            "image/jpeg".to_string()
        } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
            "image/gif".to_string()
        } else if bytes.starts_with(b"PK\x03\x04") {
            "application/zip".to_string()
        } else {
            "application/octet-stream".to_string()
        };

        // 3. Polyglot & Script Injection Detection in image/text uploads
        let content_str = String::from_utf8_lossy(bytes).to_lowercase();
        if content_str.contains("<?php") || content_str.contains("<script") || content_str.contains("system(") || content_str.contains("eval(") {
            threat_indicators.push("Polyglot web shell payload script signatures detected inside buffer".to_string());
        }

        // 4. Filename extension mismatch check
        let filename_lower = filename.to_lowercase();
        if (filename_lower.ends_with(".png") || filename_lower.ends_with(".jpg")) && (mime_detected.contains("executable") || content_str.contains("<?php")) {
            threat_indicators.push("Filename extension mismatch: image extension used for executable payload".to_string());
        }

        let safe = threat_indicators.is_empty();

        ScanResult {
            safe,
            entropy,
            mime_detected,
            threat_indicators,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_heuristic_scanner_safe_image() {
        let png_bytes = b"\x89PNG\r\n\x1a\n\x00\x00\x00\x0dIHDR\x00\x00\x00\x01";
        let res = FileHeuristicScanner::scan_buffer(png_bytes, "avatar.png");
        assert!(res.safe);
        assert_eq!(res.mime_detected, "image/png");
    }

    #[test]
    fn test_file_heuristic_scanner_malicious_exe() {
        let exe_bytes = b"MZ\x90\x00\x03\x00\x00\x00\x04\x00\x00\x00\xff\xff";
        let res = FileHeuristicScanner::scan_buffer(exe_bytes, "avatar.png");
        assert!(!res.safe);
        assert!(res.threat_indicators.iter().any(|i| i.contains("Windows PE")));
    }
}
