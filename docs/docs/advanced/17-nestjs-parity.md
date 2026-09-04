---
sidebar_position: 17
---

# NestJS Parity: The DX Engine

Ferrox brings the legendary Developer Experience (DX) of NestJS into the high-performance ecosystem of Rust.

If you are coming from NestJS, you'll feel right at home with the following modules.

## 1. Validation Pipes (`ferrox-validation`)
In NestJS, you use `class-validator` and `@UsePipes(ValidationPipe)`.
In Ferrox, simply wrap your payload in `ValidatedJson<T>`. If the JSON is invalid, Ferrox automatically returns an HTTP 400 response before the controller even executes.

```rust
use ferrox_validation::{ValidatedJson, Validate};
use serde::Deserialize;

#[derive(Deserialize, Validate)]
pub struct CreateUserDto {
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}

// Controller
pub async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserDto>
) -> impl IntoResponse {
    // If it reaches here, the email is valid and password >= 8 chars!
    "User Created"
}
```

## 2. Role-Based Guards (`ferrox-guards`)
In NestJS, you use `@Roles('admin')` and `@UseGuards(RolesGuard)`.
In Ferrox, use the `RequireRole` extractor.

```rust
use ferrox_guards::RequireRole;

// The controller will automatically return HTTP 403 Forbidden 
// if the user's JWT token doesn't have the "admin" role.
pub async fn delete_database(
    RequireRole(role): RequireRole, // Extractor checks authorization
) -> impl IntoResponse {
    "Database Wiped!"
}
```

## 3. Storage & Multer (`ferrox-storage`)
In NestJS, you use Multer.
Ferrox uses `ferrox-storage` with interchangeable providers (Local Disk or AWS S3).

```rust
use ferrox_storage::{StorageProvider, LocalStorage};

let storage = LocalStorage::new("./uploads");
let url = storage.upload("avatar.png", bytes).await.unwrap();
// url = "/uploads/avatar.png"
```

## 4. CQRS & Domain Driven Design (`ferrox-cqrs`)
For complex enterprise apps, use the CommandBus to separate reads from writes.

```rust
use ferrox_cqrs::{Command, CommandHandler, CommandBus};

struct CreateUserCommand { email: String }
impl Command for CreateUserCommand {}

// Handlers process the commands
```

## 5. String & Date Utilities (`ferrox-utils`)
Standardizes DateTime to UTC for database storage, and allows easy conversion to GMT for frontend output. Includes powerful string manipulation (camelCase, snake_case, masking).
