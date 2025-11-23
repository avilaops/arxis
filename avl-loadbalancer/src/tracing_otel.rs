//! Distributed Tracing Module
//!
//! OpenTelemetry integration for distributed tracing across the load balancer.

use opentelemetry::{
    global,
    trace::{Span, SpanKind, StatusCode, Tracer, TracerProvider as _},
    KeyValue,
};
use opentelemetry_sdk::{
    trace::{RandomIdGenerator, Sampler, TracerProvider},
    Resource,
};
use opentelemetry_otlp::WithExportConfig;
use std::time::Duration;
use tracing::info;

/// Tracing configuration
#[derive(Clone, Debug)]
pub struct TracingConfig {
    pub service_name: String,
    pub service_version: String,
    pub endpoint: String,
    pub sample_rate: f64,
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self {
            service_name: "avl-loadbalancer".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            endpoint: "http://localhost:4317".to_string(),
            sample_rate: 1.0, // 100% sampling by default
        }
    }
}

/// Initialize OpenTelemetry tracing
pub fn init_tracing(config: TracingConfig) -> anyhow::Result<()> {
    info!("Initializing OpenTelemetry tracing: {}", config.service_name);

    let exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(&config.endpoint)
        .with_timeout(Duration::from_secs(3));

    let tracer_provider = TracerProvider::builder()
        .with_batch_exporter(
            opentelemetry_otlp::new_pipeline()
                .tracing()
                .with_exporter(exporter)
                .build_span_exporter()?,
            opentelemetry_sdk::runtime::Tokio,
        )
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            config.sample_rate,
        ))))
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(Resource::new(vec![
            KeyValue::new("service.name", config.service_name.clone()),
            KeyValue::new("service.version", config.service_version.clone()),
            KeyValue::new("deployment.environment", "production"),
        ]))
        .build();

    global::set_tracer_provider(tracer_provider);

    info!("OpenTelemetry tracing initialized successfully");
    Ok(())
}

/// Shutdown OpenTelemetry tracing
pub fn shutdown_tracing() {
    info!("Shutting down OpenTelemetry tracing");
    global::shutdown_tracer_provider();
}

/// Create a new span for load balancer operation
pub fn create_lb_span(operation: &str) -> impl Span {
    let tracer = global::tracer("avl-loadbalancer");
    tracer
        .span_builder(operation)
        .with_kind(SpanKind::Server)
        .start(&tracer)
}

/// Add attributes to a span
pub fn add_span_attributes(span: &mut impl Span, attributes: Vec<KeyValue>) {
    for attr in attributes {
        span.set_attribute(attr);
    }
}

/// Mark span as error
pub fn mark_span_error(span: &mut impl Span, error: &str) {
    span.set_status(StatusCode::Error, error.to_string());
}

/// Trace context for propagating trace information
#[derive(Clone, Debug)]
pub struct TraceContext {
    pub trace_id: String,
    pub span_id: String,
    pub parent_span_id: Option<String>,
}

impl TraceContext {
    pub fn new() -> Self {
        Self {
            trace_id: uuid::Uuid::new_v4().to_string(),
            span_id: uuid::Uuid::new_v4().to_string(),
            parent_span_id: None,
        }
    }

    pub fn with_parent(mut self, parent_span_id: String) -> Self {
        self.parent_span_id = Some(parent_span_id);
        self
    }

    /// Extract trace context from headers
    pub fn from_headers(headers: &axum::http::HeaderMap) -> Option<Self> {
        let trace_id = headers
            .get("X-Trace-ID")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())?;

        let span_id = headers
            .get("X-Span-ID")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())?;

        let parent_span_id = headers
            .get("X-Parent-Span-ID")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Some(Self {
            trace_id,
            span_id,
            parent_span_id,
        })
    }

    /// Inject trace context into headers
    pub fn inject_headers(&self, headers: &mut axum::http::HeaderMap) {
        headers.insert("X-Trace-ID", self.trace_id.parse().unwrap());
        headers.insert("X-Span-ID", self.span_id.parse().unwrap());
        if let Some(ref parent) = self.parent_span_id {
            headers.insert("X-Parent-Span-ID", parent.parse().unwrap());
        }
    }
}

impl Default for TraceContext {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_context_creation() {
        let ctx = TraceContext::new();
        assert!(!ctx.trace_id.is_empty());
        assert!(!ctx.span_id.is_empty());
        assert!(ctx.parent_span_id.is_none());
    }

    #[test]
    fn test_trace_context_with_parent() {
        let ctx = TraceContext::new().with_parent("parent-123".to_string());
        assert_eq!(ctx.parent_span_id, Some("parent-123".to_string()));
    }

    #[test]
    fn test_trace_context_headers() {
        let ctx = TraceContext::new();
        let mut headers = axum::http::HeaderMap::new();

        ctx.inject_headers(&mut headers);

        assert!(headers.contains_key("X-Trace-ID"));
        assert!(headers.contains_key("X-Span-ID"));

        let extracted = TraceContext::from_headers(&headers).unwrap();
        assert_eq!(extracted.trace_id, ctx.trace_id);
        assert_eq!(extracted.span_id, ctx.span_id);
    }
}
