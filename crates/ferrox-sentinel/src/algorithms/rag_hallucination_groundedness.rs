//! # RAG Groundedness & Hallucination Guardrail Engine (`rag_hallucination_groundedness.rs`)
//!
//! RAG factual groundedness ratio scoring engine 
//! (*Building LLM-Powered Applications* & *Mastering LLM Applications*).

use std::collections::HashSet;
use serde::{Deserialize, Serialize};

/// Groundedness Assessment Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundednessAssessment {
    pub groundedness_score: f64,
    pub is_grounded: bool,
    pub unsupported_claims: Vec<String>,
    pub matched_context_facts: Vec<String>,
}

/// RAG Groundedness & Hallucination Guardrail Engine
pub struct RagHallucinationGroundednessEngine;

impl RagHallucinationGroundednessEngine {
    /// Computes factual groundedness score as the ratio of generated claims supported by retrieved context
    pub fn evaluate_groundedness(
        generated_response: &str,
        retrieved_contexts: &[String],
        groundedness_threshold: f64,
    ) -> GroundednessAssessment {
        // Extract sentences/claims from response
        let claims: Vec<&str> = generated_response
            .split(&['.', '!', '?'][..])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if claims.is_empty() {
            return GroundednessAssessment {
                groundedness_score: 1.0,
                is_grounded: true,
                unsupported_claims: Vec::new(),
                matched_context_facts: Vec::new(),
            };
        }

        let combined_context = retrieved_contexts.join(" ").to_lowercase();
        let context_words: HashSet<&str> = combined_context
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| !w.is_empty())
            .collect();

        let mut matched_claims = 0;
        let mut unsupported_claims = Vec::new();
        let mut matched_facts = Vec::new();

        for claim in &claims {
            let lower_claim = claim.to_lowercase();
            let claim_words: Vec<&str> = lower_claim
                .split(|c: char| !c.is_alphanumeric())
                .filter(|w| w.len() > 3) // Focus on significant words
                .collect();

            if claim_words.is_empty() {
                matched_claims += 1;
                continue;
            }

            let matches = claim_words.iter().filter(|w| context_words.contains(*w)).count();
            let match_ratio = matches as f64 / claim_words.len() as f64;

            if match_ratio >= 0.4 {
                matched_claims += 1;
                matched_facts.push((*claim).to_string());
            } else {
                unsupported_claims.push((*claim).to_string());
            }
        }

        let groundedness_score = matched_claims as f64 / claims.len() as f64;
        let is_grounded = groundedness_score >= groundedness_threshold;

        GroundednessAssessment {
            groundedness_score,
            is_grounded,
            unsupported_claims,
            matched_context_facts: matched_facts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rag_groundedness_scoring() {
        let contexts = vec![
            "Ferrox is a zero-trust Rust security framework designed for Linux game servers and cloud VPS.".to_string()
        ];
        let response = "Ferrox is a zero-trust Rust security framework. It works on Linux servers.";

        let result = RagHallucinationGroundednessEngine::evaluate_groundedness(response, &contexts, 0.7);
        assert!(result.is_grounded);
        assert!(result.groundedness_score >= 0.7);
    }
}
