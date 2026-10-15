# ==============================================================================
# 🦀 Ferrox Showcase - Multi-Stage Production Dockerfile
# ==============================================================================
FROM rust:1.80-alpine AS builder
RUN apk add --no-cache musl-dev openssl-dev pkgconfig
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/
COPY apps/ ./apps/

RUN cargo build --release --bin ferrox-showcase

FROM alpine:3.20 AS runner
RUN apk add --no-cache ca-certificates libssl3
WORKDIR /app

COPY --from=builder /app/target/release/ferrox-showcase /usr/local/bin/ferrox-showcase

ENV FERROX_SERVER_PORT=3000
EXPOSE 3000

HEALTHCHECK --interval=10s --timeout=3s --retries=3 \
  CMD wget --no-verbose --tries=1 --spider http://localhost:3000/health/live || exit 1

ENTRYPOINT ["/usr/local/bin/ferrox-showcase"]
