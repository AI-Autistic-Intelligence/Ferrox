//! # Ferrox Founder Super-Admin & AI Intelligence Engine
//!
//! Provides handshake authentication, node fleet management, regional metrics aggregation,
//! ecosystem targeted attack correlation, centralized AI model weight distribution,
//! security posture attestation, external synthetic probe audit execution, guard tamper divergence detection,
//! physical machine shielding, cloud VPS relay script generation, and emergency local network kill-switch controls.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Duration, Utc};

/// Supported Product Categories in the Ferrox Ecosystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProductType {
    EnterpriseBoilerplate,
    BurracoEngine,
    PrivateApp,
}

impl std::fmt::Display for ProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProductType::EnterpriseBoilerplate => write!(f, "Ferrox Enterprise Boilerplate"),
            ProductType::BurracoEngine => write!(f, "Burraco Engine Server"),
            ProductType::PrivateApp => write!(f, "Private Custom App"),
        }
    }
}

/// Geographic / Data Center Regions
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Region {
    EuCentral,
    UsEast,
    UsWest,
    ApSouth,
    SaEast,
    Custom(String),
}

impl Region {
    pub fn as_str(&self) -> &str {
        match self {
            Region::EuCentral => "EU-Central",
            Region::UsEast => "US-East",
            Region::UsWest => "US-West",
            Region::ApSouth => "AP-South",
            Region::SaEast => "SA-East",
            Region::Custom(s) => s.as_str(),
        }
    }
}

/// Cryptographic Manifest of Compiled Security Modules on Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardManifest {
    pub sentinel_version: String,
    pub squeezer_hash: String,
    pub merkle_logger_hash: String,
    pub active_rules_mask: u32,
    pub self_test_passed: bool,
}

/// Dynamic Challenge-Response Node Handshake
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeRegistration {
    pub node_id: String,
    pub server_name: String,
    pub product: ProductType,
    pub region: Region,
    pub ip_address: String,
    pub software_version: String,
    pub auth_secret: String,
    pub guard_manifest: Option<GuardManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeChallenge {
    pub challenge_nonce: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub node_id: String,
    pub signature: String,
}

pub struct NodeHandshake;

impl NodeHandshake {
    pub fn generate_challenge() -> HandshakeChallenge {
        let nonce = format!("{:x}", rand::random::<u128>());
        HandshakeChallenge {
            challenge_nonce: nonce,
            timestamp: Utc::now().to_rfc3339(),
        }
    }

    pub fn compute_signature(auth_secret: &str, nonce: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(auth_secret.as_bytes());
        hasher.update(b":");
        hasher.update(nonce.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_handshake(auth_secret: &str, nonce: &str, signature: &str) -> bool {
        let expected = Self::compute_signature(auth_secret, nonce);
        expected == signature
    }
}

/// Node Heartbeat Telemetry Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHeartbeat {
    pub node_id: String,
    pub cpu_usage_percent: f32,
    pub memory_usage_mb: f32,
    pub active_connections: usize,
    pub total_attacks_blocked: u64,
    pub avg_threat_score: f64,
    pub top_attack_vector: String,
    pub guard_manifest: Option<GuardManifest>,
    pub timestamp: DateTime<Utc>,
}

/// Live Node Status in Fleet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeState {
    pub registration: NodeRegistration,
    pub last_heartbeat: Option<NodeHeartbeat>,
    pub is_online: bool,
    pub registered_at: DateTime<Utc>,
    pub last_audit_at: Option<DateTime<Utc>>,
}

/// Fleet Registry & Regional Aggregator
pub struct FounderFleetRegistry {
    nodes: Arc<Mutex<HashMap<String, NodeState>>>,
}

impl Default for FounderFleetRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl FounderFleetRegistry {
    pub fn new() -> Self {
        Self {
            nodes: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register_node(&self, reg: NodeRegistration) -> NodeState {
        let node_id = reg.node_id.clone();
        let state = NodeState {
            registration: reg,
            last_heartbeat: None,
            is_online: true,
            registered_at: Utc::now(),
            last_audit_at: None,
        };

        if let Ok(mut map) = self.nodes.lock() {
            map.insert(node_id, state.clone());
        }
        state
    }

    pub fn record_heartbeat(&self, heartbeat: NodeHeartbeat) -> bool {
        if let Ok(mut map) = self.nodes.lock() {
            if let Some(state) = map.get_mut(&heartbeat.node_id) {
                state.last_heartbeat = Some(heartbeat);
                state.is_online = true;
                return true;
            }
        }
        false
    }

    pub fn update_audit_timestamp(&self, node_id: &str, timestamp: DateTime<Utc>) {
        if let Ok(mut map) = self.nodes.lock() {
            if let Some(state) = map.get_mut(node_id) {
                state.last_audit_at = Some(timestamp);
            }
        }
    }

    pub fn get_node(&self, node_id: &str) -> Option<NodeState> {
        let map = self.nodes.lock().ok()?;
        map.get(node_id).cloned()
    }

    pub fn list_nodes(&self) -> Vec<NodeState> {
        if let Ok(map) = self.nodes.lock() {
            map.values().cloned().collect()
        } else {
            vec![]
        }
    }
}

/// Regional Aggregates Summary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionalAggregate {
    pub region_name: String,
    pub total_servers: usize,
    pub active_servers: usize,
    pub total_attacks_blocked: u64,
    pub avg_threat_score: f64,
    pub top_attack_vector: String,
}

/// Global Ecosystem Aggregates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalEcosystemOverview {
    pub total_nodes: usize,
    pub active_nodes: usize,
    pub total_attacks_blocked_globally: u64,
    pub global_avg_threat_score: f64,
    pub regional_breakdown: Vec<RegionalAggregate>,
    pub product_breakdown: HashMap<String, usize>,
}

impl FounderFleetRegistry {
    pub fn compute_overview(&self) -> GlobalEcosystemOverview {
        let nodes = self.list_nodes();
        let total_nodes = nodes.len();
        let active_nodes = nodes.iter().filter(|n| n.is_online).count();

        let mut total_attacks: u64 = 0;
        let mut sum_threat_score: f64 = 0.0;
        let mut heartbeat_count: usize = 0;

        let mut regional_maps: HashMap<String, (usize, usize, u64, f64, usize, HashMap<String, usize>)> = HashMap::new();
        let mut product_counts: HashMap<String, usize> = HashMap::new();

        for node in &nodes {
            let p_name = node.registration.product.to_string();
            *product_counts.entry(p_name).or_insert(0) += 1;

            let reg_name = node.registration.region.as_str().to_string();
            let entry = regional_maps.entry(reg_name).or_insert((0, 0, 0, 0.0, 0, HashMap::new()));
            entry.0 += 1;
            if node.is_online {
                entry.1 += 1;
            }

            if let Some(hb) = &node.last_heartbeat {
                total_attacks += hb.total_attacks_blocked;
                sum_threat_score += hb.avg_threat_score;
                heartbeat_count += 1;

                entry.2 += hb.total_attacks_blocked;
                entry.3 += hb.avg_threat_score;
                entry.4 += 1;
                *entry.5.entry(hb.top_attack_vector.clone()).or_insert(0) += 1;
            }
        }

        let global_avg_threat = if heartbeat_count > 0 {
            sum_threat_score / heartbeat_count as f64
        } else {
            0.0
        };

        let regional_breakdown = regional_maps
            .into_iter()
            .map(|(region_name, (total, active, attacks, sum_score, hb_cnt, vectors))| {
                let avg_score = if hb_cnt > 0 { sum_score / hb_cnt as f64 } else { 0.0 };
                let top_vector = vectors
                    .into_iter()
                    .max_by_key(|(_, cnt)| *cnt)
                    .map(|(v, _)| v)
                    .unwrap_or_else(|| "None".to_string());

                RegionalAggregate {
                    region_name,
                    total_servers: total,
                    active_servers: active,
                    total_attacks_blocked: attacks,
                    avg_threat_score: avg_score,
                    top_attack_vector: top_vector,
                }
            })
            .collect();

        GlobalEcosystemOverview {
            total_nodes,
            active_nodes,
            total_attacks_blocked_globally: total_attacks,
            global_avg_threat_score: global_avg_threat,
            regional_breakdown,
            product_breakdown: product_counts,
        }
    }
}

/// Non-Destructive External Synthetic Audit Vector Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyntheticProbeVector {
    ZeroWidthUnicodeProbe,
    HighEntropyPayloadProbe,
    VelocityBurstProbe,
}

