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

pub mod time_series;
pub mod anomaly;
pub mod forecasting;
pub mod features;
pub mod decomposition;
pub mod models;
pub mod observability;

pub use time_series::TimeSeries;
pub use anomaly::{AnomalyDetector, AnomalyType, Anomaly};
pub use forecasting::{Forecaster, ForecastResult, ExponentialSmoothing, MovingAverageForecaster};
pub use features::FeatureExtractor;
pub use decomposition::{Decomposer, DecompositionType, DecompositionResult};
pub use observability::{
    GoldenSignals, DataQualityAssessment, NASAQualityControl, ServiceLevelObjective,
    ErrorBudget, StructuredLog, LogSeverity, AlertLevel, Alert, PerformanceTracker, DataQuality,
};

/// Common error type for the library
#[derive(Debug)]
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
