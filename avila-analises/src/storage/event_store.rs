use crate::models::BehaviorEvent;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Trait abstrato para storage de eventos
/// Permite trocar entre InMemory, AvilaDB, etc.
#[async_trait]
pub trait EventStore: Send + Sync {
    /// Armazenar um único evento
    async fn store(&self, event: BehaviorEvent) -> Result<(), StorageError>;

    /// Armazenar múltiplos eventos em batch
    async fn store_batch(&self, events: Vec<BehaviorEvent>) -> Result<(), StorageError>;

    /// Buscar eventos com filtros
    async fn query(&self, filter: EventFilter) -> Result<Vec<BehaviorEvent>, StorageError>;

    /// Buscar eventos por user_id
    async fn get_by_user(&self, user_id: &str, options: QueryOptions) -> Result<Vec<BehaviorEvent>, StorageError>;

    /// Buscar eventos por session_id
    async fn get_by_session(&self, session_id: &str) -> Result<Vec<BehaviorEvent>, StorageError>;

    /// Buscar eventos por intervalo de tempo
    async fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        options: QueryOptions,
    ) -> Result<Vec<BehaviorEvent>, StorageError>;

    /// Contar total de eventos
    async fn count(&self, filter: Option<EventFilter>) -> Result<usize, StorageError>;

    /// Contar eventos por tipo
    async fn count_by_type(&self) -> Result<HashMap<String, usize>, StorageError>;

    /// Deletar eventos antigos (retention policy)
    async fn delete_older_than(&self, timestamp: DateTime<Utc>) -> Result<usize, StorageError>;

    /// Obter estatísticas do storage
    async fn get_stats(&self) -> Result<StorageStats, StorageError>;

    /// Health check
    async fn health_check(&self) -> Result<(), StorageError>;
}

/// Filtros para query de eventos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFilter {
    pub user_ids: Option<Vec<String>>,
    pub session_ids: Option<Vec<String>>,
    pub event_types: Option<Vec<String>>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub metadata_filters: Option<HashMap<String, String>>,
    pub device_types: Option<Vec<String>>,
    pub countries: Option<Vec<String>>,
}

impl Default for EventFilter {
    fn default() -> Self {
        Self {
            user_ids: None,
            session_ids: None,
            event_types: None,
            start_time: None,
            end_time: None,
            metadata_filters: None,
            device_types: None,
            countries: None,
        }
    }
}

/// Opções de paginação e ordenação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryOptions {
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub order_by: OrderBy,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            limit: Some(100),
            offset: None,
            order_by: OrderBy::TimestampDesc,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderBy {
    TimestampAsc,
    TimestampDesc,
    UserIdAsc,
    EventTypeAsc,
}

/// Estatísticas do storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_events: usize,
    pub total_users: usize,
    pub total_sessions: usize,
    pub storage_size_bytes: usize,
    pub oldest_event: Option<DateTime<Utc>>,
    pub newest_event: Option<DateTime<Utc>>,
    pub events_by_type: HashMap<String, usize>,
}

/// Erros de storage
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Connection failed: {0}")]
    ConnectionError(String),

    #[error("Query failed: {0}")]
    QueryError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Storage full or quota exceeded")]
    QuotaExceeded,

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::SerializationError(err.to_string())
    }
}
