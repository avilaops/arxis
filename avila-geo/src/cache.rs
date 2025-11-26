//! Caching system for tiles and expensive computations
//!
//! Provides LRU cache for tiles, projections, and other expensive operations.

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

/// LRU Cache with fixed capacity
pub struct LruCache<K, V> {
    capacity: usize,
    cache: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K: Clone + Eq + Hash, V> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache: HashMap::with_capacity(capacity),
            order: VecDeque::with_capacity(capacity),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        if self.cache.contains_key(key) {
            // Move to front (most recently used)
            self.order.retain(|k| k != key);
            self.order.push_front(key.clone());
            self.cache.get(key)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        if self.cache.contains_key(&key) {
            // Update existing
            self.order.retain(|k| k != &key);
            self.cache.insert(key.clone(), value);
            self.order.push_front(key);
        } else {
            // Add new
            if self.cache.len() >= self.capacity {
                // Evict least recently used
                if let Some(old_key) = self.order.pop_back() {
                    self.cache.remove(&old_key);
                }
            }
            self.cache.insert(key.clone(), value);
            self.order.push_front(key);
        }
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    pub fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Thread-safe LRU cache
pub struct ConcurrentCache<K, V> {
    cache: Arc<Mutex<LruCache<K, V>>>,
}

impl<K: Clone + Eq + Hash, V: Clone> ConcurrentCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(LruCache::new(capacity))),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        self.cache.lock().unwrap().get(key).cloned()
    }

    pub fn insert(&self, key: K, value: V) {
        self.cache.lock().unwrap().insert(key, value);
    }

    pub fn len(&self) -> usize {
        self.cache.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.lock().unwrap().is_empty()
    }

    pub fn clear(&self) {
        self.cache.lock().unwrap().clear();
    }
}

impl<K: Clone + Eq + Hash, V: Clone> Clone for ConcurrentCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            cache: Arc::clone(&self.cache),
        }
    }
}

/// Tile cache for web mapping
pub mod tile_cache {
    use super::*;
    use crate::tiles::TileCoord;

    /// Cache for tile data (URLs, images, etc.)
    pub struct TileCache<T> {
        cache: ConcurrentCache<TileCoord, T>,
    }

    impl<T: Clone> TileCache<T> {
        /// Create tile cache with capacity (number of tiles)
        pub fn new(capacity: usize) -> Self {
            Self {
                cache: ConcurrentCache::new(capacity),
            }
        }

        /// Get tile from cache
        pub fn get(&self, tile: &TileCoord) -> Option<T> {
            self.cache.get(tile)
        }

        /// Insert tile into cache
        pub fn insert(&self, tile: TileCoord, data: T) {
            self.cache.insert(tile, data);
        }

        /// Get or compute tile
        pub fn get_or_insert_with<F>(&self, tile: TileCoord, f: F) -> T
        where
            F: FnOnce() -> T,
        {
            if let Some(data) = self.get(&tile) {
                data
            } else {
                let data = f();
                self.insert(tile, data.clone());
                data
            }
        }

        pub fn len(&self) -> usize {
            self.cache.len()
        }

        pub fn is_empty(&self) -> bool {
            self.cache.is_empty()
        }

        pub fn clear(&self) {
            self.cache.clear();
        }

        /// Estimate memory usage (assumes 256KB per tile average)
        pub fn estimated_memory_mb(&self) -> f64 {
            self.len() as f64 * 256.0 / 1024.0
        }
    }

    impl<T: Clone> Default for TileCache<T> {
        fn default() -> Self {
            Self::new(100) // Default 100 tiles (~25MB)
        }
    }
}

/// Projection result cache
pub mod projection_cache {
    use super::*;
    use crate::coords::{CartesianCoord, GeoCoord};

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct ProjectionKey {
        lat_micro: i32,  // Latitude * 1,000,000
        lon_micro: i32,  // Longitude * 1,000,000
        width: u32,
        height: u32,
        projection_id: u8,
    }

    impl ProjectionKey {
        fn from_geo(coord: &GeoCoord, width: u32, height: u32, projection_id: u8) -> Self {
            Self {
                lat_micro: (coord.lat * 1_000_000.0) as i32,
                lon_micro: (coord.lon * 1_000_000.0) as i32,
                width,
                height,
                projection_id,
            }
        }
    }

    /// Cache for projected coordinates
    pub struct ProjectionCache {
        cache: ConcurrentCache<ProjectionKey, CartesianCoord>,
    }

    impl ProjectionCache {
        /// Create projection cache with capacity
        pub fn new(capacity: usize) -> Self {
            Self {
                cache: ConcurrentCache::new(capacity),
            }
        }

