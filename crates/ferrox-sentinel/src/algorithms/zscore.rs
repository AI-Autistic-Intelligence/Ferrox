use std::collections::VecDeque;
use std::time::Instant;

/// Moving statistics Z-Score anomaly calculator for tracking request velocity spikes
pub struct VelocityTracker {
    timestamps: VecDeque<Instant>,
    window_secs: u64,
    historical_rates: VecDeque<f64>,
}

impl VelocityTracker {
    pub fn new(window_secs: u64) -> Self {
        Self {
            timestamps: VecDeque::new(),
            window_secs,
            historical_rates: VecDeque::with_capacity(100),
        }
    }

    /// Records a new request event and computes the Z-Score velocity anomaly rating
    pub fn record_and_evaluate(&mut self) -> f64 {
        let now = Instant::now();
        self.timestamps.push_back(now);

        // Evict expired entries outside window
        while let Some(&t) = self.timestamps.front() {
            if now.duration_since(t).as_secs() > self.window_secs {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }

        let current_rate = self.timestamps.len() as f64;

        if self.historical_rates.len() >= 100 {
            self.historical_rates.pop_front();
        }
        self.historical_rates.push_back(current_rate);

        if self.historical_rates.len() < 5 {
            return 0.0;
        }

        let mean: f64 = self.historical_rates.iter().sum::<f64>() / self.historical_rates.len() as f64;
        let variance: f64 = self.historical_rates.iter().map(|&r| (r - mean).powi(2)).sum::<f64>() / self.historical_rates.len() as f64;
        let std_dev = variance.sqrt();

        if std_dev < 1e-6 {
            0.0
        } else {
            ((current_rate - mean) / std_dev).max(0.0)
        }
    }
}