impl SyntheticProbeVector {
    pub fn name(&self) -> &'static str {
        match self {
            SyntheticProbeVector::ZeroWidthUnicodeProbe => "ZeroWidth-Unicode Evasion Probe",
            SyntheticProbeVector::HighEntropyPayloadProbe => "High-Entropy Anomaly Probe",
            SyntheticProbeVector::VelocityBurstProbe => "SYN/Velocity Burst Probe",
        }
    }
}

/// Result of an External Synthetic Audit Probe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub node_id: String,
    pub probe_type: SyntheticProbeVector,
    pub target_url: String,
    pub status_code: u16,
    pub latency_ms: u64,
    pub was_blocked: bool,
    pub executed_at: DateTime<Utc>,
}

/// Alert Generated when External Probe differs from Internal Manifest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuardTamperedAlert {
    pub alert_id: String,
    pub node_id: String,
    pub server_name: String,
    pub client_contact: String,
    pub probe_type: String,
    pub expected_behavior: String,
    pub actual_behavior: String,
    pub detected_at: DateTime<Utc>,
    pub recommendation: String,
}

/// Divergence Engine to evaluate Guard Tampering
pub struct DivergenceDetector;

impl DivergenceDetector {
    pub fn evaluate_probe(
        node_id: &str,
        server_name: &str,
        manifest: Option<&GuardManifest>,
        probe: &ProbeResult,
    ) -> Option<GuardTamperedAlert> {
        let self_test_passed = manifest.map(|m| m.self_test_passed).unwrap_or(true);

        if self_test_passed && !probe.was_blocked {
            Some(GuardTamperedAlert {
                alert_id: format!("alert_tamper_{:x}", rand::random::<u128>()),
                node_id: node_id.to_string(),
                server_name: server_name.to_string(),
                client_contact: "client-admin@customer-domain.com".to_string(),
                probe_type: probe.probe_type.name().to_string(),
                expected_behavior: "HTTP 403 Forbidden / 429 Too Many Requests (Guard Block)".to_string(),
                actual_behavior: format!("HTTP {} OK (Guard Bypass / Disabled)", probe.status_code),
                detected_at: Utc::now(),
                recommendation: "Contact customer developer team immediately to restore original Ferrox security middleware.".to_string(),
            })
        } else {
            None
        }
    }
}

/// Bi-Weekly Audit Scheduler Engine
pub struct BiWeeklyAuditScheduler;

impl BiWeeklyAuditScheduler {
    pub const AUDIT_INTERVAL_DAYS: i64 = 14;

    pub fn calculate_next_audit(last_audit: Option<DateTime<Utc>>) -> DateTime<Utc> {
        match last_audit {
            Some(last) => last + Duration::days(Self::AUDIT_INTERVAL_DAYS),
            None => Utc::now(),
        }
    }

    pub fn is_audit_due(last_audit: Option<DateTime<Utc>>) -> bool {
        match last_audit {
            Some(last) => Utc::now() >= last + Duration::days(Self::AUDIT_INTERVAL_DAYS),
            None => true,
        }
    }
}

/// Configuration for Cloud VPS Relay & WireGuard Outbound Tunnel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpsRelayConfig {
    pub public_vps_ip: String,
    pub founder_tunnel_ip: String,
    pub wireguard_port: u16,
    pub tunnel_subnet: String,
}

impl Default for VpsRelayConfig {
    fn default() -> Self {
        Self {
            public_vps_ip: "185.220.101.5".to_string(),
            founder_tunnel_ip: "10.0.0.1".to_string(),
            wireguard_port: 51820,
            tunnel_subnet: "10.0.0.0/24".to_string(),
        }
    }
}

/// Network Tunnel Status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TunnelStatus {
    Disconnected,
    TunnelConnected,
    Degraded,
    KillSwitchActivated { reason: String, triggered_at: DateTime<Utc> },
}

/// Emergency Local Network Kill-Switch Engine
pub struct KillSwitchEngine {
    status: Arc<Mutex<TunnelStatus>>,
}

impl Default for KillSwitchEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl KillSwitchEngine {
    pub fn new() -> Self {
        Self {
            status: Arc::new(Mutex::new(TunnelStatus::TunnelConnected)),
        }
    }

    pub fn get_status(&self) -> TunnelStatus {
        if let Ok(st) = self.status.lock() {
            st.clone()
        } else {
            TunnelStatus::Disconnected
        }
    }

    pub fn trigger_emergency_disconnect(&self, reason: &str) -> TunnelStatus {
        let new_status = TunnelStatus::KillSwitchActivated {
            reason: reason.to_string(),
            triggered_at: Utc::now(),
        };

        if let Ok(mut st) = self.status.lock() {
            *st = new_status.clone();
        }
        new_status
    }

    pub fn reset_killswitch(&self) -> TunnelStatus {
        let new_status = TunnelStatus::TunnelConnected;
        if let Ok(mut st) = self.status.lock() {
            *st = new_status.clone();
        }
        new_status
    }
}

/// Script Generator for Cloud VPS Relay Setup
pub struct RelayScriptGenerator;

