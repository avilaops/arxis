use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::mpsc;
use uuid::Uuid;
use tracing::{debug, info, warn};

use super::messages::{WsMessage, WsEvent};

pub type ConnectionId = String;

/// Gerenciador de conexões WebSocket
#[derive(Clone)]
pub struct ConnectionManager {
    connections: Arc<DashMap<ConnectionId, Connection>>,
    subscriptions: Arc<DashMap<String, Vec<ConnectionId>>>,
}

struct Connection {
    id: ConnectionId,
    sender: mpsc::UnboundedSender<WsMessage>,
    channels: Vec<String>,
}

impl ConnectionManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(DashMap::new()),
            subscriptions: Arc::new(DashMap::new()),
        }
    }

    /// Adicionar nova conexão
    pub fn add_connection(&self, sender: mpsc::UnboundedSender<WsMessage>) -> ConnectionId {
        let id = Uuid::new_v4().to_string();
        
        let connection = Connection {
            id: id.clone(),
            sender,
            channels: Vec::new(),
        };

        self.connections.insert(id.clone(), connection);
        info!("New WebSocket connection: {}", id);
        
        id
    }

    /// Remover conexão
    pub fn remove_connection(&self, connection_id: &str) {
        if let Some((_, connection)) = self.connections.remove(connection_id) {
            // Remover de todos os canais subscritos
            for channel in &connection.channels {
                if let Some(mut subs) = self.subscriptions.get_mut(channel) {
                    subs.retain(|id| id != connection_id);
                }
            }
            info!("WebSocket connection closed: {}", connection_id);
        }
    }

    /// Subscrever a canais
    pub fn subscribe(&self, connection_id: &str, channels: Vec<String>) {
        if let Some(mut connection) = self.connections.get_mut(connection_id) {
            for channel in channels {
                // Adicionar ao canal
                self.subscriptions
                    .entry(channel.clone())
                    .or_insert_with(Vec::new)
                    .push(connection_id.to_string());

                // Adicionar à lista de canais da conexão
                connection.channels.push(channel.clone());
                
                debug!("Connection {} subscribed to channel {}", connection_id, channel);
            }
        }
    }

    /// Desinscrever de canais
    pub fn unsubscribe(&self, connection_id: &str, channels: Vec<String>) {
        if let Some(mut connection) = self.connections.get_mut(connection_id) {
            for channel in channels {
                // Remover do canal
                if let Some(mut subs) = self.subscriptions.get_mut(&channel) {
                    subs.retain(|id| id != connection_id);
                }

                // Remover da lista de canais da conexão
                connection.channels.retain(|ch| ch != &channel);
                
                debug!("Connection {} unsubscribed from channel {}", connection_id, channel);
            }
        }
    }

    /// Enviar mensagem para uma conexão específica
    pub fn send_to_connection(&self, connection_id: &str, message: WsMessage) {
        if let Some(connection) = self.connections.get(connection_id) {
            if let Err(e) = connection.sender.send(message) {
                warn!("Failed to send message to connection {}: {}", connection_id, e);
                // Conexão pode estar fechada, remover
                drop(connection);
                self.remove_connection(connection_id);
            }
        }
    }

    /// Broadcast para um canal
    pub fn broadcast_to_channel(&self, channel: &str, message: WsMessage) {
        if let Some(subscribers) = self.subscriptions.get(channel) {
            let count = subscribers.len();
            for connection_id in subscribers.iter() {
                self.send_to_connection(connection_id, message.clone());
            }
            debug!("Broadcasted to {} subscribers on channel {}", count, channel);
        }
    }

    /// Broadcast para todas as conexões
    pub fn broadcast_all(&self, message: WsMessage) {
        let count = self.connections.len();
        for entry in self.connections.iter() {
            if let Err(e) = entry.sender.send(message.clone()) {
                warn!("Failed to broadcast to connection {}: {}", entry.id, e);
            }
        }
        debug!("Broadcasted to {} connections", count);
    }

    /// Processar evento do sistema
    pub fn process_event(&self, event: WsEvent) {
        match event {
            WsEvent::NewEvent(behavior_event) => {
                // Broadcast para canal "events"
                self.broadcast_to_channel(
                    "events",
                    WsMessage::Event { event: behavior_event }
                );
            }
            WsEvent::MetricsUpdate(metrics) => {
                // Broadcast para canal "metrics"
                self.broadcast_to_channel(
                    "metrics",
                    WsMessage::Metrics { metrics }
                );
            }
            WsEvent::AlertTriggered(alert) => {
                // Broadcast para canal "alerts"
                self.broadcast_to_channel(
                    "alerts",
                    WsMessage::Alert { alert }
                );
            }
            WsEvent::StatsUpdate(stats) => {
                // Broadcast para canal "stats"
                self.broadcast_to_channel(
                    "stats",
                    WsMessage::Stats { stats }
                );
            }
        }
    }

    /// Obter número de conexões ativas
    pub fn connection_count(&self) -> usize {
        self.connections.len()
    }

    /// Obter número de inscritos em um canal
    pub fn channel_subscriber_count(&self, channel: &str) -> usize {
        self.subscriptions
            .get(channel)
            .map(|subs| subs.len())
            .unwrap_or(0)
    }
}

impl Default for ConnectionManager {
    fn default() -> Self {
        Self::new()
    }
}
