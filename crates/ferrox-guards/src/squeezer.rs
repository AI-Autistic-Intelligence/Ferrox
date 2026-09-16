//! # Adversarial Feature Squeezer (`ferrox-guards::squeezer`)
//!
//! Implements payload canonicalization and feature squeezing to defend against
//! adversarial evasion attacks (e.g., zero-width Unicode injection, URL double-encoding,
//! and whitespace obfuscation).
//!
//! Literature reference: *ML Under Malware Attack* (Vorobeychik & Kantarcioglu).

use serde::{Deserialize, Serialize};

/// Result of an adversarial evasion check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqueezerAssessment {
    pub is_evasion_detected: bool,
    pub original_length: usize,
    pub squeezed_length: usize,
    pub zero_width_chars_removed: usize,
    pub squeezed_payload: String,
    pub rationale: String,
}

/// Feature Squeezer for input canonicalization and defense
#[derive(Debug, Clone, Default)]
pub struct FeatureSqueezer;

impl FeatureSqueezer {
    pub fn new() -> Self {
        Self
    }

    /// Squeezes and canonicalizes a payload string to eliminate adversarial noise
    pub fn squeeze_payload(&self, input: &str) -> SqueezerAssessment {
        let original_len = input.len();
        let mut zero_width_count = 0;
        let mut cleaned = String::with_capacity(original_len);

        // 1. Strip zero-width Unicode characters and control codes
        for ch in input.chars() {
            match ch {
                '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{200E}' | '\u{200F}' | '\u{FEFF}' | '\u{00AD}' => {
                    zero_width_count += 1;
                }
                c if c.is_control() && c != '\n' && c != '\r' && c != '\t' => {
                    zero_width_count += 1;
                }
                c => {
                    cleaned.push(c);
                }
            }
        }

        // 2. Canonicalize URL percentage encodings
        let unencoded = Self::decode_url_percent(&cleaned);

        // 3. Compress consecutive whitespace
        let squeezed = Self::compress_whitespace(&unencoded);

        let is_evasion = zero_width_count > 0 || (original_len > 0 && (squeezed.len() as f64 / original_len as f64) < 0.6);

        let rationale = if zero_width_count > 0 {
            format!("Removed {} zero-width/control characters used in evasion attempt", zero_width_count)
        } else if is_evasion {
            "Extreme payload compression ratio detected".to_string()
        } else {
            "Payload clean and canonicalized".to_string()
        };

        SqueezerAssessment {
            is_evasion_detected: is_evasion,
            original_length: original_len,
            squeezed_length: squeezed.len(),
            zero_width_chars_removed: zero_width_count,
            squeezed_payload: squeezed,
            rationale,
        }
    }

    fn compress_whitespace(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let mut in_space = false;
        for c in s.chars() {
            if c.is_whitespace() {
                if !in_space {
                    result.push(' ');
                    in_space = true;
                }
            } else {
                result.push(c);
                in_space = false;
            }
        }
        result.trim().to_string()
    }

    fn decode_url_percent(s: &str) -> String {
        let mut out = String::with_capacity(s.len());
        let bytes = s.as_bytes();
        let mut i = 0;
        while i < bytes.len() {
            if bytes[i] == b'%' && i + 2 < bytes.len() {
                if let (Some(h1), Some(h2)) = (Self::hex_val(bytes[i + 1]), Self::hex_val(bytes[i + 2])) {
                    let decoded_byte = (h1 << 4) | h2;
                    out.push(decoded_byte as char);
                    i += 3;
                    continue;
                }
            }
            out.push(bytes[i] as char);
            i += 1;
        }
        out
    }

    fn hex_val(b: u8) -> Option<u8> {
        match b {
            b'0'..=b'9' => Some(b - b'0'),
            b'a'..=b'f' => Some(b - b'a' + 10),
            b'A'..=b'F' => Some(b - b'A' + 10),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squeezer_zero_width_evasion() {
        let squeezer = FeatureSqueezer::new();
        // Insert zero-width space in "SELECT"
        let malicious_input = "S\u{200B}E\u{200B}L\u{200B}E\u{200B}C\u{200B}T * FROM users";
        let res = squeezer.squeeze_payload(malicious_input);

        assert!(res.is_evasion_detected);
        assert_eq!(res.zero_width_chars_removed, 5);
        assert_eq!(res.squeezed_payload, "SELECT * FROM users");
    }

    #[test]
    fn test_squeezer_percent_encoding_canonicalization() {
        let squeezer = FeatureSqueezer::new();
        let input = "UNION%20SELECT%201,%202";
        let res = squeezer.squeeze_payload(input);

        assert_eq!(res.squeezed_payload, "UNION SELECT 1, 2");
    }
}