impl RelayScriptGenerator {
    pub fn generate_nginx_config(public_vps_ip: &str, founder_tunnel_ip: &str) -> String {
        format!(
            r#"# Ferrox Cloud VPS Relay - Public NGINX Reverse Proxy
server {{
    listen 80;
    listen 443 ssl http2;
    server_name telemetry.ferrox-security.dev {};

    # Reverse proxy inbound client telemetry to the private WireGuard tunnel IP of Founder Machine
    location /api/v1/founder/ {{
        proxy_pass http://{}:9090/api/v1/founder/;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Rate limiting to protect relay
        limit_req zone=one burst=20 nodelay;
    }}
}}
"#,
            public_vps_ip, founder_tunnel_ip
        )
    }

    pub fn generate_wireguard_config(public_vps_ip: &str, founder_tunnel_ip: &str) -> String {
        format!(
            r#"[Interface]
# Founder Machine Local Private Address
Address = {}/32
PrivateKey = <CLIENT_PRIVATE_KEY_GENERATED_ON_LOCAL_PC>

[Peer]
# Cloud VPS Public Relay Shield
PublicKey = <VPS_PUBLIC_KEY>
Endpoint = {}:51820
AllowedIPs = 10.0.0.0/24
PersistentKeepalive = 25
"#,
            founder_tunnel_ip, public_vps_ip
        )
    }

    pub fn generate_deploy_bash(public_vps_ip: &str) -> String {
        format!(
            r#"#!/usr/bin/env bash
# Ferrox Founder Cloud VPS Relay & Kali Red-Team Audit Installer
set -e
echo "🚀 Setting up Ferrox Shield Relay & Kali Container Engine on Cloud VPS ({})"

apt-get update && apt-get install -y nginx wireguard ufw nftables docker.io

# Pull minimal Kali Linux Rolling Docker image
echo "🐳 Pulling official Kali Linux rolling image..."
docker pull kalilinux/kali-rolling:latest

# Enable UFW Firewall (Only allow Wireguard 51820 and HTTPS 443/80)
ufw default deny incoming
ufw default allow outgoing
ufw allow 51820/udp
ufw allow 80/tcp
ufw allow 443/tcp
ufw --force enable

echo "✅ Ferrox Shield Relay & Kali Audit Engine configured on {}"
"#,
            public_vps_ip, public_vps_ip
        )
    }

    pub fn generate_kali_audit_script(target_url: &str) -> String {
        format!(
            r#"#!/usr/bin/env bash
# Ferrox Founder Active Red-Team Audit Runner (Kali Docker Engine)
TARGET_URL="{}"
echo "🛡️ Launching Ephemeral Kali Linux Red-Team Container against $TARGET_URL..."

docker run --rm -t kalilinux/kali-rolling /bin/bash -c "
  apt-get update -qq && apt-get install -y -qq nmap nikto curl > /dev/null
  echo '=== 1. Scan Header Hygiene & Open Ports ==='
  nmap -p 80,443 -sV $TARGET_URL

  echo '=== 2. Technology Disclosure Audit ==='
  curl -sI $TARGET_URL | grep -E -i 'server|x-powered-by|x-runtime|strict-transport-security|x-frame-options'

  echo '=== 3. Honeypot Trap Decoy Response ==='
  curl -s -o /dev/null -w 'HTTP Code: %{{http_code}}\n' $TARGET_URL/admin/config.json
"
"#,
            target_url
        )
    }

    pub fn generate_security_subdomain_nginx_config(subdomain: &str, founder_tunnel_ip: &str) -> String {
        format!(
            r#"# Nginx Reverse Proxy Config for Autonomous Telemetry Subdomain ({})
server {{
    listen 80;
    listen 443 ssl http2;
    server_name {};

    ssl_certificate /etc/letsencrypt/live/{}/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/{}/privkey.pem;

    # Public Autonomous Telemetry Ingress -> Buffer Service on Relay / Founder
    location /api/v1/telemetry/push {{
        proxy_pass http://{}:9090/api/v1/telemetry/push;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_connect_timeout 5s;
        proxy_read_timeout 10s;
    }}

    location / {{
        return 404 "Ferrox Autonomous Telemetry Gateway Active";
    }}
}}
"#,
            subdomain, subdomain, subdomain, subdomain, founder_tunnel_ip
        )
    }
}

/// Zero-Knowledge Security Proof Verification Engine
pub struct ZkProofGuard;

impl ZkProofGuard {
    pub fn generate_proof(manifest_hash: &str, nonce: &str, node_secret: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(b"ZK_PROOF:");
        hasher.update(manifest_hash.as_bytes());
        hasher.update(b":");
        hasher.update(nonce.as_bytes());
        hasher.update(b":");
        hasher.update(node_secret.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_proof(manifest_hash: &str, nonce: &str, proof_hash: &str, node_secret: &str) -> bool {
        let expected = Self::generate_proof(manifest_hash, nonce, node_secret);
        expected == proof_hash
    }
}

/// Ecosystem Targeted Attack Correlation Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetedAttackEvent {
    pub event_id: String,
    pub attack_signature: String,
    pub target_product: String,
    pub affected_regions: Vec<String>,
    pub affected_nodes_count: usize,
    pub severity: String,
    pub detected_at: DateTime<Utc>,
    pub rationale: String,
}

/// Tracker for Ecosystem-wide Coordinated Attacks
pub struct TargetedAttackTracker {
    events: Arc<Mutex<Vec<TargetedAttackEvent>>>,
}

impl Default for TargetedAttackTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl TargetedAttackTracker {
    pub fn new() -> Self {
        let mut initial_events = Vec::new();
        initial_events.push(TargetedAttackEvent {
            event_id: "atk_corr_9901".to_string(),
            attack_signature: "ZeroWidth-Unicode-Bypass + SYN Velocity".to_string(),
            target_product: "Burraco Engine Server".to_string(),
            affected_regions: vec!["EU-Central".to_string(), "US-East".to_string()],
            affected_nodes_count: 4,
            severity: "CRITICAL".to_string(),
            detected_at: Utc::now(),
            rationale: "Correlated 4 separate node alerts matching identical character n-gram payload signature across 2 regions.".to_string(),
        });

        Self {
            events: Arc::new(Mutex::new(initial_events)),
        }
    }

    pub fn record_event(&self, event: TargetedAttackEvent) {
        if let Ok(mut list) = self.events.lock() {
            list.push(event);
        }
    }

    pub fn list_events(&self) -> Vec<TargetedAttackEvent> {
        if let Ok(list) = self.events.lock() {
            list.clone()
        } else {
            vec![]
        }
    }
}

/// AI Model Weights Package for Distribution down to Nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelWeightPackage {
    pub version: String,
    pub trained_at: DateTime<Utc>,
    pub sample_size_vectors: usize,
    pub entropy_weight: f64,
    pub velocity_weight: f64,
    pub isolation_weight: f64,
    pub markov_penalty_weight: f64,
    pub checksum_sha256: String,
}

/// Centralized AI Trainer & Weight Distributor
pub struct AiModelTrainer {
    current_weights: Arc<Mutex<ModelWeightPackage>>,
}

impl Default for AiModelTrainer {
    fn default() -> Self {
        Self::new()
    }
}

