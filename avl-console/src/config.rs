//! Configuration management for AVL Console

use crate::error::{ConsoleError, Result};
use serde::{Deserialize, Serialize};
use std::env;

/// Console configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleConfig {
    /// Server bind address
    pub bind_address: String,

    /// Server port
    pub port: u16,

    /// AVL Auth endpoint
    pub auth_endpoint: String,

    /// AvilaDB endpoint
    pub aviladb_endpoint: String,

    /// AVL Storage endpoint
    pub storage_endpoint: String,

    /// AVL Observability endpoint
    pub observability_endpoint: String,

    /// Session secret for cookies
    pub session_secret: String,

    /// Enable debug mode
    pub debug: bool,

    /// CORS allowed origins
    pub cors_origins: Vec<String>,

    /// Rate limit: requests per minute
    pub rate_limit: u32,

    /// WebSocket ping interval (seconds)
    pub ws_ping_interval: u64,

    /// Maximum WebSocket connections per user
    pub max_ws_connections: usize,

    /// Static files directory
    pub static_dir: String,

    /// Templates directory
    pub templates_dir: String,
}

impl Default for ConsoleConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1".to_string(),
            port: 8080,
            auth_endpoint: "http://localhost:8001".to_string(),
            aviladb_endpoint: "http://localhost:8000".to_string(),
            storage_endpoint: "http://localhost:8002".to_string(),
            observability_endpoint: "http://localhost:8003".to_string(),
            session_secret: "avl-console-secret-change-in-production".to_string(),
            debug: false,
            cors_origins: vec!["http://localhost:8080".to_string()],
            rate_limit: 100,
            ws_ping_interval: 30,
            max_ws_connections: 10,
            static_dir: "static".to_string(),
            templates_dir: "templates".to_string(),
        }
    }
}

impl ConsoleConfig {
    /// Load configuration from environment variables
    ///
    /// # Environment Variables
    ///
    /// - `AVL_CONSOLE_BIND`: Server bind address (default: 127.0.0.1)
    /// - `AVL_CONSOLE_PORT`: Server port (default: 8080)
    /// - `AVL_AUTH_ENDPOINT`: AVL Auth endpoint
    /// - `AVL_AVILADB_ENDPOINT`: AvilaDB endpoint
    /// - `AVL_STORAGE_ENDPOINT`: Storage endpoint
    /// - `AVL_OBSERVABILITY_ENDPOINT`: Observability endpoint
    /// - `AVL_CONSOLE_SECRET`: Session secret
    /// - `AVL_CONSOLE_DEBUG`: Enable debug mode
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();

        if let Ok(bind) = env::var("AVL_CONSOLE_BIND") {
            config.bind_address = bind;
        }

        if let Ok(port) = env::var("AVL_CONSOLE_PORT") {
            config.port = port.parse().map_err(|_| {
                ConsoleError::Config("Invalid port number".to_string())
            })?;
        }

        if let Ok(endpoint) = env::var("AVL_AUTH_ENDPOINT") {
            config.auth_endpoint = endpoint;
        }

        if let Ok(endpoint) = env::var("AVL_AVILADB_ENDPOINT") {
            config.aviladb_endpoint = endpoint;
        }

        if let Ok(endpoint) = env::var("AVL_STORAGE_ENDPOINT") {
            config.storage_endpoint = endpoint;
        }

        if let Ok(endpoint) = env::var("AVL_OBSERVABILITY_ENDPOINT") {
            config.observability_endpoint = endpoint;
        }

        if let Ok(secret) = env::var("AVL_CONSOLE_SECRET") {
            config.session_secret = secret;
        }

        if let Ok(debug) = env::var("AVL_CONSOLE_DEBUG") {
            config.debug = debug.parse().unwrap_or(false);
        }

        if let Ok(origins) = env::var("AVL_CONSOLE_CORS_ORIGINS") {
            config.cors_origins = origins.split(',').map(|s| s.to_string()).collect();
        }

        if let Ok(limit) = env::var("AVL_CONSOLE_RATE_LIMIT") {
            config.rate_limit = limit.parse().unwrap_or(100);
        }

        Ok(config)
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        if self.session_secret == "avl-console-secret-change-in-production" {
            tracing::warn!("⚠️  Using default session secret. Change in production!");
        }

        if self.rate_limit == 0 {
            return Err(ConsoleError::Config(
                "Rate limit must be greater than 0".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ConsoleConfig::default();
        assert_eq!(config.port, 8080);
        assert!(!config.debug);
    }

    #[test]
    fn test_config_validation() {
        let config = ConsoleConfig::default();
        assert!(config.validate().is_ok());
    }
}
