# Ferrox Schedule (`ferrox-schedule`)

`ferrox-schedule` provides an async cron job scheduler for running recurring background tasks inside Ferrox services.

## Key Features
- ⏰ **Cron Syntax**: Flexible task scheduling using standard 5-field or 6-field cron expressions.
- ⚡ **Tokio Async Worker**: Executes scheduled tasks concurrently without blocking the main event loop.
