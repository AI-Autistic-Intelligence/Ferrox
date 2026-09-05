---
sidebar_position: 1
---

# 🏗️ Controllers

Controllers are responsible for handling incoming requests and returning responses to the client.

A controller's purpose is to receive specific requests for the application. The **routing** mechanism controls which controller receives which requests. Frequently, each controller has more than one route, and different routes can perform different actions.

In order to create a basic controller, Ferrox utilizes **Axum's Handler** traits and Router definitions.

## High-Level Example

Here is a standard REST controller returning a JSON response.

```rust
use axum::{Json, routing::get, Router};
use serde::Serialize;

#[derive(Serialize)]
struct UserProfile {
    id: String,
    name: String,
}

// 1. Define the Handler (Controller method)
async fn get_profile() -> Json<UserProfile> {
    Json(UserProfile {
        id: "usr_123".into(),
        name: "Alice".into(),
    })
}

// 2. Mount it in your router
pub fn user_controller() -> Router {
    Router::new().route("/profile", get(get_profile))
}
```

## Low-Level Internal Details

Under the hood, any asynchronous function that implements the `axum::handler::Handler` trait can act as a controller in Ferrox. 

The framework heavily relies on the **Extractor Pattern**. When a request arrives, Ferrox executes extractors sequentially (from left to right in the function arguments).

```rust
use axum::{extract::{Path, State}, Json};
use ferrox_security::AuthPayload; // Custom Ferrox Extractor

async fn update_user(
    State(db_pool): State<DbPool>,         // 1. Extract global state (DI)
    Path(user_id): Path<String>,           // 2. Extract URL param
    user: AuthPayload,                     // 3. Extract JWT Claims (Zero-Trust)
    Json(payload): Json<UpdatePayload>     // 4. Extract and deserialize body
) -> Result<Json<User>, AppError> {
    
    // Business logic goes here
    
}
```

If **any** of the extractors fail (for instance, if `AuthPayload` determines the token is expired, or `Json` fails to parse a malformed string), Ferrox immediately aborts the pipeline and returns a structured `ferrox-errors::AppError` to the client. The actual body of your controller is never executed, saving precious CPU cycles.
