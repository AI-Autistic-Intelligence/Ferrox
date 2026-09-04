---
sidebar_position: 1
---

# Docker

Rust-FERROX uses optimized, multi-stage Docker builds to achieve minimal image sizes and maximize security.

## Multi-Stage Build

The `Dockerfile` in the root of the workspace builds the entire monorepo, extracting only the compiled binary for the final image.

1. **Builder Stage**: Uses a heavy `rust:latest` image. It caches dependencies utilizing `cargo-chef` to speed up subsequent builds.
2. **Runtime Stage**: Uses a lightweight `debian:buster-slim` (or `distroless` for maximum security). It copies only the compiled binary from the Builder Stage.

## Building

```bash
docker build -t rust-ferrox:latest .
```
