//! # avila-telemetry
//!
//! Time series analysis, telemetry, and forecasting library for Rust.
//!
//! ## Features
//!
//! - **Time Series Analysis**: ARIMA, SARIMA, State Space Models
//! - **Anomaly Detection**: Statistical and ML-based detection
//! - **Forecasting**: Multi-step prediction with probabilistic forecasting
//! - **Feature Engineering**: Lag features, rolling statistics, seasonality decomposition
//!
//! ## Example
//!
//! ```rust
//! use avila_telemetry::TimeSeries;
//!
//! // Create a time series
//! let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! let ts = TimeSeries::new(data);
//!
//! // Calculate moving average
//! let ma = ts.moving_average(3);
//! ```

pub mod anomaly;
pub mod decomposition;
pub mod features;
pub mod forecasting;
pub mod models;
pub mod observability;
pub mod time_series;

pub use anomaly::{Anomaly, AnomalyDetector, AnomalyType};
pub use decomposition::{Decomposer, DecompositionResult, DecompositionType};
pub use features::FeatureExtractor;
pub use forecasting::{ExponentialSmoothing, ForecastResult, Forecaster, MovingAverageForecaster};
pub use observability::{
    Alert, AlertLevel, DataQuality, DataQualityAssessment, ErrorBudget, GoldenSignals, LogSeverity,
    NASAQualityControl, PerformanceTracker, ServiceLevelObjective, StructuredLog,
};
pub use time_series::TimeSeries;

/// Common error type for the library
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum TelemetryError {
    /// Invalid input data
    InvalidData(String),
    /// Insufficient data points
    InsufficientData(String),
    /// Model fitting error
    ModelError(String),
    /// Invalid parameter
    InvalidParameter(String),
}

impl std::fmt::Display for TelemetryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TelemetryError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            TelemetryError::InsufficientData(msg) => write!(f, "Insufficient data: {}", msg),
            TelemetryError::ModelError(msg) => write!(f, "Model error: {}", msg),
            TelemetryError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
        }
    }
}

impl std::error::Error for TelemetryError {}

pub type Result<T> = std::result::Result<T, TelemetryError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ts = TimeSeries::new(data);
        assert_eq!(ts.len(), 5);
    }
}
