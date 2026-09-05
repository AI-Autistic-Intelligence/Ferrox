---
sidebar_position: 6
---

# 📄 Reports & Cloud Helper Utilities

Enterprise business applications often require generating PDF invoices, CSV data exports, and interacting with Cloud SDKs (AWS S3/DynamoDB, GCP Cloud Storage, Azure Blob Storage).

Ferrox provides `integrations/ferrox-reports` and `integrations/ferrox-cloud-helpers`.

---

## 1. CSV Data Export (`ferrox-reports`)

Generate downloadable CSV streams from Rust domain collections:

```rust
use ferrox_reports::csv::generate_csv;
use serde::Serialize;

#[derive(Serialize)]
struct UserExport {
    id: u64,
    name: String,
    email: String,
}

let users = vec![
    UserExport { id: 1, name: "Alice".into(), email: "alice@ferrox.dev".into() }
];

let csv_bytes = generate_csv(&users)?;
```

---

## 2. Cloud Helpers (`ferrox-cloud-helpers`)

Utilities for authenticating with AWS/GCP IAM roles, fetching secrets from AWS Secrets Manager, and resolving cloud instance metadata.
