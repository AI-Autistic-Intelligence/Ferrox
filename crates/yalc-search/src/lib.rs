use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use yalc_errors::AppError;

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

pub fn setup() {
    println!("yalc-search initialized: Lexical and Semantic Vector Search Engine ready.");
}
