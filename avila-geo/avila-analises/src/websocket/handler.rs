use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use crate::api::state::AppState;
use super::{
    manager::ConnectionManager,
    messages::WsMessage,
};

/// Handler WebSocket
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    
    // Criar canal para comunicação com o gerenciador
    let (tx, mut rx) = mpsc::unbounded_channel::<WsMessage>();
    
    // Registrar conexão
    let connection_id = state.ws_manager.add_connection(tx);
    
    // Task para enviar mensagens
    let ws_manager_send = state.ws_manager.clone();
    let connection_id_send = connection_id.clone();
    let mut send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            match message.to_json() {
                Ok(json) => {
                    if sender.send(Message::Text(json)).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                }
            }
        }
        
        // Cleanup quando task terminar
        ws_manager_send.remove_connection(&connection_id_send);
    });

    // Task para receber mensagens
    let ws_manager_recv = state.ws_manager.clone();
    let connection_id_recv = connection_id.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                Message::Text(text) => {
                    if let Err(e) = handle_client_message(&text, &connection_id_recv, &ws_manager_recv).await {
                        error!("Error handling message: {}", e);
                    }
                }
                Message::Binary(_) => {
                    warn!("Received binary message, ignoring");
                }
                Message::Ping(payload) => {
                    debug!("Received ping, sending pong");
                    // Axum handles pong automatically
                }
                Message::Pong(_) => {
                    debug!("Received pong");
                }
                Message::Close(_) => {
                    info!("Client requested close");
                    break;
                }
            }
        }
        
        // Cleanup quando task terminar
        ws_manager_recv.remove_connection(&connection_id_recv);
    });

    // Aguardar qualquer task terminar
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
    
    info!("WebSocket connection {} closed", connection_id);
}

async fn handle_client_message(
    text: &str,
    connection_id: &str,
    manager: &ConnectionManager,
) -> Result<(), Box<dyn std::error::Error>> {
    let message = WsMessage::from_json(text)?;
    
    match message {
        WsMessage::Subscribe { channels } => {
            manager.subscribe(connection_id, channels);
            manager.send_to_connection(
                connection_id,
                WsMessage::Event { 
                    event: crate::models::BehaviorEvent {
                        event_id: uuid::Uuid::new_v4().to_string(),
                        user_id: "system".to_string(),
                        session_id: "system".to_string(),
                        timestamp: chrono::Utc::now(),
                        event_type: crate::models::EventType::Custom {
                            name: "subscribed".to_string(),
                            properties: serde_json::json!({}),
                        },
                        metadata: std::collections::HashMap::new(),
                        context: create_system_context(),
                    }
                }
            );
        }
        WsMessage::Unsubscribe { channels } => {
            manager.unsubscribe(connection_id, channels);
        }
        WsMessage::Ping => {
            manager.send_to_connection(connection_id, WsMessage::Pong);
        }
        _ => {
            warn!("Unexpected message from client: {:?}", message);
        }
    }
    
    Ok(())
}

fn create_system_context() -> crate::models::EventContext {
    crate::models::EventContext {
        device: crate::models::DeviceInfo {
            device_type: crate::models::DeviceType::Unknown,
            os: "System".to_string(),
            browser: "System".to_string(),
            screen_resolution: (0, 0),
        },
        location: crate::models::LocationInfo {
            country: "BR".to_string(),
            city: None,
            timezone: "UTC".to_string(),
            ip_address: "127.0.0.1".to_string(),
        },
        referrer: None,
        user_agent: "System".to_string(),
        viewport: crate::models::Viewport {
            width: 0,
            height: 0,
        },
    }
}
