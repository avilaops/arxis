//! Sistema de cache para otimização de consultas espaciais
//!
//! Este módulo implementa um cache LRU (Least Recently Used) para:
//! - Resultados de consultas espaciais
//! - Cálculos de distância
//! - Interpolações
//! - Transformações de coordenadas

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

/// Cache LRU genérico para armazenar resultados de cálculos custosos
pub struct LRUCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    capacity: usize,
    cache: HashMap<K, V>,
    order: VecDeque<K>,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Cria um novo cache com capacidade especificada
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            cache: HashMap::new(),
            order: VecDeque::new(),
        }
    }

    /// Obtém um valor do cache
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.cache.get(key) {
            // Move para o final (mais recente)
            self.order.retain(|k| k != key);
            self.order.push_back(key.clone());
            Some(value.clone())
        } else {
            None
        }
    }

    /// Insere um valor no cache
    pub fn put(&mut self, key: K, value: V) {
        // Se já existe, atualiza
        if self.cache.contains_key(&key) {
            self.cache.insert(key.clone(), value);
            self.order.retain(|k| k != &key);
            self.order.push_back(key);
            return;
        }

        // Se cache cheio, remove o mais antigo
        if self.cache.len() >= self.capacity {
            if let Some(oldest) = self.order.pop_front() {
                self.cache.remove(&oldest);
            }
        }

        // Insere novo valor
        self.cache.insert(key.clone(), value);
        self.order.push_back(key);
    }

    /// Limpa o cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    /// Retorna o número de elementos no cache
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Verifica se o cache está vazio
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Retorna a capacidade do cache
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Retorna a taxa de ocupação do cache (0.0 a 1.0)
    pub fn fill_rate(&self) -> f64 {
        self.cache.len() as f64 / self.capacity as f64
    }
}

/// Estatísticas de uso do cache
#[derive(Debug, Clone, Default)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
    pub evictions: usize,
}

impl CacheStats {
    /// Taxa de acerto do cache (hit rate)
    pub fn hit_rate(&self) -> f64 {
        let total = (self.hits + self.misses) as f64;
        if total > 0.0 {
            self.hits as f64 / total
        } else {
            0.0
        }
    }

    /// Reseta as estatísticas
    pub fn reset(&mut self) {
        self.hits = 0;
        self.misses = 0;
        self.evictions = 0;
    }
}

/// Cache LRU com estatísticas de uso
pub struct LRUCacheWithStats<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    cache: LRUCache<K, V>,
    stats: CacheStats,
}

impl<K, V> LRUCacheWithStats<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    /// Cria um novo cache com estatísticas
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LRUCache::new(capacity),
            stats: CacheStats::default(),
        }
    }

    /// Obtém um valor do cache (com tracking de estatísticas)
    pub fn get(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.cache.get(key) {
            self.stats.hits += 1;
            Some(value)
        } else {
            self.stats.misses += 1;
            None
        }
    }

    /// Insere um valor no cache
    pub fn put(&mut self, key: K, value: V) {
        let was_full = self.cache.len() >= self.cache.capacity();
        self.cache.put(key, value);
        if was_full {
            self.stats.evictions += 1;
        }
    }

    /// Retorna as estatísticas atuais
    pub fn stats(&self) -> &CacheStats {
        &self.stats
    }

    /// Reseta as estatísticas
    pub fn reset_stats(&mut self) {
        self.stats.reset();
    }

    /// Limpa o cache e as estatísticas
    pub fn clear(&mut self) {
        self.cache.clear();
        self.stats.reset();
    }

    /// Retorna informações do cache
    pub fn info(&self) -> String {
        format!(
            "Cache: {}/{} ({:.1}% cheio) | Hit Rate: {:.1}% | Hits: {} | Misses: {} | Evictions: {}",
            self.cache.len(),
            self.cache.capacity(),
            self.cache.fill_rate() * 100.0,
            self.stats.hit_rate() * 100.0,
            self.stats.hits,
            self.stats.misses,
            self.stats.evictions
        )
    }
}

/// Cache especializado para distâncias geográficas
pub struct DistanceCache {
    cache: LRUCacheWithStats<(u64, u64), f64>,
}

impl DistanceCache {
    /// Cria um novo cache de distâncias
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: LRUCacheWithStats::new(capacity),
        }
    }

    /// Obtém uma distância do cache
    /// Retorna a distância entre os pontos identificados por ids
    pub fn get(&mut self, id1: u64, id2: u64) -> Option<f64> {
        // Garante ordem consistente (menor id primeiro)
        let key = if id1 < id2 {
            (id1, id2)
        } else {
            (id2, id1)
        };

        self.cache.get(&key)
    }

    /// Armazena uma distância no cache
    pub fn put(&mut self, id1: u64, id2: u64, distance: f64) {
        let key = if id1 < id2 {
            (id1, id2)
        } else {
            (id2, id1)
        };

        self.cache.put(key, distance);
    }

    /// Retorna as estatísticas do cache
    pub fn stats(&self) -> &CacheStats {
        self.cache.stats()
    }

    /// Limpa o cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Retorna informações do cache
    pub fn info(&self) -> String {
        self.cache.info()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache_basic() {
        let mut cache = LRUCache::new(3);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.put(3, "three");

        assert_eq!(cache.get(&1), Some("one"));
        assert_eq!(cache.get(&2), Some("two"));
        assert_eq!(cache.get(&3), Some("three"));
        assert_eq!(cache.len(), 3);
    }

    #[test]
    fn test_lru_cache_eviction() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.put(3, "three"); // Deve remover 1

        assert_eq!(cache.get(&1), None);
        assert_eq!(cache.get(&2), Some("two"));
        assert_eq!(cache.get(&3), Some("three"));
    }

    #[test]
    fn test_lru_cache_update() {
        let mut cache = LRUCache::new(2);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.get(&1); // Acessa 1, tornando-o mais recente
        cache.put(3, "three"); // Deve remover 2

        assert_eq!(cache.get(&1), Some("one"));
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&3), Some("three"));
    }

    #[test]
    fn test_cache_stats() {
        let mut cache = LRUCacheWithStats::new(3);

        cache.put(1, "one");
        cache.put(2, "two");

        cache.get(&1); // hit
        cache.get(&2); // hit
        cache.get(&3); // miss

        let stats = cache.stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate(), 2.0 / 3.0);
    }

    #[test]
    fn test_distance_cache() {
        let mut cache = DistanceCache::new(10);

        cache.put(1, 2, 100.0);
        cache.put(3, 4, 200.0);

        // Ordem dos ids não importa
        assert_eq!(cache.get(1, 2), Some(100.0));
        assert_eq!(cache.get(2, 1), Some(100.0));

        assert_eq!(cache.get(3, 4), Some(200.0));
        assert_eq!(cache.get(5, 6), None);

        let stats = cache.stats();
        assert_eq!(stats.hits, 3);
        assert_eq!(stats.misses, 1);
    }

    #[test]
    fn test_cache_fill_rate() {
        let mut cache = LRUCache::new(10);

        assert_eq!(cache.fill_rate(), 0.0);

        cache.put(1, "one");
        cache.put(2, "two");
        cache.put(3, "three");

        assert_eq!(cache.fill_rate(), 0.3);

        for i in 4..=10 {
            cache.put(i, "value");
        }

        assert_eq!(cache.fill_rate(), 1.0);
    }
}
