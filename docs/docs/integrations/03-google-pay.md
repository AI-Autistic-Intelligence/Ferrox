---
sidebar_position: 3
---

# Google Pay

The `integrations/ferrox-google-pay` module facilitates server-side verification and processing of Google Pay tokens.

## How it works

When a mobile or web client requests a payment via Google Pay, they receive an encrypted token. This token must be sent to your Rust-FERROX backend, where it is decrypted, validated against Google's public keys, and subsequently passed to your acquiring bank or payment processor.

```rust
use ferrox_google_pay::GooglePayClient;

let gpay = GooglePayClient::new();
let payment_data = gpay.verify_token(client_token_str).await?;
```
