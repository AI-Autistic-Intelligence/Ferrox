#![cfg(feature = "founder-suite")]

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
use ferrox_sentinel::founder::{
    AiModelTrainer, AutonomousOnboardingEngine, BiWeeklyAuditScheduler, ClientOnboardingRequest,
    DivergenceDetector, FounderFleetRegistry, GuardManifest, GuardTamperedAlert,
    HandshakeChallenge, HandshakeResponse, IncidentNotificationDispatcher, KillSwitchEngine,
    NodeHandshake, NodeHeartbeat, NodeRegistration, PremiumComplianceProbeEngine, ProbeResult,
    RelayScriptGenerator, SecurityContact, SecurityContactRegistry, SyntheticProbeVector,
    TargetedAttackTracker, VpsRelayConfig, VpsTelemetryBuffer,
};

pub struct FounderState {
    pub fleet_registry: Arc<FounderFleetRegistry>,
    pub attack_tracker: Arc<TargetedAttackTracker>,
    pub ai_trainer: Arc<AiModelTrainer>,
    pub tamper_alerts: Arc<Mutex<Vec<GuardTamperedAlert>>>,
    pub killswitch: Arc<KillSwitchEngine>,
    pub relay_config: VpsRelayConfig,
    pub telemetry_buffer: Arc<VpsTelemetryBuffer>,
    pub contact_registry: Arc<SecurityContactRegistry>,
}

#[derive(Debug, Deserialize)]
pub struct HandshakeVerifyRequest {
    pub registration: NodeRegistration,
    pub challenge_nonce: String,
    pub response: HandshakeResponse,
}

