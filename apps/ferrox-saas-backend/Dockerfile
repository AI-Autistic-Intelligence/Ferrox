# ==============================================================================
# 🦀 Ferrox SaaS Backend - Optimized Multi-Stage Production Dockerfile
# ==============================================================================
FROM rust:1.80-alpine AS chef
RUN apk add --no-cache musl-dev openssl-dev pkgconfig
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY apps/ ./apps/
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY apps/ ./apps/
RUN cargo build --release --bin ferrox-saas-backend

FROM alpine:3.20 AS runner
RUN apk add --no-cache ca-certificates libssl3
WORKDIR /app

COPY --from=builder /app/target/release/ferrox-saas-backend /usr/local/bin/ferrox-saas-backend

ENV FERROX_PORT=8080
ENV FERROX_ENV=production
EXPOSE 8080

HEALTHCHECK --interval=10s --timeout=3s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health/live || exit 1

ENTRYPOINT ["/usr/local/bin/ferrox-saas-backend"]
