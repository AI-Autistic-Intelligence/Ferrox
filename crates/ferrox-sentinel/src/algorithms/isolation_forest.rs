use rand::Rng;

/// Represents a node in an Isolation Tree
enum IsolationNode {
    Internal {
        feature_idx: usize,
        split_val: f32,
        left: Box<IsolationNode>,
        right: Box<IsolationNode>,
    },
    External {
        size: usize,
    },
}

impl IsolationNode {
    fn path_length(&self, sample: &[f32], current_height: usize) -> f64 {
        match self {
            IsolationNode::External { size } => {
                current_height as f64 + average_path_length_c(*size)
            }
            IsolationNode::Internal { feature_idx, split_val, left, right } => {
                let val = sample.get(*feature_idx).copied().unwrap_or(0.0);
                if val < *split_val {
                    left.path_length(sample, current_height + 1)
                } else {
                    right.path_length(sample, current_height + 1)
                }
            }
        }
    }
}

/// Average path length c(n) of unsuccessful search in Binary Search Tree
fn average_path_length_c(n: usize) -> f64 {
    if n <= 1 {
        0.0
    } else if n == 2 {
        1.0
    } else {
        let n_f = n as f64;
        2.0 * ((n_f - 1.0).ln() + 0.5772156649) - (2.0 * (n_f - 1.0) / n_f) // Euler's constant
    }
}

/// Lightweight Isolation Forest Anomaly Detector
pub struct IsolationForest {
    trees: Vec<IsolationNode>,
    num_trees: usize,
    subsample_size: usize,
}

impl IsolationForest {
    pub fn train(data: &[Vec<f32>], num_trees: usize, subsample_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut trees = Vec::with_capacity(num_trees);

        for _ in 0..num_trees {
            if data.is_empty() {
                break;
            }
            // Sample subset
            let sample_indices: Vec<usize> = (0..subsample_size.min(data.len()))
                .map(|_| rng.gen_range(0..data.len()))
                .collect();
            let sample_data: Vec<Vec<f32>> = sample_indices.iter().map(|&i| data[i].clone()).collect();

            let tree = build_isolation_tree(&sample_data, 0, 10, &mut rng);
            trees.push(tree);
        }

        Self {
            trees,
            num_trees,
            subsample_size: subsample_size.min(data.len()).max(2),
        }
    }

    pub fn num_trees(&self) -> usize {
        self.num_trees
    }

    /// Calculates anomaly score S(x, n) in range [0.0, 1.0].
    /// Score > 0.6 indicates high anomaly probability (zero-day threat).
    pub fn compute_anomaly_score(&self, sample: &[f32]) -> f64 {
        if self.trees.is_empty() {
            return 0.0;
        }

        let total_path_length: f64 = self.trees.iter().map(|t| t.path_length(sample, 0)).sum();
        let avg_path_length = total_path_length / self.trees.len() as f64;
        let c_n = average_path_length_c(self.subsample_size);

        if c_n < 1e-6 {
            0.0
        } else {
            2.0f64.powf(-avg_path_length / c_n)
        }
    }
}

fn build_isolation_tree(
    data: &[Vec<f32>],
    current_height: usize,
    max_height: usize,
    rng: &mut impl Rng,
) -> IsolationNode {
    if current_height >= max_height || data.len() <= 1 {
        return IsolationNode::External { size: data.len() };
    }

    let num_features = data[0].len();
    if num_features == 0 {
        return IsolationNode::External { size: data.len() };
    }

    let feature_idx = rng.gen_range(0..num_features);
    let min_val = data.iter().map(|row| row[feature_idx]).fold(f32::INFINITY, f32::min);
    let max_val = data.iter().map(|row| row[feature_idx]).fold(f32::NEG_INFINITY, f32::max);

    if (max_val - min_val).abs() < 1e-6 {
        return IsolationNode::External { size: data.len() };
    }

    let split_val = rng.gen_range(min_val..max_val);

    let left_data: Vec<Vec<f32>> = data.iter().filter(|row| row[feature_idx] < split_val).cloned().collect();
    let right_data: Vec<Vec<f32>> = data.iter().filter(|row| row[feature_idx] >= split_val).cloned().collect();

    IsolationNode::Internal {
        feature_idx,
        split_val,
        left: Box::new(build_isolation_tree(&left_data, current_height + 1, max_height, rng)),
        right: Box::new(build_isolation_tree(&right_data, current_height + 1, max_height, rng)),
    }
}