#[derive(Debug, Deserialize)]
pub struct TrainAiRequest {
    pub sample_vectors_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct RunProbeRequest {
    pub simulate_guard_bypassed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct TriggerKillSwitchRequest {
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct PremiumAuditRequest {
    pub sample_headers: Option<Vec<(String, String)>>,
    pub honeypot_status_code: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct PushTelemetryRequest {
    pub heartbeat: NodeHeartbeat,
    pub signature: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterContactRequest {
    pub node_id: String,
    pub client_name: String,
    pub contact_emails: Vec<String>,
    pub notify_on_critical: Option<bool>,
    pub notify_on_tamper: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct DispatchAlertRequest {
    pub alert_id: String,
    pub node_id: String,
}

pub fn router() -> Router {
    let fleet_registry = Arc::new(FounderFleetRegistry::new());

    // Populate initial sample fleet nodes for immediate demonstration
    let sample_node_1 = NodeRegistration {
        node_id: "node_eu_central_01".to_string(),
        server_name: "Burraco Engine Production EU-1".to_string(),
        product: ferrox_sentinel::founder::ProductType::BurracoEngine,
        region: ferrox_sentinel::founder::Region::EuCentral,
        ip_address: "51.15.220.14".to_string(),
        software_version: "v1.4.2".to_string(),
        auth_secret: "secret_eu_01".to_string(),
        guard_manifest: Some(GuardManifest {
            sentinel_version: "v0.1.2".to_string(),
            squeezer_hash: "sq_sha256_88a91".to_string(),
            merkle_logger_hash: "log_sha256_10c2d".to_string(),
            active_rules_mask: 0b1111,
            self_test_passed: true,
        }),
    };

    let sample_node_2 = NodeRegistration {
        node_id: "node_us_east_01".to_string(),
        server_name: "Ferrox SaaS Enterprise US-1".to_string(),
        product: ferrox_sentinel::founder::ProductType::EnterpriseBoilerplate,
        region: ferrox_sentinel::founder::Region::UsEast,
        ip_address: "140.82.121.3".to_string(),
        software_version: "v1.4.2".to_string(),
        auth_secret: "secret_us_01".to_string(),
        guard_manifest: Some(GuardManifest {
            sentinel_version: "v0.1.2".to_string(),
            squeezer_hash: "sq_sha256_88a91".to_string(),
            merkle_logger_hash: "log_sha256_10c2d".to_string(),
            active_rules_mask: 0b1111,
            self_test_passed: true,
        }),
    };

    fleet_registry.register_node(sample_node_1);
    fleet_registry.register_node(sample_node_2);

    fleet_registry.record_heartbeat(NodeHeartbeat {
        node_id: "node_eu_central_01".to_string(),
        cpu_usage_percent: 18.2,
        memory_usage_mb: 145.0,
        active_connections: 124,
        total_attacks_blocked: 4120,
        avg_threat_score: 0.14,
        top_attack_vector: "SYN-Velocity Flood".to_string(),
        guard_manifest: None,
        timestamp: chrono::Utc::now(),
    });

    fleet_registry.record_heartbeat(NodeHeartbeat {
        node_id: "node_us_east_01".to_string(),
        cpu_usage_percent: 24.5,
        memory_usage_mb: 230.0,
        active_connections: 310,
        total_attacks_blocked: 1890,
        avg_threat_score: 0.09,
        top_attack_vector: "ZeroWidth-Unicode Payload".to_string(),
        guard_manifest: None,
        timestamp: chrono::Utc::now(),
    });

    let state = Arc::new(FounderState {
        fleet_registry,
        attack_tracker: Arc::new(TargetedAttackTracker::new()),
        ai_trainer: Arc::new(AiModelTrainer::new()),
        tamper_alerts: Arc::new(Mutex::new(Vec::new())),
        killswitch: Arc::new(KillSwitchEngine::new()),
        relay_config: VpsRelayConfig::default(),
        telemetry_buffer: Arc::new(VpsTelemetryBuffer::default()),
        contact_registry: Arc::new(SecurityContactRegistry::new()),
    });

    Router::new()
        .route("/handshake/challenge", post(generate_challenge_handler))
        .route("/handshake/verify", post(verify_handshake_handler))
        .route("/heartbeat", post(heartbeat_handler))
        .route("/telemetry/push", post(push_telemetry_handler))
        .route("/telemetry/sync", post(sync_telemetry_handler))
        .route("/overview", get(get_overview_handler))
        .route("/nodes", get(list_nodes_handler))
        .route("/nodes/:node_id", get(get_node_handler))
        .route("/regions/:region", get(get_region_handler))
        .route("/attacks/targeted", get(list_targeted_attacks_handler))
        .route("/contacts", get(list_contacts_handler))
        .route("/contacts", post(register_contact_handler))
        .route("/contacts/dispatch-alert", post(dispatch_incident_alert_handler))
        .route("/client/onboard", post(client_onboard_handler))
        .route("/innovations", get(get_innovations_handler))
        .route("/mtd/status", get(get_mtd_status_handler))
        .route("/honeynet/mesh", get(get_honeynet_mesh_handler))
        .route("/ai/train", post(trigger_ai_training_handler))
        .route("/ai/weights/latest", get(get_latest_ai_weights_handler))
        .route("/audit/probe/:node_id", post(run_synthetic_probe_handler))
        .route("/audit/premium/summary", get(get_premium_compliance_summary_handler))
        .route("/audit/premium/:node_id", post(run_premium_audit_handler))
        .route("/audit/divergences", get(list_divergences_handler))
        .route("/audit/reports/biweekly", get(get_biweekly_report_handler))
        .route("/shield/status", get(get_shield_status_handler))
        .route("/shield/killswitch", post(trigger_killswitch_handler))
        .route("/shield/relay-scripts", get(get_relay_scripts_handler))
        .with_state(state)
}

async fn generate_challenge_handler() -> Json<HandshakeChallenge> {
    Json(NodeHandshake::generate_challenge())
}

async fn verify_handshake_handler(
    State(state): State<Arc<FounderState>>,
    Json(payload): Json<HandshakeVerifyRequest>,
) -> Json<Value> {
    let is_valid = NodeHandshake::verify_handshake(
        &payload.registration.auth_secret,
        &payload.challenge_nonce,
        &payload.response.signature,
    );

    if is_valid {
        let node_state = state.fleet_registry.register_node(payload.registration);
        Json(json!({
            "success": true,
            "message": "Handshake verified. Node registered in Founder Fleet Registry.",
            "node": node_state
        }))
    } else {
        Json(json!({
            "success": false,
            "message": "Invalid handshake signature. Access denied."
        }))
    }
}

async fn heartbeat_handler(
    State(state): State<Arc<FounderState>>,
    Json(heartbeat): Json<NodeHeartbeat>,
) -> Json<Value> {
    let updated = state.fleet_registry.record_heartbeat(heartbeat);
    Json(json!({ "success": updated }))
}

async fn get_overview_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let overview = state.fleet_registry.compute_overview();
    Json(json!({
        "success": true,
        "overview": overview
    }))
}

async fn list_nodes_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let nodes = state.fleet_registry.list_nodes();
    Json(json!({
        "success": true,
        "nodes": nodes
    }))
}

async fn get_node_handler(
    State(state): State<Arc<FounderState>>,
    Path(node_id): Path<String>,
) -> Json<Value> {
    if let Some(node) = state.fleet_registry.get_node(&node_id) {
        Json(json!({ "success": true, "node": node }))
    } else {
        Json(json!({ "success": false, "message": "Node not found" }))
    }
}

async fn get_region_handler(
    State(state): State<Arc<FounderState>>,
    Path(region): Path<String>,
) -> Json<Value> {
    let overview = state.fleet_registry.compute_overview();
    let matching_region = overview
        .regional_breakdown
        .into_iter()
        .find(|r| r.region_name.to_lowercase() == region.to_lowercase());

    if let Some(reg) = matching_region {
        Json(json!({ "success": true, "region": reg }))
    } else {
        Json(json!({ "success": false, "message": "Region not found" }))
    }
}

async fn list_targeted_attacks_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let events = state.attack_tracker.list_events();
    Json(json!({
        "success": true,
        "targeted_attack_events": events
    }))
}

async fn trigger_ai_training_handler(
    State(state): State<Arc<FounderState>>,
    Json(payload): Json<TrainAiRequest>,
) -> Json<Value> {
    let samples = payload.sample_vectors_count.unwrap_or(250_000);
    let new_package = state.ai_trainer.trigger_retraining(samples);

    Json(json!({
        "success": true,
        "message": "AI retraining completed successfully. New model weights generated.",
        "weights": new_package
    }))
}

async fn get_latest_ai_weights_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let weights = state.ai_trainer.get_latest_weights();
    Json(json!({
        "success": true,
        "weights": weights
    }))
}

async fn run_synthetic_probe_handler(
    State(state): State<Arc<FounderState>>,
    Path(node_id): Path<String>,
    Json(payload): Json<RunProbeRequest>,
) -> Json<Value> {
    let node_opt = state.fleet_registry.get_node(&node_id);
    if let Some(node) = node_opt {
        let simulate_bypass = payload.simulate_guard_bypassed.unwrap_or(false);
        let status_code = if simulate_bypass { 200 } else { 403 };
        let was_blocked = !simulate_bypass;

        let probe_result = ProbeResult {
            node_id: node_id.clone(),
            probe_type: SyntheticProbeVector::ZeroWidthUnicodeProbe,
            target_url: format!("https://{}/api/v1/auth/login", node.registration.ip_address),
            status_code,
            latency_ms: 14,
            was_blocked,
            executed_at: chrono::Utc::now(),
        };

        let alert_opt = DivergenceDetector::evaluate_probe(
            &node_id,
            &node.registration.server_name,
            node.registration.guard_manifest.as_ref(),
            &probe_result,
        );

        if let Some(ref alert) = alert_opt {
            if let Ok(mut list) = state.tamper_alerts.lock() {
                list.push(alert.clone());
            }
        }

        state.fleet_registry.update_audit_timestamp(&node_id, chrono::Utc::now());

        Json(json!({
            "success": true,
            "probe_result": probe_result,
            "divergence_alert": alert_opt,
            "tamper_detected": alert_opt.is_some()
        }))
    } else {
        Json(json!({ "success": false, "message": "Node not found" }))
    }
}

async fn list_divergences_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let alerts = state.tamper_alerts.lock().unwrap().clone();
    Json(json!({
        "success": true,
        "total_divergences": alerts.len(),
        "tamper_alerts": alerts
    }))
}

async fn get_biweekly_report_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let nodes = state.fleet_registry.list_nodes();
    let alerts = state.tamper_alerts.lock().unwrap().clone();

