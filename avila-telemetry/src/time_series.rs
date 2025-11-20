//! Core time series data structure and operations

use crate::{Result, TelemetryError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a time series with optional timestamps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeries {
    /// Data values
    pub values: Vec<f64>,
    /// Optional timestamps (if None, assumes regular intervals)
    pub timestamps: Option<Vec<DateTime<Utc>>>,
    /// Optional name/label for the series
    pub name: Option<String>,
}

impl TimeSeries {
    /// Create a new time series with just values
    pub fn new(values: Vec<f64>) -> Self {
        Self {
            values,
            timestamps: None,
            name: None,
        }
    }

    /// Create a time series with timestamps
    pub fn with_timestamps(values: Vec<f64>, timestamps: Vec<DateTime<Utc>>) -> Result<Self> {
        if values.len() != timestamps.len() {
            return Err(TelemetryError::InvalidData(
                "Values and timestamps must have the same length".to_string(),
            ));
        }
        Ok(Self {
            values,
            timestamps: Some(timestamps),
            name: None,
        })
    }

    /// Set the name of the time series
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Get the length of the time series
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Check if the time series is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Calculate simple moving average
    pub fn moving_average(&self, window: usize) -> Result<Vec<f64>> {
        if window == 0 {
            return Err(TelemetryError::InvalidParameter(
                "Window size must be greater than 0".to_string(),
            ));
        }

        if window > self.values.len() {
            return Err(TelemetryError::InsufficientData(format!(
                "Window size {} is larger than data length {}",
                window,
                self.values.len()
            )));
        }

        let mut result = Vec::with_capacity(self.values.len() - window + 1);

        for i in 0..=(self.values.len() - window) {
            let sum: f64 = self.values[i..i + window].iter().sum();
            result.push(sum / window as f64);
        }

        Ok(result)
    }

    /// Calculate exponential moving average
    pub fn exponential_moving_average(&self, alpha: f64) -> Result<Vec<f64>> {
        if alpha <= 0.0 || alpha > 1.0 {
            return Err(TelemetryError::InvalidParameter(
                "Alpha must be between 0 and 1".to_string(),
            ));
        }

        if self.is_empty() {
            return Err(TelemetryError::InsufficientData(
                "Cannot calculate EMA on empty series".to_string(),
            ));
        }

        let mut result = Vec::with_capacity(self.values.len());
        result.push(self.values[0]);

        for i in 1..self.values.len() {
            let ema = alpha * self.values[i] + (1.0 - alpha) * result[i - 1];
            result.push(ema);
        }

        Ok(result)
    }

    /// Calculate first difference
    pub fn diff(&self) -> Vec<f64> {
        if self.values.len() < 2 {
            return Vec::new();
        }

        self.values.windows(2).map(|w| w[1] - w[0]).collect()
    }

    /// Calculate percentage change
    pub fn pct_change(&self) -> Vec<f64> {
        if self.values.len() < 2 {
            return Vec::new();
        }

        self.values
            .windows(2)
            .map(|w| {
                if w[0] == 0.0 {
                    0.0
                } else {
                    (w[1] - w[0]) / w[0]
                }
            })
            .collect()
    }

    /// Get a slice of the time series
    pub fn slice(&self, start: usize, end: usize) -> Result<TimeSeries> {
        if start >= end || end > self.values.len() {
            return Err(TelemetryError::InvalidParameter(
                "Invalid slice indices".to_string(),
            ));
        }

        let values = self.values[start..end].to_vec();
        let timestamps = self.timestamps.as_ref().map(|ts| ts[start..end].to_vec());

        Ok(TimeSeries {
            values,
            timestamps,
            name: self.name.clone(),
        })
    }

    /// Calculate basic statistics
    pub fn statistics(&self) -> Statistics {
        Statistics::from_values(&self.values)
    }
}

/// Basic statistics for a time series
#[derive(Debug, Clone)]
pub struct Statistics {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
}

impl Statistics {
    pub fn from_values(values: &[f64]) -> Self {
        let count = values.len();

        if count == 0 {
            return Self {
                mean: 0.0,
                median: 0.0,
                std_dev: 0.0,
                min: 0.0,
                max: 0.0,
                count: 0,
            };
        }

        let mean = values.iter().sum::<f64>() / count as f64;

        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median = if count.is_multiple_of(2) {
            (sorted[count / 2 - 1] + sorted[count / 2]) / 2.0
        } else {
            sorted[count / 2]
        };

        let variance = values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count as f64;

        let std_dev = variance.sqrt();
        let min = *sorted.first().unwrap();
        let max = *sorted.last().unwrap();

        Self {
            mean,
            median,
            std_dev,
            min,
            max,
            count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let ts = TimeSeries::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let ma = ts.moving_average(3).unwrap();
        assert_eq!(ma, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_ema() {
        let ts = TimeSeries::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let ema = ts.exponential_moving_average(0.5).unwrap();
        assert_eq!(ema.len(), 5);
    }

    #[test]
    fn test_statistics() {
        let ts = TimeSeries::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let stats = ts.statistics();
        assert_eq!(stats.mean, 3.0);
        assert_eq!(stats.median, 3.0);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 5.0);
    }
}
