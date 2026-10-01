use async_trait::async_trait;
use ferrox_database_core::Repository;
use ferrox_errors::AppError;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

/// A Firebase Realtime Database/Firestore backed repository implementation.
pub struct FirebaseRepository<T> {
    client: Client,
    base_url: String,
    collection: String,
    _marker: PhantomData<T>,
}

impl<T> FirebaseRepository<T> {
    pub fn new(client: Client, project_id: &str, collection: &str) -> Self {
        Self {
            client,
            base_url: format!("https://{}.firebaseio.com", project_id),
            collection: collection.to_string(),
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<T> Repository<T, String> for FirebaseRepository<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Clone,
{
    async fn find_by_id(&self, id: String) -> Result<Option<T>, AppError> {
        let url = format!("{}/{}/{}.json", self.base_url, self.collection, id);
        let res = self.client.get(&url).send().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        if res.status().is_success() {
            let entity: Option<T> = res.json().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
            Ok(entity)
        } else {
            Ok(None)
        }
    }

    async fn find_all(&self) -> Result<Vec<T>, AppError> {
        let url = format!("{}/{}.json", self.base_url, self.collection);
        let res = self.client.get(&url).send().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        if res.status().is_success() {
            let map: std::collections::HashMap<String, T> = res.json().await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
            Ok(map.into_values().collect())
        } else {
            Ok(vec![])
        }
    }

    async fn insert(&self, entity: T) -> Result<T, AppError> {
        let url = format!("{}/{}.json", self.base_url, self.collection);
        
        self.client.post(&url)
            .json(&entity)
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
        Ok(entity)
    }

    async fn update(&self, id: String, entity: T) -> Result<T, AppError> {
        let url = format!("{}/{}/{}.json", self.base_url, self.collection, id);
        
        self.client.put(&url)
            .json(&entity)
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
        Ok(entity)
    }

    async fn delete(&self, id: String) -> Result<(), AppError> {
        let url = format!("{}/{}/{}.json", self.base_url, self.collection, id);
        self.client.delete(&url)
            .send()
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }
}
