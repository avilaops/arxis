//! # Metrics Storage Module
//!
//! Integration layer for persisting telemetry metrics to AvilaDB.
//! Supports time-series data storage and historical queries.

use avila_telemetry::{Anomaly, TimeSeries};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Configuration for metrics storage in AvilaDB
#[derive(Debug, Clone)]
pub struct StorageConfig {
    /// AvilaDB connection endpoint
    pub endpoint: String,
    /// Database name for metrics
    pub database: String,
    /// Collection name for metrics
    pub collection: String,
    /// Retention period in days
    pub retention_days: u32,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:8000".to_string(),
            database: "avx_metrics".to_string(),
            collection: "telemetry".to_string(),
            retention_days: 30,
        }
    }
}

/// Metrics document structure for AvilaDB storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsDocument {
    /// Unique document ID (service-timestamp format)
    pub id: String,
    /// Service name generating metrics
    pub service: String,
    /// Timestamp of metric collection
    pub timestamp: DateTime<Utc>,
    /// AVX context information
    pub context: MetricsContext,
    /// Collected metrics data
    pub metrics: MetricsData,
    /// Forecasting results (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast: Option<ForecastData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsContext {
    pub stack: String,
    pub layer: String,
    pub env: String,
    pub cluster: String,
    pub mesh: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsData {
    /// Latency statistics
    pub latency: LatencyMetrics,
    /// Traffic metrics
    pub traffic: TrafficMetrics,
    /// Error metrics
    pub errors: ErrorMetrics,
    /// Detected anomalies
    pub anomalies: Vec<AnomalyRecord>,
    /// Data quality assessment
    pub quality: QualityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatencyMetrics {
    pub mean_ms: f64,
    pub p50_ms: f64,
    pub p95_ms: f64,
    pub p99_ms: f64,
    pub max_ms: f64,
    pub min_ms: f64,
    pub std_dev_ms: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficMetrics {
    pub requests_per_second: f64,
    pub bytes_per_second: u64,
    pub active_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorMetrics {
    pub error_rate: f64,
    pub total_errors: u32,
    pub error_budget_remaining: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyRecord {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub anomaly_type: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub accuracy: f64,
    pub completeness: f64,
    pub consistency: f64,
    pub validity: f64,
    pub overall_score: f64,
    pub meets_nasa_standards: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastData {
    /// Forecasted values for next periods
    pub next_values: Vec<f64>,
    /// Forecasting model used
    pub model: String,
    /// Confidence level
    pub confidence: f64,
}

/// Metrics storage handler for AvilaDB
pub struct MetricsStorage {
    service_name: String,
    config: StorageConfig,
    // TODO: Add AvilaDB client when Rust SDK is available
    // client: AvilaClient,
}

impl MetricsStorage {
    /// Create a new metrics storage handler
    pub fn new(service_name: impl Into<String>, config: StorageConfig) -> Self {
        Self {
            service_name: service_name.into(),
            config,
        }
    }

    /// Create with default configuration
    pub fn with_defaults(service_name: impl Into<String>) -> Self {
        Self::new(service_name, StorageConfig::default())
    }

    /// Save metrics to AvilaDB
    ///
    /// # Arguments
    /// * `timestamp` - Timestamp of metric collection
    /// * `context` - AVX context information
    /// * `latencies` - Raw latency measurements
    /// * `anomalies` - Detected anomalies
    /// * `quality` - Data quality assessment
    pub async fn save_metrics(
        &self,
        timestamp: DateTime<Utc>,
        context: MetricsContext,
        latencies: &[f64],
        anomalies: Vec<Anomaly>,
        quality: avila_telemetry::observability::DataQualityAssessment,
    ) -> Result<String, String> {
        // Calculate latency statistics
        let ts = TimeSeries::new(latencies.to_vec());
        let stats = ts.statistics();

        let latency_metrics = LatencyMetrics {
            mean_ms: stats.mean,
            p50_ms: percentile(latencies, 0.50),
            p95_ms: percentile(latencies, 0.95),
            p99_ms: percentile(latencies, 0.99),
            max_ms: stats.max,
            min_ms: stats.min,
            std_dev_ms: stats.std_dev,
        };

        // Convert anomalies to records
        let anomaly_records: Vec<AnomalyRecord> = anomalies
            .iter()
            .map(|a| AnomalyRecord {
                timestamp,
                value: a.value,
                anomaly_type: format!("{:?}", a.anomaly_type),
                score: a.score,
            })
            .collect();

        // Create metrics document
        let doc_id = format!(
            "metric-{}-{}",
            self.service_name,
            timestamp.format("%Y%m%d-%H%M%S")
        );

        let document = MetricsDocument {
            id: doc_id.clone(),
            service: self.service_name.clone(),
            timestamp,
            context,
            metrics: MetricsData {
                latency: latency_metrics,
                traffic: TrafficMetrics {
                    requests_per_second: 0.0, // TODO: Calculate from time window
                    bytes_per_second: 0,
                    active_connections: 0,
                },
                errors: ErrorMetrics {
                    error_rate: 0.0,
                    total_errors: 0,
                    error_budget_remaining: 1.0,
                },
                anomalies: anomaly_records,
                quality: QualityMetrics {
                    accuracy: quality.accuracy,
                    completeness: quality.completeness,
                    consistency: quality.consistency,
                    validity: quality.validity,
                    overall_score: quality.overall_score,
                    meets_nasa_standards: quality.meets_nasa_standards(),
                },
            },
            forecast: None,
        };

        // TODO: Save to AvilaDB when Rust SDK is available
        // self.client.database(&self.config.database)
        //     .collection(&self.config.collection)
        //     .insert(document)
        //     .await?;

        tracing::info!(
            service = %self.service_name,
            doc_id = %doc_id,
            timestamp = %timestamp,
            latency_mean = %document.metrics.latency.mean_ms,
            anomalies = document.metrics.anomalies.len(),
            "Metrics document prepared for storage"
        );

        Ok(doc_id)
    }

    /// Query historical metrics from AvilaDB
    ///
    /// # Arguments
    /// * `start` - Start timestamp for query range
    /// * `end` - End timestamp for query range
    pub async fn query_metrics(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<Vec<MetricsDocument>, String> {
        // TODO: Query AvilaDB when Rust SDK is available
        // Example query:
        // SELECT * FROM telemetry
        // WHERE service = @service
        //   AND timestamp >= @start
        //   AND timestamp <= @end
        // ORDER BY timestamp DESC

        tracing::info!(
            service = %self.service_name,
            start = %start,
            end = %end,
            "Query metrics from AvilaDB (not yet implemented)"
        );

        Ok(vec![])
    }

    /// Get aggregated metrics for a time window
    pub async fn get_aggregated_metrics(
        &self,
        _start: DateTime<Utc>,
        _end: DateTime<Utc>,
        window_minutes: u32,
    ) -> Result<Vec<AggregatedMetrics>, String> {
        // TODO: Implement aggregation query
        // GROUP BY time window and calculate averages
        tracing::info!(
            service = %self.service_name,
            window_minutes = window_minutes,
            "Get aggregated metrics (not yet implemented)"
        );

        Ok(vec![])
    }

    /// Delete old metrics based on retention policy
    pub async fn cleanup_old_metrics(&self) -> Result<u32, String> {
        let cutoff = Utc::now() - chrono::Duration::days(self.config.retention_days as i64);

        // TODO: Delete documents older than cutoff
        tracing::info!(
            service = %self.service_name,
            retention_days = self.config.retention_days,
            cutoff = %cutoff,
            "Cleanup old metrics (not yet implemented)"
        );

        Ok(0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedMetrics {
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    pub avg_latency_ms: f64,
    pub max_latency_ms: f64,
    pub total_requests: u64,
    pub total_errors: u32,
    pub anomaly_count: u32,
}

/// Calculate percentile from sorted data
fn percentile(data: &[f64], p: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = (p * (sorted.len() - 1) as f64) as usize;
    sorted[index]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_percentile_calculation() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0];

        assert_eq!(percentile(&data, 0.50), 5.0); // Mediana
        assert_eq!(percentile(&data, 0.95), 9.0); // P95 = índice 8 = valor 9.0
        assert_eq!(percentile(&data, 1.00), 10.0); // Máximo
    }

    #[test]
    fn test_storage_config_default() {
        let config = StorageConfig::default();
        assert_eq!(config.database, "avx_metrics");
        assert_eq!(config.collection, "telemetry");
        assert_eq!(config.retention_days, 30);
    }

    #[tokio::test]
    async fn test_metrics_storage_creation() {
        let storage = MetricsStorage::with_defaults("test-service");
        assert_eq!(storage.service_name, "test-service");
    }

    #[tokio::test]
    async fn test_save_metrics_document_structure() {
        let storage = MetricsStorage::with_defaults("avx-gateway");

        let context = MetricsContext {
            stack: "Avx".to_string(),
            layer: "deep".to_string(),
            env: "dev".to_string(),
            cluster: "AVL-BR".to_string(),
            mesh: "internal".to_string(),
        };

        let latencies = vec![10.0, 12.0, 11.0, 13.0, 9.0];
        let anomalies = vec![];
        let quality = avila_telemetry::observability::DataQualityAssessment {
            accuracy: 0.99,
            completeness: 0.98,
            consistency: 0.97,
            timeliness_ms: 50,
            validity: 0.96,
            overall_score: 0.975,
        };

        let result = storage
            .save_metrics(Utc::now(), context, &latencies, anomalies, quality)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("metric-avx-gateway-"));
    }
}
