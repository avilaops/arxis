//! avl-observability - Distributed Tracing & Metrics
//!
//! Features:
//! - OpenTelemetry-compatible tracing
//! - Span context propagation
//! - Metrics collection
//!
//! Competing with: Jaeger, Zipkin, DataDog APM

use core::sync::atomic::{AtomicU64, Ordering};
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::BTreeMap;

/// Trace ID (128-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TraceId {
    pub high: u64,
    pub low: u64,
}

impl TraceId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self {
            high: 0,
            low: COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        if hex.len() != 32 {
            return None;
        }

        let high = u64::from_str_radix(&hex[..16], 16).ok()?;
        let low = u64::from_str_radix(&hex[16..], 16).ok()?;

        Some(Self { high, low })
    }

    pub fn to_hex(&self) -> String {
        format!("{:016x}{:016x}", self.high, self.low)
    }
}

/// Span ID (64-bit)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpanId(pub u64);

impl SpanId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub fn to_hex(&self) -> String {
        format!("{:016x}", self.0)
    }
}

/// Span represents a unit of work in a trace
#[derive(Debug, Clone)]
pub struct Span {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_span_id: Option<SpanId>,
    pub name: String,
    pub kind: SpanKind,
    pub start_time_ns: u64,
    pub end_time_ns: Option<u64>,
    pub attributes: BTreeMap<String, AttributeValue>,
    pub events: Vec<SpanEvent>,
    pub status: SpanStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanKind {
    Internal,
    Server,
    Client,
    Producer,
    Consumer,
}

#[derive(Debug, Clone)]
pub enum AttributeValue {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct SpanEvent {
    pub name: String,
    pub timestamp_ns: u64,
    pub attributes: BTreeMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpanStatus {
    Unset,
    Ok,
    Error,
}

impl Span {
    pub fn new(name: String, kind: SpanKind) -> Self {
        Self {
            trace_id: TraceId::new(),
            span_id: SpanId::new(),
            parent_span_id: None,
            name,
            kind,
            start_time_ns: current_time_ns(),
            end_time_ns: None,
            attributes: BTreeMap::new(),
            events: Vec::new(),
            status: SpanStatus::Unset,
        }
    }

    pub fn child_of(parent: &Span, name: String, kind: SpanKind) -> Self {
        Self {
            trace_id: parent.trace_id,
            span_id: SpanId::new(),
            parent_span_id: Some(parent.span_id),
            name,
            kind,
            start_time_ns: current_time_ns(),
            end_time_ns: None,
            attributes: BTreeMap::new(),
            events: Vec::new(),
            status: SpanStatus::Unset,
        }
    }

    pub fn set_attribute(&mut self, key: String, value: AttributeValue) {
        self.attributes.insert(key, value);
    }

    pub fn add_event(&mut self, name: String) {
        self.events.push(SpanEvent {
            name,
            timestamp_ns: current_time_ns(),
            attributes: BTreeMap::new(),
        });
    }

    pub fn end(&mut self) {
        self.end_time_ns = Some(current_time_ns());
    }

    pub fn duration_ns(&self) -> Option<u64> {
        self.end_time_ns.map(|end| end - self.start_time_ns)
    }

    pub fn duration_ms(&self) -> Option<f64> {
        self.duration_ns().map(|ns| ns as f64 / 1_000_000.0)
    }
}

/// Span context for propagation (W3C Trace Context)
#[derive(Debug, Clone)]
pub struct SpanContext {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub trace_flags: u8,
    pub trace_state: String,
}

impl SpanContext {
    /// Parse from W3C traceparent header
    /// Format: 00-{trace_id}-{span_id}-{flags}
    pub fn from_traceparent(header: &str) -> Option<Self> {
        let parts: Vec<&str> = header.split('-').collect();
        if parts.len() != 4 || parts[0] != "00" {
            return None;
        }

        let trace_id = TraceId::from_hex(parts[1])?;
        let span_id_raw = u64::from_str_radix(parts[2], 16).ok()?;
        let span_id = SpanId(span_id_raw);
        let trace_flags = u8::from_str_radix(parts[3], 16).ok()?;

        Some(Self {
            trace_id,
            span_id,
            trace_flags,
            trace_state: String::new(),
        })
    }

