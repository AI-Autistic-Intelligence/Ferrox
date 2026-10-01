use async_trait::async_trait;
use ferrox_database_core::Repository;
use ferrox_errors::AppError;
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

/// DynamoDB Primary Key definition, supporting both simple (PK only) and composite (PK + SK) keys.
#[derive(Clone, Debug)]
pub struct DynamoId {
    pub pk: String,
    pub sk: Option<String>,
}

impl From<&str> for DynamoId {
    fn from(s: &str) -> Self {
        DynamoId { pk: s.to_string(), sk: None }
    }
}

/// A DynamoDB-backed repository implementation.
pub struct DynamoDbRepository<T> {
    client: Client,
    table_name: String,
    partition_key_name: String,
    sort_key_name: Option<String>,
    _marker: PhantomData<T>,
}

impl<T> DynamoDbRepository<T> {
    pub fn new(client: Client, table_name: String, partition_key_name: String, sort_key_name: Option<String>) -> Self {
        Self {
            client,
            table_name,
            partition_key_name,
            sort_key_name,
            _marker: PhantomData,
        }
    }
}

#[async_trait]
impl<T> Repository<T, DynamoId> for DynamoDbRepository<T>
where
    T: Serialize + DeserializeOwned + Send + Sync + Clone,
{
    async fn find_by_id(&self, id: DynamoId) -> Result<Option<T>, AppError> {
        let mut req = self.client.get_item()
            .table_name(&self.table_name)
            .key(&self.partition_key_name, AttributeValue::S(id.pk));

        if let (Some(sk_name), Some(sk_val)) = (&self.sort_key_name, &id.sk) {
            req = req.key(sk_name, AttributeValue::S(sk_val.clone()));
        }

        let res = req.send().await
            .map_err(|e| AppError::Internal(format!("DynamoDB GetItem Error: {}", e)))?;

        if let Some(item) = res.item {
            let parsed: T = serde_dynamo::aws_sdk_dynamodb_1::from_item(item)
                .map_err(|e| AppError::Internal(format!("DynamoDB Deserialization Error: {}", e)))?;
            Ok(Some(parsed))
        } else {
            Ok(None)
        }
    }

    async fn find_all(&self) -> Result<Vec<T>, AppError> {
        // Warning: Scans are expensive in DynamoDB. Used here to satisfy the generic Repository trait.
        let res = self.client.scan()
            .table_name(&self.table_name)
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("DynamoDB Scan Error: {}", e)))?;

        let mut entities = Vec::new();
        if let Some(items) = res.items {
            for item in items {
                let parsed: T = serde_dynamo::aws_sdk_dynamodb_1::from_item(item)
                    .map_err(|e| AppError::Internal(format!("DynamoDB Deserialization Error: {}", e)))?;
                entities.push(parsed);
            }
        }
        Ok(entities)
    }

    async fn insert(&self, entity: T) -> Result<T, AppError> {
        let item = serde_dynamo::aws_sdk_dynamodb_1::to_item(entity.clone())
            .map_err(|e| AppError::Internal(format!("DynamoDB Serialization Error: {}", e)))?;

        self.client.put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await
            .map_err(|e| AppError::Internal(format!("DynamoDB PutItem Error: {}", e)))?;

        Ok(entity)
    }

    async fn update(&self, _id: DynamoId, entity: T) -> Result<T, AppError> {
        // In DynamoDB, PutItem overwrites completely. For a true update, UpdateItem is used, 
        // but PutItem satisfies the standard generic repository update definition via replacement.
        self.insert(entity).await
    }

    async fn delete(&self, id: DynamoId) -> Result<(), AppError> {
        let mut req = self.client.delete_item()
            .table_name(&self.table_name)
            .key(&self.partition_key_name, AttributeValue::S(id.pk));

        if let (Some(sk_name), Some(sk_val)) = (&self.sort_key_name, &id.sk) {
            req = req.key(sk_name, AttributeValue::S(sk_val.clone()));
        }

        req.send().await
            .map_err(|e| AppError::Internal(format!("DynamoDB DeleteItem Error: {}", e)))?;
            
        Ok(())
    }
}
