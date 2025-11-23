//! Query operations

use std::collections::HashMap;
use serde_json::Value;

use crate::{Collection, Document, QueryResult, Result};

/// SQL-like query builder
pub struct Query {
    sql: String,
    collection: Collection,
    params: HashMap<String, Value>,
}

impl Query {
    pub(crate) fn new(sql: String, collection: Collection) -> Self {
        Self {
            sql,
            collection,
            params: HashMap::new(),
        }
    }

    /// Add a query parameter
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use aviladb::Collection;
    /// # async fn example(collection: Collection) -> aviladb::Result<()> {
    /// let results = collection
    ///     .query("SELECT * FROM users WHERE level > @min AND level < @max")
    ///     .param("min", 10)
    ///     .param("max", 50)
    ///     .execute()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn param<V: serde::Serialize>(mut self, name: &str, value: V) -> Self {
        let value_json = serde_json::to_value(value).expect("Failed to serialize parameter");
        self.params.insert(name.to_string(), value_json);
        self
    }

    /// Execute the query
    pub async fn execute(self) -> Result<QueryResult> {
        // TODO: Parse SQL
        // TODO: Send QUERY request
        // TODO: Decompress results

        Ok(QueryResult {
            documents: vec![],
            total_count: 0,
            continuation_token: None,
            latency_ms: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::Config;

    #[tokio::test]
    async fn test_query_builder() {
        let config = Arc::new(Config::default());
        let collection = Collection::new(
            "users".to_string(),
            "testdb".to_string(),
            config,
        ).unwrap();

        let query = collection
            .query("SELECT * FROM users WHERE level > @min")
            .param("min", 40);

        let result = query.execute().await;
        assert!(result.is_ok());
    }
}
