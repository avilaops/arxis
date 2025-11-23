//! Application state management

use crate::{
    config::ConsoleConfig,
    error::{ConsoleError, Result},
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared application state
pub struct AppState {
    /// Configuration
    pub config: ConsoleConfig,

    /// Active WebSocket connections (user_id -> connection count)
    pub ws_connections: Arc<RwLock<HashMap<String, usize>>>,

    /// Rate limiter state (user_id -> request count)
    pub rate_limiter: Arc<RwLock<HashMap<String, (u32, std::time::Instant)>>>,

    /// Session store (session_id -> user_id)
    pub sessions: Arc<RwLock<HashMap<String, String>>>,

    /// Cached metrics for dashboard
    pub metrics_cache: Arc<RwLock<Option<DashboardMetrics>>>,
}

/// Dashboard metrics cache
#[derive(Debug, Clone)]
pub struct DashboardMetrics {
    pub database_count: usize,
    pub storage_buckets: usize,
    pub storage_size_bytes: u64,
    pub active_connections: usize,
    pub requests_per_minute: u32,
    pub last_updated: std::time::Instant,
}

// Type alias for convenience
pub type ConsoleState = AppState;

impl AppState {
    /// Create new application state
    pub async fn new(config: &ConsoleConfig) -> Result<Self> {
        config.validate()?;

        Ok(Self {
            config: config.clone(),
            ws_connections: Arc::new(RwLock::new(HashMap::new())),
            rate_limiter: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
            metrics_cache: Arc::new(RwLock::new(None)),
        })
    }

    /// Check if user can create a new WebSocket connection
    pub async fn can_create_ws_connection(&self, user_id: &str) -> bool {
        let connections = self.ws_connections.read().await;
        let count = connections.get(user_id).copied().unwrap_or(0);
        count < self.config.max_ws_connections
    }

    /// Increment WebSocket connection count for user
    pub async fn increment_ws_connection(&self, user_id: String) {
        let mut connections = self.ws_connections.write().await;
        *connections.entry(user_id).or_insert(0) += 1;
    }

    /// Decrement WebSocket connection count for user
    pub async fn decrement_ws_connection(&self, user_id: &str) {
        let mut connections = self.ws_connections.write().await;
        if let Some(count) = connections.get_mut(user_id) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                connections.remove(user_id);
            }
        }
    }

    /// Check rate limit for user
    pub async fn check_rate_limit(&self, user_id: &str) -> Result<()> {
        let mut limiter = self.rate_limiter.write().await;
        let now = std::time::Instant::now();

        if let Some((count, last_check)) = limiter.get_mut(user_id) {
            if now.duration_since(*last_check).as_secs() >= 60 {
                // Reset after 1 minute
                *count = 1;
                *last_check = now;
            } else if *count >= self.config.rate_limit {
                return Err(ConsoleError::RateLimitExceeded);
            } else {
                *count += 1;
            }
        } else {
            limiter.insert(user_id.to_string(), (1, now));
        }

        Ok(())
    }

    /// Store session
    pub async fn store_session(&self, session_id: String, user_id: String) {
        let mut sessions = self.sessions.write().await;
        sessions.insert(session_id, user_id);
    }

    /// Get user ID from session
    pub async fn get_session(&self, session_id: &str) -> Option<String> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    /// Remove session
    pub async fn remove_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    /// Update metrics cache
    pub async fn update_metrics(&self, metrics: DashboardMetrics) {
        let mut cache = self.metrics_cache.write().await;
        *cache = Some(metrics);
    }

    /// Get cached metrics
    pub async fn get_metrics(&self) -> Option<DashboardMetrics> {
        let cache = self.metrics_cache.read().await;
        cache.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ws_connection_limit() {
        let config = ConsoleConfig::default();
        let state = AppState::new(&config).await.unwrap();

        assert!(state.can_create_ws_connection("user1").await);

        for _ in 0..config.max_ws_connections {
            state.increment_ws_connection("user1".to_string()).await;
        }

        assert!(!state.can_create_ws_connection("user1").await);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let mut config = ConsoleConfig::default();
        config.rate_limit = 5;
        let state = AppState::new(&config).await.unwrap();

        for _ in 0..5 {
            assert!(state.check_rate_limit("user1").await.is_ok());
        }

        assert!(state.check_rate_limit("user1").await.is_err());
    }
}
