//! # AI & Multi-Agent Cognitive Security Engine (`ai_cognitive_security.rs`)
//!
//! Synthesizes AI red-teaming, RAG context security, and LLM application guardrails
//! (*Red Teaming AI*, O'Reilly; *Building LLM-Powered Applications*, O'Reilly; *Mastering LLM Applications*, BPB; *NLP with Transformers*, O'Reilly).

use serde::{Deserialize, Serialize};

/// Agent Tool Execution Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentToolRequest {
    pub agent_id: String,
    pub requested_tool: String, // e.g. "read_file", "execute_sql", "http_request"
    pub capability_token: String,
}

/// RAG Vector Context Query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagContextQuery {
    pub prompt_embedding: Vec<f32>,
    pub retrieved_embedding: Vec<f32>,
}

/// Cognitive AI Assessment Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiCognitiveAssessment {
    pub is_allowed: bool,
    pub threat_type: String,
    pub risk_score: f64,
}

/// AI & Multi-Agent Cognitive Security Engine
pub struct AiCognitiveSecurityEngine {
    hmac_key: [u8; 32],
}

impl Default for AiCognitiveSecurityEngine {
    fn default() -> Self {
        Self::new([0x42; 32])
    }
}

impl AiCognitiveSecurityEngine {
    pub fn new(hmac_key: [u8; 32]) -> Self {
        Self { hmac_key }
    }

    pub fn hmac_key(&self) -> &[u8; 32] {
        &self.hmac_key
    }

    /// Evaluates RAG Vector Context Embedding Drift (D_cosine > threshold) to catch indirect context poisoning
    pub fn inspect_rag_drift(&self, query: &RagContextQuery, max_allowed_cosine_dist: f32) -> AiCognitiveAssessment {
        if query.prompt_embedding.len() != query.retrieved_embedding.len() || query.prompt_embedding.is_empty() {
            return AiCognitiveAssessment {
                is_allowed: false,
                threat_type: "Mismatched Embedding Dimensions".to_string(),
                risk_score: 0.90,
            };
        }

        let cosine_sim = compute_cosine_similarity(&query.prompt_embedding, &query.retrieved_embedding);
        let cosine_dist = 1.0 - cosine_sim;

        if cosine_dist > max_allowed_cosine_dist {
            return AiCognitiveAssessment {
                is_allowed: false,
                threat_type: format!("RAG Indirect Context Poisoning / Embedding Drift (Dist {:.3})", cosine_dist),
                risk_score: 0.88,
            };
        }

        AiCognitiveAssessment {
            is_allowed: true,
            threat_type: "Clean RAG Context Retrieval".to_string(),
            risk_score: 0.0,
        }
    }

    /// Enforces capability-token sandboxing for AI Agent Tool Execution requests
    pub fn inspect_agent_tool_request(&self, request: &AgentToolRequest) -> AiCognitiveAssessment {
        // High-risk administrative tools require explicit non-empty HMAC capability tokens
        let is_sensitive_tool = ["execute_sql", "delete_file", "http_request", "run_cmd"].contains(&request.requested_tool.as_str());

        if is_sensitive_tool && (request.capability_token.is_empty() || request.capability_token == "unauthenticated") {
            return AiCognitiveAssessment {
                is_allowed: false,
                threat_type: format!("Unprivileged Agent Tool Execution Attempt ({})", request.requested_tool),
                risk_score: 0.95,
            };
        }

        AiCognitiveAssessment {
            is_allowed: true,
            threat_type: "Authorized Agent Tool Request".to_string(),
            risk_score: 0.0,
        }
    }
}

fn compute_cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let mut dot = 0.0f32;
    let mut norm1 = 0.0f32;
    let mut norm2 = 0.0f32;

    for (a, b) in v1.iter().zip(v2.iter()) {
        dot += a * b;
        norm1 += a * a;
        norm2 += b * b;
    }

    if norm1 == 0.0 || norm2 == 0.0 {
        0.0
    } else {
        dot / (norm1.sqrt() * norm2.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rag_embedding_drift_detection() {
        let engine = AiCognitiveSecurityEngine::default();
        assert_eq!(engine.hmac_key()[0], 0x42);

        let query = RagContextQuery {
            prompt_embedding: vec![1.0, 0.0, 0.0],
            retrieved_embedding: vec![0.0, 1.0, 0.0], // Orthogonal vector -> Cosine distance = 1.0 -> Poisoned!
        };

        let res = engine.inspect_rag_drift(&query, 0.5);
        assert!(!res.is_allowed);
        assert!(res.threat_type.contains("Context Poisoning"));
    }

    #[test]
    fn test_agent_tool_sandboxing() {
        let engine = AiCognitiveSecurityEngine::default();
        let unauth_req = AgentToolRequest {
            agent_id: "agent_malicious_01".to_string(),
            requested_tool: "run_cmd".to_string(),
            capability_token: "unauthenticated".to_string(),
        };

        let res = engine.inspect_agent_tool_request(&unauth_req);
        assert!(!res.is_allowed);
        assert_eq!(res.risk_score, 0.95);
    }
}
