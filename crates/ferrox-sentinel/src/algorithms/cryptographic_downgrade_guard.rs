//! # Cryptographic Downgrade & Protocol State-Confusion Watchdog (`cryptographic_downgrade_guard.rs`)
//!
//! TLS cipher downgrade & unauthenticated packet state-confusion watchdog 
//! (*Attacking Network Protocols*, No Starch Press - Ch. 7 & 8).

use serde::{Deserialize, Serialize};

/// Cipher Suite & Protocol Version Telemetry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TlsHandshakeTelemetry {
    pub client_requested_version: u16, // e.g. 0x0304 (TLS 1.3), 0x0301 (TLS 1.0)
    pub negotiated_cipher_suite: u16,  // e.g. 0x1301 (AES-128-GCM), 0x0005 (RC4-SHA)
    pub is_resumption: bool,
    pub protocol_state: u8,            // 0: Initial, 1: Handshake, 2: Authenticated Data
}

/// Cryptographic Downgrade Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DowngradeAlert {
    pub threat_type: String,
    pub severity: f64,
    pub description: String,
}

/// Cryptographic Downgrade & Protocol State-Confusion Watchdog Engine
pub struct CryptographicDowngradeGuardEngine;

impl CryptographicDowngradeGuardEngine {
    /// Inspects TLS Handshake & Protocol State machine transitions
    pub fn inspect_tls_handshake(telemetry: &TlsHandshakeTelemetry) -> Option<DowngradeAlert> {
        // 1. Detect TLS Version Downgrade (< TLS 1.2 / 0x0303)
        if telemetry.client_requested_version < 0x0303 {
            return Some(DowngradeAlert {
                threat_type: "TLS_VERSION_DOWNGRADE".to_string(),
                severity: 0.95,
                description: format!(
                    "Legacy TLS version requested (0x{:04X}). Minimum allowed TLS 1.2 (0x0303).",
                    telemetry.client_requested_version
                ),
            });
        }

        // 2. Detect Weak/Insecure Cipher Suite Negotiation (e.g. RC4 0x0005, 3DES 0x000A, NULL 0x0000)
        let weak_ciphers = [0x0000, 0x0001, 0x0002, 0x0004, 0x0005, 0x000A];
        if weak_ciphers.contains(&telemetry.negotiated_cipher_suite) {
            return Some(DowngradeAlert {
                threat_type: "WEAK_CIPHER_SUITE".to_string(),
                severity: 0.90,
                description: format!(
                    "Negotiated insecure cipher suite (0x{:04X}). Potential MITM downgrade attack.",
                    telemetry.negotiated_cipher_suite
                ),
            });
        }

        // 3. Protocol State Confusion (e.g., transmitting payload data before state == Authenticated Data (2))
        if telemetry.protocol_state == 0 && telemetry.is_resumption {
            return Some(DowngradeAlert {
                threat_type: "STATE_CONFUSION_UNAUTHENTICATED_RESUMPTION".to_string(),
                severity: 0.85,
                description: "Session resumption attempted prior to state initialization.".to_string(),
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tls_downgrade_detection() {
        let bad_telemetry = TlsHandshakeTelemetry {
            client_requested_version: 0x0301, // TLS 1.0 (Downgrade!)
            negotiated_cipher_suite: 0x1301,
            is_resumption: false,
            protocol_state: 1,
        };

        let alert = CryptographicDowngradeGuardEngine::inspect_tls_handshake(&bad_telemetry);
        assert!(alert.is_some());
        let a = alert.unwrap();
        assert_eq!(a.threat_type, "TLS_VERSION_DOWNGRADE");
    }
}
