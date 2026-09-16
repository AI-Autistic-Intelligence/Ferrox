# Ferrox Guards (`ferrox-guards`)

[![Crates.io](https://img.shields.io/crates/v/ferrox-guards.svg)](https://crates.io/crates/ferrox-guards)
[![Documentation](https://docs.rs/ferrox-guards/badge.svg)](https://docs.rs/ferrox-guards)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

`ferrox-guards` provides declarative role-based access control (RBAC) route extractors, zero-width evasion sanitizers, and session hijacking replay guards for Axum web applications.

---

## 🔑 Key Features

- 🛡️ **`FeatureSqueezer`**: Strips zero-width Unicode characters (`\u{200B}`, `\u{FEFF}`), canonicalizes percent-encoded strings, and normalizes whitespace to prevent WAF evasion.
- 🔒 **`SessionReplayDetector`**: Binds PASETO/JWT session tokens to client User-Agent and HTTP header structure fingerprints (`ClientFingerprint`), invalidating stolen token replay attempts.
- 🔐 **Declarative Guard Extractors**: Axum extractors (`RequireRole`) protecting handlers with compile-safe role constraints.

---

## 🚀 Quickstart Usage

Add `ferrox-guards` to your `Cargo.toml`:

```toml
[dependencies]
ferrox-guards = "0.1.2"
axum = "0.7"
```

### 1. Zero-Width Unicode Evasion Sanitization (`FeatureSqueezer`)

```rust
use ferrox_guards::FeatureSqueezer;

fn main() {
    let raw_payload = "admin\u{200B}user%20login";
    let sanitized = FeatureSqueezer::squeeze(raw_payload);
    
    assert_eq!(sanitized.clean_text, "adminuser login");
    assert!(sanitized.zero_width_detected);
}
```

### 2. Session Replay & Fingerprint Guard (`SessionReplayDetector`)

```rust
use ferrox_guards::{ClientFingerprint, SessionReplayDetector, SessionStatus};

fn verify_session(user_agent: &str, accept_lang: &str, accept_enc: &str, bound_hash: &str) {
    let current_fp = ClientFingerprint::new(user_agent, accept_lang, accept_enc);
    
    match SessionReplayDetector::evaluate_session(bound_hash, &current_fp) {
        SessionStatus::Valid => println!("Session authenticated successfully"),
        SessionStatus::HijackDetected { .. } => println!("Session hijacking attempt detected!"),
    }
}
```

---

## 📜 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
