//! Health check endpoints and monitoring

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Health status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthStatus {
    /// Overall status
    pub status: String,

    /// Service version
    pub version: String,

    /// Uptime in seconds
    pub uptime: u64,

    /// Upstream services status
    pub upstreams: Vec<UpstreamHealth>,
}

/// Health status of an upstream service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpstreamHealth {
    /// Service name/URL
    pub name: String,

    /// Health status
    pub status: ServiceStatus,

    /// Last check timestamp
    pub last_check: Option<u64>,

    /// Response time in milliseconds
    pub response_time_ms: Option<u64>,
}

/// Service health status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    /// Service is healthy
    Healthy,

    /// Service is degraded
    Degraded,

    /// Service is unhealthy
    Unhealthy,

    /// Service status unknown
    Unknown,
}

/// Health checker
#[derive(Clone)]
pub struct HealthChecker {
    start_time: std::time::Instant,
    upstreams: Arc<RwLock<Vec<UpstreamHealth>>>,
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            upstreams: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Register an upstream service
    pub async fn register_upstream(&self, name: String) {
        let mut upstreams = self.upstreams.write().await;
        upstreams.push(UpstreamHealth {
            name,
            status: ServiceStatus::Unknown,
            last_check: None,
            response_time_ms: None,
        });
    }

    /// Update upstream health status
    pub async fn update_upstream_health(
        &self,
        name: &str,
        status: ServiceStatus,
        response_time_ms: Option<u64>,
    ) {
        let mut upstreams = self.upstreams.write().await;
        if let Some(upstream) = upstreams.iter_mut().find(|u| u.name == name) {
            upstream.status = status;
            upstream.last_check = Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            );
            upstream.response_time_ms = response_time_ms;
        }
    }

    /// Get overall health status
    pub async fn get_health(&self) -> HealthStatus {
        let upstreams = self.upstreams.read().await.clone();

        let overall_status = if upstreams.iter().all(|u| u.status == ServiceStatus::Healthy) {
            "healthy".to_string()
        } else if upstreams.iter().any(|u| u.status == ServiceStatus::Unhealthy) {
            "unhealthy".to_string()
        } else {
            "degraded".to_string()
        };

        HealthStatus {
            status: overall_status,
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime: self.start_time.elapsed().as_secs(),
            upstreams,
        }
    }

    /// Check if the service is ready
    pub async fn is_ready(&self) -> bool {
        let upstreams = self.upstreams.read().await;
        !upstreams.is_empty() && upstreams.iter().any(|u| u.status == ServiceStatus::Healthy)
    }
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

/// Health check handler
pub async fn health_handler(
    State(checker): State<HealthChecker>,
) -> impl IntoResponse {
    let health = checker.get_health().await;

    let status_code = match health.status.as_str() {
        "healthy" => StatusCode::OK,
        "degraded" => StatusCode::OK,
        "unhealthy" => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    };

    (status_code, Json(health))
}

/// Readiness check handler
pub async fn readiness_handler(
    State(checker): State<HealthChecker>,
) -> impl IntoResponse {
    if checker.is_ready().await {
        (
            StatusCode::OK,
            Json(serde_json::json!({"ready": true})),
        )
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({"ready": false})),
        )
    }
}

/// Liveness check handler (always returns OK if the service is running)
pub async fn liveness_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({"alive": true})),
    )
}
