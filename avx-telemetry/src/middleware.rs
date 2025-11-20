/// Middleware para coleta automática de métricas de latência
///
/// Este módulo implementa middleware Axum para tracking automático de latências
/// de requisições HTTP, detecção de anomalias em tempo real e integração com
/// o sistema de telemetria AVX.
///
/// # Casos de Uso
/// - Coletar latências de todas as requisições automaticamente
/// - Detectar anomalias em tempo real (ex: latência > threshold)
/// - Exportar métricas para AvilaDB periodicamente
/// - Rastreamento distribuído com OpenTelemetry
///
/// # Exemplo
/// ```rust,no_run
/// use axum::{Router, routing::get};
/// use avx_telemetry::middleware::LatencyMiddleware;
///
/// #[tokio::main]
/// async fn main() {
///     let middleware = LatencyMiddleware::new("avx-gateway");
///
///     let app = Router::new()
///         .route("/health", get(health_check))
///         .layer(middleware.into_layer());
///
///     // Server initialization...
/// }
///
/// async fn health_check() -> &'static str {
///     "OK"
/// }
/// ```
use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Instant;
use tower::{Layer, Service};

use crate::AvxMetrics;

/// Collector de latências em memória
///
/// Mantém um buffer circular das últimas N latências para análise em tempo real
/// e detecção de anomalias.
#[derive(Clone, Debug)]
pub struct LatencyCollector {
    /// Nome do serviço
    service_name: String,

    /// Buffer de latências (em millisegundos)
    latencies: Arc<Mutex<Vec<f64>>>,

    /// Tamanho máximo do buffer
    max_size: usize,

    /// AvxMetrics para detecção de anomalias
    metrics: Arc<AvxMetrics>,
}

impl LatencyCollector {
    /// Cria um novo collector
    ///
    /// # Argumentos
    /// * `service_name` - Nome do serviço
    /// * `max_size` - Tamanho máximo do buffer (default: 1000)
    pub fn new(service_name: impl Into<String>) -> Self {
        Self::with_capacity(service_name, 1000)
    }

    /// Cria um novo collector com capacidade específica
    pub fn with_capacity(service_name: impl Into<String>, max_size: usize) -> Self {
        Self {
            service_name: service_name.into(),
            latencies: Arc::new(Mutex::new(Vec::with_capacity(max_size))),
            max_size,
            metrics: Arc::new(AvxMetrics::new()),
        }
    }

    /// Registra uma latência
    ///
    /// Se o buffer estiver cheio, remove os 10% mais antigos
    pub fn record(&self, latency_ms: f64) {
        if let Ok(mut latencies) = self.latencies.lock() {
            latencies.push(latency_ms);

            // Manter buffer circular
            if latencies.len() > self.max_size {
                let drain_count = self.max_size / 10; // Remove 10%
                latencies.drain(0..drain_count);
            }

            // Log se latência estiver muito alta
            if latency_ms > 1000.0 {
                tracing::warn!(
                    service = %self.service_name,
                    latency_ms = latency_ms,
                    "High latency detected"
                );
            }
        }
    }

    /// Retorna snapshot atual das latências
    pub fn snapshot(&self) -> Vec<f64> {
        self.latencies.lock().map(|l| l.clone()).unwrap_or_default()
    }

    /// Retorna estatísticas das latências
    pub fn statistics(&self) -> LatencyStatistics {
        let latencies = self.snapshot();

        if latencies.is_empty() {
            return LatencyStatistics::default();
        }

        let mut sorted = latencies.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let sum: f64 = latencies.iter().sum();
        let mean = sum / latencies.len() as f64;

        let variance: f64 =
            latencies.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / latencies.len() as f64;
        let std_dev = variance.sqrt();

        LatencyStatistics {
            count: latencies.len(),
            mean_ms: mean,
            std_dev_ms: std_dev,
            min_ms: *sorted.first().unwrap(),
            max_ms: *sorted.last().unwrap(),
            p50_ms: percentile(&sorted, 0.50),
            p95_ms: percentile(&sorted, 0.95),
            p99_ms: percentile(&sorted, 0.99),
        }
    }

    /// Detecta anomalias nas latências recentes
    ///
    /// Usa o AvxMetrics para detecção via Z-score e IQR
    pub fn detect_anomalies(&self) -> Result<Vec<crate::Anomaly>, crate::TelemetryError> {
        let latencies = self.snapshot();

        if latencies.is_empty() {
            return Ok(vec![]);
        }

        self.metrics.track_latencies(latencies)
    }