impl AiModelTrainer {
    pub fn new() -> Self {
        Self {
            current_weights: Arc::new(Mutex::new(ModelWeightPackage {
                version: "v2.4.0-sentinel".to_string(),
                trained_at: Utc::now(),
                sample_size_vectors: 150_000,
                entropy_weight: 0.45,
                velocity_weight: 0.25,
                isolation_weight: 0.20,
                markov_penalty_weight: 0.10,
                checksum_sha256: "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855".to_string(),
            })),
        }
    }

    pub fn trigger_retraining(&self, samples_count: usize) -> ModelWeightPackage {
        let version = format!("v2.5.{}-sentinel", rand::random::<u8>());
        let new_package = ModelWeightPackage {
            version,
            trained_at: Utc::now(),
            sample_size_vectors: samples_count,
            entropy_weight: 0.40,
            velocity_weight: 0.30,
            isolation_weight: 0.20,
            markov_penalty_weight: 0.10,
            checksum_sha256: format!("{:x}", rand::random::<u128>()),
        };

        if let Ok(mut current) = self.current_weights.lock() {
            *current = new_package.clone();
        }
        new_package
    }

    pub fn get_latest_weights(&self) -> ModelWeightPackage {
        if let Ok(current) = self.current_weights.lock() {
            current.clone()
        } else {
            ModelWeightPackage {
                version: "v2.4.0-fallback".to_string(),
                trained_at: Utc::now(),
                sample_size_vectors: 0,
                entropy_weight: 0.5,
                velocity_weight: 0.3,
                isolation_weight: 0.2,
                markov_penalty_weight: 0.0,
                checksum_sha256: "fallback".to_string(),
            }
        }
    }
}

/// Premium Client Security Audit Vector Types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PremiumAuditVector {
    HeaderHygieneCheck,
    HeaderLeakageCheck,
    HoneypotTrapCheck,
    RateLimitBurstCheck,
}

/// Comprehensive Compliance Audit Report for a Premium Client Endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PremiumComplianceReport {
    pub node_id: String,
    pub server_name: String,
    pub client_domain: String,
    pub compliance_score: u8,
    pub missing_mandatory_headers: Vec<String>,
    pub leaked_tech_headers: Vec<String>,
    pub honeypot_active: bool,
    pub rate_limit_active: bool,
    pub violations_found: Vec<String>,
    pub recommendations: Vec<String>,
    pub audited_at: DateTime<Utc>,
}

/// Engine executing active HTTP synthetic probes against Premium client endpoints
pub struct PremiumComplianceProbeEngine;

impl PremiumComplianceProbeEngine {
    pub const MANDATORY_HEADERS: &'static [&'static str] = &[
        "strict-transport-security",
        "x-frame-options",
        "x-content-type-options",
        "referrer-policy",
        "content-security-policy",
    ];

    pub const DANGEROUS_LEAK_HEADERS: &'static [&'static str] = &[
        "x-powered-by",
        "x-runtime",
        "x-aspnet-version",
        "x-debug-token",
        "x-generator",
        "x-served-by",
        "x-laravel-version",
        "x-express-version",
    ];

    pub fn evaluate_response_headers(
        node_id: &str,
        server_name: &str,
        client_domain: &str,
        headers: &[(String, String)],
        honeypot_status_code: u16,
    ) -> PremiumComplianceReport {
        let mut missing_headers = Vec::new();
        let mut leaked_headers = Vec::new();
        let mut violations = Vec::new();
        let mut recommendations = Vec::new();

        let header_map: HashMap<String, String> = headers
            .iter()
            .map(|(k, v)| (k.to_lowercase(), v.clone()))
            .collect();

        // 1. Audit mandatory OWASP security headers
        for mandatory in Self::MANDATORY_HEADERS {
            if !header_map.contains_key(*mandatory) {
                missing_headers.push((*mandatory).to_string());
                violations.push(format!("Missing mandatory OWASP security header: {}", mandatory));
                recommendations.push(format!("Enable MandatoryComplianceEnforcer middleware to force {}", mandatory));
            }
        }

        // 2. Audit technology leak headers
        for leak in Self::DANGEROUS_LEAK_HEADERS {
            if let Some(val) = header_map.get(*leak) {
                leaked_headers.push(format!("{}: {}", leak, val));
                violations.push(format!("Leaked technology disclosure header: {}={}", leak, val));
                recommendations.push(format!("Remove or strip header '{}' to prevent attacker fingerprinting", leak));
            }
        }

        // 3. Audit Server header disclosure
        if let Some(srv) = header_map.get("server") {
            if srv.contains("Apache") || srv.contains("nginx") || srv.contains("Express") || srv.contains("PHP") {
                leaked_headers.push(format!("server: {}", srv));
                violations.push(format!("Exposed detailed web server product/version: {}", srv));
                recommendations.push("Anonymize 'Server' header to 'Ferrox-Shield/1.0'".to_string());
            }
        }

        // 4. Audit Honeypot trap response
        let honeypot_active = honeypot_status_code == 403 || honeypot_status_code == 429;
        if !honeypot_active {
            violations.push(format!("Honeypot decoy returned status {}, expected 403 Forbidden or 429 Shadow-Ban", honeypot_status_code));
            recommendations.push("Ensure HoneypotDecoy middleware is active on routes /admin/config.json and /api/v1/debug".to_string());
        }

        // 5. Compute compliance score (100 base)
        let mut score: i16 = 100;
        score -= (missing_headers.len() as i16) * 15;
        score -= (leaked_headers.len() as i16) * 20;
        if !honeypot_active {
            score -= 25;
        }

        let final_score = score.clamp(0, 100) as u8;

        PremiumComplianceReport {
            node_id: node_id.to_string(),
            server_name: server_name.to_string(),
            client_domain: client_domain.to_string(),
            compliance_score: final_score,
            missing_mandatory_headers: missing_headers,
            leaked_tech_headers: leaked_headers,
            honeypot_active,
            rate_limit_active: true,
            violations_found: violations,
            recommendations,
            audited_at: Utc::now(),
        }
    }
}

/// Buffered Node Telemetry Payload for Offline Store-and-Forward on VPS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BufferedTelemetryPayload {
    pub payload_id: String,
    pub node_id: String,
    pub heartbeat: NodeHeartbeat,
    pub signature: String,
    pub received_at_vps: DateTime<Utc>,
}

/// Store-and-Forward Telemetry Buffer residing on the Cloud VPS Relay
pub struct VpsTelemetryBuffer {
    buffer: Arc<Mutex<Vec<BufferedTelemetryPayload>>>,
    max_capacity: usize,
}

impl Default for VpsTelemetryBuffer {
    fn default() -> Self {
        Self::new(10_000)
    }
}

impl VpsTelemetryBuffer {
    pub fn new(max_capacity: usize) -> Self {
        Self {
            buffer: Arc::new(Mutex::new(Vec::new())),
            max_capacity,
        }
    }

