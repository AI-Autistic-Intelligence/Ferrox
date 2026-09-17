//! # Network Protocol Frame & State Machine Sanitizer (`protocol_fuzzer_sanitizer.rs`)
//!
//! Inspects binary framing payloads (gRPC, HTTP/2, custom TCP streams) for length manipulation,
//! invalid protocol state transitions, and fuzzing payloads (*Attacking Network Protocols*, No Starch Press).

use serde::{Deserialize, Serialize};

/// Network Binary Frame Metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryFrameHeader {
    pub declared_length: u32,
    pub actual_bytes_received: u32,
    pub stream_id: u32,
    pub frame_type: u8,
    pub flags: u8,
}

/// Protocol Frame Validation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolSanitizeResult {
    pub is_valid: bool,
    pub anomaly_type: String,
    pub risk_score: f64,
}

/// Protocol Fuzzer & Framing Sanitizer
pub struct ProtocolFuzzerSanitizerEngine;

impl ProtocolFuzzerSanitizerEngine {
    /// Validates frame header consistency and guards against integer overflow / length truncation attacks
    pub fn sanitize_frame(header: &BinaryFrameHeader) -> ProtocolSanitizeResult {
        // Length Mismatch / Buffer Underflow Fuzzing Attack
        if header.declared_length > 10_000_000 {
            return ProtocolSanitizeResult {
                is_valid: false,
                anomaly_type: "Excessive Frame Length Allocation (DDoS Vector)".to_string(),
                risk_score: 0.90,
            };
        }

        if header.declared_length > header.actual_bytes_received + 65536 {
            return ProtocolSanitizeResult {
                is_valid: false,
                anomaly_type: "Frame Length Discrepancy / Truncation Fuzzing Payload".to_string(),
                risk_score: 0.85,
            };
        }

        // Invalid Stream ID Flag Manipulation
        if header.stream_id == 0 && header.frame_type != 0x0 {
            return ProtocolSanitizeResult {
                is_valid: false,
                anomaly_type: "Invalid Control Frame Stream ID Assignment".to_string(),
                risk_score: 0.75,
            };
        }

        ProtocolSanitizeResult {
            is_valid: true,
            anomaly_type: "Clean Protocol Frame".to_string(),
            risk_score: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_protocol_fuzzing_frame_sanitization() {
        let malicious_header = BinaryFrameHeader {
            declared_length: 50_000_000, // 50 MB declared frame length!
            actual_bytes_received: 1024,
            stream_id: 1,
            frame_type: 0x1,
            flags: 0x0,
        };

        let result = ProtocolFuzzerSanitizerEngine::sanitize_frame(&malicious_header);
        assert!(!result.is_valid);
        assert!(result.risk_score >= 0.85);
        assert!(result.anomaly_type.contains("Excessive Frame Length"));
    }
}