    /// Limpa o buffer de latências
    pub fn clear(&self) {
        if let Ok(mut latencies) = self.latencies.lock() {
            latencies.clear();
        }
    }

    /// Retorna o nome do serviço
    pub fn service_name(&self) -> &str {
        &self.service_name
    }
}

/// Estatísticas de latência
#[derive(Debug, Clone, Default)]
pub struct LatencyStatistics {
    pub count: usize,
    pub mean_ms: f64,
    pub std_dev_ms: f64,
    pub min_ms: f64,
    pub max_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
}

impl LatencyStatistics {
    /// Verifica se as latências estão saudáveis
    pub fn is_healthy(&self, p99_threshold_ms: f64) -> bool {
        self.p99_ms <= p99_threshold_ms
    }

    /// Retorna JSON com as estatísticas
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "count": self.count,
            "mean_ms": self.mean_ms,
            "std_dev_ms": self.std_dev_ms,
            "min_ms": self.min_ms,
            "max_ms": self.max_ms,
            "percentiles": {
                "p50": self.p50_ms,
                "p95": self.p95_ms,
                "p99": self.p99_ms,
            }
        })
    }
}

/// Middleware Axum para tracking de latência
///
/// # Exemplo
/// ```rust,no_run
/// use axum::Router;
/// use avx_telemetry::middleware::LatencyMiddleware;
///
/// let middleware = LatencyMiddleware::new("my-service");
/// let app = Router::new()
///     .layer(middleware.into_layer());
/// ```
#[derive(Clone)]
pub struct LatencyMiddleware {
    collector: LatencyCollector,
}

impl LatencyMiddleware {
    /// Cria um novo middleware
    pub fn new(service_name: impl Into<String>) -> Self {
        Self {
            collector: LatencyCollector::new(service_name),
        }
    }

    /// Cria um novo middleware com collector customizado
    pub fn with_collector(collector: LatencyCollector) -> Self {
        Self { collector }
    }

    /// Retorna referência ao collector
    pub fn collector(&self) -> &LatencyCollector {
        &self.collector
    }

    /// Converte para Layer do Tower
    pub fn into_layer(self) -> LatencyLayer {
        LatencyLayer {
            collector: self.collector,
        }
    }
}

/// Layer do Tower para integração com Axum
#[derive(Clone)]
pub struct LatencyLayer {
    collector: LatencyCollector,
}

impl<S> Layer<S> for LatencyLayer {
    type Service = LatencyService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LatencyService {
            inner,
            collector: self.collector.clone(),
        }
    }
}

/// Service do Tower que registra latências
#[derive(Clone)]
pub struct LatencyService<S> {
    inner: S,
    collector: LatencyCollector,
}

impl<S> Service<Request<Body>> for LatencyService<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let collector = self.collector.clone();
        let mut inner = self.inner.clone();

        // Marcar início
        let start = Instant::now();

        // Extrair informações da requisição para logging
        let method = request.method().clone();
        let uri = request.uri().clone();

        Box::pin(async move {
            // Executar requisição
            let response = inner.call(request).await?;

            // Calcular latência
            let duration = start.elapsed();
            let latency_ms = duration.as_secs_f64() * 1000.0;

            // Registrar latência
            collector.record(latency_ms);

            // Log estruturado
            tracing::debug!(
                service = %collector.service_name(),
                method = %method,
                uri = %uri,
                status = response.status().as_u16(),
                latency_ms = latency_ms,
                "Request completed"
            );

            Ok(response)
        })
    }
}

/// Helper para calcular percentil
fn percentile(sorted_data: &[f64], p: f64) -> f64 {
    if sorted_data.is_empty() {
        return 0.0;
    }

    let index = (p * (sorted_data.len() - 1) as f64).round() as usize;
    sorted_data[index.min(sorted_data.len() - 1)]
}

