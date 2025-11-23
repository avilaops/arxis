# AVL Observability - Copilot Instructions

## Project Identity
**AVL Observability** is the **complete observability stack** for AVL Cloud (metrics, logs, traces). Embodies Arxis philosophy:
- **ARX (Fortress)**: Complete visibility, reliable collection, durable storage
- **AXIS (Engine)**: Real-time aggregation, fast queries, low overhead

## Core Principles
```rust
// ✅ ALWAYS: Collect metrics, logs, and traces
// ✅ ALWAYS: Use structured logging (JSON format)
// ✅ ALWAYS: Propagate trace context (W3C standard)
// ✅ ALWAYS: Store in AvilaDB for long-term retention
// ✅ NEVER: Drop metrics/logs (buffering + backpressure)
// ✅ NEVER: Impact application performance (< 1% overhead)
```

## Three Pillars

### 1. Metrics (Prometheus-compatible)
- **Counters**: Monotonically increasing (requests_total, errors_total)
- **Gauges**: Current value (memory_usage, active_connections)
- **Histograms**: Distribution (latency_seconds, request_size_bytes)

### 2. Logs (Structured JSON)
```rust
tracing::info!(
    user_id = "user123",
    action = "login",
    ip = "192.168.1.1",
    "User logged in successfully"
);
```

### 3. Traces (OpenTelemetry)
- **Spans**: Individual operations with start/end time
- **Trace Context**: Propagated across service boundaries
- **Sampling**: Configurable sample rates

## Performance Targets
- **Collection Overhead**: < 1% CPU, < 50 MB RAM
- **Ingestion Rate**: 100K+ events/sec
- **Query Latency**: < 500ms for 1-hour window

## Related Crates
- **avx-telemetry**: Base telemetry primitives
- **avila-telemetry**: Time series analysis
- **prometheus**: Metrics exposition
- **opentelemetry**: Distributed tracing
- **aviladb**: Long-term storage

🏛️ Built by Avila | 📊 Complete Visibility | ⚙️ Real-Time Monitoring
