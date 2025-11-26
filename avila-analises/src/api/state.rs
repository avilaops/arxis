use crate::storage::EventStore;
use crate::websocket::ConnectionManager;
use std::sync::Arc;

/// Estado compartilhado da aplicação
#[derive(Clone)]
pub struct AppState {
    pub event_store: Arc<dyn EventStore>,
    pub config: AppConfig,
    pub ws_manager: ConnectionManager,
}

impl AppState {
    pub fn new(event_store: Arc<dyn EventStore>, config: AppConfig) -> Self {
        Self {
            event_store,
            config,
            ws_manager: ConnectionManager::new(),
        }
    }
}

/// Configuração da aplicação
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub api_version: String,
    pub max_batch_size: usize,
    pub max_query_limit: usize,
    pub default_page_size: usize,
    pub enable_cors: bool,
    pub allowed_origins: Vec<String>,
    pub api_keys: Vec<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_version: "v1".to_string(),
            max_batch_size: 1000,
            max_query_limit: 10000,
            default_page_size: 100,
            enable_cors: true,
            allowed_origins: vec!["*".to_string()],
            api_keys: vec!["dev-key-123".to_string()], // Em produção, vem de env
        }
    }
}