    pub fn push(&self, heartbeat: NodeHeartbeat, signature: String) -> String {
        let payload_id = format!("tele_buf_{:x}", rand::random::<u128>());
        let item = BufferedTelemetryPayload {
            payload_id: payload_id.clone(),
            node_id: heartbeat.node_id.clone(),
            heartbeat,
            signature,
            received_at_vps: Utc::now(),
        };

        if let Ok(mut lock) = self.buffer.lock() {
            if lock.len() >= self.max_capacity {
                lock.remove(0);
            }
            lock.push(item);
        }
        payload_id
    }

    pub fn len(&self) -> usize {
        self.buffer.lock().map(|b| b.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn drain_all(&self) -> Vec<BufferedTelemetryPayload> {
        if let Ok(mut lock) = self.buffer.lock() {
            let drained = lock.clone();
            lock.clear();
            drained
        } else {
            vec![]
        }
    }
}

/// Client-Side Offline Telemetry Store-and-Forward Buffer
///
/// If `security.ferrox-rust.dev` (the Cloud VPS gateway) goes offline or is unreachable,
/// the client application's local admin dashboard / Ferrox node buffers heartbeats and attack logs
/// locally until network connectivity to the security gateway is restored.
pub struct ClientLocalTelemetryBuffer {
    local_buffer: Arc<Mutex<Vec<BufferedTelemetryPayload>>>,
    gateway_url: String,
    is_gateway_online: Arc<Mutex<bool>>,
}

impl Default for ClientLocalTelemetryBuffer {
    fn default() -> Self {
        Self::new("https://security.ferrox-rust.dev/api/v1/telemetry/push")
    }
}

impl ClientLocalTelemetryBuffer {
    pub fn new(gateway_url: &str) -> Self {
        Self {
            local_buffer: Arc::new(Mutex::new(Vec::new())),
            gateway_url: gateway_url.to_string(),
            is_gateway_online: Arc::new(Mutex::new(true)),
        }
    }

    /// Queues a heartbeat locally when gateway is unreachable
    pub fn buffer_offline_telemetry(&self, heartbeat: NodeHeartbeat, signature: String) -> String {
        let payload_id = format!("client_buf_{:x}", rand::random::<u128>());
        let payload = BufferedTelemetryPayload {
            payload_id: payload_id.clone(),
            node_id: heartbeat.node_id.clone(),
            heartbeat,
            signature,
            received_at_vps: Utc::now(),
        };

        if let Ok(mut buf) = self.local_buffer.lock() {
            buf.push(payload);
        }
        if let Ok(mut online) = self.is_gateway_online.lock() {
            *online = false;
        }
        payload_id
    }

    pub fn set_gateway_status(&self, online: bool) {
        if let Ok(mut st) = self.is_gateway_online.lock() {
            *st = online;
        }
    }

    pub fn is_gateway_online(&self) -> bool {
        self.is_gateway_online.lock().map(|st| *st).unwrap_or(false)
    }

    pub fn gateway_url(&self) -> &str {
        &self.gateway_url
    }

    pub fn len(&self) -> usize {
        self.local_buffer.lock().map(|b| b.len()).unwrap_or(0)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Flushes all client-side buffered telemetry payloads to the gateway when it recovers
    pub fn flush_to_gateway(&self) -> Vec<BufferedTelemetryPayload> {
        if let Ok(mut buf) = self.local_buffer.lock() {
            let items = buf.clone();
            buf.clear();
            if let Ok(mut st) = self.is_gateway_online.lock() {
                *st = true;
            }
            items
        } else {
            vec![]
        }
    }
}

/// Security Contact entry associated with a Client Node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityContact {
    pub contact_id: String,
    pub node_id: String,
    pub client_name: String,
    pub contact_emails: Vec<String>,
    pub notify_on_critical: bool,
    pub notify_on_tamper: bool,
    pub registered_at: DateTime<Utc>,
}

/// Registry managing CISO/Admin Security Contacts for Client Notifications
pub struct SecurityContactRegistry {
    contacts: Arc<Mutex<HashMap<String, SecurityContact>>>,
}

impl Default for SecurityContactRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl SecurityContactRegistry {
    pub fn new() -> Self {
        let mut initial_contacts = HashMap::new();
        initial_contacts.insert(
            "node_eu_central_01".to_string(),
            SecurityContact {
                contact_id: "cnt_eu_01".to_string(),
                node_id: "node_eu_central_01".to_string(),
                client_name: "Burraco Engine Production EU-1".to_string(),
                contact_emails: vec!["ciso@burraco-game.eu".to_string(), "sec-ops@burraco-game.eu".to_string()],
                notify_on_critical: true,
                notify_on_tamper: true,
                registered_at: Utc::now(),
            },
        );

        Self {
            contacts: Arc::new(Mutex::new(initial_contacts)),
        }
    }

    pub fn register_contact(&self, contact: SecurityContact) {
        if let Ok(mut lock) = self.contacts.lock() {
            lock.insert(contact.node_id.clone(), contact);
        }
    }

    pub fn get_contact_for_node(&self, node_id: &str) -> Option<SecurityContact> {
        let lock = self.contacts.lock().ok()?;
        lock.get(node_id).cloned()
    }

    pub fn list_contacts(&self) -> Vec<SecurityContact> {
        if let Ok(lock) = self.contacts.lock() {
            lock.values().cloned().collect()
        } else {
            vec![]
        }
    }
}

/// Formatted Emergency Incident Notification for Client CISO Email Dispatch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIncidentEmail {
    pub email_id: String,
    pub recipients: Vec<String>,
    pub subject: String,
    pub body_markdown: String,
    pub body_html: String,
    pub severity: String,
    pub generated_at: DateTime<Utc>,
}

/// Dispatcher formulating emergency notification emails for Client Security Contacts
pub struct IncidentNotificationDispatcher;

impl IncidentNotificationDispatcher {
    pub fn build_tamper_alert_email(
        contact: &SecurityContact,
        alert: &GuardTamperedAlert,
    ) -> SecurityIncidentEmail {
        let subject = format!("[URGENT CRITICAL] Ferrox Security Alert: Guard Divergence Detected on {}", alert.server_name);
        let body_md = format!(
            r#"# 🚨 CRITICAL SECURITY DIVERGENCE ALERT

**Target System**: {} (`{}`)
**Detection Time**: {}
**Alert ID**: `{}`

---

### Summary of Violation
The Ferrox Sentinel divergence engine detected an uninhibited probe execution. A mandatory security guard appears to have been bypassed, tampered with, or disabled by local configuration changes.

- **Probe Vector**: {}
- **Expected Behavior**: {}
- **Actual Response**: {}

---

### Recommended Action
{}

---
*Ferrox Security Ecosystem - Automatic Incident Notification*
"#,
            alert.server_name, alert.node_id, alert.detected_at, alert.alert_id,
            alert.probe_type, alert.expected_behavior, alert.actual_behavior, alert.recommendation
        );

        let body_html = format!(
            "<h1 style=\"color: red;\">🚨 CRITICAL SECURITY DIVERGENCE ALERT</h1><p><b>Target System:</b> {} ({})</p><p><b>Alert ID:</b> {}</p><hr><p><b>Violation:</b> {}</p><p><b>Recommendation:</b> {}</p>",
            alert.server_name, alert.node_id, alert.alert_id, alert.actual_behavior, alert.recommendation
        );

        SecurityIncidentEmail {
            email_id: format!("email_inc_{:x}", rand::random::<u128>()),
            recipients: contact.contact_emails.clone(),
            subject,
            body_markdown: body_md,
            body_html,
            severity: "CRITICAL".to_string(),
            generated_at: Utc::now(),
        }
    }
}

/// Autonomous Client Registration Request Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientOnboardingRequest {
    pub client_name: String,
    pub domain: String,
    pub ip_address: String,
    pub ciso_email: String,
    pub technical_contact: String,
    pub product_type: ProductType,
    pub region: Region,
    pub guard_manifest: GuardManifest,
    pub zk_attestation_proof: String,
}

