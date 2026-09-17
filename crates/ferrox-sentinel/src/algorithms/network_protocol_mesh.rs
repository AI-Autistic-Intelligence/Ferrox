//! # Multi-Protocol Network Threat & Flow Mining Mesh (`network_protocol_mesh.rs`)
//!
//! Synthesizes network protocol security, NetFlow analysis, and packet dissection principles
//! (*Attacking Network Protocols*, James Forshaw; *Network Security Through Data Analysis*, Michael Collins; *Network Basics for Hackers*, OccupyTheWeb).

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// DNS Query Analysis Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsQueryPayload {
    pub query_name: String,
    pub query_type: String, // A, AAAA, TXT, CNAME
    pub payload_len: usize,
}

/// NetFlow / IPFIX Session Flow Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetFlowRecord {
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub packets_sent: u64,
    pub packets_received: u64,
}

/// Network Protocol Assessment Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSecurityAssessment {
    pub is_anomaly_detected: bool,
    pub threat_type: String,
    pub risk_score: f64,
}

/// Multi-Protocol Network Threat Mesh Engine
pub struct NetworkProtocolMeshEngine {
    arp_table: HashMap<String, String>, // IP -> MAC address mapping
}

impl Default for NetworkProtocolMeshEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkProtocolMeshEngine {
    pub fn new() -> Self {
        Self {
            arp_table: HashMap::new(),
        }
    }

    /// Evaluates DNS queries for DNS Tunneling exfiltration vectors (high subdomain entropy & large TXT payloads)
    pub fn inspect_dns_query(&self, query: &DnsQueryPayload) -> NetworkSecurityAssessment {
        let entropy = calculate_entropy(&query.query_name);

        // DNS Tunneling Vector: High entropy subdomain (> 4.2) or abnormally large TXT record payload (> 200 bytes)
        if query.query_type == "TXT" && query.payload_len > 180 {
            return NetworkSecurityAssessment {
                is_anomaly_detected: true,
                threat_type: "DNS TXT Payload Exfiltration Tunneling".to_string(),
                risk_score: 0.92,
            };
        }

        if entropy > 4.3 && query.query_name.len() > 30 {
            return NetworkSecurityAssessment {
                is_anomaly_detected: true,
                threat_type: "High-Entropy Subdomain Encoding (DNS Tunnel)".to_string(),
                risk_score: 0.88,
            };
        }

        NetworkSecurityAssessment {
            is_anomaly_detected: false,
            threat_type: "Benign DNS Query".to_string(),
            risk_score: 0.0,
        }
    }

    /// Monitors ARP replies for Gratuitous ARP Poisoning & MITM MAC Address Drift
    pub fn inspect_arp_reply(&mut self, ip: &str, mac: &str, is_gratuitous: bool) -> NetworkSecurityAssessment {
        if let Some(existing_mac) = self.arp_table.get(ip) {
            if existing_mac != mac {
                return NetworkSecurityAssessment {
                    is_anomaly_detected: true,
                    threat_type: format!("Gratuitous ARP Spoofing / MAC Mismatch ({}) -> ({})", existing_mac, mac),
                    risk_score: 0.95,
                };
            }
        } else {
            self.arp_table.insert(ip.to_string(), mac.to_string());
        }

        if is_gratuitous {
            return NetworkSecurityAssessment {
                is_anomaly_detected: true,
                threat_type: "Unsolicited Gratuitous ARP Notice (Possible MitM Probe)".to_string(),
                risk_score: 0.65,
            };
        }

        NetworkSecurityAssessment {
            is_anomaly_detected: false,
            threat_type: "Valid ARP Entry".to_string(),
            risk_score: 0.0,
        }
    }

    /// Analyzes NetFlow / IPFIX flow volume asymmetry ratio (R_in/out) to detect stealth C2 beaconing
    pub fn inspect_netflow(&self, flow: &NetFlowRecord) -> NetworkSecurityAssessment {
        if flow.bytes_sent > 0 && flow.bytes_received > 0 {
            let ratio = (flow.bytes_sent as f64) / (flow.bytes_received as f64);
            // Stealth C2 Exfiltration: Outbound bytes vastly exceed inbound response bytes (> 50:1 ratio)
            if ratio > 50.0 && flow.bytes_sent > 100_000 {
                return NetworkSecurityAssessment {
                    is_anomaly_detected: true,
                    threat_type: format!("Stealth Flow Exfiltration Asymmetry (Ratio {:.1}:1)", ratio),
                    risk_score: 0.85,
                };
            }
        }

        NetworkSecurityAssessment {
            is_anomaly_detected: false,
            threat_type: "Balanced NetFlow Record".to_string(),
            risk_score: 0.0,
        }
    }
}

fn calculate_entropy(s: &str) -> f64 {
    if s.is_empty() { return 0.0; }
    let mut counts = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0usize) += 1;
    }
    let len = s.len() as f64;
    counts.values().map(|&c| {
        let p = c as f64 / len;
        -p * p.log2()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dns_tunneling_detection() {
        let engine = NetworkProtocolMeshEngine::new();
        let payload = DnsQueryPayload {
            query_name: "a8f3b9c1d2e4f5a6b7c8d9e0f1.exfil.domain.com".to_string(),
            query_type: "TXT".to_string(),
            payload_len: 250,
        };

        let res = engine.inspect_dns_query(&payload);
        assert!(res.is_anomaly_detected);
        assert!(res.risk_score >= 0.88);
    }

    #[test]
    fn test_arp_spoofing_detection() {
        let mut engine = NetworkProtocolMeshEngine::new();
        engine.inspect_arp_reply("192.168.1.1", "00:11:22:33:44:55", false);

        // MAC changes -> Spoofing alert!
        let spoof = engine.inspect_arp_reply("192.168.1.1", "AA:BB:CC:DD:EE:FF", true);
        assert!(spoof.is_anomaly_detected);
        assert_eq!(spoof.risk_score, 0.95);
    }

    #[test]
    fn test_netflow_asymmetry_detection() {
        let engine = NetworkProtocolMeshEngine::new();
        let flow = NetFlowRecord {
            src_ip: "10.0.0.5".to_string(),
            dst_ip: "198.51.100.1".to_string(),
            src_port: 54321,
            dst_port: 443,
            bytes_sent: 5_000_000,
            bytes_received: 10_000,
            packets_sent: 4000,
            packets_received: 200,
        };

        let res = engine.inspect_netflow(&flow);
        assert!(res.is_anomaly_detected);
        assert!(res.threat_type.contains("Asymmetry"));
    }
}
