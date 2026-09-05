---
sidebar_position: 3
---

# 📡 Server-Sent Events (SSE)

For real-time, unidirectional server-to-client data streaming (such as live stock price ticks, progress bars, or AI text streaming responses like ChatGPT), Server-Sent Events (SSE) provide a lightweight HTTP-native protocol.

Ferrox provides `ferrox-sse` helpers built on Axum's `Sse` response type.

---

## 1. Creating an SSE Event Stream

```rust
use axum::response::sse::{Event, Sse};
use futures::stream::{self, Stream};
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt;

pub async fn sse_tick_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = tokio_stream::wrappers::IntervalStream::new(
        tokio::time::interval(Duration::from_secs(1))
    ).map(|_| {
        let now = chrono::Utc::now().to_rfc3339();
        Ok(Event::default().data(format!("Tick: {}", now)))
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
```

---

## 2. Attaching SSE to Routers

```rust
use axum::{routing::get, Router};

pub fn sse_router() -> Router {
    Router::new()
        .route("/events/ticks", get(sse_tick_handler))
}
```

---

## 3. Frontend JavaScript Consumption

Frontend clients connect using the standard browser `EventSource` API:

```javascript
const eventSource = new EventSource('/events/ticks');

eventSource.onmessage = (event) => {
  console.log('Received real-time update:', event.data);
};
```