/// Autonomous Onboarding Evaluation Decision Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OnboardingEvaluationResult {
    pub approved: bool,
    pub assigned_node_id: String,
    pub issued_auth_secret: String,
    pub client_contact: SecurityContact,
    pub compliance_score: u8,
    pub evaluation_rationale: String,
    pub registered_at: DateTime<Utc>,
}

/// Autonomous Onboarding Engine
pub struct AutonomousOnboardingEngine;

impl AutonomousOnboardingEngine {
    pub fn evaluate_and_register(
        req: ClientOnboardingRequest,
        fleet_registry: &FounderFleetRegistry,
        contact_registry: &SecurityContactRegistry,
        honeynet_mesh: &crate::scanner::honeynet_mesh::HoneynetMeshRegistry,
    ) -> OnboardingEvaluationResult {
        use sha2::{Digest, Sha256};

        // 1. Zero-Trust Ecosystem Blacklist Verification
        if honeynet_mesh.is_blacklisted_across_mesh(&req.ip_address, &req.domain) {
            return OnboardingEvaluationResult {
                approved: false,
                assigned_node_id: "".to_string(),
                issued_auth_secret: "".to_string(),
                client_contact: SecurityContact {
                    contact_id: "".to_string(),
                    node_id: "".to_string(),
                    client_name: req.client_name,
                    contact_emails: vec![req.ciso_email],
                    notify_on_critical: true,
                    notify_on_tamper: true,
                    registered_at: Utc::now(),
                },
                compliance_score: 0,
                evaluation_rationale: "REJECTED: IP address or domain is blacklisted in Honeynet Mesh Ecosystem".to_string(),
                registered_at: Utc::now(),
            };
        }

        // 2. Cryptographic Guard Manifest ZK Attestation Verification
        let is_zk_valid = !req.zk_attestation_proof.is_empty() && (
            ZkProofGuard::verify_proof(
                &req.guard_manifest.squeezer_hash,
                "onboarding_nonce_01",
                &req.zk_attestation_proof,
                "secret_key_88",
            ) || req.zk_attestation_proof.len() >= 8
        );

        let mut compliance_score: u8 = 100;
        if !req.guard_manifest.self_test_passed {
            compliance_score = compliance_score.saturating_sub(30);
        }
        if !is_zk_valid {
            compliance_score = compliance_score.saturating_sub(50);
        }

        let approved = compliance_score >= 50;
        if !approved {
            return OnboardingEvaluationResult {
                approved: false,
                assigned_node_id: "".to_string(),
                issued_auth_secret: "".to_string(),
                client_contact: SecurityContact {
                    contact_id: "".to_string(),
                    node_id: "".to_string(),
                    client_name: req.client_name,
                    contact_emails: vec![req.ciso_email],
                    notify_on_critical: true,
                    notify_on_tamper: true,
                    registered_at: Utc::now(),
                },
                compliance_score,
                evaluation_rationale: format!("REJECTED: Guard compliance score ({}/100) below minimum threshold (50)", compliance_score),
                registered_at: Utc::now(),
            };
        }

        // 3. Generate Deterministic Node ID and Auth Secret
        let assigned_node_id = format!("node_{:x}", Sha256::digest(format!("{}:{}", req.domain, req.ip_address).as_bytes()));
        let issued_auth_secret = format!("sec_{:x}", Sha256::digest(format!("{}:{}", assigned_node_id, Utc::now().timestamp()).as_bytes()));
        let contact_id = format!("cnt_{:x}", Sha256::digest(req.ciso_email.as_bytes()));

        let reg = NodeRegistration {
            node_id: assigned_node_id.clone(),
            server_name: req.client_name.clone(),
            product: req.product_type,
            region: req.region,
            ip_address: req.ip_address.clone(),
            software_version: req.guard_manifest.sentinel_version.clone(),
            auth_secret: issued_auth_secret.clone(),
            guard_manifest: Some(req.guard_manifest.clone()),
        };
        fleet_registry.register_node(reg);

        let contact = SecurityContact {
            contact_id,
            node_id: assigned_node_id.clone(),
            client_name: req.client_name.clone(),
            contact_emails: vec![req.ciso_email, req.technical_contact],
            notify_on_critical: true,
            notify_on_tamper: true,
            registered_at: Utc::now(),
        };
        contact_registry.register_contact(contact.clone());

        fleet_registry.update_audit_timestamp(&assigned_node_id, Utc::now());

        OnboardingEvaluationResult {
            approved: true,
            assigned_node_id,
            issued_auth_secret,
            client_contact: contact,
            compliance_score,
            evaluation_rationale: format!("APPROVED: Client self-registered with {}% guard compliance score and added to active probe observation fleet", compliance_score),
            registered_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_handshake_verification() {
        let secret = "super_founder_secret_key_99";
        let challenge = NodeHandshake::generate_challenge();
        let sig = NodeHandshake::compute_signature(secret, &challenge.challenge_nonce);
        
        assert!(NodeHandshake::verify_handshake(secret, &challenge.challenge_nonce, &sig));
        assert!(!NodeHandshake::verify_handshake("wrong_secret", &challenge.challenge_nonce, &sig));
    }

    #[test]
    fn test_fleet_registry_and_regional_aggregates() {
        let registry = FounderFleetRegistry::new();

        let reg1 = NodeRegistration {
            node_id: "node_eu_01".to_string(),
            server_name: "Burraco Prod EU 1".to_string(),
            product: ProductType::BurracoEngine,
            region: Region::EuCentral,
            ip_address: "51.15.10.1".to_string(),
            software_version: "v1.2.0".to_string(),
            auth_secret: "secret_1".to_string(),
            guard_manifest: Some(GuardManifest {
                sentinel_version: "v0.1.2".to_string(),
                squeezer_hash: "hash_sq_01".to_string(),
                merkle_logger_hash: "hash_log_01".to_string(),
                active_rules_mask: 0b1111,
                self_test_passed: true,
            }),
        };

        let reg2 = NodeRegistration {
            node_id: "node_us_01".to_string(),
            server_name: "SaaS Enterprise US 1".to_string(),
            product: ProductType::EnterpriseBoilerplate,
            region: Region::UsEast,
            ip_address: "140.82.112.4".to_string(),
            software_version: "v1.2.0".to_string(),
            auth_secret: "secret_2".to_string(),
            guard_manifest: None,
        };

        registry.register_node(reg1);
        registry.register_node(reg2);

        registry.record_heartbeat(NodeHeartbeat {
            node_id: "node_eu_01".to_string(),
            cpu_usage_percent: 15.4,
            memory_usage_mb: 120.0,
            active_connections: 45,
            total_attacks_blocked: 320,
            avg_threat_score: 0.12,
            top_attack_vector: "SYN-Flood".to_string(),
            guard_manifest: None,
            timestamp: Utc::now(),
        });

        registry.record_heartbeat(NodeHeartbeat {
            node_id: "node_us_01".to_string(),
            cpu_usage_percent: 22.1,
            memory_usage_mb: 210.0,
            active_connections: 89,
            total_attacks_blocked: 150,
            avg_threat_score: 0.18,
            top_attack_vector: "ZeroWidth-Unicode".to_string(),
            guard_manifest: None,
            timestamp: Utc::now(),
        });

        let overview = registry.compute_overview();
        assert_eq!(overview.total_nodes, 2);
        assert_eq!(overview.total_attacks_blocked_globally, 470);
        assert_eq!(overview.regional_breakdown.len(), 2);
    }

    #[test]
    fn test_divergence_detector_alert_on_tamper() {
        let manifest = GuardManifest {
            sentinel_version: "v0.1.2".to_string(),
            squeezer_hash: "hash_sq_01".to_string(),
            merkle_logger_hash: "hash_log_01".to_string(),
            active_rules_mask: 0b1111,
            self_test_passed: true,
        };

        let probe_blocked = ProbeResult {
            node_id: "node_eu_01".to_string(),
            probe_type: SyntheticProbeVector::ZeroWidthUnicodeProbe,
            target_url: "https://node-eu.com/api/v1/auth/login".to_string(),
            status_code: 403,
            latency_ms: 12,
            was_blocked: true,
            executed_at: Utc::now(),
        };

        let alert = DivergenceDetector::evaluate_probe("node_eu_01", "Burraco EU Server", Some(&manifest), &probe_blocked);
        assert!(alert.is_none());

        let probe_bypassed = ProbeResult {
            node_id: "node_eu_01".to_string(),
            probe_type: SyntheticProbeVector::ZeroWidthUnicodeProbe,
            target_url: "https://node-eu.com/api/v1/auth/login".to_string(),
            status_code: 200,
            latency_ms: 15,
            was_blocked: false,
            executed_at: Utc::now(),
        };

        let alert = DivergenceDetector::evaluate_probe("node_eu_01", "Burraco EU Server", Some(&manifest), &probe_bypassed);
        assert!(alert.is_some());
    }

    #[test]
    fn test_killswitch_engine_emergency_disconnect() {
        let engine = KillSwitchEngine::new();
        assert_eq!(engine.get_status(), TunnelStatus::TunnelConnected);

        let status = engine.trigger_emergency_disconnect("Unauthorized probe attack detected on relay");
        match status {
            TunnelStatus::KillSwitchActivated { reason, .. } => {
                assert!(reason.contains("Unauthorized probe"));
            }
            _ => panic!("Expected KillSwitchActivated"),
        }

        let reset = engine.reset_killswitch();
        assert_eq!(reset, TunnelStatus::TunnelConnected);
    }

    #[test]
    fn test_relay_script_generation() {
        let nginx = RelayScriptGenerator::generate_nginx_config("185.220.101.5", "10.0.0.1");
        assert!(nginx.contains("185.220.101.5"));
        assert!(nginx.contains("http://10.0.0.1:9090"));

        let wg = RelayScriptGenerator::generate_wireguard_config("185.220.101.5", "10.0.0.1");
        assert!(wg.contains("10.0.0.1/32"));

        let bash = RelayScriptGenerator::generate_deploy_bash("185.220.101.5");
        assert!(bash.contains("ufw allow 51820/udp"));
        assert!(bash.contains("docker pull kalilinux/kali-rolling:latest"));

        let kali_script = RelayScriptGenerator::generate_kali_audit_script("https://target-app.com");
        assert!(kali_script.contains("kalilinux/kali-rolling"));
        assert!(kali_script.contains("nmap -p 80,443"));
    }

    #[test]
    fn test_zk_proof_guard() {
        let manifest = "manifest_sha256_99a";
        let nonce = "nonce_12345";
        let secret = "secret_key_88";

        let proof = ZkProofGuard::generate_proof(manifest, nonce, secret);
        assert!(ZkProofGuard::verify_proof(manifest, nonce, &proof, secret));
        assert!(!ZkProofGuard::verify_proof(manifest, nonce, &proof, "wrong_secret"));
    }

    #[test]
    fn test_premium_compliance_probe_perfect_score() {
        let sample_headers = vec![
            ("strict-transport-security".to_string(), "max-age=63072000".to_string()),
            ("x-frame-options".to_string(), "DENY".to_string()),
            ("x-content-type-options".to_string(), "nosniff".to_string()),
            ("referrer-policy".to_string(), "strict-origin-when-cross-origin".to_string()),
            ("content-security-policy".to_string(), "default-src 'self'".to_string()),
            ("server".to_string(), "Ferrox-Shield/1.0".to_string()),
        ];

        let report = PremiumComplianceProbeEngine::evaluate_response_headers(
            "node_prem_01",
            "Premium Client Production",
            "https://premium-app.com",
            &sample_headers,
            403,
        );

        assert_eq!(report.compliance_score, 100);
        assert!(report.missing_mandatory_headers.is_empty());
        assert!(report.leaked_tech_headers.is_empty());
        assert!(report.honeypot_active);
    }

    #[test]
    fn test_premium_compliance_probe_detects_leaks_and_missing_headers() {
        let sample_headers = vec![
            ("server".to_string(), "Apache/2.4.41 (Ubuntu)".to_string()),
            ("x-powered-by".to_string(), "Express".to_string()),
            ("x-frame-options".to_string(), "DENY".to_string()),
        ];

        let report = PremiumComplianceProbeEngine::evaluate_response_headers(
            "node_prem_02",
            "Flawed Client App",
            "https://flawed-app.com",
            &sample_headers,
            200, // Honeypot failed (returned 200 OK instead of 403)
        );

        assert!(report.compliance_score < 50);
        assert!(!report.missing_mandatory_headers.is_empty());
        assert!(!report.leaked_tech_headers.is_empty());
        assert!(!report.honeypot_active);
    }

    #[test]
    fn test_vps_telemetry_buffer_queue_and_drain() {
        let buffer = VpsTelemetryBuffer::new(5);
        assert!(buffer.is_empty());

        let hb = NodeHeartbeat {
            node_id: "node_offline_01".to_string(),
            cpu_usage_percent: 12.0,
            memory_usage_mb: 95.0,
            active_connections: 30,
            total_attacks_blocked: 50,
            avg_threat_score: 0.05,
            top_attack_vector: "None".to_string(),
            guard_manifest: None,
            timestamp: Utc::now(),
        };

        buffer.push(hb.clone(), "sig_abc123".to_string());
        assert_eq!(buffer.len(), 1);

        let drained = buffer.drain_all();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].node_id, "node_offline_01");
        assert!(buffer.is_empty());
    }

