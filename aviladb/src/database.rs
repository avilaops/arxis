//! Database operations

use std::sync::Arc;
use crate::{Collection, Config, Result};

/// Database handle for collections
#[derive(Debug, Clone)]
pub struct Database {
    name: String,
    config: Arc<Config>,
}

impl Database {
    pub(crate) fn new(name: String, config: Arc<Config>) -> Result<Self> {
        Ok(Self { name, config })
    }

    /// Get database name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get a collection handle
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use aviladb::{AvilaClient, Database};
    /// # async fn example(db: Database) -> aviladb::Result<()> {
    /// let players = db.collection("players").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn collection(&self, name: &str) -> Result<Collection> {
        Collection::new(name.to_string(), self.name.clone(), self.config.clone())
    }

    /// Create a new collection
    pub async fn create_collection(&self, name: &str) -> Result<Collection> {
        // TODO: Send CREATE COLLECTION request
        self.collection(name).await
    }

    /// List all collections
    pub async fn list_collections(&self) -> Result<Vec<String>> {
        // TODO: Send LIST COLLECTIONS request
        Ok(vec![])
    }

    /// Delete a collection
    pub async fn delete_collection(&self, _name: &str) -> Result<()> {
        // TODO: Send DELETE COLLECTION request
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_database_collection() {
        let config = Arc::new(Config::default());
        let db = Database::new("testdb".to_string(), config).unwrap();

        let collection = db.collection("users").await;
        assert!(collection.is_ok());
    }
}
