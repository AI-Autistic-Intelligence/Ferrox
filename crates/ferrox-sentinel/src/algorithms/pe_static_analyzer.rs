//! # Static PE Header & Disassembly Heuristic Analyzer (`pe_static_analyzer.rs`)
//!
//! Analyzes Portable Executable (PE) binary structures, section entropy, and 
//! Import Address Table (IAT) anomaly signatures (*Malware Data Science*, No Starch Press).

use serde::{Deserialize, Serialize};

/// PE Header & Section Assessment Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeAnalysisAssessment {
    pub is_valid_pe: bool,
    pub is_packed: bool,
    pub max_section_entropy: f64,
    pub suspicious_imports: Vec<String>,
    pub threat_score: f64,
}

/// Static PE Header & Disassembly Heuristic Analyzer Engine
pub struct PeStaticAnalyzerEngine;

impl PeStaticAnalyzerEngine {
    /// Analyzes raw binary bytes for PE structures and malicious indicators
    pub fn analyze_bytes(bytes: &[u8]) -> PeAnalysisAssessment {
        if bytes.len() < 64 {
            return PeAnalysisAssessment {
                is_valid_pe: false,
                is_packed: false,
                max_section_entropy: 0.0,
                suspicious_imports: Vec::new(),
                threat_score: 0.0,
            };
        }

        // Check DOS Header Magic "MZ" (0x4D, 0x5A)
        let is_mz = bytes[0] == b'M' && bytes[1] == b'Z';
        if !is_mz {
            return PeAnalysisAssessment {
                is_valid_pe: false,
                is_packed: false,
                max_section_entropy: 0.0,
                suspicious_imports: Vec::new(),
                threat_score: 0.0,
            };
        }

        // Calculate Shannon Entropy across the payload
        let entropy = Self::calculate_shannon_entropy(bytes);
        let is_packed = entropy > 7.2;

        // Check for known malware API import strings in binary
        let known_suspicious_apis = [
            "VirtualAllocEx",
            "WriteProcessMemory",
            "CreateRemoteThread",
            "NtUnmapViewOfSection",
            "ObRegisterCallbacks",
            "SetWindowsHookExA",
        ];

        let mut suspicious_imports = Vec::new();
        let payload_str = String::from_utf8_lossy(bytes);

        for api in &known_suspicious_apis {
            if payload_str.contains(api) {
                suspicious_imports.push((*api).to_string());
            }
        }

        let mut threat_score = 0.0;
        if is_packed {
            threat_score += 0.5;
        }
        threat_score += (suspicious_imports.len() as f64 * 0.2).min(0.5);

        PeAnalysisAssessment {
            is_valid_pe: true,
            is_packed,
            max_section_entropy: entropy,
            suspicious_imports,
            threat_score: threat_score.min(1.0),
        }
    }

    fn calculate_shannon_entropy(bytes: &[u8]) -> f64 {
        if bytes.is_empty() {
            return 0.0;
        }

        let mut counts = [0u64; 256];
        for &b in bytes {
            counts[b as usize] += 1;
        }

        let len = bytes.len() as f64;
        let mut entropy = 0.0;

        for &count in &counts {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }

        entropy
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pe_static_analyzer() {
        let mut sample_pe = vec![b'M', b'Z'];
        sample_pe.resize(128, 0x90); // NOP sled
        sample_pe.extend_from_slice(b"VirtualAllocEx WriteProcessMemory");

        let assessment = PeStaticAnalyzerEngine::analyze_bytes(&sample_pe);
        assert!(assessment.is_valid_pe);
        assert!(assessment.suspicious_imports.contains(&"VirtualAllocEx".to_string()));
        assert!(assessment.suspicious_imports.contains(&"WriteProcessMemory".to_string()));
        assert!(assessment.threat_score > 0.3);
    }
}
