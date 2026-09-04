---
sidebar_position: 2
---

# Architecture Overview

The Rust-YALC workspace follows a **monorepo** pattern, structured into multiple layers.

## The Crates

- **`yalc-app`**: The core application lifecycle manager. It wraps the `axum` router, injecting Helmet-like security headers (CORS, HSTS, X-Frame-Options), global timeout middlewares, and catch-panic layers.
- **`yalc-errors`**: Centralized error handling using `thiserror`. It standardizes API responses so the frontend always receives a consistent `{ "error": "Message" }` format.
- **`yalc-security`**: Cryptographic primitives. Instead of JWT, we use PASETO (Platform-Agnostic Security Tokens) for modern, secure, and compact stateless auth. Includes Argon2 password hashing.

## Data & Observability
- **`yalc-logger`**: Implements JSON structured logging and OpenTelemetry traces. 
- **`yalc-metrics`**: Automatically tracks HTTP request latencies and exposes a Prometheus `/metrics` endpoint.
- **`yalc-database-*`**: High-performance connection pools for Postgres (SeaORM), MongoDB, and Redis.

## Background Processing
- **`yalc-jobs`**: A distributed background job processor using Apalis and Redis, essential for moving heavy tasks (like sending emails) off the main HTTP thread.
