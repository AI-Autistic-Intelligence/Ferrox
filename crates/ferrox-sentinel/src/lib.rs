//! # Ferrox Sentinel (`ferrox-sentinel`)
//!
//! AI/ML Security Analytics & Threat Detection Engine for Ferrox Enterprise backends.
//!
//! Implements real-time feature extraction (Shannon Entropy, Feature Hashing Trick),
//! velocity Z-score anomaly tracking, MinHash LSH campaign attribution, and Isolation Forest scoring.

use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};

pub mod features;
pub mod algorithms;
pub mod middleware;
pub mod scanner;
pub mod graph;
pub mod vps_guard;
#[cfg(feature = "founder-suite")]
pub mod founder;

use features::entropy::evaluate_payload_entropy;
use features::hashing_trick::FeatureHasher;
use features::ngrams::extract_character_ngrams;
use algorithms::zscore::VelocityTracker;
use algorithms::minhash_lsh::LshClusterIndex;
use algorithms::isolation_forest::IsolationForest;

/// Classification rating of an evaluated request
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {
    Benign,
    LowRisk,
    MediumRisk,
    HighRisk,
    CriticalAnomaly,
}

/// Comprehensive threat assessment result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAssessment {
    pub client_ip: String,
    pub threat_score: f64, // 0.0 to 1.0
    pub level: ThreatLevel,
    pub entropy_score: f64,
    pub velocity_zscore: f64,
    pub isolation_anomaly_score: f64,
    pub cluster_campaign_detected: bool,
    pub rationale: String,
}

/// Engine Configuration settings
#[derive(Debug, Clone)]
pub struct SentinelConfig {
    pub hashing_buckets: usize,
    pub velocity_window_secs: u64,
    pub minhash_bands: usize,
    pub minhash_rows: usize,
}

impl Default for SentinelConfig {
    fn default() -> Self {
        Self {
            hashing_buckets: 1024,
            velocity_window_secs: 10,
            minhash_bands: 5,
            minhash_rows: 4,
        }
    }
}

/// Primary Sentinel AI/ML Security Engine instance
pub struct SentinelEngine {
    config: SentinelConfig,
    feature_hasher: FeatureHasher,
    velocity_tracker: Arc<Mutex<VelocityTracker>>,
    lsh_index: Arc<Mutex<LshClusterIndex>>,
    isolation_forest: Arc<IsolationForest>,
}

impl SentinelEngine {
    pub fn new(config: SentinelConfig) -> Self {
        let feature_hasher = FeatureHasher::new(config.hashing_buckets);
        let velocity_tracker = Arc::new(Mutex::new(VelocityTracker::new(config.velocity_window_secs)));
        let lsh_index = Arc::new(Mutex::new(LshClusterIndex::new(config.minhash_bands, config.minhash_rows)));

        // Train a default baseline Isolation Forest model
        let dummy_normal_data: Vec<Vec<f32>> = vec![
            vec![0.1; config.hashing_buckets],
            vec![0.05; config.hashing_buckets],
            vec![0.12; config.hashing_buckets],
        ];
        let isolation_forest = Arc::new(IsolationForest::train(&dummy_normal_data, 10, 5));

        Self {
            config,
            feature_hasher,
            velocity_tracker,
            lsh_index,
            isolation_forest,
        }
    }

    pub fn config(&self) -> &SentinelConfig {
        &self.config
    }

    /// Evaluates an incoming HTTP request across all ML feature and anomaly dimensions
    pub fn evaluate_request(
        &self,
        client_ip: &str,
        uri: &str,
        headers: &str,
        body: &str,
    ) -> ThreatAssessment {
        // 1. Shannon Entropy Analysis
        let payload_sample = format!("{} {}", uri, body);
        let entropy_analysis = evaluate_payload_entropy(&payload_sample);

        // 2. Velocity Z-Score Anomaly Tracking
        let velocity_zscore = if let Ok(mut tracker) = self.velocity_tracker.lock() {
            tracker.record_and_evaluate()
        } else {
            0.0
        };

        // 3. Feature Hashing Trick Vectorization
        let raw_features = vec![uri, headers, body];
        let feat_vector = self.feature_hasher.transform(&raw_features);

        // 4. Isolation Forest Anomaly Scoring
        let isolation_score = self.isolation_forest.compute_anomaly_score(&feat_vector.buckets);

        // 5. MinHash LSH Campaign Attribution & Clustering
        let ngrams = extract_character_ngrams(uri, 3);
        let campaign_detected = if let Ok(mut index) = self.lsh_index.lock() {
            let matches = index.find_similar(&ngrams);
            index.insert(client_ip, &ngrams);
            matches.len() > 3 && !matches.contains(client_ip)
        } else {
            false
        };

        // Combined ensemble threat score weighting
        let mut composite_score = 0.5 * entropy_analysis.is_suspicious
            + 0.3 * (velocity_zscore / 10.0).min(1.0)
            + 0.2 * isolation_score
            + if campaign_detected { 0.15 } else { 0.0 };

        if entropy_analysis.is_suspicious > 0.5 {
            composite_score = (composite_score + 0.35).min(1.0);
        }

        composite_score = composite_score.min(1.0).max(0.0);

        let level = if composite_score >= 0.85 {
            ThreatLevel::CriticalAnomaly
        } else if composite_score >= 0.65 {
            ThreatLevel::HighRisk
        } else if composite_score >= 0.40 {
            ThreatLevel::MediumRisk
        } else if composite_score >= 0.20 {
            ThreatLevel::LowRisk
        } else {
            ThreatLevel::Benign
        };

        let rationale = format!(
            "Entropy ({:.2}), Velocity Z-Score ({:.2}), Isolation Score ({:.2}), Campaign Cluster: {}",
            entropy_analysis.entropy_score, velocity_zscore, isolation_score, campaign_detected
        );

        ThreatAssessment {
            client_ip: client_ip.to_string(),
            threat_score: composite_score,
            level,
            entropy_score: entropy_analysis.entropy_score,
            velocity_zscore,
            isolation_anomaly_score: isolation_score,
            cluster_campaign_detected: campaign_detected,
            rationale,
        }
    }
}
