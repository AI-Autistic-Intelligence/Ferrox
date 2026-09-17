//! # Static PE Header & Heuristic Disassembly Analyzer (`pe_static_analyzer.rs`)
//!
//! Analyzes Portable Executable (PE) header structures, section entropy, and suspicious disassembly import graphs
//! (*Malware Data Science*, Saxe & Sanders; *Machine Learning & Security*, O'Reilly).

use serde::{Deserialize, Serialize};

/// PE Section Header Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeSectionInfo {
    pub name: String,
    pub virtual_size: u32,
    pub raw_data_size: u32,
    pub entropy: f64,
}

/// PE File Analysis Assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeAnalysisAssessment {
    pub is_suspicious: bool,
    pub is_packed_or_encrypted: bool,
    pub section_size_discrepancy: bool,
    pub overall_threat_score: f64,
    pub suspicious_imports: Vec<String>,
}

/// Static PE Header & Heuristic Disassembly Analyzer
pub struct PeStaticAnalyzer;

impl PeStaticAnalyzer {
    /// Inspects PE headers, section entropy, and import tables for static malware heuristics
    pub fn inspect_pe_binary(
        sections: &[PeSectionInfo],
        import_table: &[&str],
    ) -> PeAnalysisAssessment {
        let mut is_packed_or_encrypted = false;
        let mut section_size_discrepancy = false;
        let mut suspicious_imports = Vec::new();

        // 1. High Section Entropy Detection (Packed / Encrypted Code)
        for sec in sections {
            if sec.entropy > 7.2 {
                is_packed_or_encrypted = true;
            }
            // Discrepancy between virtual size and raw data size (often indicates process hollowing allocation)
            if sec.virtual_size > 0 && sec.raw_data_size > 0 {
                let ratio = (sec.virtual_size as f64) / (sec.raw_data_size as f64);
                if ratio > 10.0 || ratio < 0.1 {
                    section_size_discrepancy = true;
                }
            }
        }

        // 2. Suspicious Win32 API Import Analysis
        let high_risk_apis = [
            "VirtualAllocEx",
            "WriteProcessMemory",
            "CreateRemoteThread",
            "NtUnmapViewOfSection",
            "QueueUserAPC",
            "SetThreadContext",
        ];

        for &imp in import_table {
            if high_risk_apis.contains(&imp) {
                suspicious_imports.push(imp.to_string());
            }
        }

        let threat_score = (
            if is_packed_or_encrypted { 0.40 } else { 0.0 }
            + if section_size_discrepancy { 0.35 } else { 0.0 }
            + (suspicious_imports.len() as f64 * 0.15).min(0.45)
        ).clamp(0.0, 1.0);

        let is_suspicious = threat_score >= 0.50;

        PeAnalysisAssessment {
            is_suspicious,
            is_packed_or_encrypted,
            section_size_discrepancy,
            overall_threat_score: threat_score,
            suspicious_imports,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pe_static_analysis() {
        let sections = vec![
            PeSectionInfo {
                name: ".text".to_string(),
                virtual_size: 500_000,
                raw_data_size: 1_000, // Massive discrepancy ratio (500:1) -> Process Hollowing Allocation!
                entropy: 7.85,        // High Entropy -> Packed!
            }
        ];

        let imports = vec!["VirtualAllocEx", "WriteProcessMemory", "CreateRemoteThread"];

        let res = PeStaticAnalyzer::inspect_pe_binary(&sections, &imports);
        assert!(res.is_suspicious);
        assert!(res.is_packed_or_encrypted);
        assert!(res.section_size_discrepancy);
        assert_eq!(res.suspicious_imports.len(), 3);
        assert!(res.overall_threat_score >= 0.75);
    }
}
