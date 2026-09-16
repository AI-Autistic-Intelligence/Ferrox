//! # Ferrox Search (`ferrox-search`)
//!
//! `ferrox-search` provides full-text search integration for Ferrox applications, featuring client wrappers for Meilisearch and Elasticsearch.
//!
//! ## Key Features
//! - 🔍 **Meilisearch Integration**: Async document indexing, typo-tolerant full-text search, and faceted filtering.
//! - 🔎 **Elasticsearch Integration**: Support for complex search queries and index management.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use ferrox_errors::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub score: f32,
    pub document: Value,
}

#[async_trait]
pub trait SearchEngine: Send + Sync {
    /// Standard lexical full-text search (BM25, Typo-tolerance)
    async fn search(&self, index: &str, query: &str) -> Result<Vec<SearchResult>, AppError>;
    
    /// Semantic Vector search (for AI Embeddings / RAG)
    async fn vector_search(&self, index: &str, vector: &[f32]) -> Result<Vec<SearchResult>, AppError>;
    
    /// Index or upsert a document
    async fn index_document(&self, index: &str, id: &str, document: Value) -> Result<(), AppError>;
}

#[cfg(feature = "elastic")]
pub mod elastic;

#[cfg(feature = "meili")]
pub mod meili;

#[cfg(feature = "qdrant")]
pub mod qdrant;

/// Computes Cosine Similarity between two embedding vectors: \cos(\theta) = \frac{A \cdot B}{\|A\| \|B\|}
pub fn cosine_similarity(vec_a: &[f32], vec_b: &[f32]) -> f32 {
    if vec_a.len() != vec_b.len() || vec_a.is_empty() {
        return 0.0;
    }

    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;

    for i in 0..vec_a.len() {
        dot += vec_a[i] * vec_b[i];
        norm_a += vec_a[i] * vec_a[i];
        norm_b += vec_b[i] * vec_b[i];
    }

    let denom = norm_a.sqrt() * norm_b.sqrt();
    if denom < 1e-6 {
        0.0
    } else {
        dot / denom
    }
}

pub fn setup() {
    println!("ferrox-search initialized: Lexical and Semantic Vector Search Engine ready.");
}