/// Função de middleware standalone (alternativa ao Layer)
///
/// # Exemplo
/// ```rust,no_run
/// use axum::{Router, middleware};
/// use avx_telemetry::middleware::{latency_middleware, LatencyCollector};
/// use std::sync::Arc;
///
/// #[tokio::main]
/// async fn main() {
///     let collector = Arc::new(LatencyCollector::new("my-service"));
///
///     let app = Router::new()
///         .layer(middleware::from_fn(move |req, next| {
///             latency_middleware(req, next, collector.clone())
///         }));
/// }
/// ```
pub async fn latency_middleware(
    request: Request,
    next: Next,
    collector: Arc<LatencyCollector>,
) -> Response {
    let start = Instant::now();

    // Extrair informações
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Executar requisição
    let response = next.run(request).await;

    // Calcular e registrar latência
    let duration = start.elapsed();
    let latency_ms = duration.as_secs_f64() * 1000.0;
    collector.record(latency_ms);

    // Log
    tracing::debug!(
        service = %collector.service_name(),
        method = %method,
        uri = %uri,
        status = response.status().as_u16(),
        latency_ms = latency_ms,
        "Request completed"
    );

    response
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latency_collector_creation() {
        let collector = LatencyCollector::new("test-service");
        assert_eq!(collector.service_name(), "test-service");
        assert_eq!(collector.snapshot().len(), 0);
    }

    #[test]
    fn test_latency_recording() {
        let collector = LatencyCollector::new("test");

        collector.record(10.0);
        collector.record(20.0);
        collector.record(30.0);

        let snapshot = collector.snapshot();
        assert_eq!(snapshot.len(), 3);
        assert_eq!(snapshot, vec![10.0, 20.0, 30.0]);
    }

    #[test]
    fn test_buffer_circular_behavior() {
        let collector = LatencyCollector::with_capacity("test", 10);

        // Adicionar mais que a capacidade
        for i in 0..15 {
            collector.record(i as f64);
        }

        let snapshot = collector.snapshot();

        // Deve ter removido os mais antigos
        assert!(snapshot.len() <= 10);
    }

    #[test]
    fn test_latency_statistics() {
        let collector = LatencyCollector::new("test");

        let latencies = vec![10.0, 20.0, 30.0, 40.0, 50.0];
        for latency in &latencies {
            collector.record(*latency);
        }

        let stats = collector.statistics();

        assert_eq!(stats.count, 5);
        assert_eq!(stats.mean_ms, 30.0);
        assert_eq!(stats.min_ms, 10.0);
        assert_eq!(stats.max_ms, 50.0);
        assert_eq!(stats.p50_ms, 30.0);
    }

    #[test]
    fn test_statistics_empty_collector() {
        let collector = LatencyCollector::new("test");
        let stats = collector.statistics();

        assert_eq!(stats.count, 0);
        assert_eq!(stats.mean_ms, 0.0);
    }

    #[test]
    fn test_statistics_is_healthy() {
        let collector = LatencyCollector::new("test");

        for i in 0..100 {
            collector.record((i % 20) as f64); // Latências 0-19ms
        }

        let stats = collector.statistics();

        // P99 deve estar bem abaixo de 50ms
        assert!(stats.is_healthy(50.0));
        assert!(stats.p99_ms < 50.0);
    }

    #[test]
    fn test_clear_collector() {
        let collector = LatencyCollector::new("test");

        collector.record(10.0);
        collector.record(20.0);
        assert_eq!(collector.snapshot().len(), 2);

        collector.clear();
        assert_eq!(collector.snapshot().len(), 0);
    }

    #[test]
    fn test_percentile_calculation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        assert_eq!(percentile(&data, 0.50), 5.0);
        assert_eq!(percentile(&data, 0.95), 10.0);
        assert_eq!(percentile(&data, 0.00), 1.0);
        assert_eq!(percentile(&data, 1.00), 10.0);
    }

    #[test]
    fn test_statistics_to_json() {
        let stats = LatencyStatistics {
            count: 100,
            mean_ms: 15.5,
            std_dev_ms: 3.2,
            min_ms: 10.0,
            max_ms: 25.0,
            p50_ms: 15.0,
            p95_ms: 22.0,
            p99_ms: 24.0,
        };

        let json = stats.to_json();

        assert_eq!(json["count"], 100);
        assert_eq!(json["mean_ms"], 15.5);
        assert_eq!(json["percentiles"]["p50"], 15.0);
        assert_eq!(json["percentiles"]["p99"], 24.0);
    }

    #[tokio::test]
    async fn test_detect_anomalies() {
        let collector = LatencyCollector::new("test");

        // Adicionar latências normais
        for _ in 0..100 {
            collector.record(10.0);
        }

        // Adicionar anomalia
        collector.record(1000.0);

        let anomalies = collector.detect_anomalies().unwrap();

        // Deve detectar a anomalia
        assert!(!anomalies.is_empty());
    }

    #[test]
    fn test_middleware_creation() {
        let middleware = LatencyMiddleware::new("test-service");
        assert_eq!(middleware.collector().service_name(), "test-service");
    }
}
