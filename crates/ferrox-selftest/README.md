# Ferrox Self-Test (`ferrox-selftest`)

[![Crates.io](https://img.shields.io/crates/v/ferrox-selftest.svg)](https://crates.io/crates/ferrox-selftest)
[![Documentation](https://docs.rs/ferrox-selftest/badge.svg)](https://docs.rs/ferrox-selftest)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

`ferrox-selftest` provides an automated OWASP Web Security Testing Guide (WSTG) self-test pipeline, vulnerability dictionary, and executive compliance report generator for Ferrox and Rust web applications.

---

## 🔑 Key Features

- 📖 **`VulnerabilityDictionary`**: Plain-language dictionary mapping OWASP WSTG IDs (e.g. `WSTG-INP-01`, `WSTG-ATH-01`) to real-world business risks and actionable remediations.
- ⚙️ **`ThreeStepAuditPipeline`**: Automated 3-step audit runner orchestrating Environment Checks, Data/Session Preparation, and Non-Destructive Vulnerability Audits.
- 📊 **Executive Report Exporter**: Formats findings into Markdown and PDF executive security reports.

---

## 🚀 Quickstart Usage

Add `ferrox-selftest` to your `Cargo.toml`:

```toml
[dependencies]
ferrox-selftest = "0.1.2"
tokio = { version = "1.0", features = ["full"] }
```

### 1. Lookup OWASP Vulnerability Remediations (`VulnerabilityDictionary`)

```rust
use ferrox_selftest::vocabulary::VulnerabilityDictionary;

fn main() {
    if let Some(entry) = VulnerabilityDictionary::lookup("WSTG-INP-01") {
        println!("Category: {}", entry.category);
        println!("Plain Language: {}", entry.plain_language);
        println!("Business Risk: {}", entry.business_risk);
        println!("Remediation: {}", entry.owasp_remediation);
    }
}
```

### 2. Execute 3-Step Audit Pipeline (`ThreeStepAuditPipeline`)

```rust,no_run
use ferrox_selftest::pipeline::ThreeStepAuditPipeline;

#[tokio::main]
async fn main() {
    let mut pipeline = ThreeStepAuditPipeline::new("http://localhost:8080");
    
    let report = pipeline.run_full_audit().await;
    println!("Audit Status: {:?}", report.status);
    println!("Findings Count: {}", report.findings.len());
}
```

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
