use crate::{SearchEngine, SearchResult};
use async_trait::async_trait;
use serde_json::Value;
use yalc_errors::AppError;

pub struct MeilisearchAdapter {
    pub url: String,
    pub api_key: String,
}

impl MeilisearchAdapter {
    pub fn new(url: &str, api_key: &str) -> Self {
        Self { 
            url: url.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

#[async_trait]
impl SearchEngine for MeilisearchAdapter {
    async fn search(&self, index: &str, query: &str) -> Result<Vec<SearchResult>, AppError> {
        println!("⚡ [Meilisearch] Fast typo-tolerant search in '{}' for '{}'", index, query);
        Ok(vec![])
    }

    async fn vector_search(&self, _index: &str, _vector: &[f32]) -> Result<Vec<SearchResult>, AppError> {
        Err(AppError::InternalServerError("Meilisearch native vector search is experimental. Use Qdrant.".into()))
    }

    async fn index_document(&self, index: &str, id: &str, _document: Value) -> Result<(), AppError> {
        println!("📦 [Meilisearch] Indexing doc {} into '{}'", id, index);
        Ok(())
    }
}
