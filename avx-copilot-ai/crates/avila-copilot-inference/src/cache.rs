// KV cache for faster inference

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

/// Key-Value cache for transformer attention
pub struct KVCache {
    num_layers: usize,
    num_heads: usize,
    max_seq_len: usize,
    cache: RwLock<HashMap<CacheKey, CacheValue>>,
    hits: AtomicU64,
    misses: AtomicU64,
}

impl KVCache {
    pub fn new(num_layers: usize, num_heads: usize, max_seq_len: usize) -> Self {
        Self {
            num_layers,
            num_heads,
            max_seq_len,
            cache: RwLock::new(HashMap::new()),
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
        }
    }

    /// Get cached KV for layer and position
    pub fn get(&self, layer: usize, position: usize) -> Option<CacheValue> {
        let key = CacheKey { layer, position };

        let cache = self.cache.read().ok()?;
        if let Some(value) = cache.get(&key) {
            self.hits.fetch_add(1, Ordering::Relaxed);
            Some(value.clone())
        } else {
            self.misses.fetch_add(1, Ordering::Relaxed);
            None
        }
    }

    /// Store KV in cache
    pub fn put(&self, layer: usize, position: usize, value: CacheValue) {
        let key = CacheKey { layer, position };

        if let Ok(mut cache) = self.cache.write() {
            cache.insert(key, value);
        }
    }

    /// Clear cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.write() {
            cache.clear();
        }
    }

    /// Get cache size
    pub fn size(&self) -> usize {
        self.cache.read().map(|c| c.len()).unwrap_or(0)
    }

    /// Get cache hit rate
    pub fn hit_rate(&self) -> f64 {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let total = hits + misses;

        if total == 0 {
            return 0.0;
        }

        hits as f64 / total as f64
    }
}

/// Cache key
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CacheKey {
    layer: usize,
    position: usize,
}

/// Cache value (K and V tensors)
#[derive(Debug, Clone)]
pub struct CacheValue {
    pub k: Vec<f32>,
    pub v: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kv_cache() {
        let cache = KVCache::new(12, 12, 2048);

        let value = CacheValue {
            k: vec![1.0, 2.0, 3.0],
            v: vec![4.0, 5.0, 6.0],
        };

        cache.put(0, 0, value.clone());

        let retrieved = cache.get(0, 0);
        assert!(retrieved.is_some());

        let hit_rate = cache.hit_rate();
        assert!(hit_rate > 0.0);
    }

    #[test]
    fn test_cache_clear() {
        let cache = KVCache::new(12, 12, 2048);

        let value = CacheValue {
            k: vec![1.0],
            v: vec![2.0],
        };

        cache.put(0, 0, value);
        assert_eq!(cache.size(), 1);

        cache.clear();
        assert_eq!(cache.size(), 0);
    }
}