        /// Get projected coordinate from cache
        pub fn get(
            &self,
            coord: &GeoCoord,
            width: u32,
            height: u32,
            projection_id: u8,
        ) -> Option<CartesianCoord> {
            let key = ProjectionKey::from_geo(coord, width, height, projection_id);
            self.cache.get(&key)
        }

        /// Insert projected coordinate
        pub fn insert(
            &self,
            coord: &GeoCoord,
            width: u32,
            height: u32,
            projection_id: u8,
            result: CartesianCoord,
        ) {
            let key = ProjectionKey::from_geo(coord, width, height, projection_id);
            self.cache.insert(key, result);
        }

        pub fn len(&self) -> usize {
            self.cache.len()
        }

        pub fn is_empty(&self) -> bool {
            self.cache.is_empty()
        }

        pub fn clear(&self) {
            self.cache.clear();
        }
    }

    impl Default for ProjectionCache {
        fn default() -> Self {
            Self::new(10_000) // Cache 10k projections
        }
    }
}

/// Distance calculation cache
pub mod distance_cache {
    use super::*;
    use crate::coords::GeoCoord;

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    struct DistanceKey {
        lat1_micro: i32,
        lon1_micro: i32,
        lat2_micro: i32,
        lon2_micro: i32,
    }

    impl DistanceKey {
        fn from_coords(from: &GeoCoord, to: &GeoCoord) -> Self {
            // Normalize order (distance is symmetric)
            let (c1, c2) = if from.lat < to.lat || (from.lat == to.lat && from.lon < to.lon) {
                (from, to)
            } else {
                (to, from)
            };

            Self {
                lat1_micro: (c1.lat * 1_000_000.0) as i32,
                lon1_micro: (c1.lon * 1_000_000.0) as i32,
                lat2_micro: (c2.lat * 1_000_000.0) as i32,
                lon2_micro: (c2.lon * 1_000_000.0) as i32,
            }
        }
    }

    /// Cache for distance calculations
    pub struct DistanceCache {
        cache: ConcurrentCache<DistanceKey, f64>,
    }

    impl DistanceCache {
        pub fn new(capacity: usize) -> Self {
            Self {
                cache: ConcurrentCache::new(capacity),
            }
        }

        pub fn get(&self, from: &GeoCoord, to: &GeoCoord) -> Option<f64> {
            let key = DistanceKey::from_coords(from, to);
            self.cache.get(&key)
        }

        pub fn insert(&self, from: &GeoCoord, to: &GeoCoord, distance: f64) {
            let key = DistanceKey::from_coords(from, to);
            self.cache.insert(key, distance);
        }

        /// Get or compute distance
        pub fn get_or_compute<F>(&self, from: &GeoCoord, to: &GeoCoord, f: F) -> f64
        where
            F: FnOnce() -> f64,
        {
            if let Some(dist) = self.get(from, to) {
                dist
            } else {
                let dist = f();
                self.insert(from, to, dist);
                dist
            }
        }

        pub fn len(&self) -> usize {
            self.cache.len()
        }

        pub fn is_empty(&self) -> bool {
            self.cache.is_empty()
        }

        pub fn clear(&self) {
            self.cache.clear();
        }
    }

    impl Default for DistanceCache {
        fn default() -> Self {
            Self::new(5_000)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = LruCache::new(3);

        cache.insert("a", 1);
        cache.insert("b", 2);
        cache.insert("c", 3);

        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"a"), Some(&1));

        // Insert 4th item, should evict "b" (least recently used)
        cache.insert("d", 4);
        assert_eq!(cache.len(), 3);
        assert_eq!(cache.get(&"b"), None);
        assert_eq!(cache.get(&"d"), Some(&4));
    }

    #[test]
    fn test_concurrent_cache() {
        let cache = ConcurrentCache::new(10);

        cache.insert("test", 42);
        assert_eq!(cache.get(&"test"), Some(42));

        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_tile_cache() {
        use crate::tiles::TileCoord;

        let cache = tile_cache::TileCache::new(5);
        let tile = TileCoord::new(10, 20, 5);

        cache.insert(tile, "tile_data");
        assert_eq!(cache.get(&tile), Some("tile_data"));
    }

    #[test]
    fn test_distance_cache() {
        use crate::coords::GeoCoord;

        let cache = distance_cache::DistanceCache::new(10);
        let sp = GeoCoord::new(-23.55, -46.63);
        let rio = GeoCoord::new(-22.91, -43.17);

        let dist = cache.get_or_compute(&sp, &rio, || 360_000.0);
        assert_eq!(dist, 360_000.0);

        // Should hit cache
        let dist2 = cache.get_or_compute(&sp, &rio, || 999.0);
        assert_eq!(dist2, 360_000.0); // Cached value
    }
}
