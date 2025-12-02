// Layer 1: Model Storage
// High-performance local model storage with compression and caching

use avila_compress::{compress, decompress, CompressionAlgorithm};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub mod error;
pub mod loader;
pub mod storage;
mod cache;

pub use error::{Result, StorageError};
use cache::LruCache;

/// Model storage system with compression and caching
pub struct ModelStorage {
    base_path: PathBuf,
    cache: Arc<Mutex<LruCache<String, Arc<Vec<u8>>>>>,
    compression: CompressionAlgorithm,
}

impl ModelStorage {
    /// Create new model storage
    pub async fn new(base_path: &str, cache_size_mb: usize) -> Result<Self> {
        let base_path = PathBuf::from(base_path);

        // Create directory if it doesn't exist
        if !base_path.exists() {
            std::fs::create_dir_all(&base_path)?;
        }

        let cache_capacity = (cache_size_mb * 1024 * 1024) / 1024; // Approximate number of items
        let cache = Arc::new(Mutex::new(LruCache::new(cache_capacity)));

        Ok(Self {
            base_path,
            cache,
            compression: CompressionAlgorithm::Lz4,
        })
    }

    /// Load model weights
    pub async fn load_model(&self, model_name: &str) -> Result<Arc<Vec<u8>>> {
        // Check cache first
        {
            let mut cache = self.cache.lock().unwrap();
            if let Some(cached) = cache.get(&model_name.to_string()) {
                return Ok(Arc::clone(cached));
            }
        }

        // Load from disk
        let model_path = self.base_path.join(format!("{}.bin.lz4", model_name));

        if !model_path.exists() {
            return Err(StorageError::ModelNotFound(model_name.to_string()));
        }

        let compressed_data = std::fs::read(&model_path)?;
        let data = decompress(&compressed_data)?;

        let data = Arc::new(data);

        // Cache for future use
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(model_name.to_string(), Arc::clone(&data));
        }

        Ok(data)
    }

    /// Save model weights
    pub async fn save_model(&self, model_name: &str, data: &[u8]) -> Result<()> {
        let compressed = compress(data)?;

        let model_path = self.base_path.join(format!("{}.bin.lz4", model_name));
        std::fs::write(&model_path, &compressed)?;

        // Update cache
        {
            let mut cache = self.cache.lock().unwrap();
            cache.insert(model_name.to_string(), Arc::new(data.to_vec()));
        }

        Ok(())
    }

    /// Get model metadata
    pub async fn get_metadata(&self, model_name: &str) -> Result<ModelMetadata> {
        let model_path = self.base_path.join(format!("{}.bin.lz4", model_name));

        if !model_path.exists() {
            return Err(StorageError::ModelNotFound(model_name.to_string()));
        }

        let metadata = std::fs::metadata(&model_path)?;
        let compressed_size = metadata.len();

        Ok(ModelMetadata {
            name: model_name.to_string(),
            compressed_size,
            compression: self.compression,
        })
    }

    /// List all available models
    pub fn list_models(&self) -> Result<Vec<String>> {
        let mut models = Vec::new();

        for entry in std::fs::read_dir(&self.base_path)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(ext) = path.extension() {
                if ext == "lz4" {
                    if let Some(stem) = path.file_stem() {
                        if let Some(name) = stem.to_str() {
                            // Remove .bin extension
                            let name = name.trim_end_matches(".bin");
                            models.push(name.to_string());
                        }
                    }
                }
            }
        }

        Ok(models)
    }

    /// Clear cache
    pub fn clear_cache(&self) {
        let mut cache = self.cache.lock().unwrap();
        *cache = LruCache::new(cache.capacity());
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        let cache = self.cache.lock().unwrap();
        let (hits, misses, hit_rate) = cache.stats();
        CacheStats {
            size: cache.map.len(),
            hit_rate,
        }
    }
}

/// Model metadata
#[derive(Debug, Clone)]
pub struct ModelMetadata {
    pub name: String,
    pub compressed_size: u64,
    pub compression: CompressionAlgorithm,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub size: usize,
    pub hit_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_model_storage_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let storage = ModelStorage::new(temp_dir.path().to_str().unwrap(), 128)
            .await
            .unwrap();

        let test_data = vec![1, 2, 3, 4, 5];
        storage.save_model("test_model", &test_data).await.unwrap();

        let loaded = storage.load_model("test_model").await.unwrap();
        assert_eq!(*loaded, test_data);
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        let temp_dir = TempDir::new().unwrap();
        let storage = ModelStorage::new(temp_dir.path().to_str().unwrap(), 128)
            .await
            .unwrap();

        let test_data = vec![1, 2, 3, 4, 5];
        storage.save_model("test_model", &test_data).await.unwrap();

        // First load - cache miss
        let _ = storage.load_model("test_model").await.unwrap();

        // Second load - cache hit
        let loaded = storage.load_model("test_model").await.unwrap();
        assert_eq!(*loaded, test_data);

        let stats = storage.cache_stats();
        assert!(stats.hit_rate > 0.0);
    }
}
