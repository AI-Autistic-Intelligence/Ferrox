---
sidebar_position: 11
---

# CLI & Code Generation (DX)

`Rust-YALC` provides an ultimate Developer Experience (DX) through its interactive CLI and built-in Code Generation capabilities.

## 1. The Interactive CLI (`yalc-cli`)
Instead of copying boilerplate around, you can use the official `yalc` command line tool to scaffold new projects or manage existing ones.

```bash
cargo run -p yalc-cli -- init
```
*(Or if installed globally: `yalc init`)*

The CLI is fully interactive. It will ask you:
1. The project name.
2. Which Database engine to use (PostgreSQL via SeaORM or MongoDB).
3. Whether to enable Redis for caching.
4. Whether to enable Qdrant for AI Vector Search.

Based on your answers, it instantly generates a Zero-Trust `docker-compose.yml` and a pre-configured `main.rs` file. You are ready to code in seconds.

## 2. Frontend TS/React Generation
`Rust-YALC` eliminates the need to manually write Fetch API calls or TypeScript interfaces on your frontend. 

By taking advantage of our GraphQL-first architecture, the framework automatically exports a `schema.graphql` file when starting in Dev mode.

### Exporting the Schema
In your `main.rs`, simply add:
```rust
use yalc_graphql::{build_schema, export_sdl};

let schema = build_schema();
// Export the schema to a file for the frontend to consume
export_sdl(&schema, "../frontend/schema.graphql").unwrap();
```

### Generating the Client (React example)
Inside your frontend React app, install `@graphql-codegen/cli`:
```bash
npm install -D @graphql-codegen/cli @graphql-codegen/typescript-react-apollo
```
Configure your `codegen.yml` to point to `schema.graphql`. When you run the codegen, it will automatically generate strictly-typed React Hooks (e.g. `useGetUserQuery()`) for every endpoint exposed by your Rust server!

## 3. WebSockets (Real-Time)
`Rust-YALC` provides native WebSocket support via `yalc-transports`.

To spin up a high-performance WebSocket server alongside your REST or GraphQL APIs, simply add it to your `YalcApp` bootstrap:

```rust
use yalc_app::YalcApp;
use yalc_transports::WsTransport;

let app = YalcApp::new()
    .add_transport(WsTransport::new(3001, "/ws")); // Starts on ws://0.0.0.0:3001/ws

app.start().await.unwrap();
```
