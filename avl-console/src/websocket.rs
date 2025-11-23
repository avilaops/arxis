//! WebSocket support for real-time updates

use crate::state::AppState;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{interval, Duration};

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/", get(websocket_handler))
        .with_state(state)
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let user_id = "user_001".to_string(); // TODO: Extract from auth

    // Check connection limit
    if !state.can_create_ws_connection(&user_id).await {
        let _ = sender
            .send(Message::Text(
                serde_json::to_string(&WsMessage::error(
                    "Maximum WebSocket connections reached",
                ))
                .unwrap(),
            ))
            .await;
        return;
    }

    state.increment_ws_connection(user_id.clone()).await;

    // Send welcome message
    let welcome = WsMessage::connected("Welcome to AVL Console");
    if sender
        .send(Message::Text(serde_json::to_string(&welcome).unwrap()))
        .await
        .is_err()
    {
        state.decrement_ws_connection(&user_id).await;
        return;
    }

    // Spawn ping task
    let mut ping_interval = interval(Duration::from_secs(state.config.ws_ping_interval));
    let (ping_tx, mut ping_rx) = tokio::sync::mpsc::channel(10);

    tokio::spawn(async move {
        loop {
            ping_interval.tick().await;
            if ping_tx.send(()).await.is_err() {
                break;
            }
        }
    });

    // Handle messages
    loop {
        tokio::select! {
            msg = receiver.next() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(ws_msg) = serde_json::from_str::<WsMessage>(&text) {
                            handle_message(ws_msg, &mut sender, &state).await;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        break;
                    }
                    _ => {}
                }
            }
            _ = ping_rx.recv() => {
                if sender.send(Message::Ping(vec![])).await.is_err() {
                    break;
                }
            }
        }
    }

    state.decrement_ws_connection(&user_id).await;
}

async fn handle_message(
    msg: WsMessage,
    sender: &mut futures::stream::SplitSink<WebSocket, Message>,
    _state: &Arc<AppState>,
) {
    match msg.msg_type.as_str() {
        "subscribe" => {
            let response = WsMessage::subscribed(&msg.payload.unwrap_or_default());
            let _ = sender
                .send(Message::Text(serde_json::to_string(&response).unwrap()))
                .await;
        }
        "ping" => {
            let response = WsMessage::pong();
            let _ = sender
                .send(Message::Text(serde_json::to_string(&response).unwrap()))
                .await;
        }
        _ => {}
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WsMessage {
    #[serde(rename = "type")]
    msg_type: String,
    payload: Option<String>,
}

impl WsMessage {
    fn connected(msg: &str) -> Self {
        Self {
            msg_type: "connected".to_string(),
            payload: Some(msg.to_string()),
        }
    }

    fn error(msg: &str) -> Self {
        Self {
            msg_type: "error".to_string(),
            payload: Some(msg.to_string()),
        }
    }

    fn subscribed(topic: &str) -> Self {
        Self {
            msg_type: "subscribed".to_string(),
            payload: Some(topic.to_string()),
        }
    }

    fn pong() -> Self {
        Self {
            msg_type: "pong".to_string(),
            payload: None,
        }
    }
}