    /// Generate W3C traceparent header
    pub fn to_traceparent(&self) -> String {
        format!(
            "00-{}-{}-{:02x}",
            self.trace_id.to_hex(),
            self.span_id.to_hex(),
            self.trace_flags
        )
    }
}

/// Trace collector - batches and exports spans
pub struct TraceCollector {
    spans: Vec<Span>,
    max_batch_size: usize,
}

impl TraceCollector {
    pub fn new(max_batch_size: usize) -> Self {
        Self {
            spans: Vec::with_capacity(max_batch_size),
            max_batch_size,
        }
    }

    pub fn record_span(&mut self, span: Span) {
        self.spans.push(span);

        if self.spans.len() >= self.max_batch_size {
            self.flush();
        }
    }

    pub fn flush(&mut self) {
        if self.spans.is_empty() {
            return;
        }

        // Export spans to backend (would send to Jaeger, Zipkin, etc.)
        // For now, just clear
        self.spans.clear();
    }

    pub fn pending_spans(&self) -> usize {
        self.spans.len()
    }
}

/// Metric types
#[derive(Debug, Clone)]
pub enum Metric {
    Counter { name: String, value: u64, labels: BTreeMap<String, String> },
    Gauge { name: String, value: f64, labels: BTreeMap<String, String> },
    Histogram { name: String, values: Vec<f64>, labels: BTreeMap<String, String> },
}

/// Metrics collector
pub struct MetricsCollector {
    metrics: Vec<Metric>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
        }
    }

    pub fn increment_counter(&mut self, name: String, labels: BTreeMap<String, String>) {
        // Find existing counter or create new
        for metric in &mut self.metrics {
            if let Metric::Counter { name: n, value, labels: l } = metric {
                if n == &name && l == &labels {
                    *value += 1;
                    return;
                }
            }
        }

        self.metrics.push(Metric::Counter { name, value: 1, labels });
    }

    pub fn set_gauge(&mut self, name: String, value: f64, labels: BTreeMap<String, String>) {
        // Update existing gauge or create new
        for metric in &mut self.metrics {
            if let Metric::Gauge { name: n, value: v, labels: l } = metric {
                if n == &name && l == &labels {
                    *v = value;
                    return;
                }
            }
        }

        self.metrics.push(Metric::Gauge { name, value, labels });
    }

    pub fn record_histogram(&mut self, name: String, value: f64, labels: BTreeMap<String, String>) {
        // Find existing histogram or create new
        for metric in &mut self.metrics {
            if let Metric::Histogram { name: n, values, labels: l } = metric {
                if n == &name && l == &labels {
                    values.push(value);
                    return;
                }
            }
        }

        self.metrics.push(Metric::Histogram {
            name,
            values: vec![value],
            labels,
        });
    }

    pub fn get_metrics(&self) -> &[Metric] {
        &self.metrics
    }
}

fn current_time_ns() -> u64 {
    // In production, would use proper monotonic clock
    // For now, return mock value
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_id_hex() {
        let trace_id = TraceId { high: 0x1234567890abcdef, low: 0xfedcba0987654321 };
        let hex = trace_id.to_hex();
        assert_eq!(hex, "1234567890abcdeffedcba0987654321");

        let parsed = TraceId::from_hex(&hex).unwrap();
        assert_eq!(parsed, trace_id);
    }

    #[test]
    fn test_span_context_w3c() {
        let ctx = SpanContext {
            trace_id: TraceId { high: 0, low: 0x123456789abcdef0 },
            span_id: SpanId(0xabcdef0123456789),
            trace_flags: 1,
            trace_state: String::new(),
        };

        let header = ctx.to_traceparent();
        assert!(header.starts_with("00-"));

        let parsed = SpanContext::from_traceparent(&header).unwrap();
        assert_eq!(parsed.trace_id, ctx.trace_id);
        assert_eq!(parsed.span_id, ctx.span_id);
    }

    #[test]
    fn test_span_hierarchy() {
        let parent = Span::new("parent".to_string(), SpanKind::Server);
        let child = Span::child_of(&parent, "child".to_string(), SpanKind::Internal);

        assert_eq!(child.trace_id, parent.trace_id);
        assert_eq!(child.parent_span_id, Some(parent.span_id));
    }

    #[test]
    fn test_metrics_counter() {
        let mut collector = MetricsCollector::new();
        let labels = BTreeMap::new();

        collector.increment_counter("requests".to_string(), labels.clone());
        collector.increment_counter("requests".to_string(), labels.clone());

        let metrics = collector.get_metrics();
        assert_eq!(metrics.len(), 1);

        if let Metric::Counter { value, .. } = &metrics[0] {
            assert_eq!(*value, 2);
        }
    }
}
