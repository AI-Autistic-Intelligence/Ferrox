---
sidebar_position: 3
---

# 🍃 Document Databases (MongoDB)

For document-oriented storage, Ferrox provides `ferrox-database-mongo`, wrapping the official `mongodb` driver for Rust.

Document databases excel at hierarchical schema flexibility, real-time analytics, and high-throughput write workloads.

---

## 1. Connecting to MongoDB

Establish a MongoDB client connection:

```rust
use mongodb::{Client, options::ClientOptions, Collection};
use ferrox_errors::AppError;

pub async fn init_mongo(uri: &str, db_name: &str) -> Result<mongodb::Database, AppError> {
    let client_options = ClientOptions::parse(uri)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let client = Client::with_options(client_options)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(client.database(db_name))
}
```

---

## 2. Defining BSON Documents

Annotate your domain structs with `#[derive(Serialize, Deserialize)]`:

```rust
use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub views: u64,
}
```

---

## 3. Mongo Repository Implementation

```rust
use mongodb::{Database, Collection, bson::doc};
use ferrox_errors::AppError;
use crate::models::ArticleDocument;

pub struct MongoArticleRepository {
    collection: Collection<ArticleDocument>,
}

impl MongoArticleRepository {
    pub fn new(db: Database) -> Self {
        Self {
            collection: db.collection("articles"),
        }
    }

    pub async fn find_by_tag(&self, tag: &str) -> Result<Vec<ArticleDocument>, AppError> {
        let filter = doc! { "tags": tag };
        let mut cursor = self.collection.find(filter, None)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let mut results = Vec::new();
        while cursor.advance().await.map_err(|e| AppError::DatabaseError(e.to_string()))? {
            results.push(cursor.deserialize_current().map_err(|e| AppError::DatabaseError(e.to_string()))?);
        }

        Ok(results)
    }
}
```

---

## 4. ✅ Best Practices

- **Create indexes on frequently queried fields**: Define indexes during application startup to prevent collection scans.
