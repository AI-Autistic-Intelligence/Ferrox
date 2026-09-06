# Ferrox Errors (`ferrox-errors`)

`ferrox-errors` defines the unified error handling strategy for Ferrox applications. It provides the standard
`AppError` enum and implements Axum's `IntoResponse` trait to automatically convert application errors into
consistent JSON responses with standard HTTP status codes.

## Rationale & Design
In enterprise Rust microservices, unhandled errors or raw `Result<T, E>` returns can leak internal stack traces
or produce inconsistent JSON structures for frontend consumers. `ferrox-errors` solves this by forcing all
service layers to emit `AppError`, which serializes into predictable `{ "status": u16, "message": String }` responses.

## Key Features
- 🚫 **AppError Enum**: Structured variants for `NotFound`, `ValidationError`, `Unauthorized`, `InternalError`, and `DatabaseError`.
- ⚡ **Axum IntoResponse**: Seamless integration with Axum route handlers without manual status code mappings.
- 🔒 **Security Sanitization**: Internal server errors and database errors are logged privately while safe generic error messages are exposed to clients.

## Example Usage
```rust
use ferrox_errors::{AppError, ErrorResponse};
use axum::response::IntoResponse;

fn find_user(id: u64) -> Result<String, AppError> {
    if id == 0 {
        Err(AppError::NotFound("User not found".into()))
    } else {
        Ok("Alice".into())
    }
}
```
