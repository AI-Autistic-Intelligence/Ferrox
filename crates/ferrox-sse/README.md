# Ferrox SSE (`ferrox-sse`)

`ferrox-sse` simplifies pushing real-time Server-Sent Events (SSE) over HTTP connections in Axum web applications.

## Key Features
- 📡 **`SseStreamBuilder`**: Easily wrap Tokio channels or event streams into standard `text/event-stream` responses.
- 🔄 **Auto-Keepalive**: Automatic ping comments to prevent proxy and browser timeouts.
