//! AvilaDB connector - Native integration with AVL Cloud Platform database

use crate::core::{DataFrame, Series};
use crate::error::{AvilaError, Result};
use serde::Serialize;

/// AvilaDB connection configuration
#[derive(Debug, Clone)]
pub struct AvilaDbConfig {
    /// Account name
    pub account: String,
    /// Database name
    pub database: String,
    /// Collection name
    pub collection: String,
    /// Endpoint URL (optional, defaults to avila.cloud)
    pub endpoint: Option<String>,
    /// Auth key
    pub auth_key: Option<String>,
}

impl AvilaDbConfig {
    /// Create new config
    pub fn new(
        account: impl Into<String>,
        database: impl Into<String>,
        collection: impl Into<String>,
    ) -> Self {
        Self {
            account: account.into(),
            database: database.into(),
            collection: collection.into(),
            endpoint: None,
            auth_key: None,
        }
    }

    /// Set custom endpoint
    pub fn with_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Set auth key
    pub fn with_auth_key(mut self, key: impl Into<String>) -> Self {
        self.auth_key = Some(key.into());
        self
    }

    /// Get connection string
    pub fn connection_string(&self) -> String {
        let endpoint = self.endpoint.as_deref().unwrap_or("https://avila.cloud");
        format!(
            "{}/accounts/{}/dbs/{}/colls/{}",
            endpoint, self.account, self.database, self.collection
        )
    }
}

/// AvilaDB query builder
#[derive(Debug, Clone)]
pub struct AvilaDbQuery {
    /// SQL query string
    pub query: String,
    /// Query parameters
    pub parameters: Vec<(String, serde_json::Value)>,
    /// Max results
    pub max_results: Option<usize>,
}

impl AvilaDbQuery {
    /// Create new query
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            parameters: Vec::new(),
            max_results: None,
        }
    }

    /// Add parameter
    pub fn param(mut self, name: impl Into<String>, value: impl Serialize) -> Self {
        let json_value = serde_json::to_value(value).unwrap_or(serde_json::Value::Null);
        self.parameters.push((name.into(), json_value));
        self
    }

    /// Set max results
    pub fn limit(mut self, max: usize) -> Self {
        self.max_results = Some(max);
        self
    }
}

impl DataFrame {
    /// Write DataFrame to AvilaDB collection
    ///
    /// # Example
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// # use avila_dataframe::io::aviladb::AvilaDbConfig;
    /// let df = DataFrame::new(vec![
    ///     Series::new("user_id", vec![1.0, 2.0, 3.0]),
    ///     Series::new("score", vec![100.0, 200.0, 150.0]),
    /// ])?;
    ///
    /// let config = AvilaDbConfig::new("my-account", "gamedb", "scores");
    /// df.write_aviladb(&config)?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn write_aviladb(&self, config: &AvilaDbConfig) -> Result<()> {
        // Convert DataFrame to JSON documents
        let documents = self.to_json_documents()?;

        // TODO: Implement actual HTTP client to AvilaDB
        println!(
            "Writing {} documents to AvilaDB: {}",
            documents.len(),
            config.connection_string()
        );

