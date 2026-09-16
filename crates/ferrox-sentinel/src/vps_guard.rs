//! # VPS Hardening & Network Security Guard (`ferrox-sentinel::vps_guard`)
//!
//! Provides native Rust network connection monitoring, SYN flood velocity tracking,
//! and system-level socket pressure telemetry for VPS infrastructure protection.

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Assessment result for VPS network health
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpsHealthAssessment {
    pub is_syn_flood_detected: bool,
    pub active_connections_per_sec: f64,
    pub socket_pressure_score: f64,
    pub recommended_action: String,
}

/// VPS Hardening Engine tracking socket connection rates and network DDoS telemetry
#[derive(Debug, Clone)]
pub struct VpsHardeningEngine {
    max_syn_per_sec_threshold: f64,
    connection_timestamps: Arc<Mutex<Vec<Instant>>>,
}

impl Default for VpsHardeningEngine {
    fn default() -> Self {
        Self {
            max_syn_per_sec_threshold: 100.0,
            connection_timestamps: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl VpsHardeningEngine {
    pub fn new(max_syn_per_sec_threshold: f64) -> Self {
        Self {
            max_syn_per_sec_threshold,
            connection_timestamps: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Records an incoming TCP handshake/connection event and evaluates network velocity
    pub fn record_incoming_connection(&self) -> VpsHealthAssessment {
        let now = Instant::now();
        let mut times = self.connection_timestamps.lock().unwrap();

        times.push(now);

        // Retain timestamps within the last 1 second window
        let window = Duration::from_secs(1);
        times.retain(|t| now.duration_since(*t) <= window);

        let rate_per_sec = times.len() as f64;
        let is_syn_flood = rate_per_sec > self.max_syn_per_sec_threshold;

        let pressure_score = (rate_per_sec / self.max_syn_per_sec_threshold).min(1.0);

        let action = if is_syn_flood {
            "ENFORCE_EBPF_SYN_COOKIE_AND_RATE_LIMIT"
        } else if pressure_score > 0.7 {
            "MONITOR_HIGH_CONNECTION_RATE"
        } else {
            "SYSTEM_NOMINAL"
        };

        VpsHealthAssessment {
            is_syn_flood_detected: is_syn_flood,
            active_connections_per_sec: rate_per_sec,
            socket_pressure_score: pressure_score,
            recommended_action: action.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vps_guard_normal_traffic() {
        let engine = VpsHardeningEngine::new(50.0);
        let status = engine.record_incoming_connection();

        assert!(!status.is_syn_flood_detected);
        assert_eq!(status.active_connections_per_sec, 1.0);
    }

    #[test]
    fn test_vps_guard_syn_flood_detection() {
        let engine = VpsHardeningEngine::new(10.0);
        for _ in 0..15 {
            engine.record_incoming_connection();
        }
        let status = engine.record_incoming_connection();

        assert!(status.is_syn_flood_detected);
        assert_eq!(status.recommended_action, "ENFORCE_EBPF_SYN_COOKIE_AND_RATE_LIMIT");
    }
}
