use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::models::BehaviorEvent;
use crate::storage::StorageStats;

/// Mensagens WebSocket entre cliente e servidor
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsMessage {
    // Cliente -> Servidor
    Subscribe { channels: Vec<String> },
    Unsubscribe { channels: Vec<String> },
    Ping,
    
    // Servidor -> Cliente
    Event { event: BehaviorEvent },
    Metrics { metrics: RealtimeMetrics },
    Alert { alert: AlertMessage },
    Stats { stats: StorageStats },
    Pong,
    Error { message: String },
}

/// Eventos do sistema para broadcast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WsEvent {
    NewEvent(BehaviorEvent),
    MetricsUpdate(RealtimeMetrics),
    AlertTriggered(AlertMessage),
    StatsUpdate(StorageStats),
}

/// Métricas em tempo real
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealtimeMetrics {
    pub timestamp: DateTime<Utc>,
    pub active_users: usize,
    pub events_per_second: f64,
    pub events_last_minute: usize,
    pub top_pages: Vec<(String, usize)>,
    pub conversion_rate: f64,
    pub avg_session_duration: f64,
}

/// Mensagem de alerta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertMessage {
    pub id: String,
    pub severity: AlertSeverity,
    pub title: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

impl WsMessage {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}
