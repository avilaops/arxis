//! WebSocket support for the gateway
//!
//! This module provides WebSocket proxying capabilities, allowing
//! the gateway to handle WebSocket connections and forward them
//! to upstream services.

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as TungsteniteMessage};
use tracing::{error, info};

/// WebSocket route configuration
#[derive(Debug, Clone)]
pub struct WebSocketRoute {
    /// Client-facing path
    pub path: String,

    /// Upstream WebSocket URL
    pub upstream: String,

    /// Connection timeout in seconds
    pub timeout_seconds: Option<u64>,
}

impl WebSocketRoute {
    /// Create a new WebSocket route
    pub fn new(path: impl Into<String>, upstream: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            upstream: upstream.into(),
            timeout_seconds: None,
        }
    }

    /// Set connection timeout
    pub fn with_timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }
}

/// WebSocket proxy handler
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    upstream_url: String,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, upstream_url))
}

/// Handle WebSocket connection
async fn handle_socket(client_socket: WebSocket, upstream_url: String) {
    info!("WebSocket connection established, connecting to {}", upstream_url);

    // Connect to upstream WebSocket
    let upstream_result = connect_async(&upstream_url).await;

    let (upstream_ws, _) = match upstream_result {
        Ok(ws) => ws,
        Err(e) => {
            error!("Failed to connect to upstream WebSocket: {}", e);
            return;
        }
    };

    let (mut client_sender, mut client_receiver) = client_socket.split();
    let (mut upstream_sink, mut upstream_stream) = upstream_ws.split();

    // Forward messages from client to upstream
    let client_to_upstream = tokio::spawn(async move {
        while let Some(msg) = client_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if upstream_sink
                        .send(TungsteniteMessage::Text(text))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Ok(Message::Binary(data)) => {
                    if upstream_sink
                        .send(TungsteniteMessage::Binary(data))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Ok(Message::Ping(data)) => {
                    if upstream_sink
                        .send(TungsteniteMessage::Ping(data))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Ok(Message::Pong(data)) => {
                    if upstream_sink
                        .send(TungsteniteMessage::Pong(data))
                        .await
                        .is_err()
                    {
                        break;
                    }
                }
                Ok(Message::Close(_)) => {
                    let _ = upstream_sink.send(TungsteniteMessage::Close(None)).await;
                    break;
                }
                Err(e) => {
                    error!("WebSocket client error: {}", e);
                    break;
                }
            }
        }
    });

    // Forward messages from upstream to client
    let upstream_to_client = tokio::spawn(async move {
        while let Some(msg) = upstream_stream.next().await {
            match msg {
                Ok(TungsteniteMessage::Text(text)) => {
                    if client_sender.send(Message::Text(text)).await.is_err() {
                        break;
                    }
                }
                Ok(TungsteniteMessage::Binary(data)) => {
                    if client_sender.send(Message::Binary(data)).await.is_err() {
                        break;
                    }
                }
                Ok(TungsteniteMessage::Ping(data)) => {
                    if client_sender.send(Message::Ping(data)).await.is_err() {
                        break;
                    }
                }
                Ok(TungsteniteMessage::Pong(data)) => {
                    if client_sender.send(Message::Pong(data)).await.is_err() {
                        break;
                    }
                }
                Ok(TungsteniteMessage::Close(_)) => {
                    let _ = client_sender.send(Message::Close(None)).await;
                    break;
                }
                Err(e) => {
                    error!("WebSocket upstream error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for both tasks to complete
    let _ = tokio::join!(client_to_upstream, upstream_to_client);

    info!("WebSocket connection closed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_websocket_route_creation() {
        let route = WebSocketRoute::new("/ws", "ws://localhost:8080/ws");
        assert_eq!(route.path, "/ws");
        assert_eq!(route.upstream, "ws://localhost:8080/ws");
        assert_eq!(route.timeout_seconds, None);
    }

    #[test]
    fn test_websocket_route_with_timeout() {
        let route = WebSocketRoute::new("/ws", "ws://localhost:8080/ws")
            .with_timeout(30);
        assert_eq!(route.timeout_seconds, Some(30));
    }
}
