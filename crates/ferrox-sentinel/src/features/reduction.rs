/// Feature Reduction & Normalization Guard (ML Under Malware Attack - Ch. 11)
/// Strips adversarial padding noise and normalizes payload feature vectors before classification.

pub struct FeatureReducer;

impl FeatureReducer {
    /// Reduces raw payload by stripping non-essential adversarial padding (null bytes, repeated spaces, comments)
    pub fn reduce_payload(raw_payload: &str) -> String {
        let mut cleaned = raw_payload.replace('\0', "");

        // Collapse comment padding
        cleaned = cleaned.replace("/**/", " ");

        // Collapse consecutive spaces repeatedly
        let mut prev_len = 0;
        while cleaned.len() != prev_len {
            prev_len = cleaned.len();
            cleaned = cleaned.replace("  ", " ");
        }

        cleaned.trim().to_string()
    }

    /// L2 Feature Vector Normalization to prevent feature magnitude inflation attacks
    pub fn normalize_vector(vector: &mut [f32]) {
        let sum_sq: f32 = vector.iter().map(|&x| x * x).sum();
        if sum_sq > 0.0 {
            let norm = sum_sq.sqrt();
            for val in vector.iter_mut() {
                *val /= norm;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_reduction() {
        let adversarial_input = "SELECT   *   FROM   users  /**/ WHERE  1=1";
        let reduced = FeatureReducer::reduce_payload(adversarial_input);
        assert_eq!(reduced, "SELECT * FROM users WHERE 1=1");
    }
}
