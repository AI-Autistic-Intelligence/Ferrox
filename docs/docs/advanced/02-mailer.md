---
sidebar_position: 2
---

# Mailer (Lettre)

Rust-YALC uses the `lettre` library wrapped inside `yalc-mailer` for sending asynchronous emails via SMTP.

## Usage

```rust
use yalc_mailer::{Mailer, EmailParams};

let mailer = Mailer::new();
let email = EmailParams {
    to: "user@example.com".to_string(),
    subject: "Welcome!".to_string(),
    body_html: "<h1>Welcome to Rust-YALC</h1>".to_string(),
};

mailer.send(email).await?;
```
