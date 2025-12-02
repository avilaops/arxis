//! Monitoring and observability

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

pub struct MonitoringService {
    http_requests: Arc<AtomicU64>,
    active_instances: Arc<AtomicU64>,
}

impl MonitoringService {
    pub fn new() -> Self {
        Self {
            http_requests: Arc::new(AtomicU64::new(0)),
            active_instances: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_request(&self) {
        self.http_requests.fetch_add(1, Ordering::Relaxed);
    }

    pub fn set_active_instances(&self, count: u64) {
        self.active_instances.store(count, Ordering::Relaxed);
    }

    pub fn get_metrics(&self) -> Metrics {
        Metrics {
            http_requests_total: self.http_requests.load(Ordering::Relaxed),
            active_instances: self.active_instances.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct Metrics {
    pub http_requests_total: u64,
    pub active_instances: u64,
}
