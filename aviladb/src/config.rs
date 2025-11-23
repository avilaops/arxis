//! Configuration for AvilaDB

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// AvilaDB Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Data directory for storage
    pub data_dir: PathBuf,

    /// Enable compression (LZ4)
    pub enable_compression: bool,

    /// Enable vector search
    pub enable_vector_search: bool,

    /// Maximum connections
    pub max_connections: usize,

    /// Connection timeout (seconds)
    pub connection_timeout: u64,

    /// Request timeout (seconds)
    pub request_timeout: u64,

    /// Maximum document size (bytes)
    pub max_document_size: usize,

    /// Enable multi-region replication
    pub enable_replication: bool,

    /// Replication endpoints
    pub replication_endpoints: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: PathBuf::from("./aviladb_data"),
            enable_compression: true,
            enable_vector_search: false,
            max_connections: 1000,
            connection_timeout: 30,
            request_timeout: 60,
            max_document_size: 4 * 1024 * 1024, // 4 MB
            enable_replication: false,
            replication_endpoints: vec![],
        }
    }
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set data directory
    pub fn with_data_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.data_dir = dir.into();
        self
    }

    /// Enable compression
    pub fn with_compression(mut self, enabled: bool) -> Self {
        self.enable_compression = enabled;
        self
    }

    /// Enable vector search
    pub fn with_vector_search(mut self, enabled: bool) -> Self {
        self.enable_vector_search = enabled;
        self
    }

    /// Set maximum connections
    pub fn with_max_connections(mut self, max: usize) -> Self {
        self.max_connections = max;
        self
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.max_connections == 0 {
            return Err("max_connections must be greater than 0".to_string());
        }

        if self.connection_timeout == 0 {
            return Err("connection_timeout must be greater than 0".to_string());
        }

        if self.max_document_size > 4 * 1024 * 1024 {
            return Err("max_document_size cannot exceed 4 MB".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.max_connections, 1000);
        assert!(config.enable_compression);
    }

    #[test]
    fn test_config_builder() {
        let config = Config::new()
            .with_data_dir("/tmp/aviladb")
            .with_compression(false)
            .with_max_connections(500);

        assert_eq!(config.data_dir, PathBuf::from("/tmp/aviladb"));
        assert!(!config.enable_compression);
        assert_eq!(config.max_connections, 500);
    }

    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());

        let mut invalid_config = Config::default();
        invalid_config.max_connections = 0;
        assert!(invalid_config.validate().is_err());
    }
}