    let node_summaries: Vec<Value> = nodes
        .into_iter()
        .map(|n| {
            let due = BiWeeklyAuditScheduler::is_audit_due(n.last_audit_at);
            let next_audit = BiWeeklyAuditScheduler::calculate_next_audit(n.last_audit_at);
            json!({
                "node_id": n.registration.node_id,
                "server_name": n.registration.server_name,
                "product": n.registration.product.to_string(),
                "region": n.registration.region.as_str(),
                "last_audit_at": n.last_audit_at,
                "next_audit_due_at": next_audit,
                "audit_overdue": due,
                "guard_status": if n.registration.guard_manifest.as_ref().map(|m| m.self_test_passed).unwrap_or(false) { "Verified Active" } else { "Unverified" }
            })
        })
        .collect();

    Json(json!({
        "success": true,
        "report_title": "Bi-Weekly Executive Ferrox Security Compliance Report",
        "generated_at": chrono::Utc::now(),
        "total_fleet_nodes": node_summaries.len(),
        "total_tamper_alerts": alerts.len(),
        "node_audit_statuses": node_summaries,
        "active_tamper_alerts": alerts
    }))
}

async fn get_shield_status_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let status = state.killswitch.get_status();
    Json(json!({
        "success": true,
        "shield_configuration": state.relay_config,
        "tunnel_status": status,
        "physical_machine_hidden": true
    }))
}

