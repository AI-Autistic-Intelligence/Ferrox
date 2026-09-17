# ⚡ Ferrox CLI (`cargo-ferrox`)

`ferrox-cli` provides the `cargo ferrox` developer tool for project scaffolding, TypeScript DTO code generation, OWASP WSTG security auditing, Moving Target Defense (MTD) inspection, Honeynet deception status monitoring, VPS gateway script generation, and autonomous node self-registration.

---

## 🛠️ Command Reference

### 1. `cargo ferrox init`
Interactive CLI wizard to scaffold a production-ready Ferrox enterprise backend with internationalized prompts (English, Italian, Chinese, Spanish).

```bash
cargo ferrox init --name my-enterprise-api
```

### 2. `cargo ferrox generate`
Generates strongly-typed TypeScript DTO clients (`FerroxClient.ts`) synchronized with Axum controllers.

```bash
cargo ferrox generate --lang ts --output ./frontend/src/api
```

### 3. `cargo ferrox audit`
Executes an OWASP WSTG synthetic security self-test against a target endpoint and exports markdown reports.

```bash
cargo ferrox audit --url http://127.0.0.1:8080 --export-md owasp_report.md
```

### 4. `cargo ferrox mtd`
Inspects Moving Target Defense ($T_{\text{rotate}} = 60\text{s}$) pseudo-random seed rotation, active window index, and dynamic ingress port offsets.

```bash
cargo ferrox mtd --secret mtd_secret_key_8899 --base-port 8080
```

### 5. `cargo ferrox honeynet`
Inspects the ecosystem-wide Distributed Honeynet Deception Mesh, active deception traps, and globally shadow-banned IP addresses.

```bash
cargo ferrox honeynet
```

### 6. `cargo ferrox vps-deploy`
Generates production VPS installer bash scripts (`ufw`, `docker`, `wireguard`) and Nginx reverse proxy configs for `security.ferrox-rust.dev`.

```bash
cargo ferrox vps-deploy --vps-ip 185.220.101.5 --subdomain security.ferrox-rust.dev
```

### 7. `cargo ferrox register-node`
Simulates autonomous client self-registration with ZK guard manifest attestation to the Founder Security Gateway (`security.ferrox-rust.dev`).

```bash
cargo ferrox register-node --client-name "Acme Corp" --domain "acme-store.com" --ip "198.51.100.44" --ciso-email "ciso@acme-store.com"
```
