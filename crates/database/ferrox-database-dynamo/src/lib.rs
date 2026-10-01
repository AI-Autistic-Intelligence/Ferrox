use async_trait::async_trait;
use ferrox_database_core::repository::Repository;
use ferrox_errors::FerroxError;
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;
use std::collections::HashMap;

/// A DynamoDB-backed repository implementation.
pub struct DynamoDbRepository<T> {
    client: Client,
    table_name: String,
    partition_key: String,
    _marker: PhantomData<T>,
}

impl<T> DynamoDbRepository<T> {
    pub fn new(client: Client, table_name: String, partition_key: String) -> Self {
        Self {
            client,
            table_name,
            partition_key,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<T> Repository<T> for DynamoDbRepository<T>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, FerroxError> {
        let res = self.client.get_item()
            .table_name(&self.table_name)
            .key(&self.partition_key, AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| FerroxError::DatabaseError(e.to_string()))?;

        if let Some(item) = res.item {
            let parsed: T = serde_dynamo::aws_sdk_dynamodb_1::from_item(item)
                .map_err(|e| FerroxError::DatabaseError(format!("DynamoDB Deserialization Error: {}", e)))?;
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }

    async fn save(&self, entity: &T) -> Result<(), FerroxError> {
        let item = serde_dynamo::aws_sdk_dynamodb_1::to_item(entity)
            .map_err(|e| FerroxError::DatabaseError(format!("DynamoDB Serialization Error: {}", e)))?;

        let _req = self.client.put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| FerroxError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: &str) -> Result<(), FerroxError> {
        let _res = self.client.delete_item()
            .table_name(&self.table_name)
            .key(&self.partition_key, AttributeValue::S(id.to_string()))
            .send()
            .await
            .map_err(|e| FerroxError::DatabaseError(e.to_string()))?;
        Ok(())
    }
}
