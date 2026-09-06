# Ferrox Guards (`ferrox-guards`)

`ferrox-guards` provides declarative role-based access control (RBAC) extractors (e.g. `RequireRole`, `RequirePermission`)
for protecting Axum endpoints in Ferrox applications.

## Key Features
- 🛡️ **Declarative Guard Extractors**: Protect handlers with compile-safe role constraints.
- 🔒 **PASETO/JWT Integration**: Inspects authenticated user claims directly from request extensions.
