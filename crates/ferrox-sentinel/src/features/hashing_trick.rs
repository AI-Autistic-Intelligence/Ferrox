use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

/// Fixed-size feature vector generated via the Hashing Trick (Feature Hashing).
/// Compresses high-cardinality string features into an N-bucket numeric representation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FeatureVector {
    pub buckets: Vec<f32>,
    pub num_buckets: usize,
}

pub struct FeatureHasher {
    num_buckets: usize,
}

impl FeatureHasher {
    pub fn new(num_buckets: usize) -> Self {
        Self { num_buckets }
    }

    /// Hashes a single feature string into a bucket index in range [0, num_buckets)
    fn hash_feature(&self, feature: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        feature.hash(&mut hasher);
        (hasher.finish() as usize) % self.num_buckets
    }

    /// Transforms a set of raw HTTP feature strings into a normalized feature vector
    pub fn transform(&self, features: &[&str]) -> FeatureVector {
        let mut buckets = vec![0.0f32; self.num_buckets];

        for &feat in features {
            let idx = self.hash_feature(feat);
            buckets[idx] += 1.0;
        }

        // Normalize vector (L2 norm)
        let norm_sq: f32 = buckets.iter().map(|v| v * v).sum();
        if norm_sq > 0.0 {
            let norm = norm_sq.sqrt();
            for val in &mut buckets {
                *val /= norm;
            }
        }

        FeatureVector {
            buckets,
            num_buckets: self.num_buckets,
        }
    }
}
