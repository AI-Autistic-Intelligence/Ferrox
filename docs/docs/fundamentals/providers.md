---
sidebar_position: 2
---

# 💉 Providers & State

In standard object-oriented frameworks like NestJS or Spring Boot, Dependency Injection (DI) is handled via Providers or Services marked with `@Injectable()`. 

In Ferrox, because Rust enforces strict compile-time safety and memory ownership, we handle Dependency Injection using the **State Extractor** pattern.

## High-Level Example

Here is how you inject a database connection pool or a global configuration object into your controllers.

```rust
use axum::{extract::State, Json, routing::get, Router};

// 1. Define your Application State
#[derive(Clone)]
pub struct AppState {
    pub db_pool: String, // E.g., SeaOrm DatabaseConnection
    pub config_url: String,
}

// 2. The Controller simply asks for the State in its arguments
async fn get_dashboard(State(state): State<AppState>) -> Json<String> {
    // You now have thread-safe access to your database!
    Json(format!("Connecting to {}", state.db_pool))
}

// 3. Inject the state when booting the server
pub fn build_router() -> Router {
    let global_state = AppState {
        db_pool: "postgres://localhost:5432/ferrox".into(),
        config_url: "https://api.ferrox.dev".into(),
    };

    Router::new()
        .route("/dashboard", get(get_dashboard))
        .with_state(global_state) // <--- Dependency Injection happens here
}
```

## Low-Level Internal Details

Why do we require `#[derive(Clone)]` on our `AppState`?

Unlike JavaScript/TypeScript which uses Garbage Collection, Rust manages memory linearly. When a request hits your Axum router, it spawns a new asynchronous Tokio task. 
To avoid complex Mutex locks, the `State` extractor relies on inexpensive cloning.

In an Enterprise environment, your `AppState` should **only** contain `Arc<T>` (Atomic Reference Counted) smart pointers, or database pools that wrap an `Arc` internally (like SQLx or SeaORM pools). This means that cloning the state for every request takes virtually zero CPU time (it just increments an atomic counter), while giving the controller a safe reference to the underlying memory.
