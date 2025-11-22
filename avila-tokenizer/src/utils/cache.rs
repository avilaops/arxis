use lru::LruCache;
use std::hash::Hash;
use std::num::NonZeroUsize;

/// Thread-safe LRU cache wrapper for tokenization results
#[derive(Debug, Clone)]
pub struct TokenCache<K, V>
where
    K: Hash + Eq,
{
    cache: LruCache<K, V>,
}

impl<K, V> TokenCache<K, V>
where
    K: Hash + Eq,
{
    /// Create a new cache with specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LruCache::new(NonZeroUsize::new(capacity).unwrap()),
        }
    }

    /// Get a value from the cache
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.cache.get(key)
    }

    /// Insert a value into the cache
    pub fn put(&mut self, key: K, value: V) {
        self.cache.put(key, value);
    }

    /// Check if cache contains a key
    pub fn contains(&self, key: &K) -> bool {
        self.cache.contains(key)
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Get cache capacity
    pub fn cap(&self) -> usize {
        self.cache.cap().get()
    }
}

impl<K, V> Default for TokenCache<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::new(10_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let mut cache = TokenCache::new(3);

        cache.put("hello", vec![1, 2, 3]);
        cache.put("world", vec![4, 5, 6]);

        assert_eq!(cache.get(&"hello"), Some(&vec![1, 2, 3]));
        assert_eq!(cache.get(&"world"), Some(&vec![4, 5, 6]));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_cache_eviction() {
        let mut cache = TokenCache::new(2);

        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3); // Should evict "a"

        assert_eq!(cache.get(&"a"), None);
        assert_eq!(cache.get(&"b"), Some(&2));
        assert_eq!(cache.get(&"c"), Some(&3));
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = TokenCache::new(10);

        cache.put("test", 123);
        assert_eq!(cache.len(), 1);

        cache.clear();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }
}
