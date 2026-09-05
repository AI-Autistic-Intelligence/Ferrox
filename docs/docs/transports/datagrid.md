---
sidebar_position: 5
---

# 📊 Enterprise DataGrid Query Translators

Enterprise admin dashboards rely on rich frontend data grid libraries like **AG Grid**, **MUI X DataGrid**, and **TanStack Table**. These grids send complex URL query parameters containing filtering, multi-column sorting, page sizes, and search constraints.

Translating raw HTTP query strings into safe SQL/MongoDB queries manually requires hundreds of lines of tedious code.

`ferrox-datagrid` provides automated parsers and translators for AG Grid, MUI X, and TanStack Table directly to SeaORM and MongoDB queries.

---

## 1. AG Grid Query Translation

```rust
use axum::{extract::Query, Json};
use ferrox_datagrid::{AgGridQuery, AgGridTranslator};
use ferrox_errors::AppError;

pub async fn get_datagrid_users(
    Query(params): Query<AgGridQuery>,
) -> Result<Json<Vec<UserModel>>, AppError> {
    // Automatically translates AG Grid JSON filters/sorts into SeaORM Select queries!
    let seaorm_query = AgGridTranslator::to_seaorm(&params)?;
    
    // Execute translated query against database...
    Ok(Json(vec![]))
}
```

---

## 2. Supported Grid Libraries

| Grid Library | Parser Module | Database Translators |
|---|---|---|
| **AG Grid** | `ferrox_datagrid::parsers::aggrid` | SeaORM, MongoDB |
| **MUI X DataGrid** | `ferrox_datagrid::parsers::mui` | SeaORM, MongoDB |
| **TanStack Table** | `ferrox_datagrid::parsers::tanstack` | SeaORM, MongoDB |

---

## 3. Benefits

- **Prevent SQL Injection**: Parameter values in filtering operations are safely parameterized.
- **Zero Frontend Glue Code**: Frontend grids work out of the box with Ferrox backend controllers.