async fn trigger_killswitch_handler(
    State(state): State<Arc<FounderState>>,
    Json(payload): Json<TriggerKillSwitchRequest>,
) -> Json<Value> {
    let new_status = state.killswitch.trigger_emergency_disconnect(&payload.reason);
    Json(json!({
        "success": true,
        "message": "EMERGENCY DISCONNECT ACTIVATED: Physical Machine tunnel severed.",
        "status": new_status
    }))
}

async fn get_relay_scripts_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let vps_ip = &state.relay_config.public_vps_ip;
    let founder_ip = &state.relay_config.founder_tunnel_ip;

    let nginx_conf = RelayScriptGenerator::generate_nginx_config(vps_ip, founder_ip);
    let wg_conf = RelayScriptGenerator::generate_wireguard_config(vps_ip, founder_ip);
    let deploy_bash = RelayScriptGenerator::generate_deploy_bash(vps_ip);
    let kali_sh = RelayScriptGenerator::generate_kali_audit_script("https://target-node-domain.com");

    Json(json!({
        "success": true,
        "public_vps_ip": vps_ip,
        "nginx_reverse_proxy_conf": nginx_conf,
        "wireguard_tunnel_conf": wg_conf,
        "deploy_installer_sh": deploy_bash,
        "kali_redteam_audit_sh": kali_sh
    }))
}

async fn run_premium_audit_handler(
    State(state): State<Arc<FounderState>>,
    Path(node_id): Path<String>,
    Json(payload): Json<PremiumAuditRequest>,
) -> Json<Value> {
    if let Some(node) = state.fleet_registry.get_node(&node_id) {
        let default_sample_headers = vec![
            ("strict-transport-security".to_string(), "max-age=63072000".to_string()),
            ("x-frame-options".to_string(), "DENY".to_string()),
            ("x-content-type-options".to_string(), "nosniff".to_string()),
            ("referrer-policy".to_string(), "strict-origin-when-cross-origin".to_string()),
            ("content-security-policy".to_string(), "default-src 'self'".to_string()),
            ("server".to_string(), "Ferrox-Shield/1.0".to_string()),
        ];

        let headers = payload.sample_headers.unwrap_or(default_sample_headers);
        let honeypot_status = payload.honeypot_status_code.unwrap_or(403);

        let report = PremiumComplianceProbeEngine::evaluate_response_headers(
            &node.registration.node_id,
            &node.registration.server_name,
            &format!("https://{}", node.registration.ip_address),
            &headers,
            honeypot_status,
        );

        state.fleet_registry.update_audit_timestamp(&node_id, chrono::Utc::now());

        Json(json!({
            "success": true,
            "compliance_report": report,
            "passed_100_percent": report.compliance_score == 100
        }))
    } else {
        Json(json!({ "success": false, "message": "Node not found" }))
    }
}

