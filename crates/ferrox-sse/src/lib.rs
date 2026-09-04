use axum::response::sse::{Event, Sse};
use futures::stream::Stream;
use std::convert::Infallible;
use tokio_stream::StreamExt as _;

/// Helper to create an SSE Stream from a standard tokio stream or channel
pub fn create_sse_stream<S>(stream: S) -> Sse<impl Stream<Item = Result<Event, Infallible>>>
where
    S: Stream<Item = String> + Send + 'static,
{
    let mapped = stream.map(|data| Ok(Event::default().data(data)));
    Sse::new(mapped).keep_alive(axum::response::sse::KeepAlive::new())
}
