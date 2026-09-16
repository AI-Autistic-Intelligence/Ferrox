use std::collections::{HashMap, HashSet};
use rand::Rng;

/// MinHash Signature Generator for approximating Jaccard Similarity J(A,B)
pub struct MinHashEngine {
    num_hashes: usize,
    hash_a: Vec<u64>,
    hash_b: Vec<u64>,
    prime: u64,
}

impl MinHashEngine {
    pub fn new(num_hashes: usize) -> Self {
        let mut rng = rand::thread_rng();
        let prime = 4294967311u64; // Large prime > 2^32

        let mut hash_a = Vec::with_capacity(num_hashes);
        let mut hash_b = Vec::with_capacity(num_hashes);

        for _ in 0..num_hashes {
            hash_a.push(rng.gen_range(1..prime));
            hash_b.push(rng.gen_range(0..prime));
        }

        Self {
            num_hashes,
            hash_a,
            hash_b,
            prime,
        }
    }

    /// Computes MinHash signature vector for a set of token features
    pub fn compute_signature(&self, features: &[String]) -> Vec<u64> {
        let mut signature = vec![u64::MAX; self.num_hashes];
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        for feat in features {
            let mut h = DefaultHasher::new();
            feat.hash(&mut h);
            let item_hash = h.finish();

            for i in 0..self.num_hashes {
                let val = ((self.hash_a[i].wrapping_mul(item_hash)).wrapping_add(self.hash_b[i])) % self.prime;
                if val < signature[i] {
                    signature[i] = val;
                }
            }
        }

        signature
    }

    /// Approximates Jaccard Similarity using two MinHash signature vectors
    pub fn estimate_jaccard(sig_a: &[u64], sig_b: &[u64]) -> f64 {
        if sig_a.len() != sig_b.len() || sig_a.is_empty() {
            return 0.0;
        }

        let matches = sig_a.iter().zip(sig_b.iter()).filter(|(&a, &b)| a == b).count();
        matches as f64 / sig_a.len() as f64
    }
}

/// Locality-Sensitive Hashing (LSH) Cluster Index for grouping threat campaigns
pub struct LshClusterIndex {
    minhash: MinHashEngine,
    bands: usize,
    rows_per_band: usize,
    // Bucket key -> Set of item identifiers (e.g. IP or payload ID)
    buckets: HashMap<u64, HashSet<String>>,
}

impl LshClusterIndex {
    pub fn new(bands: usize, rows_per_band: usize) -> Self {
        let num_hashes = bands * rows_per_band;
        Self {
            minhash: MinHashEngine::new(num_hashes),
            bands,
            rows_per_band,
            buckets: HashMap::new(),
        }
    }

    /// Indexes an attacker item signature into LSH buckets
    pub fn insert(&mut self, item_id: &str, features: &[String]) {
        let sig = self.minhash.compute_signature(features);

        for band in 0..self.bands {
            let start = band * self.rows_per_band;
            let end = start + self.rows_per_band;
            let band_slice = &sig[start..end];

            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut h = DefaultHasher::new();
            band_slice.hash(&mut h);
            let bucket_key = h.finish();

            self.buckets.entry(bucket_key).or_default().insert(item_id.to_string());
        }
    }

    /// Queries LSH index to find candidate similar campaign attack profiles
    pub fn find_similar(&self, features: &[String]) -> HashSet<String> {
        let sig = self.minhash.compute_signature(features);
        let mut candidates = HashSet::new();

        for band in 0..self.bands {
            let start = band * self.rows_per_band;
            let end = start + self.rows_per_band;
            let band_slice = &sig[start..end];

            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut h = DefaultHasher::new();
            band_slice.hash(&mut h);
            let bucket_key = h.finish();

            if let Some(matching_items) = self.buckets.get(&bucket_key) {
                candidates.extend(matching_items.clone());
            }
        }

        candidates
    }
}
