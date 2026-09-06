# Ferrox Storage (`ferrox-storage`)

`ferrox-storage` defines a unified `StorageProvider` trait for managing file uploads and downloads across local disk storage, AWS S3, and MinIO.

## Key Features
- 📁 **`LocalStorage` Engine**: Fast disk file storage for local development and single-node setups.
- ☁️ **S3 / MinIO Engine**: Cloud object storage integration with support for presigned upload URLs.
