---
sidebar_position: 2
---

# Architecture Overview

The Rust-FERROX workspace follows a **monorepo** pattern, structured into multiple layers.

## The Crates

- **`ferrox-app`**: The core application lifecycle manager. It wraps the `axum` router, injecting Helmet-like security headers (CORS, HSTS, X-Frame-Options), global timeout middlewares, and catch-panic layers.
- **`ferrox-errors`**: Centralized error handling using `thiserror`. It standardizes API responses so the frontend always receives a consistent `{ "error": "Message" }` format.
- **`ferrox-security`**: Cryptographic primitives. Instead of JWT, we use PASETO (Platform-Agnostic Security Tokens) for modern, secure, and compact stateless auth. Includes Argon2 password hashing.

## Data & Observability
- **`ferrox-logger`**: Implements JSON structured logging and OpenTelemetry traces. 
- **`ferrox-metrics`**: Automatically tracks HTTP request latencies and exposes a Prometheus `/metrics` endpoint.
- **`ferrox-database-*`**: High-performance connection pools for Postgres (SeaORM), MongoDB, and Redis.

## Background Processing
- **`ferrox-jobs`**: A distributed background job processor using Apalis and Redis, essential for moving heavy tasks (like sending emails) off the main HTTP thread.
