# 🦀 `cargo-ferrox`: Official Ferrox Developer CLI

`cargo-ferrox` is the official command-line interface and development lifecycle orchestrator for the **Ferrox Polyglot Ecosystem**. Inspired by `cargo-leptos` and `create-next-app`, it provides instant DDD scaffolding, zero-cost local dev-orchestration, and strict Definition of Done (DoD) quality gates enforcing 100% test coverage and zero debug debris.

---

## ⚡ Installation & Invocation

`cargo-ferrox` is distributed as a native Cargo subcommand. You can install it directly from the workspace:

```bash
cargo install --path crates/ferrox-cli
```

Once installed, it can be executed either as a cargo subcommand or directly:

```bash
cargo ferrox --help
# Or directly:
cargo-ferrox --help
```

---

## 🚀 Commands Reference

### 1. `cargo ferrox new <name> [--template <template>]`
Scaffolds a production-ready, fully compliant project following the **Ferrox Dual-Layer Agile Scrum Architecture**.

```bash
cargo ferrox new my-microservice --template clean-service
```

#### Available Archetypes & Templates

| Template | Description | Key Components Included |
| :--- | :--- | :--- |
| **`clean-service`** *(Default)* | Hexagonal Domain-Driven Microservice | Pure Domain Aggregate (`src/domain/`), CQRS Application Handlers, PASETO v4 Auth, Distroless Dockerfile (<25MB), `.coderabbit.yaml`, sub-5ms domain unit tests. |
| **`trading-desk`** | High-Frequency Matching Desk | Sub-microsecond in-memory `OrderBook`, bid/ask depth ladder, crossing engine, RFC-6455 WebSocket gateway. |
| **`polyglot-mesh`** | Distributed Event Mesh Node | CloudEvents 1.0 JSON serialization, Redpanda/Kafka broker bindings, Redis Redlock distributed mutex, `docker-compose.yml`. |
| **`wasm-spa`** | Pure Rust Reactive WebAssembly SPA | Fine-grained reactive Signals, 60 FPS vector SVG chart rendering, `Trunk.toml`, `index.html` zero-framework frontend. |

Every generated project includes:
- **`Ferrox.toml`**: Canonical project manifest defining architecture, purity constraints, and coverage thresholds.
- **`Cargo.toml`**: Optimized dependencies pinned to zero-alloc, fast-path libraries.
- **`Dockerfile`**: Multi-stage distroless container image strictly $< 25\text{MB}$.
- **`.coderabbit.yaml`**: Pre-configured CodeRabbit AI review rules enforcing Ferrox DDD conventions.
- **`.github/workflows/ci.yml`**: GitHub Actions pipeline checking `fmt`, `clippy -D warnings`, and 100% test coverage.
- **`docs/ARCHITECTURE.md`**: Architectural specification and domain invariants.

---

### 2. `cargo ferrox dev [--port <port>] [--dry-run]`
Local development orchestrator operating at **zero cloud costs**.

```bash
cargo ferrox dev
```

- **Auto-Detection**: Inspects `Ferrox.toml` and project directory.
- **WebAssembly Frontends**: If `wasm-spa` or `Trunk.toml` is detected, launches `trunk serve --port 4000` with hot-module replacement (HMR).
- **Backend Services**: Launches the Rust fast-path server with hot-reload watcher (`cargo watch -x run` or `cargo run`), binding local ports (`8086`, `19092`, `6379`).
- **Dry-Run Mode**: Pass `--dry-run` to print the orchestration commands without executing child processes.

---

### 3. `cargo ferrox test [--allow-debris]`
Automated Quality Gate enforcing the strict Ferrox Definition of Done (DoD):

```bash
cargo ferrox test
```

1. **Phase 1: Debug Debris Scanner**:
   Recursively scans the entire source tree for forbidden development artifacts:
   - Rust: `dbg!`, `todo!()`, `unimplemented!()`
   - TypeScript/JavaScript: `console.log(`, `console.debug(`, `debugger;`
   - Python: `print(`, `breakpoint()`
   - PHP: `var_dump(`, `dd(`, `print_r(`
   - Java: `System.out.println(`
   *If any debug statements are found, the quality gate fails immediately unless `--allow-debris` is explicitly provided.*
2. **Phase 2: Strict Test Execution**:
   Executes `cargo test --all` and ensures a 100% pass rate.

---

### 4. `cargo ferrox check [--strict]`
Architectural Domain Purity Sentinel.

```bash
cargo ferrox check --strict
```

Audits the pure domain layer (`src/domain/`) to enforce strict hexagonal boundaries:
- **Zero ORM Leakage**: Forbids `sea_orm`, `sqlx`, `diesel` in the pure domain.
- **Zero Framework Couplings**: Forbids `axum`, `actix_web`, `warp`, `rocket`, and raw socket networking in domain entities.
- Ensures domain logic depends purely on value objects, domain invariants, and CQRS ports.

---

### 5. `cargo ferrox codex [--port <port>] [--open]`
Previews project and ecosystem documentation locally via **Ferrox Codex**.

```bash
cargo ferrox codex --port 4500 --open
```

Automatically indexes all Markdown modules in `./docs` and displays an interactive browser preview.

---

## 🛠️ Typical Development Workflow

```bash
# 1. Scaffold new hexagonal microservice
cargo ferrox new payments-service --template clean-service
cd payments-service

# 2. Develop domain invariants and run local dev server
cargo ferrox dev

# 3. Check architectural DDD compliance
cargo ferrox check --strict

# 4. Verify Definition of Done (100% tests & zero debug debris)
cargo ferrox test

# 5. Preview documentation on Ferrox Codex
cargo ferrox codex
```
