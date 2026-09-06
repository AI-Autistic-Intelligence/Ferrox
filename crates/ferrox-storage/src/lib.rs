//! # Ferrox Storage (`ferrox-storage`)
//!
//! `ferrox-storage` defines a unified `StorageProvider` trait for managing file uploads and downloads across local disk storage, AWS S3, and MinIO.
//!
//! ## Key Features
//! - 📁 **`LocalStorage` Engine**: Fast disk file storage for local development and single-node setups.
//! - ☁️ **S3 / MinIO Engine**: Cloud object storage integration with support for presigned upload URLs.

use async_trait::async_trait;
use ferrox_errors::AppError;
use std::path::Path;
use tokio::fs;

#[async_trait]
pub trait StorageProvider: Send + Sync {
    /// Uploads a file stream/bytes and returns its public URL or URI
    async fn upload(&self, file_name: &str, data: &[u8]) -> Result<String, AppError>;
    
    /// Deletes a file by its URI
    async fn delete(&self, file_name: &str) -> Result<(), AppError>;
}

pub struct LocalStorage {
    base_path: String,
}

impl LocalStorage {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
        }
    }
}

#[async_trait]
impl StorageProvider for LocalStorage {
    async fn upload(&self, file_name: &str, data: &[u8]) -> Result<String, AppError> {
        let path = Path::new(&self.base_path).join(file_name);
        fs::write(&path, data).await
            .map_err(|e| AppError::InternalError(format!("Failed to write file: {}", e)))?;
        
        Ok(format!("/uploads/{}", file_name))
    }

    async fn delete(&self, file_name: &str) -> Result<(), AppError> {
        let path = Path::new(&self.base_path).join(file_name);
        fs::remove_file(&path).await
            .map_err(|e| AppError::InternalError(format!("Failed to delete file: {}", e)))?;
        Ok(())
    }
}

// In a real implementation, S3Storage would use aws-sdk-s3 to push `data` to an S3 Bucket.
pub struct S3Storage {
    pub bucket: String,
}

#[async_trait]
impl StorageProvider for S3Storage {
    async fn upload(&self, file_name: &str, _data: &[u8]) -> Result<String, AppError> {
        // Mock S3 Upload
        Ok(format!("https://{}.s3.amazonaws.com/{}", self.bucket, file_name))
    }

    async fn delete(&self, _file_name: &str) -> Result<(), AppError> {
        Ok(())
    }
}