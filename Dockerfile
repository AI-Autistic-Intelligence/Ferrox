# --- Stage 1: Recipe Planner ---
FROM lukemathwalker/cargo-chef:latest-rust-1.75 AS chef
WORKDIR /app

# --- Stage 2: Cacher ---
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin yalc-app

# --- Stage 3: Runtime ---
# We use a distroless-like minimal debian base
FROM debian:buster-slim AS runtime
WORKDIR /app
# Install OpenSSL if needed for Reqwest/Postgres
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/yalc-app yalc-app

# Run as non-root user for Zero Trust Security
RUN useradd -m -s /bin/bash yalc
USER yalc

EXPOSE 3000
CMD ["./yalc-app"]
