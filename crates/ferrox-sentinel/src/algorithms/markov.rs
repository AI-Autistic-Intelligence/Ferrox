//! # Markov Behavior Engine (`ferrox-sentinel::algorithms::markov`)
//!
//! Implements state-transition probability matrices ($P(S_{t+1} \mid S_t)$) and inter-action
//! timing analysis to detect automated bots and unnatural client action sequences.
//!
//! Literature reference: *Malware Data Science* (Saxe & Sanders), Chapter 8.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Action transition assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorAssessment {
    pub anomaly_score: f64,
    pub is_bot_sequence: bool,
    pub average_delay_ms: f64,
    pub lowest_transition_prob: f64,
    pub rationale: String,
}

/// State transition probability tracker for user/client API action sequences
#[derive(Debug, Clone)]
pub struct MarkovBehaviorEngine {
    /// Transition counts: `from_state -> (to_state -> count)`
    transition_counts: HashMap<String, HashMap<String, u32>>,
    /// State total transition counts
    state_totals: HashMap<String, u32>,
    /// Minimum threshold for inter-action delay considered human (ms)
    min_human_delay_ms: f64,
}

impl Default for MarkovBehaviorEngine {
    fn default() -> Self {
        let mut engine = Self {
            transition_counts: HashMap::new(),
            state_totals: HashMap::new(),
            min_human_delay_ms: 30.0,
        };
        engine.seed_baseline_transitions();
        engine
    }
}

impl MarkovBehaviorEngine {
    pub fn new(min_human_delay_ms: f64) -> Self {
        let mut engine = Self {
            transition_counts: HashMap::new(),
            state_totals: HashMap::new(),
            min_human_delay_ms,
        };
        engine.seed_baseline_transitions();
        engine
    }

    /// Seeds standard human baseline transitions
    fn seed_baseline_transitions(&mut self) {
        self.record_transition("Login", "Draw", 100);
        self.record_transition("Draw", "Meld", 40);
        self.record_transition("Draw", "Discard", 60);
        self.record_transition("Meld", "Discard", 90);
        self.record_transition("Discard", "Draw", 80);
    }

    /// Records an observed transition from state A to state B
    pub fn record_transition(&mut self, from: &str, to: &str, count: u32) {
        let entry = self.transition_counts.entry(from.to_string()).or_default();
        *entry.entry(to.to_string()).or_insert(0) += count;
        *self.state_totals.entry(from.to_string()).or_insert(0) += count;
    }

    /// Calculates transition probability $P(\text{to} \mid \text{from})$
    pub fn get_transition_probability(&self, from: &str, to: &str) -> f64 {
        if let Some(total) = self.state_totals.get(from) {
            if *total == 0 { return 0.01; }
            if let Some(to_map) = self.transition_counts.get(from) {
                if let Some(count) = to_map.get(to) {
                    return (*count as f64) / (*total as f64);
                }
            }
        }
        0.01 // Smoothing for unobserved transitions
    }

    /// Evaluates a sequence of states and timestamp delays (in ms)
    pub fn evaluate_sequence(&self, actions: &[&str], delays_ms: &[f64]) -> BehaviorAssessment {
        if actions.len() < 2 {
            return BehaviorAssessment {
                anomaly_score: 0.0,
                is_bot_sequence: false,
                average_delay_ms: 500.0,
                lowest_transition_prob: 1.0,
                rationale: "Sequence too short for evaluation".to_string(),
            };
        }

        let mut min_prob = 1.0f64;

        for window in actions.windows(2) {
            let p = self.get_transition_probability(window[0], window[1]);
            if p < min_prob { min_prob = p; }
        }

        let avg_delay: f64 = if !delays_ms.is_empty() {
            delays_ms.iter().sum::<f64>() / delays_ms.len() as f64
        } else {
            500.0
        };

        // Timing penalty if average delay is faster than human capability (< min_human_delay_ms)
        let timing_penalty = if avg_delay < self.min_human_delay_ms {
            ((self.min_human_delay_ms - avg_delay) / self.min_human_delay_ms).min(1.0)
        } else {
            0.0
        };

        // Low probability transition penalty
        let transition_penalty = (1.0 - min_prob).max(0.0);

        let composite_anomaly = (0.6 * timing_penalty + 0.4 * transition_penalty).clamp(0.0, 1.0);
        let is_bot_sequence = composite_anomaly >= 0.6 || avg_delay < (self.min_human_delay_ms * 0.5);

        let rationale = format!(
            "Avg Delay: {:.1}ms (Human Min: {:.1}ms), Min Transition Prob: {:.3}, Composite Anomaly: {:.2}",
            avg_delay, self.min_human_delay_ms, min_prob, composite_anomaly
        );

        BehaviorAssessment {
            anomaly_score: composite_anomaly,
            is_bot_sequence,
            average_delay_ms: avg_delay,
            lowest_transition_prob: min_prob,
            rationale,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_markov_normal_human_sequence() {
        let engine = MarkovBehaviorEngine::default();
        let actions = vec!["Login", "Draw", "Discard"];
        let delays = vec![400.0, 600.0];

        let result = engine.evaluate_sequence(&actions, &delays);
        assert!(!result.is_bot_sequence);
        assert!(result.anomaly_score < 0.4);
    }

    #[test]
    fn test_markov_superhuman_bot_sequence() {
        let engine = MarkovBehaviorEngine::default();
        let actions = vec!["Login", "Draw", "Discard"];
        let delays = vec![5.0, 4.0]; // 5ms delays!

        let result = engine.evaluate_sequence(&actions, &delays);
        assert!(result.is_bot_sequence);
        assert!(result.anomaly_score >= 0.6);
    }
}
