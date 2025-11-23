//! Metrics collection and monitoring

use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// Gateway metrics
#[derive(Clone)]
pub struct GatewayMetrics {
    /// Total number of requests
    total_requests: Arc<AtomicU64>,

    /// Total number of successful requests (2xx)
    successful_requests: Arc<AtomicU64>,

    /// Total number of client errors (4xx)
    client_errors: Arc<AtomicU64>,

    /// Total number of server errors (5xx)
    server_errors: Arc<AtomicU64>,

    /// Total request duration in milliseconds
    total_duration_ms: Arc<AtomicU64>,

    /// Number of active connections
    active_connections: Arc<AtomicU64>,

    /// Total bytes sent
    bytes_sent: Arc<AtomicU64>,

    /// Total bytes received
    bytes_received: Arc<AtomicU64>,
}

impl GatewayMetrics {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            total_requests: Arc::new(AtomicU64::new(0)),
            successful_requests: Arc::new(AtomicU64::new(0)),
            client_errors: Arc::new(AtomicU64::new(0)),
            server_errors: Arc::new(AtomicU64::new(0)),
            total_duration_ms: Arc::new(AtomicU64::new(0)),
            active_connections: Arc::new(AtomicU64::new(0)),
            bytes_sent: Arc::new(AtomicU64::new(0)),
            bytes_received: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Record a request
    pub fn record_request(&self, status_code: u16, duration_ms: u64) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.total_duration_ms.fetch_add(duration_ms, Ordering::Relaxed);

        match status_code {
            200..=299 => {
                self.successful_requests.fetch_add(1, Ordering::Relaxed);
            }
            400..=499 => {
                self.client_errors.fetch_add(1, Ordering::Relaxed);
            }
            500..=599 => {
                self.server_errors.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }
    }

    /// Increment active connections
    pub fn increment_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
    }

    /// Decrement active connections
    pub fn decrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Record bytes sent
    pub fn record_bytes_sent(&self, bytes: u64) {
        self.bytes_sent.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Record bytes received
    pub fn record_bytes_received(&self, bytes: u64) {
        self.bytes_received.fetch_add(bytes, Ordering::Relaxed);
    }

    /// Get metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        let total = self.total_requests.load(Ordering::Relaxed);
        let successful = self.successful_requests.load(Ordering::Relaxed);
        let client_errors = self.client_errors.load(Ordering::Relaxed);
        let server_errors = self.server_errors.load(Ordering::Relaxed);
        let total_duration = self.total_duration_ms.load(Ordering::Relaxed);

        let avg_duration = if total > 0 {
            total_duration / total
        } else {
            0
        };

        let success_rate = if total > 0 {
            (successful as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        MetricsSnapshot {
            total_requests: total,
            successful_requests: successful,
            client_errors,
            server_errors,
            success_rate,
            avg_duration_ms: avg_duration,
            active_connections: self.active_connections.load(Ordering::Relaxed),
            bytes_sent: self.bytes_sent.load(Ordering::Relaxed),
            bytes_received: self.bytes_received.load(Ordering::Relaxed),
        }
    }

    /// Get Prometheus-format metrics
    pub fn prometheus_format(&self) -> String {
        let snapshot = self.snapshot();
        format!(
            "# HELP gateway_requests_total Total number of requests\n\
             # TYPE gateway_requests_total counter\n\
             gateway_requests_total {}\n\
             \n\
             # HELP gateway_requests_successful Successful requests (2xx)\n\
             # TYPE gateway_requests_successful counter\n\
             gateway_requests_successful {}\n\
             \n\
             # HELP gateway_requests_client_errors Client errors (4xx)\n\
             # TYPE gateway_requests_client_errors counter\n\
             gateway_requests_client_errors {}\n\
             \n\
             # HELP gateway_requests_server_errors Server errors (5xx)\n\
             # TYPE gateway_requests_server_errors counter\n\
             gateway_requests_server_errors {}\n\
             \n\
             # HELP gateway_request_duration_ms Average request duration in milliseconds\n\
             # TYPE gateway_request_duration_ms gauge\n\
             gateway_request_duration_ms {}\n\
             \n\
             # HELP gateway_active_connections Current active connections\n\
             # TYPE gateway_active_connections gauge\n\
             gateway_active_connections {}\n\
             \n\
             # HELP gateway_bytes_sent Total bytes sent\n\
             # TYPE gateway_bytes_sent counter\n\
             gateway_bytes_sent {}\n\
             \n\
             # HELP gateway_bytes_received Total bytes received\n\
             # TYPE gateway_bytes_received counter\n\
             gateway_bytes_received {}\n",
            snapshot.total_requests,
            snapshot.successful_requests,
            snapshot.client_errors,
            snapshot.server_errors,
            snapshot.avg_duration_ms,
            snapshot.active_connections,
            snapshot.bytes_sent,
            snapshot.bytes_received,
        )
    }
}

impl Default for GatewayMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of current metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    /// Total requests
    pub total_requests: u64,

    /// Successful requests
    pub successful_requests: u64,

    /// Client errors
    pub client_errors: u64,

    /// Server errors
    pub server_errors: u64,

    /// Success rate percentage
    pub success_rate: f64,

    /// Average request duration in milliseconds
    pub avg_duration_ms: u64,

    /// Active connections
    pub active_connections: u64,

    /// Bytes sent
    pub bytes_sent: u64,

    /// Bytes received
    pub bytes_received: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_recording() {
        let metrics = GatewayMetrics::new();

        metrics.record_request(200, 10);
        metrics.record_request(201, 20);
        metrics.record_request(404, 5);
        metrics.record_request(500, 100);

        let snapshot = metrics.snapshot();

        assert_eq!(snapshot.total_requests, 4);
        assert_eq!(snapshot.successful_requests, 2);
        assert_eq!(snapshot.client_errors, 1);
        assert_eq!(snapshot.server_errors, 1);
        assert_eq!(snapshot.success_rate, 50.0);
    }

    #[test]
    fn test_connections() {
        let metrics = GatewayMetrics::new();

        metrics.increment_connections();
        metrics.increment_connections();
        metrics.increment_connections();

        assert_eq!(metrics.snapshot().active_connections, 3);

        metrics.decrement_connections();
        assert_eq!(metrics.snapshot().active_connections, 2);
    }
}
