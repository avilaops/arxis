//! AvilaDB client implementation

use std::sync::Arc;
use std::time::Duration;

use crate::{Config, Database, Result};

/// AvilaDB client for connecting to the database
#[derive(Debug, Clone)]
pub struct AvilaClient {
    config: Arc<Config>,
    // Internal HTTP client would go here
}

impl AvilaClient {
    /// Connect to AvilaDB with default configuration
    ///
    /// # Example
    ///
    /// ```no_run
    /// use aviladb::AvilaClient;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = AvilaClient::connect("http://localhost:8000").await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect(endpoint: &str) -> Result<Self> {
        let config = Config {
            endpoint: endpoint.to_string(),
            ..Default::default()
        };
        Self::with_config(config).await
    }

    /// Connect with custom configuration
    pub async fn with_config(config: Config) -> Result<Self> {
        // TODO: Initialize HTTP client, verify connection
        Ok(Self {
            config: Arc::new(config),
        })
    }

    /// Get a database handle
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use aviladb::AvilaClient;
    /// # async fn example(client: AvilaClient) -> aviladb::Result<()> {
    /// let db = client.database("gamedb").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn database(&self, name: &str) -> Result<Database> {
        Database::new(name.to_string(), self.config.clone())
    }

    /// Create a new database
    pub async fn create_database(&self, name: &str) -> Result<Database> {
        // TODO: Send CREATE DATABASE request
        self.database(name).await
    }

    /// List all databases
    pub async fn list_databases(&self) -> Result<Vec<String>> {
        // TODO: Send LIST DATABASES request
        Ok(vec![])
    }

    /// Delete a database
    pub async fn delete_database(&self, name: &str) -> Result<()> {
        // TODO: Send DELETE DATABASE request
        Ok(())
    }

    /// Get client configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_connect() {
        let client = AvilaClient::connect("http://localhost:8000").await;
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_client_database() {
        let client = AvilaClient::connect("http://localhost:8000").await.unwrap();
        let db = client.database("testdb").await;
        assert!(db.is_ok());
    }
}