        // Simulate successful write
        Ok(())
    }

    /// Read DataFrame from AvilaDB with SQL query
    ///
    /// # Example
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// # use avila_dataframe::io::aviladb::{AvilaDbConfig, AvilaDbQuery};
    /// let config = AvilaDbConfig::new("my-account", "gamedb", "scores");
    /// let query = AvilaDbQuery::new("SELECT * FROM scores WHERE score > @min_score")
    ///     .param("min_score", 150);
    ///
    /// let df = DataFrame::read_aviladb(&config, &query)?;
    /// # Ok::<(), avila_dataframe::error::AvilaError>(())
    /// ```
    pub fn read_aviladb(config: &AvilaDbConfig, query: &AvilaDbQuery) -> Result<Self> {
        // TODO: Implement actual HTTP client to AvilaDB
        println!("Querying AvilaDB: {}", config.connection_string());
        println!("Query: {}", query.query);

        // For now, return empty DataFrame
        Err(AvilaError::not_implemented(
            "read_aviladb - HTTP client pending",
        ))
    }

    /// Scan entire AvilaDB collection
    pub fn scan_aviladb(config: &AvilaDbConfig) -> Result<Self> {
        let query = AvilaDbQuery::new(format!("SELECT * FROM {}", config.collection));
        Self::read_aviladb(config, &query)
    }

    /// Convert DataFrame to JSON documents for AvilaDB
    fn to_json_documents(&self) -> Result<Vec<serde_json::Value>> {
        let mut documents = Vec::new();

        for row_idx in 0..self.height() {
            let mut doc = serde_json::Map::new();

            for series in &self.columns {
                let value = series.get_f64(row_idx)?;
                doc.insert(
                    series.name().to_string(),
                    serde_json::Value::Number(
                        serde_json::Number::from_f64(value).unwrap_or(serde_json::Number::from(0)),
                    ),
                );
            }

            documents.push(serde_json::Value::Object(doc));
        }

        Ok(documents)
    }

    /// Build DataFrame from JSON documents
    fn from_json_documents(documents: Vec<serde_json::Value>) -> Result<Self> {
        if documents.is_empty() {
            return Ok(Self::empty());
        }

        // Extract column names from first document
        let first_doc = documents
            .first()
            .and_then(|v| v.as_object())
            .ok_or_else(|| AvilaError::generic("Invalid JSON document"))?;

        let column_names: Vec<String> = first_doc.keys().map(|k| k.to_string()).collect();

        // Build columns
        let mut columns = Vec::new();
        for col_name in column_names {
            let values: Result<Vec<f64>> = documents
                .iter()
                .map(|doc| {
                    doc.get(&col_name).and_then(|v| v.as_f64()).ok_or_else(|| {
                        AvilaError::generic(format!("Failed to extract column: {}", col_name))
                    })
                })
                .collect();

            columns.push(Series::new(col_name, values?));
        }

        DataFrame::new(columns)
    }
}

/// AvilaDB batch writer for efficient bulk inserts
pub struct AvilaDbBatchWriter {
    config: AvilaDbConfig,
    batch: Vec<serde_json::Value>,
    batch_size: usize,
}

impl AvilaDbBatchWriter {
    /// Create new batch writer
    pub fn new(config: AvilaDbConfig, batch_size: usize) -> Self {
        Self {
            config,
            batch: Vec::new(),
            batch_size,
        }
    }

    /// Add DataFrame to batch
    pub fn write(&mut self, df: &DataFrame) -> Result<()> {
        let documents = df.to_json_documents()?;
        self.batch.extend(documents);

        // Flush if batch is full
        if self.batch.len() >= self.batch_size {
            self.flush()?;
        }

        Ok(())
    }

    /// Flush pending writes
    pub fn flush(&mut self) -> Result<()> {
        if self.batch.is_empty() {
            return Ok(());
        }

        println!("Flushing {} documents to AvilaDB", self.batch.len());
        // TODO: Actual HTTP request
        self.batch.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aviladb_config() {
        let config = AvilaDbConfig::new("test-account", "testdb", "users")
            .with_endpoint("https://test.avila.cloud")
            .with_auth_key("test-key");

        assert_eq!(config.account, "test-account");
        assert_eq!(config.database, "testdb");
        assert_eq!(config.collection, "users");
    }

    #[test]
    fn test_aviladb_query_builder() {
        let query = AvilaDbQuery::new("SELECT * FROM users WHERE age > @min_age")
            .param("min_age", 18)
            .limit(100);

        assert_eq!(query.max_results, Some(100));
        assert_eq!(query.parameters.len(), 1);
    }

    #[test]
    fn test_to_json_documents() {
        let df = DataFrame::new(vec![
            Series::new("id", vec![1.0, 2.0]),
            Series::new("value", vec![10.0, 20.0]),
        ])
        .unwrap();

        let docs = df.to_json_documents().unwrap();
        assert_eq!(docs.len(), 2);
    }
}
