//! Storage layer using RocksDB

use rocksdb::{DB, Options, WriteBatch};
use std::path::Path;
use std::sync::Arc;

use crate::error::{AvilaError, Result};

/// Storage backend for AvilaDB
pub struct Storage {
    db: Arc<DB>,
}

impl Storage {
    /// Open a new storage instance
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_compression_type(rocksdb::DBCompressionType::Lz4);

        let db = DB::open(&opts, path)?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Put a key-value pair
    pub fn put(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.db.put(key, value)?;
        Ok(())
    }

    /// Get a value by key
    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?)
    }

    /// Delete a key
    pub fn delete(&self, key: &[u8]) -> Result<()> {
        self.db.delete(key)?;
        Ok(())
    }

    /// Check if key exists
    pub fn exists(&self, key: &[u8]) -> Result<bool> {
        Ok(self.db.get(key)?.is_some())
    }

    /// Batch write operations
    pub fn write_batch(&self, batch: WriteBatch) -> Result<()> {
        self.db.write(batch)?;
        Ok(())
    }

    /// Create a new write batch
    pub fn create_batch(&self) -> WriteBatch {
        WriteBatch::default()
    }

    /// Flush data to disk
    pub fn flush(&self) -> Result<()> {
        self.db.flush()?;
        Ok(())
    }

    /// Get approximate size of database
    pub fn size_on_disk(&self) -> Result<u64> {
        // Placeholder - would need to walk directory
        Ok(0)
    }
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_storage_basic_ops() {
        let dir = tempdir().unwrap();
        let storage = Storage::open(dir.path()).unwrap();

        // Put
        storage.put(b"key1", b"value1").unwrap();

        // Get
        let value = storage.get(b"key1").unwrap();
        assert_eq!(value, Some(b"value1".to_vec()));

        // Delete
        storage.delete(b"key1").unwrap();
        let value = storage.get(b"key1").unwrap();
        assert_eq!(value, None);
    }

    #[test]
    fn test_storage_batch() {
        let dir = tempdir().unwrap();
        let storage = Storage::open(dir.path()).unwrap();

        let mut batch = storage.create_batch();
        batch.put(b"key1", b"value1");
        batch.put(b"key2", b"value2");

        storage.write_batch(batch).unwrap();

        assert_eq!(storage.get(b"key1").unwrap(), Some(b"value1".to_vec()));
        assert_eq!(storage.get(b"key2").unwrap(), Some(b"value2".to_vec()));
    }
}
