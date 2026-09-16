use serde::{Deserialize, Serialize};

/// Prometheus and JSON Telemetry Metrics Exporter for Enterprise Security Dashboard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityMetricsSnapshot {
    pub total_evaluations: u64,
    pub benign_count: u64,
    pub low_risk_count: u64,
    pub medium_risk_count: u64,
    pub high_risk_count: u64,
    pub critical_anomaly_count: u64,
    pub active_mtd_rotations: u32,
    pub uptime_seconds: u64,
}

pub struct TelemetryExporter;

impl TelemetryExporter {
    /// Formats security metrics into standard Prometheus Exposition format
    pub fn to_prometheus(snapshot: &SecurityMetricsSnapshot) -> String {
        format!(
            "# HELP ferrox_security_evaluations_total Total HTTP requests evaluated by Sentinel AI/ML.\n\
             # TYPE ferrox_security_evaluations_total counter\n\
             ferrox_security_evaluations_total {}\n\n\
             # HELP ferrox_security_critical_anomalies_total Total critical anomalies detected.\n\
             # TYPE ferrox_security_critical_anomalies_total counter\n\
             ferrox_security_critical_anomalies_total {}\n\n\
             # HELP ferrox_security_mtd_rotations Active MTD key mutations.\n\
             # TYPE ferrox_security_mtd_rotations gauge\n\
             ferrox_security_mtd_rotations {}\n",
            snapshot.total_evaluations, snapshot.critical_anomaly_count, snapshot.active_mtd_rotations
        )
    }
}
