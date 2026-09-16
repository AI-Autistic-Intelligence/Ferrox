//! # ADWIN Concept Drift Detector (`ferrox-sentinel::algorithms::drift`)
//!
//! Implementation of the ADWIN (Adaptive Windowing) algorithm for real-time concept drift detection.
//! Monitors sliding window feature mean variations and emits `DriftAlert` when statistical shifts occur.
//! Reference: *Machine Learning Under Malware Attack* (Ch. 5) & *Machine Learning and Security*.

use serde::{Deserialize, Serialize};

/// Alert status for concept drift monitoring
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DriftStatus {
    NoDrift,
    Warning,
    DriftDetected,
}

/// An alert emitted when concept drift is evaluated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftAlert {
    pub status: DriftStatus,
    pub window_size: usize,
    pub variance: f64,
    pub delta_mean: f64,
}

/// ADWIN Adaptive Windowing Concept Drift Detector
pub struct ConceptDriftDetector {
    window: Vec<f64>,
    max_size: usize,
    delta: f64,
}

impl ConceptDriftDetector {
    /// Creates a new `ConceptDriftDetector` with specified maximum window size and confidence threshold delta
    pub fn new(max_size: usize, delta: f64) -> Self {
        Self {
            window: Vec::with_capacity(max_size),
            max_size,
            delta,
        }
    }

    /// Adds a new numerical feature observation to the sliding window and checks for drift
    pub fn update(&mut self, val: f64) -> DriftAlert {
        if self.window.len() >= self.max_size {
            self.window.remove(0);
        }
        self.window.push(val);

        if self.window.len() < 10 {
            return DriftAlert {
                status: DriftStatus::NoDrift,
                window_size: self.window.len(),
                variance: 0.0,
                delta_mean: 0.0,
            };
        }

        let n = self.window.len();
        let mid = n / 2;

        let w0 = &self.window[0..mid];
        let w1 = &self.window[mid..n];

        let mean0 = w0.iter().sum::<f64>() / w0.len() as f64;
        let mean1 = w1.iter().sum::<f64>() / w1.len() as f64;

        let delta_mean = (mean0 - mean1).abs();

        let mean_total = self.window.iter().sum::<f64>() / n as f64;
        let variance = self.window.iter().map(|x| (x - mean_total).powi(2)).sum::<f64>() / n as f64;

        // Cut threshold calculation: epsilon = sqrt((1 / 2m) * ln(4 / delta))
        let m = 1.0 / (1.0 / w0.len() as f64 + 1.0 / w1.len() as f64);
        let epsilon = ((1.0 / (2.0 * m)) * (4.0 / self.delta).ln()).sqrt();

        let status = if delta_mean > epsilon * 1.5 {
            // Trim old window entries upon drift detection
            self.window.drain(0..mid / 2);
            DriftStatus::DriftDetected
        } else if delta_mean > epsilon {
            DriftStatus::Warning
        } else {
            DriftStatus::NoDrift
        };

        DriftAlert {
            status,
            window_size: self.window.len(),
            variance,
            delta_mean,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concept_drift_detector() {
        let mut detector = ConceptDriftDetector::new(100, 0.05);

        // Stable distribution
        for _ in 0..30 {
            let alert = detector.update(0.1);
            assert_ne!(alert.status, DriftStatus::DriftDetected);
        }

        // Shift distribution (Abrupt concept drift)
        let mut drift_found = false;
        for _ in 0..40 {
            let alert = detector.update(10.0);
            if alert.status == DriftStatus::DriftDetected || alert.status == DriftStatus::Warning {
                drift_found = true;
                break;
            }
        }

        assert!(drift_found, "ADWIN should detect distribution shift concept drift!");
    }
}
