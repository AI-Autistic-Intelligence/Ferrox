//! # Differential Privacy Engine (`ferrox-logger::dp`)
//!
//! Provides Laplace Noise mechanism for exporting metric telemetry and security benchmarks
//! under $(\epsilon, \delta)$-Differential Privacy guarantees.
//! Reference: *Machine Learning and Security* (Ch. 8 - Privacy-Preserving Machine Learning).

use rand::Rng;

/// Differential Privacy Engine using Laplace distribution noise sampling
pub struct DifferentialPrivacyEngine;

impl DifferentialPrivacyEngine {
    /// Generates a noise sample from a Laplace distribution with scale $b = \Delta f / \epsilon$
    pub fn laplace_sample(scale: f64) -> f64 {
        let mut rng = rand::thread_rng();
        let u: f64 = rng.gen_range(-0.4999..0.4999);
        let sgn: f64 = if u < 0.0 { -1.0 } else { 1.0 };
        let abs_u: f64 = u.abs();
        let term: f64 = (1.0 - 2.0 * abs_u).max(1e-10);
        -scale * sgn * term.ln()
    }

    /// Adds Laplace noise to a numerical telemetry metric to preserve $(\epsilon, \delta)$-privacy
    pub fn sanitize_metric(value: f64, epsilon: f64, sensitivity: f64) -> f64 {
        if epsilon <= 0.0 {
            return value;
        }
        let scale = sensitivity / epsilon;
        let noise = Self::laplace_sample(scale);
        (value + noise).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_differential_privacy_engine() {
        let original_val = 100.0;
        let epsilon = 1.0;
        let sensitivity = 1.0;

        let sanitized = DifferentialPrivacyEngine::sanitize_metric(original_val, epsilon, sensitivity);
        assert!(sanitized >= 0.0);
        // Sanitized value should be close to original value with bounded noise
        assert!((sanitized - original_val).abs() < 25.0);
    }
}
