//! # Self-Optimizing Threat Weight Reinforcement Engine (`reinforcement_tuner.rs`)
//!
//! Dynamic threat weight optimization via Multi-Armed Bandits / Reinforcement Learning principles.
//! Dynamically adjusts weights between Shannon Entropy, Velocity Z-Score, Isolation Forest, and Markov Bot detection based on feedback loops.

use serde::{Deserialize, Serialize};

/// Dynamic Sentinel Algorithm Weights State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunedAlgorithmWeights {
    pub entropy_weight: f64,
    pub velocity_weight: f64,
    pub isolation_weight: f64,
    pub markov_weight: f64,
    pub total_tuning_episodes: usize,
}

/// Reinforcement Learning Multi-Armed Bandit Tuner
pub struct MultiArmedBanditTuner {
    pub weights: TunedAlgorithmWeights,
    learning_rate: f64,
}

impl Default for MultiArmedBanditTuner {
    fn default() -> Self {
        Self::new()
    }
}

impl MultiArmedBanditTuner {
    pub fn new() -> Self {
        Self {
            weights: TunedAlgorithmWeights {
                entropy_weight: 0.40,
                velocity_weight: 0.30,
                isolation_weight: 0.20,
                markov_weight: 0.10,
                total_tuning_episodes: 0,
            },
            learning_rate: 0.05,
        }
    }

    pub fn tune_weights(&mut self, feedback_reward: f64) -> TunedAlgorithmWeights {
        self.weights.total_tuning_episodes += 1;

        // Apply positive/negative reward update
        let adjustment = self.learning_rate * feedback_reward;
        self.weights.entropy_weight = (self.weights.entropy_weight + adjustment).max(0.10).min(0.60);
        self.weights.velocity_weight = (self.weights.velocity_weight + adjustment * 0.5).max(0.10).min(0.50);
        self.weights.isolation_weight = (self.weights.isolation_weight - adjustment * 0.2).max(0.10).min(0.40);

        // Normalize weights to sum to 1.0
        let sum = self.weights.entropy_weight + self.weights.velocity_weight + self.weights.isolation_weight + self.weights.markov_weight;
        self.weights.entropy_weight /= sum;
        self.weights.velocity_weight /= sum;
        self.weights.isolation_weight /= sum;
        self.weights.markov_weight /= sum;

        self.weights.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_armed_bandit_tuner() {
        let mut tuner = MultiArmedBanditTuner::new();
        let initial_episodes = tuner.weights.total_tuning_episodes;

        let updated = tuner.tune_weights(1.0); // Positive reward feedback
        assert_eq!(updated.total_tuning_episodes, initial_episodes + 1);

        let sum = updated.entropy_weight + updated.velocity_weight + updated.isolation_weight + updated.markov_weight;
        assert!((sum - 1.0).abs() < 1e-5);
    }
}
