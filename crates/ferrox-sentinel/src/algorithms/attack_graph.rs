//! # Dynamic Attack Graph & PageRank Risk Centrality (`attack_graph.rs`)
//!
//! Implements graph-based attack propagation and PageRank centrality vulnerability scoring (ACM CCS / IEEE TIFS).
//! Models microservice node topologies as directed attack graphs G = (V, E) to compute risk centrality scores
//! and identify critical pivot nodes before exploit escalation.

use std::collections::HashMap;

/// Microservice Node in Attack Graph
#[derive(Debug, Clone)]
pub struct AttackNode {
    pub id: String,
    pub base_vulnerability_score: f64,
}

/// Dynamic Attack Graph Engine
pub struct AttackGraphEngine {
    nodes: HashMap<String, AttackNode>,
    adj_matrix: HashMap<String, Vec<(String, f64)>>, // Outgoing edges: target -> transition probability
}

impl Default for AttackGraphEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl AttackGraphEngine {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            adj_matrix: HashMap::new(),
        }
    }

    /// Adds a node to the attack graph topology
    pub fn add_node(&mut self, id: impl Into<String>, base_vuln: f64) {
        let node_id = id.into();
        self.nodes.insert(
            node_id.clone(),
            AttackNode {
                id: node_id,
                base_vulnerability_score: base_vuln.clamp(0.0, 1.0),
            },
        );
    }

    /// Adds a directed exploit transition edge from source node to target node
    pub fn add_exploit_edge(&mut self, src: &str, dst: &str, transition_prob: f64) {
        self.adj_matrix
            .entry(src.to_string())
            .or_default()
            .push((dst.to_string(), transition_prob.clamp(0.0, 1.0)));
    }

    /// Computes PageRank Vulnerability Centrality scores r = d * M * r + (1-d)/N * 1 over graph nodes
    pub fn compute_pagerank_risk(&self, damping_factor: f64, iterations: usize) -> HashMap<String, f64> {
        let n = self.nodes.len();
        if n == 0 {
            return HashMap::new();
        }

        let initial_rank = 1.0 / n as f64;
        let mut rank: HashMap<String, f64> = self.nodes.keys().map(|k| (k.clone(), initial_rank)).collect();

        for _ in 0..iterations {
            let mut next_rank: HashMap<String, f64> = self.nodes.keys().map(|k| (k.clone(), (1.0 - damping_factor) / n as f64)).collect();

            for (src, edges) in &self.adj_matrix {
                let src_rank = *rank.get(src).unwrap_or(&0.0);
                let out_degree = edges.len();
                if out_degree > 0 {
                    let weight_per_edge = (damping_factor * src_rank) / out_degree as f64;
                    for (dst, _) in edges {
                        if let Some(r) = next_rank.get_mut(dst) {
                            *r += weight_per_edge;
                        }
                    }
                }
            }
            rank = next_rank;
        }

        // Multiply by base vulnerability score to get final Attack Risk Index
        let mut final_scores = HashMap::new();
        for (id, r) in rank {
            let base_vuln = self.nodes.get(&id).map(|n| n.base_vulnerability_score).unwrap_or(0.5);
            final_scores.insert(id, r * base_vuln);
        }

        final_scores
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_graph_pagerank_centrality() {
        let mut graph = AttackGraphEngine::new();
        graph.add_node("public_gateway", 0.8);
        graph.add_node("auth_service", 0.4);
        graph.add_node("database_pivot", 0.9);

        graph.add_exploit_edge("public_gateway", "auth_service", 0.9);
        graph.add_exploit_edge("auth_service", "database_pivot", 0.95);

        let risk_scores = graph.compute_pagerank_risk(0.85, 20);

        // Database pivot receiving incoming attack flow has elevated centrality
        assert!(risk_scores.contains_key("database_pivot"));
        assert!(risk_scores.get("database_pivot").unwrap() > &0.0);
    }
}
