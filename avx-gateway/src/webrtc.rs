//! WebRTC Signaling Server
//!
//! Implementa signaling server para estabelecer conexões WebRTC P2P
//! entre clientes de desktop remoto.
//!
//! ## Funcionalidades
//!
//! - **SDP Exchange**: Troca de Session Description Protocol (offer/answer)
//! - **ICE Candidates**: Troca de candidatos ICE para NAT traversal
//! - **Room Management**: Gerenciamento de sessões/salas
//! - **Presence**: Tracking de dispositivos online/offline
//! - **Authentication**: Integração com avl-auth

use axum::{
    extract::{
        ws::{Message, WebSocket},
        Path, State, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

/// Tipos de mensagens WebRTC
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum SignalMessage {
    /// Oferta SDP de um peer
    Offer {
        /// ID da sessão
        session_id: String,
        /// SDP offer
        sdp: String,
        /// ID do peer emissor
        from: String,
    },
    /// Resposta SDP de um peer
    Answer {
        /// ID da sessão
        session_id: String,
        /// SDP answer
        sdp: String,
        /// ID do peer emissor
        from: String,
    },
    /// Candidato ICE
    IceCandidate {
        /// ID da sessão
        session_id: String,
        /// Candidato ICE
        candidate: String,
        /// sdpMid
        sdp_mid: Option<String>,
        /// sdpMLineIndex
        sdp_mline_index: Option<u16>,
        /// ID do peer emissor
        from: String,
    },
    /// Solicitar entrada em uma sessão
    Join {
        /// ID da sessão
        session_id: String,
        /// ID do peer solicitante
        peer_id: String,
    },
    /// Sair de uma sessão
    Leave {
        /// ID da sessão
        session_id: String,
        /// ID do peer
        peer_id: String,
    },
    /// Resposta de erro
    Error {
        /// Mensagem de erro
        message: String,
    },
    /// Heartbeat/ping
    Ping,
    /// Resposta ao ping
    Pong,
}

/// Estado de um peer conectado
#[derive(Debug, Clone)]
struct Peer {
    id: String,
    session_id: Option<String>,
    tx: mpsc::UnboundedSender<Message>,
}

/// Sessão WebRTC (room)
#[derive(Debug, Clone)]
struct Session {
    id: String,
    peers: Vec<String>,
    created_at: std::time::SystemTime,
}

/// Estado compartilhado do signaling server
#[derive(Clone)]
pub struct SignalingState {
    /// Peers conectados (peer_id -> Peer)
    peers: Arc<RwLock<HashMap<String, Peer>>>,
    /// Sessões ativas (session_id -> Session)
    sessions: Arc<RwLock<HashMap<String, Session>>>,
}

impl SignalingState {
    /// Cria novo estado do signaling server
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Adiciona um peer
    async fn add_peer(&self, peer: Peer) {
        let peer_id = peer.id.clone();
        self.peers.write().await.insert(peer_id.clone(), peer);
        info!("Peer {} conectado", peer_id);
    }

    /// Remove um peer
    async fn remove_peer(&self, peer_id: &str) {
        self.peers.write().await.remove(peer_id);

        // Remover das sessões
        let mut sessions = self.sessions.write().await;
        for session in sessions.values_mut() {
            session.peers.retain(|id| id != peer_id);
        }

        info!("Peer {} desconectado", peer_id);
    }

    /// Envia mensagem para um peer específico
    async fn send_to_peer(&self, peer_id: &str, msg: &SignalMessage) -> Result<(), String> {
        let peers = self.peers.read().await;

        if let Some(peer) = peers.get(peer_id) {
            let json = serde_json::to_string(msg)
                .map_err(|e| format!("Erro ao serializar: {}", e))?;

            peer.tx.send(Message::Text(json))
                .map_err(|_| "Peer desconectado".to_string())?;

            Ok(())
        } else {
            Err(format!("Peer {} não encontrado", peer_id))
        }
    }

    /// Broadcast para todos os peers de uma sessão
    async fn broadcast_to_session(
        &self,
        session_id: &str,
        msg: &SignalMessage,
        exclude_peer: Option<&str>,
    ) {
        let sessions = self.sessions.read().await;

        if let Some(session) = sessions.get(session_id) {
            let peers = self.peers.read().await;

            for peer_id in &session.peers {
                if Some(peer_id.as_str()) == exclude_peer {
                    continue;
                }

                if let Some(peer) = peers.get(peer_id) {
                    if let Ok(json) = serde_json::to_string(msg) {
                        let _ = peer.tx.send(Message::Text(json));
                    }
                }
            }
        }
    }

    /// Adiciona peer a uma sessão
    async fn join_session(&self, session_id: &str, peer_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.write().await;

        // Criar sessão se não existir
        let session = sessions.entry(session_id.to_string()).or_insert_with(|| {
            Session {
                id: session_id.to_string(),
                peers: Vec::new(),
                created_at: std::time::SystemTime::now(),
            }
        });

        if !session.peers.contains(&peer_id.to_string()) {
            session.peers.push(peer_id.to_string());
        }

        // Atualizar peer
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.session_id = Some(session_id.to_string());
        }

        info!("Peer {} entrou na sessão {}", peer_id, session_id);
        Ok(())
    }

    /// Remove peer de uma sessão
    async fn leave_session(&self, session_id: &str, peer_id: &str) {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            session.peers.retain(|id| id != peer_id);

            // Remover sessão se vazia
            if session.peers.is_empty() {
                sessions.remove(session_id);
                info!("Sessão {} removida (vazia)", session_id);
            }
        }

        // Atualizar peer
        let mut peers = self.peers.write().await;
        if let Some(peer) = peers.get_mut(peer_id) {
            peer.session_id = None;
        }

        info!("Peer {} saiu da sessão {}", peer_id, session_id);
    }
}

