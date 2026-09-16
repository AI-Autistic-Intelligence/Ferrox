use std::collections::HashMap;

/// Calculates Shannon Entropy H(X) = - \sum p(x) log2 p(x) for a given byte slice or string.
/// Returns a float between 0.0 (completely uniform/single byte) and ~8.0 (maximum byte randomness).
pub fn calculate_shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut byte_counts = HashMap::new();
    for &byte in data {
        *byte_counts.entry(byte).or_insert(0usize) += 1;
    }

    let len_f = data.len() as f64;
    let mut entropy = 0.0;

    for &count in byte_counts.values() {
        let p = count as f64 / len_f;
        if p > 0.0 {
            entropy -= p * p.log2();
        }
    }

    entropy
}

/// Evaluates payload entropy characteristics to flag suspicious obfuscation, shellcode, or injection patterns.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntropyAnalysis {
    pub entropy_score: f64,
    pub is_suspicious: f64, // Normalized score 0.0 to 1.0
    pub rationale: String,
}

pub fn evaluate_payload_entropy(payload: &str) -> EntropyAnalysis {
    let entropy = calculate_shannon_entropy(payload.as_bytes());

    let payload_upper = payload.to_uppercase();
    let has_injection_keyword = payload_upper.contains("UNION")
        || payload_upper.contains("SELECT")
        || payload_upper.contains("<SCRIPT>")
        || payload_upper.contains("OR 1=1")
        || payload_upper.contains("CHAR(");

    let mut suspicion = if entropy > 4.2 {
        ((entropy - 4.2) / 2.5).min(1.0)
    } else {
        0.0
    };

    if has_injection_keyword {
        suspicion = (suspicion + 0.75).min(1.0);
    }

    let rationale = if suspicion > 0.4 {
        format!(
            "High Shannon entropy ({:.2} bits/byte) or suspicious injection keywords detected.",
            entropy
        )
    } else {
        "Normal character entropy distribution.".to_string()
    };

    EntropyAnalysis {
        entropy_score: entropy,
        is_suspicious: suspicion,
        rationale,
    }
}