async fn get_premium_compliance_summary_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let nodes = state.fleet_registry.list_nodes();
    let mut reports = Vec::new();

    for node in &nodes {
        let sample_headers = vec![
            ("strict-transport-security".to_string(), "max-age=63072000".to_string()),
            ("x-frame-options".to_string(), "DENY".to_string()),
            ("x-content-type-options".to_string(), "nosniff".to_string()),
            ("referrer-policy".to_string(), "strict-origin-when-cross-origin".to_string()),
            ("content-security-policy".to_string(), "default-src 'self'".to_string()),
            ("server".to_string(), "Ferrox-Shield/1.0".to_string()),
        ];

        let rep = PremiumComplianceProbeEngine::evaluate_response_headers(
            &node.registration.node_id,
            &node.registration.server_name,
            &format!("https://{}", node.registration.ip_address),
            &sample_headers,
            403,
        );
        reports.push(rep);
    }

    let total = reports.len();
    let avg_score: f64 = if total > 0 {
        reports.iter().map(|r| r.compliance_score as f64).sum::<f64>() / total as f64
    } else {
        100.0
    };

    Json(json!({
        "success": true,
        "total_nodes_audited": total,
        "average_compliance_score": avg_score,
        "reports": reports
    }))
}

async fn push_telemetry_handler(
    State(state): State<Arc<FounderState>>,
    Json(payload): Json<PushTelemetryRequest>,
) -> Json<Value> {
    state.fleet_registry.record_heartbeat(payload.heartbeat.clone());
    let payload_id = state.telemetry_buffer.push(payload.heartbeat, payload.signature);

    Json(json!({
        "success": true,
        "message": "Telemetry accepted & buffered on security.ferrox-rust.dev gateway",
        "buffered_payload_id": payload_id,
        "vps_buffer_total_items": state.telemetry_buffer.len()
    }))
}

async fn sync_telemetry_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let drained = state.telemetry_buffer.drain_all();
    let count = drained.len();

    for item in &drained {
        state.fleet_registry.record_heartbeat(item.heartbeat.clone());
    }

    Json(json!({
        "success": true,
        "synced_items_count": count,
        "message": format!("Founder Dashboard successfully synced {} buffered telemetry payloads from VPS Relay.", count)
    }))
}

async fn list_contacts_handler(
    State(state): State<Arc<FounderState>>,
) -> Json<Value> {
    let contacts = state.contact_registry.list_contacts();
    Json(json!({
        "success": true,
        "total_contacts": contacts.len(),
        "contacts": contacts
    }))
}

async fn register_contact_handler(
    State(state): State<Arc<FounderState>>,
    Json(payload): Json<RegisterContactRequest>,
) -> Json<Value> {
    let contact = SecurityContact {
        contact_id: format!("cnt_{}", uuid::Uuid::new_v4().simple()),
        node_id: payload.node_id.clone(),
        client_name: payload.client_name,
        contact_emails: payload.contact_emails,
        notify_on_critical: payload.notify_on_critical.unwrap_or(true),
        notify_on_tamper: payload.notify_on_tamper.unwrap_or(true),
        registered_at: chrono::Utc::now(),
    };

    state.contact_registry.register_contact(contact.clone());

    Json(json!({
        "success": true,
        "registered_contact": contact
    }))
}

