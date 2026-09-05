---
sidebar_position: 4
---

# 📁 File Upload & Storage Abstractions

Handling user file uploads (profile avatars, document attachments, media assets) requires decoupled storage backends that can switch seamlessly between local filesystem storage in development and S3/MinIO cloud object storage in production.

Ferrox provides `ferrox-storage` and `integrations/ferrox-storage`.

---

## 1. Storage Abstraction Interface

```rust
use async_trait::async_trait;
use ferrox_errors::AppError;

#[async_trait]
pub trait StorageService: Send + Sync {
    /// Uploads a file stream/bytes to storage
    async fn upload(&self, path: &str, bytes: &[u8]) -> Result<String, AppError>;
    
    /// Downloads file bytes from storage
    async fn download(&self, path: &str) -> Result<Vec<u8>, AppError>;
    
    /// Deletes a file from storage
    async fn delete(&self, path: &str) -> Result<(), AppError>;
}
```

---

## 2. Local Disk Storage Implementation

```rust
pub struct LocalStorageService {
    base_dir: String,
}

impl LocalStorageService {
    pub fn new(base_dir: &str) -> Self {
        std::fs::create_dir_all(base_dir).ok();
        Self { base_dir: base_dir.to_string() }
    }
}
```

---

## 3. Streaming Multipart Upload Handler

To prevent high RAM consumption during large file uploads, stream bytes directly using Axum's `Multipart` extractor:

```rust
use axum::extract::Multipart;
use ferrox_errors::AppError;

pub async fn upload_avatar(mut multipart: Multipart) -> Result<String, AppError> {
    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::ValidationError(e.to_string()))? {
        let file_name = field.file_name().unwrap_or("upload.bin").to_string();
        let bytes = field.bytes().await.map_err(|e| AppError::InternalServerError(Box::new(e)))?;
        
        println!("Saved {} bytes for file {}", bytes.len(), file_name);
        return Ok(format!("/uploads/{}", file_name));
    }
    
    Err(AppError::ValidationError("No file uploaded".into()))
}
```
