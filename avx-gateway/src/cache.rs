//! Caching layer for responses

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;

/// Cache entry
#[derive(Debug, Clone)]
struct CacheEntry {
    /// Response body
    body: Vec<u8>,

    /// Response status code
    status: u16,

    /// Response headers (serialized)
    headers: Vec<(String, String)>,

    /// Timestamp when entry was created
    created_at: Instant,

    /// Time-to-live
    ttl: Duration,
}

impl CacheEntry {
    /// Check if entry is expired
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

/// Cache strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CacheStrategy {
    /// Cache all successful responses
    All,

    /// Cache only GET requests
    GetOnly,

    /// Cache based on status codes
    StatusBased,

    /// No caching
    None,
}

/// Response cache
#[derive(Clone)]
pub struct ResponseCache {
    /// Cache storage
    cache: Arc<RwLock<HashMap<String, CacheEntry>>>,

    /// Default TTL
    default_ttl: Duration,

    /// Maximum cache size (number of entries)
    max_size: usize,

    /// Cache strategy
    strategy: CacheStrategy,
}

impl ResponseCache {
    /// Create a new response cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            default_ttl: Duration::from_secs(300), // 5 minutes
            max_size: 1000,
            strategy: CacheStrategy::GetOnly,
        }
    }

    /// Set default TTL
    pub fn with_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = ttl;
        self
    }

    /// Set maximum cache size
    pub fn with_max_size(mut self, max_size: usize) -> Self {
        self.max_size = max_size;
        self
    }

    /// Set cache strategy
    pub fn with_strategy(mut self, strategy: CacheStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Generate cache key
    fn cache_key(&self, method: &str, path: &str, query: Option<&str>) -> String {
        if let Some(q) = query {
            format!("{}:{}?{}", method, path, q)
        } else {
            format!("{}:{}", method, path)
        }
    }

    /// Check if request is cacheable
    pub fn is_cacheable(&self, method: &str, status: u16) -> bool {
        match self.strategy {
            CacheStrategy::None => false,
            CacheStrategy::GetOnly => method == "GET" && (200..300).contains(&status),
            CacheStrategy::All => (200..300).contains(&status),
            CacheStrategy::StatusBased => (200..300).contains(&status),
        }
    }

    /// Get cached response
    pub async fn get(
        &self,
        method: &str,
        path: &str,
        query: Option<&str>,
    ) -> Option<(u16, Vec<(String, String)>, Vec<u8>)> {
        let key = self.cache_key(method, path, query);
        let cache = self.cache.read().await;

        if let Some(entry) = cache.get(&key) {
            if !entry.is_expired() {
                return Some((entry.status, entry.headers.clone(), entry.body.clone()));
            }
        }

        None
    }

    /// Store response in cache
    pub async fn put(
        &self,
        method: &str,
        path: &str,
        query: Option<&str>,
        status: u16,
        headers: Vec<(String, String)>,
        body: Vec<u8>,
        ttl: Option<Duration>,
    ) {
        if !self.is_cacheable(method, status) {
            return;
        }

        let key = self.cache_key(method, path, query);
        let mut cache = self.cache.write().await;

        // Evict expired entries if cache is full
        if cache.len() >= self.max_size {
            cache.retain(|_, entry| !entry.is_expired());

            // If still full, remove oldest entries
            if cache.len() >= self.max_size {
                let keys_to_remove: Vec<_> = {
                    let mut entries: Vec<_> = cache.iter().collect();
                    entries.sort_by_key(|(_, entry)| entry.created_at);

                    let to_remove = cache.len() - self.max_size + 1;
                    entries.iter()
                        .take(to_remove)
                        .map(|(k, _)| k.clone())
                        .collect()
                };

                for key in keys_to_remove {
                    cache.remove(key.as_str());
                }
            }
        }

        let entry = CacheEntry {
            body,
            status,
            headers,
            created_at: Instant::now(),
            ttl: ttl.unwrap_or(self.default_ttl),
        };

        cache.insert(key, entry);
    }

    /// Invalidate cache entry
    pub async fn invalidate(&self, method: &str, path: &str, query: Option<&str>) {
        let key = self.cache_key(method, path, query);
        let mut cache = self.cache.write().await;
        cache.remove(&key);
    }

    /// Clear all cache
    pub async fn clear(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let cache = self.cache.read().await;
        let total_entries = cache.len();
        let expired_entries = cache.values().filter(|e| e.is_expired()).count();

        CacheStats {
            total_entries,
            expired_entries,
            active_entries: total_entries - expired_entries,
            max_size: self.max_size,
        }
    }
}

impl Default for ResponseCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    /// Total number of entries
    pub total_entries: usize,

    /// Number of expired entries
    pub expired_entries: usize,

    /// Number of active entries
    pub active_entries: usize,

    /// Maximum cache size
    pub max_size: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache_put_and_get() {
        let cache = ResponseCache::new();

        cache
            .put(
                "GET",
                "/api/test",
                None,
                200,
                vec![],
                b"test response".to_vec(),
                None,
            )
            .await;

        let result = cache.get("GET", "/api/test", None).await;
        assert!(result.is_some());

        let (status, _, body) = result.unwrap();
        assert_eq!(status, 200);
        assert_eq!(body, b"test response");
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = ResponseCache::new().with_ttl(Duration::from_millis(100));

        cache
            .put(
                "GET",
                "/api/test",
                None,
                200,
                vec![],
                b"test".to_vec(),
                None,
            )
            .await;

        // Should be cached
        assert!(cache.get("GET", "/api/test", None).await.is_some());

        // Wait for expiration
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Should be expired
        assert!(cache.get("GET", "/api/test", None).await.is_none());
    }
}