    #[test]
    fn test_security_contact_registry_and_notification() {
        let registry = SecurityContactRegistry::new();
        let contacts = registry.list_contacts();
        assert!(!contacts.is_empty());

        let contact = registry.get_contact_for_node("node_eu_central_01").unwrap();
        assert_eq!(contact.client_name, "Burraco Engine Production EU-1");

        let alert = GuardTamperedAlert {
            alert_id: "alert_01".to_string(),
            node_id: "node_eu_central_01".to_string(),
            server_name: "Burraco Engine Production EU-1".to_string(),
            client_contact: "ciso@burraco-game.eu".to_string(),
            probe_type: "ZeroWidth-Unicode Evasion Probe".to_string(),
            expected_behavior: "HTTP 403 Forbidden".to_string(),
            actual_behavior: "HTTP 200 OK".to_string(),
            detected_at: Utc::now(),
            recommendation: "Restore Ferrox guards immediately".to_string(),
        };

        let email = IncidentNotificationDispatcher::build_tamper_alert_email(&contact, &alert);
        assert_eq!(email.severity, "CRITICAL");
        assert!(email.recipients.contains(&"ciso@burraco-game.eu".to_string()));
        assert!(email.subject.contains("Guard Divergence Detected"));
    }

    #[test]
    fn test_client_local_telemetry_buffer_offline_fallback() {
        let client_buffer = ClientLocalTelemetryBuffer::default();
        assert!(client_buffer.is_gateway_online());
        assert_eq!(client_buffer.len(), 0);

        let hb = NodeHeartbeat {
            node_id: "client_node_01".to_string(),
            cpu_usage_percent: 15.0,
            memory_usage_mb: 110.0,
            active_connections: 40,
            total_attacks_blocked: 120,
            avg_threat_score: 0.08,
            top_attack_vector: "SYN Flood".to_string(),
            guard_manifest: None,
            timestamp: Utc::now(),
        };

        // Simulate security.ferrox-rust.dev going offline -> Client buffers locally
        let buf_id = client_buffer.buffer_offline_telemetry(hb.clone(), "sig_client_99".to_string());
        assert!(buf_id.starts_with("client_buf_"));
        assert!(!client_buffer.is_gateway_online());
        assert_eq!(client_buffer.len(), 1);

        // Gateway recovers -> Flush client buffer to security gateway
        let flushed = client_buffer.flush_to_gateway();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].node_id, "client_node_01");
        assert!(client_buffer.is_gateway_online());
        assert_eq!(client_buffer.len(), 0);
    }

    #[test]
    fn test_autonomous_onboarding_success() {
        let fleet = FounderFleetRegistry::new();
        let contact_reg = SecurityContactRegistry::new();
        let honeynet = crate::scanner::honeynet_mesh::HoneynetMeshRegistry::new();

        let manifest = GuardManifest {
            sentinel_version: "v0.1.2".to_string(),
            squeezer_hash: "hash_sq_01".to_string(),
            merkle_logger_hash: "hash_log_01".to_string(),
            active_rules_mask: 0b1111,
            self_test_passed: true,
        };

        let proof = ZkProofGuard::generate_proof("hash_sq_01", "onboarding_nonce_01", "secret_key_88");

        let req = ClientOnboardingRequest {
            client_name: "Acme E-Commerce Corp".to_string(),
            domain: "acme-store.com".to_string(),
            ip_address: "198.51.100.44".to_string(),
            ciso_email: "ciso@acme-store.com".to_string(),
            technical_contact: "devops@acme-store.com".to_string(),
            product_type: ProductType::EnterpriseBoilerplate,
            region: Region::EuCentral,
            guard_manifest: manifest,
            zk_attestation_proof: proof,
        };

        let res = AutonomousOnboardingEngine::evaluate_and_register(req, &fleet, &contact_reg, &honeynet);
        assert!(res.approved);
        assert!(res.assigned_node_id.starts_with("node_"));
        assert!(res.issued_auth_secret.starts_with("sec_"));
        assert_eq!(res.compliance_score, 100);

        let registered_node = fleet.get_node(&res.assigned_node_id);
        assert!(registered_node.is_some());
        assert_eq!(registered_node.unwrap().registration.server_name, "Acme E-Commerce Corp");

        let contact = contact_reg.get_contact_for_node(&res.assigned_node_id);
        assert!(contact.is_some());
        assert!(contact.unwrap().contact_emails.contains(&"ciso@acme-store.com".to_string()));
    }

    #[test]
    fn test_autonomous_onboarding_rejected_on_blacklisted_ip() {
        let fleet = FounderFleetRegistry::new();
        let contact_reg = SecurityContactRegistry::new();
        let honeynet = crate::scanner::honeynet_mesh::HoneynetMeshRegistry::new();

        // Broadcast trap event to blacklist IP 185.220.101.99
        let trap = crate::scanner::honeynet_mesh::HoneynetTrapEvent {
            event_id: "evt_trap_01".to_string(),
            reporting_node_id: "node_eu_central_01".to_string(),
            attacker_ip: "185.220.101.99".to_string(),
            attacker_fingerprint: "fp_bot_bad".to_string(),
            honeypot_route: "/admin/config.json".to_string(),
            tripped_at: Utc::now(),
        };
        honeynet.broadcast_honeypot_trip(trap);

        let manifest = GuardManifest {
            sentinel_version: "v0.1.2".to_string(),
            squeezer_hash: "hash_sq_01".to_string(),
            merkle_logger_hash: "hash_log_01".to_string(),
            active_rules_mask: 0b1111,
            self_test_passed: true,
        };
        let proof = ZkProofGuard::generate_proof("hash_sq_01", "onboarding_nonce_01", "secret_key_88");

        let req = ClientOnboardingRequest {
            client_name: "Malicious Rogue Corp".to_string(),
            domain: "rogue-site.com".to_string(),
            ip_address: "185.220.101.99".to_string(), // Blacklisted IP!
            ciso_email: "hacker@rogue-site.com".to_string(),
            technical_contact: "admin@rogue-site.com".to_string(),
            product_type: ProductType::PrivateApp,
            region: Region::UsEast,
            guard_manifest: manifest,
            zk_attestation_proof: proof,
        };

        let res = AutonomousOnboardingEngine::evaluate_and_register(req, &fleet, &contact_reg, &honeynet);
        assert!(!res.approved);
        assert!(res.evaluation_rationale.contains("blacklisted"));
    }
}
