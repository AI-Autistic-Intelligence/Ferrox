use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityThreat {
    pub id: String,
    pub ip: String,
    pub threat_type: String,
    pub timestamp_rfc3339: String,
    pub details: String,
    pub strikes: u32,
    pub is_banned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityTelemetrySummary {
    pub total_threats_detected: u64,
    pub active_banned_ips_count: usize,
    pub brute_force_attempts_today: u64,
    pub rate_limit_violations_today: u64,
    pub active_banned_ips: Vec<String>,
    pub recent_threats: Vec<SecurityThreat>,
}

struct IpTracker {
    failed_auth_count: u32,
    request_timestamps: Vec<Instant>,
    banned_until: Option<Instant>,
    total_strikes: u32,
}

pub struct SecurityManager {
    ip_records: RwLock<HashMap<String, IpTracker>>,
    threat_log: RwLock<Vec<SecurityThreat>>,
    total_threats_detected: RwLock<u64>,
    brute_force_attempts_today: RwLock<u64>,
    rate_limit_violations_today: RwLock<u64>,
}

impl SecurityManager {
    pub fn new() -> Self {
        Self {
            ip_records: RwLock::new(HashMap::new()),
            threat_log: RwLock::new(Vec::new()),
            total_threats_detected: RwLock::new(0),
            brute_force_attempts_today: RwLock::new(0),
            rate_limit_violations_today: RwLock::new(0),
        }
    }

    pub async fn is_ip_banned(&self, ip: &str) -> bool {
        let mut records = self.ip_records.write().await;
        if let Some(tracker) = records.get_mut(ip) {
            if let Some(banned_until) = tracker.banned_until {
                if Instant::now() < banned_until {
                    return true;
                } else {
                    // Lockout period expired
                    tracker.banned_until = None;
                }
            }
        }
        false
    }

    pub async fn record_request(&self, ip: &str) -> Result<(), &'static str> {
        if self.is_ip_banned(ip).await {
            return Err("IP address is temporarily banned due to security violations");
        }

        let now = Instant::now();
        let mut records = self.ip_records.write().await;
        let tracker = records.entry(ip.to_string()).or_insert_with(|| IpTracker {
            failed_auth_count: 0,
            request_timestamps: Vec::new(),
            banned_until: None,
            total_strikes: 0,
        });

        // Retain request timestamps within the last 10 seconds window
        tracker.request_timestamps.retain(|&t| now.duration_since(t) < Duration::from_secs(10));
        tracker.request_timestamps.push(now);

        // Burst rate limit check: > 50 requests in 10 seconds triggers a rate limit violation strike
        if tracker.request_timestamps.len() > 50 {
            tracker.total_strikes += 1;
            *self.rate_limit_violations_today.write().await += 1;
            *self.total_threats_detected.write().await += 1;

            let threat = SecurityThreat {
                id: uuid::Uuid::new_v4().to_string(),
                ip: ip.to_string(),
                threat_type: "RateLimitExceeded".to_string(),
                timestamp_rfc3339: chrono::Utc::now().to_rfc3339(),
                details: format!("Excessive burst request rate: {} requests in 10s", tracker.request_timestamps.len()),
                strikes: tracker.total_strikes,
                is_banned: false,
            };

            self.add_threat_log(threat).await;

            if tracker.total_strikes >= 3 {
                tracker.banned_until = Some(now + Duration::from_secs(900)); // 15 min ban
                return Err("Rate limit threshold exceeded. IP temporarily banned for 15 minutes.");
            }
        }

        Ok(())
    }

    pub async fn record_auth_failure(&self, ip: &str, username_or_email: &str) {
        let now = Instant::now();
        let mut records = self.ip_records.write().await;
        let tracker = records.entry(ip.to_string()).or_insert_with(|| IpTracker {
            failed_auth_count: 0,
            request_timestamps: Vec::new(),
            banned_until: None,
            total_strikes: 0,
        });

        tracker.failed_auth_count += 1;
        tracker.total_strikes += 1;
        *self.brute_force_attempts_today.write().await += 1;
        *self.total_threats_detected.write().await += 1;

        let should_ban = tracker.failed_auth_count >= 5;
        if should_ban {
            tracker.banned_until = Some(now + Duration::from_secs(1800)); // 30 min ban for brute force
        }

        let threat = SecurityThreat {
            id: uuid::Uuid::new_v4().to_string(),
            ip: ip.to_string(),
            threat_type: "BruteForceAttempt".to_string(),
            timestamp_rfc3339: chrono::Utc::now().to_rfc3339(),
            details: format!("Failed authentication attempt #{} for target: '{}'", tracker.failed_auth_count, username_or_email),
            strikes: tracker.total_strikes,
            is_banned: should_ban,
        };

        drop(records);
        self.add_threat_log(threat).await;
    }

    pub async fn record_auth_success(&self, ip: &str) {
        let mut records = self.ip_records.write().await;
        if let Some(tracker) = records.get_mut(ip) {
            tracker.failed_auth_count = 0;
        }
    }

    pub async fn ban_ip(&self, ip: &str, duration_minutes: u64) {
        let mut records = self.ip_records.write().await;
        let tracker = records.entry(ip.to_string()).or_insert_with(|| IpTracker {
            failed_auth_count: 0,
            request_timestamps: Vec::new(),
            banned_until: None,
            total_strikes: 0,
        });

        tracker.banned_until = Some(Instant::now() + Duration::from_secs(duration_minutes * 60));
        tracker.total_strikes += 5;

        let threat = SecurityThreat {
            id: uuid::Uuid::new_v4().to_string(),
            ip: ip.to_string(),
            threat_type: "ManualAdminBan".to_string(),
            timestamp_rfc3339: chrono::Utc::now().to_rfc3339(),
            details: format!("IP address manually banned by administrator for {} minutes", duration_minutes),
            strikes: tracker.total_strikes,
            is_banned: true,
        };

        drop(records);
        self.add_threat_log(threat).await;
    }

    pub async fn unblock_ip(&self, ip: &str) -> bool {
        let mut records = self.ip_records.write().await;
        if let Some(tracker) = records.get_mut(ip) {
            tracker.banned_until = None;
            tracker.failed_auth_count = 0;
            return true;
        }
        false
    }

    pub async fn get_summary(&self) -> SecurityTelemetrySummary {
        let records = self.ip_records.read().await;
        let threat_log = self.threat_log.read().await;
        let now = Instant::now();

        let active_banned_ips: Vec<String> = records.iter()
            .filter(|(_, tracker)| tracker.banned_until.map_or(false, |until| now < until))
            .map(|(ip, _)| ip.clone())
            .collect();

        let active_banned_count = active_banned_ips.len();
        let recent_threats = threat_log.iter().rev().take(20).cloned().collect();

        SecurityTelemetrySummary {
            total_threats_detected: *self.total_threats_detected.read().await,
            active_banned_ips_count: active_banned_count,
            brute_force_attempts_today: *self.brute_force_attempts_today.read().await,
            rate_limit_violations_today: *self.rate_limit_violations_today.read().await,
            active_banned_ips,
            recent_threats,
        }
    }

    async fn add_threat_log(&self, threat: SecurityThreat) {
        let mut log = self.threat_log.write().await;
        if log.len() >= 100 {
            log.remove(0);
        }
        log.push(threat.clone());
        drop(log);

        // Dispath alert to Discord Webhook if configured
        tokio::spawn(async move {
            if let Ok(webhook_url) = std::env::var("DISCORD_WEBHOOK_URL") {
                let client = reqwest::Client::new();
                let color = if threat.is_banned { 0xE74C3C } else { 0xF39C12 };
                
                let payload = serde_json::json!({
                    "username": "Ferrox Security Mesh",
                    "avatar_url": "https://i.imgur.com/4M34hi2.png", // Optional shield icon
                    "embeds": [{
                        "title": format!("🚨 Security Alert: {}", threat.threat_type),
                        "description": format!("**IP:** {}\n**Details:** {}\n**Strikes:** {}\n**Banned:** {}", 
                            threat.ip, threat.details, threat.strikes, threat.is_banned),
                        "color": color,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    }]
                });

                let _ = client.post(&webhook_url)
                    .json(&payload)
                    .send()
                    .await;
            }
        });
    }
}
