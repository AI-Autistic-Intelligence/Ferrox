use async_trait::async_trait;
use ferrox_database_core::repository::Repository;
use ferrox_errors::FerroxError;
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
impl<T> Repository<T> for FirebaseRepository<T>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, FerroxError> {
        let url = format!("{}/{}/{}.json", self.base_url, self.collection, id);
        let res = self.client.get(&url).send().await.map_err(|e| FerroxError::DatabaseError(e.to_string()))?;
        
        if res.status().is_success() {
            let entity: Option<T> = res.json().await.map_err(|e| FerroxError::DatabaseError(e.to_string()))?;
            Ok(entity)
        } else {
            Ok(None)
        }
    }

    async fn save(&self, entity: &T) -> Result<(), FerroxError> {
        // For demonstration, let's assume T has a way to get its ID, or we auto-generate.
        // In a real implementation we'd require an Entity trait with `get_id()`.
        let url = format!("{}/{}.json", self.base_url, self.collection);
        
        let _res = self.client.post(&url)
            .json(entity)
            .send()
            .await
            .map_err(|e| FerroxError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), FerroxError> {
        let url = format!("{}/{}/{}.json", self.base_url, self.collection, id);
        let _res = self.client.delete(&url)
            .send()
            .await
            .map_err(|e| FerroxError::DatabaseError(e.to_string()))?;
            
        Ok(())
    }
}
