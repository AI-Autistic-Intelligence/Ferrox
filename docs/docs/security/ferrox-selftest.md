---
sidebar_position: 8
title: "🧪 Ferrox Selftest (Automated Audit & Benchmarks)"
---

# 🧪 Ferrox Selftest: Continuous Security Benchmark & Red-Team Audit Engine

`ferrox-selftest` is the autonomous verification, continuous integration security benchmark runner, and OWASP Web Security Testing Guide (WSTG) self-attack auditor for the Ferrox ecosystem.

It provides zero-trust automated regression testing across all 35 SOTA literature innovations, ensuring that no build is released to production unless the entire security mesh is 100% operational.

---

## 🎯 Key Capabilities

1. **Continuous Security Benchmark Runner**: Tests all 35 SOTA algorithms in real-time, outputting an ecosystem health score (0-100%) and detailed academic innovation audit reports.
2. **Autonomous Red-Team Self-Attack Suite**: Simulates hostile penetration testing attacks (SQLi, XSS, SSRF, JWT forgery, replay attacks, timing attacks, prompt injection) against running Ferrox instances.
3. **WSTG Compliance Auditor**: Automatically maps threat vulnerabilities against OWASP Top 10 and WSTG v4.2 test categories.

---

## 🛠️ Usage Example

Add `ferrox-selftest` to your `Cargo.toml`:

```toml
[dependencies]
ferrox-selftest = "0.5.0"
```

### Running the Continuous Benchmark

```rust
use ferrox_selftest::continuous_benchmark::ContinuousSecurityBenchmark;

fn main() {
    let report = ContinuousSecurityBenchmark::run_benchmark();

    println!("Ecosystem Health Score: {}%", report.ecosystem_health_score);
    println!("Total Innovations Tested: {}", report.total_innovations_tested);
    println!("Passed Innovations: {}", report.passed_innovations);

    for result in report.results {
        println!("- [{}]: {} ({})", 
            if result.passed { "PASS" } else { "FAIL" }, 
            result.innovation_name, 
            result.academic_reference
        );
    }
}
```

---

## 🧪 Terminal Execution

You can also trigger `ferrox-selftest` via `cargo-ferrox` CLI:

```bash
cargo ferrox selftest --full
```
