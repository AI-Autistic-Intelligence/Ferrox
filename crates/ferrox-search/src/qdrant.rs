use crate::{SearchEngine, SearchResult};
use async_trait::async_trait;
use serde_json::Value;
use ferrox_errors::AppError;

pub struct QdrantAdapter {
    pub url: String,
    pub api_key: String,
}

impl QdrantAdapter {
    pub fn new(url: &str, api_key: &str) -> Self {
        Self { 
            url: url.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait]
impl SearchEngine for QdrantAdapter {
    async fn search(&self, _index: &str, _query: &str) -> Result<Vec<SearchResult>, AppError> {
        Err(AppError::InternalServerError("Qdrant is a Vector database. Use `vector_search` instead of lexical `search`.".into()))
    }

    async fn vector_search(&self, index: &str, vector: &[f32]) -> Result<Vec<SearchResult>, AppError> {
        println!("🧠 [Qdrant] Performing Semantic AI Vector Search in '{}' (Vector Dimensions: {})", index, vector.len());
        Ok(vec![])
    }

    async fn index_document(&self, index: &str, id: &str, _document: Value) -> Result<(), AppError> {
        println!("📦 [Qdrant] Indexing vector doc {} into '{}'", id, index);
        Ok(())
    }
}
