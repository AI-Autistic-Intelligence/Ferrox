# Multi-stage Distroless Container Build (<25MB)
FROM rust:1.80-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src/ ./src/
RUN cargo build --release

FROM gcr.io/distroless/static-debian12:nonroot
WORKDIR /app
COPY --from=builder /app/target/release/ferrox-settlement-hub /app/service
USER nonroot:nonroot
EXPOSE 8080
ENTRYPOINT ["/app/service"]
