# Ferrox CRUD Generator (`ferrox-crud-gen`)

`ferrox-crud-gen` provides high-level procedural macros (`crud_router!`, `vertical_slice!`) that eliminate boilerplate code
when building standard RESTful CRUD (Create, Read, Update, Delete) routes over repositories in Ferrox applications.

## Design Rationale
Backend engineering teams frequently write repetitive HTTP route handlers, query parameter parsers, and service calls
for basic entity operations. `ferrox-crud-gen` generates these endpoints at compile-time with full type safety and Axum compatibility.

## Key Features
- 🚀 **`crud_router!` Macro**: Automatically synthesizes `GET /`, `GET /:id`, `POST /`, `PUT /:id`, and `DELETE /:id` Axum routers.
- 🧱 **Vertical Slice Architecture**: Group handlers, models, and service logic into cohesive domain slices.
- 🔍 **Paginated Listing**: Seamlessly plugs into `Repository` traits and `Pagination` parameters.

## Example Usage
```rust,ignore
use ferrox_crud_gen::crud_router;

// Automatically generates standard CRUD routes connected to the UserService
let user_routes = crud_router!(user_service);
```
