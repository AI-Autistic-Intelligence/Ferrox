//! # Deterministic Lockstep Game State Verification (`deterministic_replay.rs`)
//!
//! Implements lockstep state vector verification for Burraco and multiplayer games.
//! Computes rolling SHA-256 state hashes over every turn. If a client engine's state hash diverges from the server's lockstep vector, an instant Desync Cheat Alert is triggered.

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use chrono::{DateTime, Utc};

/// Lockstep State Vector Payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockstepStateVector {
    pub game_id: String,
    pub turn_index: u32,
    pub player_id: String,
    pub melds_hash: String,
    pub discard_pile_hash: String,
    pub pozzetto_claimed: bool,
}

/// Desync Cheat Alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateDesyncAlert {
    pub alert_id: String,
    pub game_id: String,
    pub player_id: String,
    pub turn_index: u32,
    pub expected_server_hash: String,
    pub actual_client_hash: String,
    pub detected_at: DateTime<Utc>,
}

/// Deterministic Replay & Lockstep Verifier
pub struct LockstepStateVerifier;

impl LockstepStateVerifier {
    pub fn compute_state_hash(vector: &LockstepStateVector) -> String {
        let mut hasher = Sha256::new();
        hasher.update(vector.game_id.as_bytes());
        hasher.update(b":");
        hasher.update(vector.turn_index.to_le_bytes());
        hasher.update(b":");
        hasher.update(vector.player_id.as_bytes());
        hasher.update(b":");
        hasher.update(vector.melds_hash.as_bytes());
        hasher.update(b":");
        hasher.update(vector.discard_pile_hash.as_bytes());
        hasher.update(b":");
        hasher.update(if vector.pozzetto_claimed { b"1" } else { b"0" });
        format!("{:x}", hasher.finalize())
    }

    pub fn verify_state(server_vector: &LockstepStateVector, client_reported_hash: &str) -> Option<StateDesyncAlert> {
        let server_hash = Self::compute_state_hash(server_vector);
        if server_hash == client_reported_hash {
            None
        } else {
            Some(StateDesyncAlert {
                alert_id: format!("alert_desync_{:x}", rand::random::<u128>()),
                game_id: server_vector.game_id.clone(),
                player_id: server_vector.player_id.clone(),
                turn_index: server_vector.turn_index,
                expected_server_hash: server_hash,
                actual_client_hash: client_reported_hash.to_string(),
                detected_at: Utc::now(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lockstep_state_verification() {
        let server_vector = LockstepStateVector {
            game_id: "game_burraco_101".to_string(),
            turn_index: 4,
            player_id: "usr_cheater_99".to_string(),
            melds_hash: "meld_hash_valid".to_string(),
            discard_pile_hash: "discard_hash_valid".to_string(),
            pozzetto_claimed: false,
        };

        let server_hash = LockstepStateVerifier::compute_state_hash(&server_vector);

        // Matching hash -> Clean
        let res_clean = LockstepStateVerifier::verify_state(&server_vector, &server_hash);
        assert!(res_clean.is_none());

        // Altered client hash -> Desync Alert
        let res_tampered = LockstepStateVerifier::verify_state(&server_vector, "tampered_client_hash_00");
        assert!(res_tampered.is_some());
        let alert = res_tampered.unwrap();
        assert_eq!(alert.player_id, "usr_cheater_99");
    }
}
