//! Event-driven HTTP middleware and integrations
//!
//! This module provides integration between avx-http and avx-events,
//! enabling event-driven HTTP architectures.

#[cfg(feature = "events")]
use avx_events::{Event, EventBus};
use crate::server::{Request, Response};
use crate::middleware::{Middleware, Next};
use crate::error::Result;
use async_trait::async_trait;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// HTTP request event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpRequestEvent {
    pub method: String,
    pub path: String,
    pub status_code: u16,
    pub duration_ms: u64,
    pub user_agent: Option<String>,
    pub remote_addr: Option<String>,
}

#[cfg(feature = "events")]
impl Event for HttpRequestEvent {
    fn event_type(&self) -> &'static str {
        "http.request.completed"
    }

    fn aggregate_id(&self) -> String {
        format!("http:{}:{}", self.method, self.path)
    }
}

/// HTTP error event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpErrorEvent {
    pub method: String,
    pub path: String,
    pub error: String,
    pub status_code: u16,
}

#[cfg(feature = "events")]
impl Event for HttpErrorEvent {
    fn event_type(&self) -> &'static str {
        "http.request.error"
    }

    fn aggregate_id(&self) -> String {
        format!("http:{}:{}", self.method, self.path)
    }
}

/// Middleware that publishes HTTP events to an event bus
#[cfg(feature = "events")]
pub struct EventPublisher {
    bus: Arc<EventBus>,
    include_successful: bool,
    include_errors: bool,
}

#[cfg(feature = "events")]
impl EventPublisher {
    /// Create a new event publisher middleware
    pub fn new(bus: Arc<EventBus>) -> Self {
        Self {
            bus,
            include_successful: true,
            include_errors: true,
        }
    }

    /// Only publish error events
    pub fn errors_only(mut self) -> Self {
        self.include_successful = false;
        self
    }

    /// Only publish successful request events
    pub fn successful_only(mut self) -> Self {
        self.include_errors = false;
        self
    }
}

#[cfg(feature = "events")]
#[async_trait]
impl Middleware for EventPublisher {
    async fn handle(&self, request: Request, next: Next) -> Result<Response> {
        let method = request.method.to_string();
        let path = request.path.clone();
        let user_agent = request
            .headers
            .get("user-agent")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let start = std::time::Instant::now();

        // Execute request
        let result = next.run(request).await;

        let duration_ms = start.elapsed().as_millis() as u64;

        match &result {
            Ok(response) => {
                let status_code = response.status().as_u16();

                if self.include_successful && status_code < 400 {
                    // Publish successful request event
                    let event = HttpRequestEvent {
                        method: method.clone(),
                        path: path.clone(),
                        status_code,
                        duration_ms,
                        user_agent: user_agent.clone(),
                        remote_addr: None,
                    };

                    let _ = self.bus.publish(event).await;
                }

                if self.include_errors && status_code >= 400 {
                    // Publish error event
                    let event = HttpErrorEvent {
                        method,
                        path,
                        error: format!("HTTP {}", status_code),
                        status_code,
                    };

                    let _ = self.bus.publish(event).await;
                }
            }
            Err(e) => {
                if self.include_errors {
                    // Publish error event
                    let event = HttpErrorEvent {
                        method,
                        path,
                        error: e.to_string(),
                        status_code: 500,
                    };

                    let _ = self.bus.publish(event).await;
                }
            }
        }

        result
    }
}

/// Metrics middleware that tracks HTTP statistics
pub struct Metrics {
    request_count: Arc<std::sync::atomic::AtomicU64>,
    error_count: Arc<std::sync::atomic::AtomicU64>,
}

impl Metrics {
    /// Create a new metrics middleware
    pub fn new() -> Self {
        Self {
            request_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
            error_count: Arc::new(std::sync::atomic::AtomicU64::new(0)),
        }
    }

    /// Get total request count
    pub fn request_count(&self) -> u64 {
        self.request_count.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Get total error count
    pub fn error_count(&self) -> u64 {
        self.error_count.load(std::sync::atomic::Ordering::Relaxed)
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for Metrics {
    async fn handle(&self, request: Request, next: Next) -> Result<Response> {
        self.request_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let result = next.run(request).await;

        if let Ok(response) = &result {
            if response.status().as_u16() >= 400 {
                self.error_count
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        } else {
            self.error_count
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        result
    }
}

/// Health check endpoint helper
pub fn health_check_response() -> Response {
    Response::json(&serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "service": "avx-http"
    }))
    .with_status(StatusCode::OK)
}

/// Metrics endpoint helper
pub fn metrics_response(metrics: &Metrics) -> Response {
    Response::json(&serde_json::json!({
        "requests": {
            "total": metrics.request_count(),
            "errors": metrics.error_count(),
            "success": metrics.request_count() - metrics.error_count(),
        },
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
    .with_status(StatusCode::OK)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_request_event() {
        let event = HttpRequestEvent {
            method: "GET".into(),
            path: "/api/users".into(),
            status_code: 200,
            duration_ms: 42,
            user_agent: Some("test".into()),
            remote_addr: None,
        };

        assert_eq!(event.method, "GET");
        assert_eq!(event.status_code, 200);
    }

    #[tokio::test]
    async fn test_metrics_middleware() {
        use crate::middleware::Handler;
        use http::Method;

        struct TestHandler;

        #[async_trait]
        impl Handler for TestHandler {
            async fn handle(&self, _request: Request) -> Result<Response> {
                Ok(Response::text("OK"))
            }
        }

        let metrics = Metrics::new();
        let handler = Arc::new(TestHandler);
        let request = Request {
            method: Method::GET,
            path: "/test".to_string(),
            headers: http::HeaderMap::new(),
            body: bytes::Bytes::new(),
        };

        let next = Next::new(vec![], handler);
        let _ = metrics.handle(request, next).await;

        assert_eq!(metrics.request_count(), 1);
        assert_eq!(metrics.error_count(), 0);
    }
}
