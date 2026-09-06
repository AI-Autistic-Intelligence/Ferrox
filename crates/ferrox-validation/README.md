# Ferrox Validation (`ferrox-validation`)

`ferrox-validation` provides the `ValidatedJson<T>` extractor for Axum, automating request payload validation
using the `validator` crate before route handlers execute.

## Defensive Request Pipeline
In Ferrox's onion architecture, requests containing invalid request bodies (e.g. malformed email addresses, out-of-range numbers)
are short-circuited early with HTTP 400 Bad Request error responses before business logic is invoked.

## Key Features
- 🛡️ **`ValidatedJson<T>` Extractor**: Automatic JSON payload deserialization and constraint validation.
- ❌ **Detailed Error Reporting**: Returns structured JSON arrays detailing every field constraint failure.
