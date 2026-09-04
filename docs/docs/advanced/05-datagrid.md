---
sidebar_position: 5
---

# Universal DataGrid (ferrox-datagrid)

The `ferrox-datagrid` crate allows Rust-FERROX to parse complex sorting, filtering, and pagination states from the most popular frontend DataGrid libraries and translate them natively into backend queries.

## Supported Frontend Grids
Our Universal Parser supports:
- **AG-Grid** (`aggrid` feature)
- **MUI DataGrid** (`mui` feature)
- **TanStack Table (React Table)** (`tanstack` feature)

## Supported Database Translators
Once parsed into our universal `DataGridRequest` struct, it can be translated into:
- **SeaORM** (`seaorm` feature): Generates dynamic SQL `Condition` and `Order By` clauses.
- **MongoDB** (`mongo` feature): Generates dynamic `bson::Document` filters.

## Example Flow

```rust
use serde_json::Value;
use ferrox_datagrid::parsers::mui;
use ferrox_datagrid::translators::seaorm::translate_filters;
use my_app::entities::users;

// 1. You receive a JSON payload from your frontend MUI DataGrid
let payload: Value = get_payload_from_request();

// 2. Parse it into the universal DataGridRequest
let grid_req = mui::parse(payload).unwrap();

// 3. Translate it into a SeaORM SQL Condition
let sql_condition = translate_filters::<users::Entity>(&grid_req).unwrap();

// 4. Execute the query
// let results = users::Entity::find().filter(sql_condition).all(db).await;
```

This ensures absolute DRY compliance. If your frontend team switches from AG-Grid to MUI DataGrid, you only need to change the `parser::` import, and the entire database layer remains untouched.
