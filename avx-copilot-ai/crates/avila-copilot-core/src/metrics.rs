// Performance metrics tracking

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Performance metrics for Copilot
#[derive(Debug)]
pub struct CopilotMetrics {
    total_completions: AtomicU64,
    total_latency_ms: AtomicU64,
    cache_hits: AtomicU64,
    cache_misses: AtomicU64,
}

impl CopilotMetrics {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            total_completions: AtomicU64::new(0),
            total_latency_ms: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
        })
    }

    pub fn record_completion(&self, latency_ms: u64) {
        self.total_completions.fetch_add(1, Ordering::Relaxed);
        self.total_latency_ms.fetch_add(latency_ms, Ordering::Relaxed);
    }

    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn average_latency_ms(&self) -> f64 {
        let total = self.total_completions.load(Ordering::Relaxed);
        if total == 0 {
            return 0.0;
        }
        let latency = self.total_latency_ms.load(Ordering::Relaxed);
        latency as f64 / total as f64
    }

    pub fn cache_hit_rate(&self) -> f64 {
        let hits = self.cache_hits.load(Ordering::Relaxed);
        let misses = self.cache_misses.load(Ordering::Relaxed);
        let total = hits + misses;
        if total == 0 {
            return 0.0;
        }
        hits as f64 / total as f64
    }
}

impl Default for CopilotMetrics {
    fn default() -> Self {
        Self {
            total_completions: AtomicU64::new(0),
            total_latency_ms: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            cache_misses: AtomicU64::new(0),
        }
    }
}
