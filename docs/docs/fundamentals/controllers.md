---
sidebar_position: 1
---

# 🕹️ Controllers

Controllers are responsible for handling incoming **HTTP requests** and returning **responses** to the client.

A controller's purpose is to receive specific requests for the application. The **routing** mechanism controls which controller receives which requests. Frequently, each controller has more than one route, and different routes can perform different actions.

In Ferrox, we achieve this by combining Rust's powerful type system with **Axum's Extraction** mechanism, allowing you to build declarative, type-safe route handlers.

## Routing

To create a basic controller, we use plain Rust asynchronous functions. Unlike Node.js frameworks where you might decorate a class, in Ferrox you define modular router groups.

```rust
use axum::{routing::get, Router};

// This is our Controller function
async fn find_all() -> &'static str {
    "This action returns all cats"
}

// We map the route to the controller function
pub fn cats_controller() -> Router {
    Router::new().route("/cats", get(find_all))
}
```

The `get()` routing method maps an HTTP GET request to our `find_all` handler. When a request matches `GET /cats`, Ferrox will execute the `find_all` asynchronous function.

## Request Object

Handlers often need access to the client request details. Ferrox uses **Extractors** to parse the request automatically. You simply declare what you need in the function signature, and Ferrox injects it.

Here is a list of the most common extractors:

| Ferrox Extractor | HTTP Request Part | NestJS Equivalent |
| --- | --- | --- |
| `Path<T>` | Route Parameters (`/users/:id`) | `@Param()` |
| `Query<T>` | Query String (`?name=ferrox`) | `@Query()` |
| `Json<T>` | JSON Request Body | `@Body()` |
| `HeaderMap` | All HTTP Headers | `@Headers()` |
| `TypedHeader<T>`| Strongly-typed HTTP Header | N/A |
| `ConnectInfo<T>`| IP Address and Connection Info | `@Ip()` / `@HostParam()` |

### Extracting Data Example

```rust
use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaginationQuery {
    limit: usize,
    offset: usize,
}

#[derive(Deserialize)]
pub struct CreateUserDto {
    name: String,
}

async fn get_user(
    Path(user_id): Path<String>,           // Extracts /users/:user_id
    Query(pagination): Query<PaginationQuery>, // Extracts ?limit=10&offset=0
    headers: HeaderMap,                    // Extracts all headers
    Json(payload): Json<CreateUserDto>,    // Parses the JSON body
) -> String {
    format!(
        "User: {}, Limit: {}, Name: {}, User-Agent: {:?}",
        user_id,
        pagination.limit,
        payload.name,
        headers.get("user-agent")
    )
}
```

> [!TIP]
> Notice how we don't have to manually parse JSON or cast strings to integers. If the client sends `?limit=abc`, Ferrox will automatically intercept the request and return a `400 Bad Request` before the controller even executes, guaranteeing type safety!

## Status Codes

By default, the response status code is always **200 OK**, except for POST requests which don't automatically default to 201 in raw Axum. 

To easily change the status code dynamically, you return an `impl IntoResponse`. Ferrox provides a highly declarative tuple syntax for responses: `(StatusCode, Payload)`.

```rust
use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

async fn create_cat() -> impl IntoResponse {
    // We return a tuple: HTTP 201 Created + JSON Body
    (
        StatusCode::CREATED, 
        Json(json!({ "message": "Cat created successfully" }))
    )
}
```

## Custom Headers

You can also specify custom response headers using the exact same tuple syntax.

```rust
use axum::{http::{HeaderMap, StatusCode}, response::IntoResponse};

async fn custom_header_route() -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    headers.insert("X-Custom-Header", "FerroxIsAwesome".parse().unwrap());
    
    // Ferrox resolves the tuple (StatusCode, Headers, Body) automatically
    (StatusCode::OK, headers, "Hello World")
}
```

## Request Payloads (DTOs)

In Enterprise applications, you must use **Data Transfer Objects (DTO)** to define how data is sent over the network. 

In Ferrox, DTOs are just Rust structs annotated with `serde::Deserialize` and our validation macros. Thanks to the Code Factory, these DTOs are also automatically converted into TypeScript interfaces for the Frontend.

```rust
use validator::Validate;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, Serialize, Validate, TS)]
#[ts(export)]
pub struct CreateCatDto {
    #[validate(length(min = 3))]
    pub name: String,
    
    #[validate(range(min = 1, max = 20))]
    pub age: u8,
    
    pub breed: String,
}
```

## Handling Errors

Unlike JavaScript where you `throw new Error()`, Rust uses the explicit `Result<T, E>` type. 

In Ferrox, you should **never** `unwrap()` inside a controller, as it will crash the Thread. Instead, you return a `Result<T, AppError>`, and Ferrox will automatically map the Error to a standard JSON HTTP response.

```rust
use ferrox_errors::AppError;

async fn find_one(Path(id): Path<String>) -> Result<Json<Cat>, AppError> {
    let cat = database.find(&id).await?; // Use the `?` operator!
    
    if cat.is_none() {
        // This will automatically return an HTTP 404 Not Found to the client
        return Err(AppError::NotFound("Cat not found".to_string()));
    }
    
    Ok(Json(cat.unwrap()))
}
```

## Full Resource Controller Example

Here is a complete, real-world example of a RESTful Controller mapping standard CRUD operations.

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use ferrox_errors::AppError;

pub fn cats_controller() -> Router<AppState> {
    Router::new()
        .route("/cats", get(find_all).post(create))
        .route("/cats/:id", get(find_one).put(update).delete(remove))
}

async fn create(
    State(db): State<DatabaseConnection>,
    Json(create_cat_dto): Json<CreateCatDto>,
) -> Result<impl IntoResponse, AppError> {
    let new_cat = db.cats().insert(create_cat_dto).await?;
    Ok((StatusCode::CREATED, Json(new_cat)))
}

async fn find_all(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<Cat>>, AppError> {
    let cats = db.cats().find_all().await?;
    Ok(Json(cats))
}

async fn find_one(
    State(db): State<DatabaseConnection>,
    Path(id): Path<String>,
) -> Result<Json<Cat>, AppError> {
    let cat = db.cats().find_by_id(&id).await?
        .ok_or(AppError::NotFound("Cat not found".to_string()))?;
    Ok(Json(cat))
}
```
