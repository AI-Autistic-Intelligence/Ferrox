---
sidebar_position: 2
---

# MongoDB

Per i carichi di lavoro NoSQL (come log di audit intensivi, configurazioni flessibili o analytics), Rust-YALC include `yalc-database-mongo`.

## Configurazione

Il crate espone un manager che si connette e verifica l'istanza con un ping automatico per assicurare la raggiungibilità.

```rust
use yalc_database_mongo::MongoManager;

let mongo = MongoManager::new().await;
let collection = mongo.get_collection::<Document>("my_collection");
```

## Pattern di Utilizzo
Ti consigliamo di utilizzare SeaORM per le relazioni transazionali primarie e MongoDB per collezioni event-driven.
