// Simple LRU cache implementation
use std::collections::HashMap;
use std::hash::Hash;

pub struct LruCache<K, V> {
    pub(crate) map: HashMap<K, V>,
    capacity: usize,
    hits: u64,
    misses: u64,
}

impl<K: Eq + Hash + Clone, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            capacity,
            hits: 0,
            misses: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.map.get(key) {
            self.hits += 1;
            Some(value)
        } else {
            self.misses += 1;
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.map.len() >= self.capacity {
            // Simple eviction: remove first entry (not true LRU, but works)
            if let Some(k) = self.map.keys().next().cloned() {
                self.map.remove(&k);
            }
        }
        self.map.insert(key, value);
    }

    pub fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 {
            0.0
        } else {
            self.hits as f64 / total as f64
        }
    }

    pub fn stats(&self) -> (u64, u64, f64) {
        (self.hits, self.misses, self.hit_rate())
    }
}