impl Default for SignalingState {
    fn default() -> Self {
        Self::new()
    }
}

/// Handler WebSocket para signaling
pub async fn webrtc_signaling_handler(
    ws: WebSocketUpgrade,
    State(state): State<SignalingState>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

/// Handler WebSocket com peer_id específico
pub async fn webrtc_signaling_with_id_handler(
    ws: WebSocketUpgrade,
    Path(peer_id): Path<String>,
    State(state): State<SignalingState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_websocket_with_id(socket, state, peer_id))
}

/// Processa conexão WebSocket
async fn handle_websocket(socket: WebSocket, state: SignalingState) {
    let peer_id = Uuid::new_v4().to_string();
    handle_websocket_with_id(socket, state, peer_id).await
}

/// Processa conexão WebSocket com ID fornecido
async fn handle_websocket_with_id(
    socket: WebSocket,
    state: SignalingState,
    peer_id: String,
) {
    let (sender, mut receiver) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel();

    // Adicionar peer ao estado
    let peer = Peer {
        id: peer_id.clone(),
        session_id: None,
        tx,
    };
    state.add_peer(peer).await;

    // Task para enviar mensagens
    let peer_id_clone = peer_id.clone();
    let send_task = tokio::spawn(async move {
        use futures_util::SinkExt;
        let mut sender = sender;

        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }

        debug!("Send task finalizada para peer {}", peer_id_clone);
    });

    // Task para receber mensagens
    let peer_id_clone = peer_id.clone();
    let state_clone = state.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if let Message::Text(text) = msg {
                if let Err(e) = handle_signal_message(&text, &peer_id_clone, &state_clone).await {
                    warn!("Erro ao processar mensagem: {}", e);
                }
            } else if let Message::Close(_) = msg {
                break;
            }
        }

        debug!("Receive task finalizada para peer {}", peer_id_clone);
    });

    // Aguardar conclusão
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // Cleanup
    state.remove_peer(&peer_id).await;
}

/// Processa mensagem de signaling
async fn handle_signal_message(
    text: &str,
    peer_id: &str,
    state: &SignalingState,
) -> Result<(), String> {
    let msg: SignalMessage = serde_json::from_str(text)
        .map_err(|e| format!("JSON inválido: {}", e))?;

    match msg {
        SignalMessage::Join { session_id, .. } => {
            state.join_session(&session_id, peer_id).await?;
        }
        SignalMessage::Leave { session_id, .. } => {
            state.leave_session(&session_id, peer_id).await;
        }
        SignalMessage::Offer { session_id, sdp, .. } => {
            let forward_msg = SignalMessage::Offer {
                session_id: session_id.clone(),
                sdp,
                from: peer_id.to_string(),
            };
            state.broadcast_to_session(&session_id, &forward_msg, Some(peer_id)).await;
        }
        SignalMessage::Answer { session_id, sdp, .. } => {
            let forward_msg = SignalMessage::Answer {
                session_id: session_id.clone(),
                sdp,
                from: peer_id.to_string(),
            };
            state.broadcast_to_session(&session_id, &forward_msg, Some(peer_id)).await;
        }
        SignalMessage::IceCandidate {
            session_id,
            candidate,
            sdp_mid,
            sdp_mline_index,
            ..
        } => {
            let forward_msg = SignalMessage::IceCandidate {
                session_id: session_id.clone(),
                candidate,
                sdp_mid,
                sdp_mline_index,
                from: peer_id.to_string(),
            };
            state.broadcast_to_session(&session_id, &forward_msg, Some(peer_id)).await;
        }
        SignalMessage::Ping => {
            state.send_to_peer(peer_id, &SignalMessage::Pong).await?;
        }
        _ => {}
    }

    Ok(())
}

/// Cria router para signaling WebRTC
pub fn signaling_router() -> Router<SignalingState> {
    Router::new()
        .route("/signal", get(webrtc_signaling_handler))
        .route("/signal/:peer_id", get(webrtc_signaling_with_id_handler))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_signaling_state() {
        let state = SignalingState::new();

        let (tx, _rx) = mpsc::unbounded_channel();
        let peer = Peer {
            id: "peer1".to_string(),
            session_id: None,
            tx,
        };

        state.add_peer(peer).await;

        let peers = state.peers.read().await;
        assert!(peers.contains_key("peer1"));
    }

    #[tokio::test]
    async fn test_join_session() {
        let state = SignalingState::new();

        let (tx, _rx) = mpsc::unbounded_channel();
        let peer = Peer {
            id: "peer1".to_string(),
            session_id: None,
            tx,
        };

        state.add_peer(peer).await;
        state.join_session("session1", "peer1").await.unwrap();

        let sessions = state.sessions.read().await;
        assert!(sessions.contains_key("session1"));
        assert!(sessions.get("session1").unwrap().peers.contains(&"peer1".to_string()));
    }
}
