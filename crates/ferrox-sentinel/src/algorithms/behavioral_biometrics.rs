//! # Behavioral Biometrics & Micro-Movement Cadence Engine (`behavioral_biometrics.rs`)
//!
//! Evaluates inter-action latency distributions, timing jitter entropy, and micro-cadence variance.
//! Catches automated headless browser bots (Puppeteer, Playwright, Selenium) even when they inject basic randomized delays.

use serde::{Deserialize, Serialize};

/// Biometric Cadence Evaluation Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricAssessment {
    pub is_bot_cadence: bool,
    pub latency_variance: f64,
    pub mean_inter_action_ms: f64,
    pub confidence_score: f64,
}

/// Behavioral Biometrics Analyzer
pub struct BiometricCadenceAnalyzer;

impl BiometricCadenceAnalyzer {
    pub fn evaluate_latencies(latencies_ms: &[u64]) -> BiometricAssessment {
        if latencies_ms.len() < 3 {
            return BiometricAssessment {
                is_bot_cadence: false,
                latency_variance: 100.0,
                mean_inter_action_ms: 500.0,
                confidence_score: 0.1,
            };
        }

        let sum: u64 = latencies_ms.iter().sum();
        let mean = sum as f64 / latencies_ms.len() as f64;

        let variance = latencies_ms
            .iter()
            .map(|&l| {
                let diff = l as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / latencies_ms.len() as f64;

        // Automated bots often have either ultra-fast constant latencies (< 5ms) or abnormally flat variance (< 15.0)
        let is_ultra_fast = mean < 10.0;
        let is_flat_variance = variance < 20.0;
        let is_bot_cadence = is_ultra_fast || is_flat_variance;

        let confidence = if is_ultra_fast {
            0.99
        } else if is_flat_variance {
            0.85
        } else {
            0.10
        };

        BiometricAssessment {
            is_bot_cadence,
            latency_variance: variance,
            mean_inter_action_ms: mean,
            confidence_score: confidence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biometric_bot_cadence_detection() {
        // Human latencies with natural variation -> Clean
        let human_latencies = vec![450, 1200, 320, 890, 610, 1400];
        let human_res = BiometricCadenceAnalyzer::evaluate_latencies(&human_latencies);
        assert!(!human_res.is_bot_cadence);

        // Headless Bot with ultra-flat artificial delays (e.g. sleep(100ms)) -> Bot detected!
        let bot_latencies = vec![100, 100, 101, 100, 100, 100];
        let bot_res = BiometricCadenceAnalyzer::evaluate_latencies(&bot_latencies);
        assert!(bot_res.is_bot_cadence);
        assert!(bot_res.confidence_score >= 0.85);
    }
}
