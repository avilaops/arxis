//! AI Metrics and Observability

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMetrics {
    pub total_queries: u64,
    pub successful_queries: u64,
    pub failed_queries: u64,
    pub blocked_queries: u64,
    pub avg_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub tokens_generated: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub embedding_operations: u64,
    pub rag_retrievals: u64,
}

impl Default for AIMetrics {
    fn default() -> Self {
        Self {
            total_queries: 0,
            successful_queries: 0,
            failed_queries: 0,
            blocked_queries: 0,
            avg_latency_ms: 0.0,
            p95_latency_ms: 0.0,
            p99_latency_ms: 0.0,
            tokens_generated: 0,
            cache_hits: 0,
            cache_misses: 0,
            embedding_operations: 0,
            rag_retrievals: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryMetric {
    pub duration: Duration,
    pub success: bool,
    pub blocked: bool,
    pub tokens: usize,
    pub cache_hit: bool,
}

pub struct AIMetricsCollector {
    metrics: Arc<Mutex<AIMetrics>>,
    latencies: Arc<Mutex<Vec<u64>>>,
    backend_stats: Arc<Mutex<HashMap<String, u64>>>,
}

impl AIMetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(AIMetrics::default())),
            latencies: Arc::new(Mutex::new(Vec::new())),
            backend_stats: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn record_query(&self, metric: QueryMetric) {
        let mut metrics = self.metrics.lock().unwrap();
        let mut latencies = self.latencies.lock().unwrap();

        metrics.total_queries += 1;

        if metric.blocked {
            metrics.blocked_queries += 1;
        } else if metric.success {
            metrics.successful_queries += 1;
        } else {
            metrics.failed_queries += 1;
        }

        let latency_ms = metric.duration.as_millis() as u64;
        latencies.push(latency_ms);

        if latencies.len() > 1000 {
            latencies.remove(0);
        }

        let sum: u64 = latencies.iter().sum();
        metrics.avg_latency_ms = sum as f64 / latencies.len() as f64;

        let mut sorted = latencies.clone();
        sorted.sort_unstable();
        if !sorted.is_empty() {
            let p95_idx = (sorted.len() as f64 * 0.95) as usize;
            let p99_idx = (sorted.len() as f64 * 0.99) as usize;
            metrics.p95_latency_ms = sorted[p95_idx.min(sorted.len() - 1)] as f64;
            metrics.p99_latency_ms = sorted[p99_idx.min(sorted.len() - 1)] as f64;
        }

        metrics.tokens_generated += metric.tokens as u64;

        if metric.cache_hit {
            metrics.cache_hits += 1;
        } else {
            metrics.cache_misses += 1;
        }
    }

    pub fn record_embedding(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.embedding_operations += 1;
    }

    pub fn record_rag_retrieval(&self) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.rag_retrievals += 1;
    }

    pub fn record_backend_usage(&self, backend: &str) {
        let mut stats = self.backend_stats.lock().unwrap();
        *stats.entry(backend.to_string()).or_insert(0) += 1;
    }

    pub fn get_metrics(&self) -> AIMetrics {
        self.metrics.lock().unwrap().clone()
    }

    pub fn get_backend_stats(&self) -> HashMap<String, u64> {
        self.backend_stats.lock().unwrap().clone()
    }

    pub fn reset(&self) {
        *self.metrics.lock().unwrap() = AIMetrics::default();
        self.latencies.lock().unwrap().clear();
        self.backend_stats.lock().unwrap().clear();
    }
}

impl Default for AIMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector() {
        let collector = AIMetricsCollector::new();

        collector.record_query(QueryMetric {
            duration: Duration::from_millis(100),
            success: true,
            blocked: false,
            tokens: 50,
            cache_hit: false,
        });

        let metrics = collector.get_metrics();
        assert_eq!(metrics.total_queries, 1);
        assert_eq!(metrics.successful_queries, 1);
        assert_eq!(metrics.tokens_generated, 50);
    }

    #[test]
    fn test_latency_percentiles() {
        let collector = AIMetricsCollector::new();

        for i in 1..=100 {
            collector.record_query(QueryMetric {
                duration: Duration::from_millis(i),
                success: true,
                blocked: false,
                tokens: 10,
                cache_hit: false,
            });
        }

        let metrics = collector.get_metrics();
        assert!(metrics.avg_latency_ms > 0.0);
        assert!(metrics.p95_latency_ms >= 95.0);
        assert!(metrics.p99_latency_ms >= 99.0);
    }

    #[test]
    fn test_backend_stats() {
        let collector = AIMetricsCollector::new();
        collector.record_backend_usage("pattern");
        collector.record_backend_usage("pattern");
        collector.record_backend_usage("local");

        let stats = collector.get_backend_stats();
        assert_eq!(stats.get("pattern"), Some(&2));
        assert_eq!(stats.get("local"), Some(&1));
    }
}