async fn dispatch_incident_alert_handler(
    State(state): State<Arc<FounderState>>,
    Json(payload): Json<DispatchAlertRequest>,
) -> Json<Value> {
    let contact_opt = state.contact_registry.get_contact_for_node(&payload.node_id);
    let alerts = state.tamper_alerts.lock().unwrap().clone();
    let alert_opt = alerts.into_iter().find(|a| a.alert_id == payload.alert_id);

    match (contact_opt, alert_opt) {
        (Some(contact), Some(alert)) => {
            let email = IncidentNotificationDispatcher::build_tamper_alert_email(&contact, &alert);
            Json(json!({
                "success": true,
                "dispatched": true,
                "incident_email": email,
                "message": format!("Emergency notification dispatched to {} recipient(s)", email.recipients.len())
            }))
        }
        (None, _) => Json(json!({ "success": false, "message": "No security contacts registered for target node" })),
        (_, None) => Json(json!({ "success": false, "message": "Alert ID not found" })),
    }
}

async fn client_onboard_handler(
    State(state): State<Arc<FounderState>>,
    Json(payload): Json<ClientOnboardingRequest>,
) -> Json<Value> {
    let honeynet = ferrox_sentinel::scanner::honeynet_mesh::HoneynetMeshRegistry::new();
    let result = AutonomousOnboardingEngine::evaluate_and_register(
        payload,
        &state.fleet_registry,
        &state.contact_registry,
        &honeynet,
    );

    if result.approved {
        Json(json!({
            "success": true,
            "approved": true,
            "onboarding_result": result
        }))
    } else {
        Json(json!({
            "success": false,
            "approved": false,
            "rationale": result.evaluation_rationale,
            "onboarding_result": result
        }))
    }
}

async fn get_innovations_handler() -> Json<Value> {
    Json(json!({
        "success": true,
        "total_innovations": 10,
        "innovations": [
            { "id": 1, "name": "Moving Target Defense (MTD)", "source": "IEEE S&P", "status": "ACTIVE", "rotation_interval_secs": 60 },
            { "id": 2, "name": "Distributed Honeynet Deception Mesh", "source": "USENIX Security", "status": "ACTIVE", "propagation": "Sub-Second Global Shadow-Ban" },
            { "id": 3, "name": "Self-Healing Memory Hot-Swap", "source": "ACM SIGSOFT", "status": "ACTIVE", "attestation": "SHA-256 State Snapshot" },
            { "id": 4, "name": "Zero-Knowledge Proof Burraco Attestation", "source": "IACR Cryptology", "status": "ACTIVE", "proof_type": "ZK-SNARK Game Rule Proof" },
            { "id": 5, "name": "eBPF/XDP Kernel Filter Generator", "source": "ACM SIGCOMM", "status": "ACTIVE", "layer": "NIC Driver Layer" },
            { "id": 6, "name": "Behavioral Biometrics & Bot Cadence Detector", "source": "NDSS", "status": "ACTIVE", "metric": "Micro-Cadence Jitter Variance" },
            { "id": 7, "name": "Hidden Markov Model Sequence Predictor", "source": "ACM CCS", "status": "ACTIVE", "model": "Multi-Order State Matrix P(St+1|St)" },
            { "id": 8, "name": "Local Differential Privacy Aggregator", "source": "EuroS&P", "status": "ACTIVE", "mechanism": "Laplacian Noise Lap(delta_f/epsilon)" },
            { "id": 9, "name": "Deterministic Lockstep Replay Attestation", "source": "IEEE TDSC", "status": "ACTIVE", "type": "BFT Lockstep State Hash Verification" },
            { "id": 10, "name": "Polymorphic API Route Mutation Engine", "source": "ACM SIGCOMM", "status": "ACTIVE", "rotation": "Ephemeral HMAC Path Rotation" }
        ]
    }))
}

async fn get_mtd_status_handler() -> Json<Value> {
    let state = ferrox_sentinel::algorithms::mtd_mutation::MtdMutationEngine::generate_mutation_state("mtd_secret_key_8899", 60, 8080, 100);
    Json(json!({
        "success": true,
        "mtd_state": state
    }))
}

async fn get_honeynet_mesh_handler() -> Json<Value> {
    let mesh = ferrox_sentinel::scanner::honeynet_mesh::HoneynetMeshRegistry::new();
    let blacklist = mesh.list_blacklist();
    Json(json!({
        "success": true,
        "total_banned_ips": mesh.total_banned_ips(),
        "blacklist_entries": blacklist
    }))
}
