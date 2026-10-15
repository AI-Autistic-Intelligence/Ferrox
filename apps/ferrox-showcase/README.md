# 🦀 Ferrox Showcase Application

[![Rust Version](https://img.shields.io/badge/Rust-1.80%2B-orange?style=flat-square&logo=rust)](https://rust-lang.org)
[![HTTP Transport](https://img.shields.io/badge/Transport-Axum-blue?style=flat-square)](https://github.com/tokio-rs/axum)
[![Security](https://img.shields.io/badge/Security-PASETO%20v4-blueviolet?style=flat-square)](#security)
[![OpenAPI](https://img.shields.io/badge/OpenAPI-3.0.3%20Swagger%20UI-success?style=flat-square)](http://localhost:3000/docs)

**`ferrox-showcase`** is the lightweight reference application demonstrating the core primitives of the Ferrox Framework in pure Rust:
- Lifecycle management & multi-transport orchestrator (`FerroxApp`)
- Zero-Trust CORS isolation
- PASETO v4 local token generation
- Kubernetes-compliant `/health/live` and `/health/ready` probes
- Interactive Swagger UI documentation on `/docs`

---

## 🏛️ Showcase Architecture

```mermaid
graph TD
    Client[Web / HTTP Client] -->|HTTP / JSON| Transport[Axum HttpTransport :3000]
    
    subgraph "FerroxApp Lifecycle Manager"
        Transport --> Logger[Ferrox Logger & Sentry Guard]
        Transport --> Cors[Strict Zero-Trust CORS]
        Transport --> Health["/health, /health/live, /health/ready"]
        Transport --> Docs["/docs (Swagger UI) & /docs/openapi.json"]
        Transport --> Auth["/api/v1/auth/token (PASETO v4)"]
    end
```

---

## 📋 API Route Table

| Method | Endpoint | Description |
|---|---|---|
| `GET` | `/docs` | Interactive Swagger UI API Explorer |
| `GET` | `/docs/openapi.json` | OpenAPI 3.0.3 Schema Definition |
| `GET` | `/health` | Basic Healthcheck |
| `GET` | `/health/live` | Kubernetes Liveness Probe |
| `GET` | `/health/ready` | Kubernetes Readiness Probe |
| `GET` | `/api/v1/auth/token` | Demonstration PASETO v4 Token Generation |

---

## 🚀 Quickstart & Docker

### 1. Launch with Docker Compose
```bash
cd apps/ferrox-showcase
docker-compose up --build -d
```
Access Swagger UI at `http://localhost:3000/docs`.

### 2. Run Locally in Development
```bash
cargo run --bin ferrox-showcase
```

---

## 📡 API Testing & `curl` Examples

```bash
# Check Liveness Probe
curl -i http://localhost:3000/health/live

# Check Readiness Probe
curl -i http://localhost:3000/health/ready

# Generate PASETO v4 Demonstration Token
curl -i http://localhost:3000/api/v1/auth/token
```
