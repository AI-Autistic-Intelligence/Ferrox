---
sidebar_position: 1
---

# Introduction to Rust-VELOX

**Rust-VELOX** is a full-featured, Cloud-Native, enterprise-grade Rust workspace designed to bring the ergonomics of NestJS into the blazing-fast and type-safe world of Rust.

## What is Rust-VELOX?

It is a collection of over 20 modular, highly-optimized crates that provide everything a modern web application needs out of the box:
- Telemetry & Logging (OpenTelemetry, JSON logs, Sentry)
- Security (PASETO, Argon2, Axum middlewares)
- Database connectivity (Postgres via SeaORM, MongoDB, Redis)
- Integrations (Stripe, Slack, S3 Storage, Lettre Mailer)
- Advanced Utilities (Background Jobs via Apalis, Rate Limiting, Feature Flags)

## Why Rust?

While NestJS (TypeScript) provides incredible developer experience, Rust offers:
- **Zero-Cost Abstractions**: Your application uses fractions of the memory compared to Node.js.
- **Type Safety & No Null Pointers**: If it compiles, it works. Say goodbye to `undefined is not a function`.
- **True Concurrency**: Tokio provides lightweight, massive concurrency for handling tens of thousands of requests per second.

## Getting Started

In the next sections, we will walk you through the architecture and how to run the built-in `rust-boilerplate`.
