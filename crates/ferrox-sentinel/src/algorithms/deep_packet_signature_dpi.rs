//! # Deep Packet Signature Inspection (DPI) Engine (`deep_packet_signature_dpi.rs`)
//!
//! Raw magic-byte signature matching for non-standard port protocol identification 
//! (*Network Security Through Data Analysis*, O'Reilly - Ch. 8).

use serde::{Deserialize, Serialize};

/// Identified Protocol Classification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IdentifiedProtocol {
    GrpcHttp2,
    TlsHandshake,
    SshSession,
    ModbusIcs,
    DnsQuery,
    Unknown,
}

/// DPI Identification Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DpiInspectionResult {
    pub identified_protocol: IdentifiedProtocol,
    pub is_non_standard_port: bool,
    pub port: u16,
    pub payload_magic_hex: String,
    pub anomaly_alert: Option<String>,
}

/// Deep Packet Signature Inspection (DPI) Engine
pub struct DeepPacketSignatureDpiEngine;

impl DeepPacketSignatureDpiEngine {
    /// Inspects raw packet bytes and destination port to detect protocol mismatch on non-standard ports
    pub fn inspect_payload(port: u16, payload: &[u8]) -> DpiInspectionResult {
        let magic_hex = payload.iter().take(8).map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");

        let identified_protocol = if payload.starts_with(b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n") {
            IdentifiedProtocol::GrpcHttp2
        } else if payload.len() >= 3 && payload[0] == 0x16 && payload[1] == 0x03 {
            IdentifiedProtocol::TlsHandshake
        } else if payload.starts_with(b"SSH-2.0") || payload.starts_with(b"SSH-1.99") {
            IdentifiedProtocol::SshSession
        } else if payload.len() >= 7 && payload[2] == 0x00 && payload[3] == 0x00 { // Modbus MBAP Header
            IdentifiedProtocol::ModbusIcs
        } else {
            IdentifiedProtocol::Unknown
        };

        let is_non_standard_port = match identified_protocol {
            IdentifiedProtocol::SshSession => port != 22,
            IdentifiedProtocol::TlsHandshake | IdentifiedProtocol::GrpcHttp2 => port != 443 && port != 8443,
            IdentifiedProtocol::ModbusIcs => port != 502,
            _ => false,
        };

        let anomaly_alert = if is_non_standard_port {
            Some(format!(
                "Protocol {:?} detected on non-standard port {}",
                identified_protocol, port
            ))
        } else {
            None
        };

        DpiInspectionResult {
            identified_protocol,
            is_non_standard_port,
            port,
            payload_magic_hex: magic_hex,
            anomaly_alert,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dpi_ssh_over_https_port() {
        let ssh_payload = b"SSH-2.0-OpenSSH_8.9p1 Ubuntu-3ubuntu0.1\r\n";
        let port = 443; // Non-standard port for SSH!

        let result = DeepPacketSignatureDpiEngine::inspect_payload(port, ssh_payload);
        assert_eq!(result.identified_protocol, IdentifiedProtocol::SshSession);
        assert!(result.is_non_standard_port);
        assert!(result.anomaly_alert.is_some());
    }
}
