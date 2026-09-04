---
sidebar_position: 1
---

# App Lifecycle

The `yalc-app` module is the backbone of the framework. It handles bootstrapping the HTTP server safely.

## YalcApp Structure

```rust
use yalc_app::YalcApp;
use axum::{Router, routing::get};

let router = Router::new().route("/", get(|| async { "Hello" }));
let app = YalcApp::new(router).with_port(3000);

app.start().await.unwrap();
```

## Built-in Security

By default, `YalcApp` injects:
1. **Helmet Middleware**: HSTS, X-Frame-Options, X-Content-Type-Options.
2. **CORS**: Configured to be secure out of the box.
3. **Catch Panic**: If your code `panic!()`s, it intercepts it and returns a `500 Internal Server Error` instead of crashing the process.
4. **Timeouts**: Every request has a strict 30-second global timeout.
