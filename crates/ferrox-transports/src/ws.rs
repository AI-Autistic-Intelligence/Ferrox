use crate::Transport;
use async_trait::async_trait;
use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    response::IntoResponse,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use ferrox_errors::AppError;

pub struct WsTransport {
    pub port: u16,
    pub path: String,
}

impl WsTransport {
    pub fn new(port: u16, path: &str) -> Self {
        Self { 
            port, 
            path: path.to_string(),
        }
    }
}

#[async_trait]
impl Transport for WsTransport {
    async fn start(&self) -> Result<(), AppError> {
        let app = Router::new().route(&self.path, get(ws_handler));
        
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.map_err(|e| {
            AppError::InternalError(format!("Failed to bind WS port {}: {}", self.port, e))
        })?;
        
        println!("📡 Starting WebSocket Transport on ws://{}{} ", addr, self.path);
        
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .map_err(|e| AppError::InternalError(format!("WebSocket Server Error: {}", e)))?;
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "WebSocket"
    }
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            match msg {
                Message::Text(t) => {
                    println!("Client sent string: {:?}", t);
                    // Echo back
                    if socket.send(Message::Text(format!("Server received: {}", t))).await.is_err() {
                        println!("Client disconnected");
                        return;
                    }
                }
                Message::Binary(_) => {
                    println!("Client sent binary data");
                }
                Message::Ping(_) => {
                    println!("Socket ping");
                }
                Message::Pong(_) => {
                    println!("Socket pong");
                }
                Message::Close(_) => {
                    println!("Client disconnected");
                    return;
                }
            }
        } else {
            println!("Client disconnected");
            return;
        }
    }
}
