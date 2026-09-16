/// Universal Adversarial Perturbation (UAP) Defense Engine (ML Under Malware Attack - Ch. 7 & 11)
/// Detects universal adversarial perturbations and micro-signature noise designed to bypass classifiers.

pub struct UapDetector {
    perturbation_threshold: f64,
}

impl UapDetector {
    pub fn new(perturbation_threshold: f64) -> Self {
        Self { perturbation_threshold }
    }

    /// Evaluates if input vector exhibits UAP perturbation signatures (high variance with low semantic entropy)
    pub fn is_uap_perturbed(&self, raw_entropy: f64, feature_norm: f64) -> bool {
        // UAP attacks introduce high-dimensional feature norm variance while maintaining near-normal entropy
        let perturbation_metric = (feature_norm as f64) / (raw_entropy + 1e-5);
        perturbation_metric > self.perturbation_threshold
    }
}
