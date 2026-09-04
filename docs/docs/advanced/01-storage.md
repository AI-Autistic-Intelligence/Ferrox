---
sidebar_position: 1
---

# S3 Storage

The `yalc-storage` crate provides a unified interface for Amazon S3 and compatible services (like MinIO).

## Initialization

```rust
use yalc_storage::StorageService;

let storage = StorageService::new().await;
```

It automatically pulls credentials from the environment variables (e.g. `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY`, `AWS_REGION`).

## Usage

Uploading a file:
```rust
storage.upload_file("my-bucket", "user-avatars/123.png", file_bytes, "image/png").await?;
```
