use crate::{SearchEngine, SearchResult};
use async_trait::async_trait;
use serde_json::Value;
use ferrox_errors::AppError;

pub struct ElasticsearchAdapter {
    pub url: String,
}

impl ElasticsearchAdapter {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string() }
    }
}

#[async_trait]
impl SearchEngine for ElasticsearchAdapter {
    async fn search(&self, index: &str, query: &str) -> Result<Vec<SearchResult>, AppError> {
        println!("🔍 [Elasticsearch] Searching in '{}' for '{}'", index, query);
        // reqwest::Client::new().post(&format!("{}/{}/_search", self.url, index))...
        Ok(vec![])
    }

    async fn vector_search(&self, _index: &str, _vector: &[f32]) -> Result<Vec<SearchResult>, AppError> {
        Err(AppError::InternalServerError("Elasticsearch adapter in this version does not support native dense vectors. Use Qdrant.".into()))
    }

    async fn index_document(&self, index: &str, id: &str, _document: Value) -> Result<(), AppError> {
        println!("📦 [Elasticsearch] Indexing doc {} into '{}'", id, index);
        Ok(())
    }
}
