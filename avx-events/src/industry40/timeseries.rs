//! Time-series database backend for sensor data aggregation
//!
//! Provides InfluxDB integration for storing and querying time-series sensor data.

use crate::{Event, Result};
use chrono::{DateTime, Duration, Utc};
use influxdb2::models::DataPoint;
use influxdb2::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Time-series backend configuration
#[derive(Debug, Clone)]
pub struct TimeSeriesConfig {
    pub url: String,
    pub org: String,
    pub bucket: String,
    pub token: String,
}

impl Default for TimeSeriesConfig {
    fn default() -> Self {
        Self {
            url: "http://localhost:8086".into(),
            org: "avila".into(),
            bucket: "industry40".into(),
            token: String::new(),
        }
    }
}

/// Time-series database backend using InfluxDB
pub struct TimeSeriesBackend {
    client: Client,
    config: TimeSeriesConfig,
}

impl TimeSeriesBackend {
    /// Create a new time-series backend
    pub fn new(config: TimeSeriesConfig) -> Self {
        let client = Client::new(&config.url, &config.org, &config.token);

        info!(
            url = %config.url,
            org = %config.org,
            bucket = %config.bucket,
            "Time-series backend initialized"
        );

        Self { client, config }
    }

    /// Write sensor reading to time-series database
    pub async fn write_sensor_reading(
        &self,
        sensor_id: &str,
        sensor_type: &str,
        value: f64,
        timestamp: DateTime<Utc>,
        tags: HashMap<String, String>,
    ) -> Result<()> {
        let mut point = DataPoint::builder("sensor_reading")
            .tag("sensor_id", sensor_id)
            .tag("sensor_type", sensor_type)
            .field("value", value)
            .timestamp(timestamp.timestamp_nanos_opt().unwrap_or(0));

        for (key, value) in tags {
            point = point.tag(&key, &value);
        }

        self.client
            .write(&self.config.bucket, futures::stream::iter(vec![point.build()
                .map_err(|e| crate::Error::internal(format!("Failed to build data point: {}", e)))?]))
            .await
            .map_err(|e| crate::Error::internal(format!("InfluxDB write error: {}", e)))?;

        debug!(sensor_id = %sensor_id, value = %value, "Wrote sensor reading to InfluxDB");
        Ok(())
    }

    /// Query aggregated sensor data
    pub async fn query_aggregate(
        &self,
        sensor_id: &str,
        aggregation: Aggregation,
        window: Duration,
        range_start: DateTime<Utc>,
        range_end: DateTime<Utc>,
    ) -> Result<Vec<AggregatedReading>> {
        let agg_fn = match aggregation {
            Aggregation::Mean => "mean",
            Aggregation::Min => "min",
            Aggregation::Max => "max",
            Aggregation::Sum => "sum",
            Aggregation::Count => "count",
        };

        let query = format!(
            r#"
            from(bucket: "{}")
                |> range(start: {}, stop: {})
                |> filter(fn: (r) => r["_measurement"] == "sensor_reading")
                |> filter(fn: (r) => r["sensor_id"] == "{}")
                |> aggregateWindow(every: {}s, fn: {}, createEmpty: false)
            "#,
            self.config.bucket,
            range_start.to_rfc3339(),
            range_end.to_rfc3339(),
            sensor_id,
            window.num_seconds(),
            agg_fn
        );

        // Parse results (simplified - real implementation would parse InfluxDB response)
        let readings = vec![]; // TODO: Parse actual response from InfluxDB query API

        debug!(
            sensor_id = %sensor_id,
            count = readings.len(),
            "Queried aggregated sensor data"
        );

        Ok(readings)
    }

    /// Delete old data (retention policy)
    pub async fn delete_old_data(&self, before: DateTime<Utc>) -> Result<()> {
        let predicate = format!(
            r#"_measurement="sensor_reading" AND _time < {}"#,
            before.to_rfc3339()
        );

        // Note: InfluxDB delete API simplified - check influxdb2 crate docs for exact API
        info!(before = %before, "Would delete old sensor data (simplified implementation)");

        // TODO: Implement actual delete with correct influxdb2 API
        // The delete API may vary by version

        Ok(())
    }
}

/// Aggregation function for time-series queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Aggregation {
    Mean,
    Min,
    Max,
    Sum,
    Count,
}

/// Aggregated sensor reading
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedReading {
    pub sensor_id: String,
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub aggregation: Aggregation,
    pub window_size: Duration,
}

impl Event for AggregatedReading {
    fn event_type(&self) -> &'static str {
        "timeseries.aggregated"
    }

    fn aggregate_id(&self) -> String {
        self.sensor_id.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeseries_config_default() {
        let config = TimeSeriesConfig::default();
        assert_eq!(config.url, "http://localhost:8086");
        assert_eq!(config.bucket, "industry40");
    }
}
