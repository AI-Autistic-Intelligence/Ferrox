---
sidebar_position: 2
---

# MongoDB

For NoSQL workloads (such as intensive audit logging, flexible configurations, or analytics), Rust-FERROX includes `ferrox-database-mongo`.

## Configuration

The crate exposes a manager that connects and verifies the instance with an automatic ping to ensure reachability (Fail-Fast).

```rust
use ferrox_database_mongo::MongoManager;

let mongo = MongoManager::new().await;
let collection = mongo.get_collection::<Document>("my_collection");
```

## Usage Patterns
We recommend using SeaORM for primary transactional relationships and MongoDB for event-driven or unstructured collections.
