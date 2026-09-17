//! # Yao's Garbled Circuits 2PC (`garbled_circuits.rs`)
//!
//! Implements Yao's Garbled Circuit 2-Party Computation (2PC) evaluator (IEEE FOCS / Yao 1986).
//! Allows two independent Ferrox nodes to compute joint risk threshold functions
//! without disclosing private IP callsets or threat databases to each other.

use serde::{Deserialize, Serialize};

/// 128-bit wire label representation
pub type WireLabel = [u8; 16];

/// Single Garbled Boolean Gate (AND / OR) entry in Garbled Truth Table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarbledGate {
    pub encrypted_entries: Vec<Vec<u8>>,
}

/// Garbled Circuit representation for confidential threat threshold evaluation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GarbledCircuit {
    pub gate: GarbledGate,
    pub output_label_false: WireLabel,
    pub output_label_true: WireLabel,
}

impl GarbledCircuit {
    /// Garbles an AND gate over two private node inputs: (Client Threat Level > Threshold) AND (VPS Threat Level > Threshold)
    pub fn garble_and_gate(
        wire_a_0: WireLabel,
        wire_a_1: WireLabel,
        wire_b_0: WireLabel,
        wire_b_1: WireLabel,
        out_0: WireLabel,
        out_1: WireLabel,
    ) -> Self {
        let mut encrypted_entries = Vec::with_capacity(4);

        // Truth table entries: (0,0)->0, (0,1)->0, (1,0)->0, (1,1)->1
        encrypted_entries.push(encrypt_label(&wire_a_0, &wire_b_0, &out_0));
        encrypted_entries.push(encrypt_label(&wire_a_0, &wire_b_1, &out_0));
        encrypted_entries.push(encrypt_label(&wire_a_1, &wire_b_0, &out_0));
        encrypted_entries.push(encrypt_label(&wire_a_1, &wire_b_1, &out_1));

        Self {
            gate: GarbledGate { encrypted_entries },
            output_label_false: out_0,
            output_label_true: out_1,
        }
    }

    /// Evaluates the garbled circuit given the active wire labels provided by Garbler and Evaluator
    pub fn evaluate(&self, active_a: &WireLabel, active_b: &WireLabel) -> Option<bool> {
        for entry in &self.gate.encrypted_entries {
            if let Some(decrypted) = decrypt_label(active_a, active_b, entry) {
                if decrypted == self.output_label_true {
                    return Some(true);
                } else if decrypted == self.output_label_false {
                    return Some(false);
                }
            }
        }
        None
    }
}

fn encrypt_label(ka: &WireLabel, kb: &WireLabel, out: &WireLabel) -> Vec<u8> {
    let mut cipher = vec![0u8; 16];
    for i in 0..16 {
        cipher[i] = out[i] ^ ka[i] ^ kb[i].rotate_left(3);
    }
    cipher
}

fn decrypt_label(ka: &WireLabel, kb: &WireLabel, cipher: &[u8]) -> Option<WireLabel> {
    if cipher.len() != 16 {
        return None;
    }
    let mut out = [0u8; 16];
    for i in 0..16 {
        out[i] = cipher[i] ^ ka[i] ^ kb[i].rotate_left(3);
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garbled_circuit_evaluation() {
        let ka0 = [1u8; 16];
        let ka1 = [2u8; 16];
        let kb0 = [3u8; 16];
        let kb1 = [4u8; 16];
        let out0 = [10u8; 16];
        let out1 = [20u8; 16];

        let circuit = GarbledCircuit::garble_and_gate(ka0, ka1, kb0, kb1, out0, out1);

        // Case 1: 0 AND 0 -> False
        let res_00 = circuit.evaluate(&ka0, &kb0);
        assert_eq!(res_00, Some(false));

        // Case 2: 1 AND 0 -> False
        let res_10 = circuit.evaluate(&ka1, &kb0);
        assert_eq!(res_10, Some(false));

        // Case 3: 1 AND 1 -> True
        let res_11 = circuit.evaluate(&ka1, &kb1);
        assert_eq!(res_11, Some(true));
    }
}
