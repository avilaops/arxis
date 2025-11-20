//! Anomaly detection algorithms

use crate::{Result, TelemetryError, TimeSeries};

/// Type of anomaly detected
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AnomalyType {
    /// Point anomaly (single outlier)
    Point,
    /// Contextual anomaly (unusual in context)
    Contextual,
    /// Collective anomaly (unusual pattern)
    Collective,
}

/// Represents a detected anomaly
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Anomaly {
    /// Index in the time series
    pub index: usize,
    /// Value at the anomaly point
    pub value: f64,
    /// Type of anomaly
    pub anomaly_type: AnomalyType,
    /// Anomaly score (higher = more anomalous)
    pub score: f64,
}

/// Anomaly detector using various statistical methods
#[derive(Debug)]
pub struct AnomalyDetector {
    /// Z-score threshold for detection
    pub z_threshold: f64,
    /// IQR multiplier for detection
    pub iqr_multiplier: f64,
}

impl Default for AnomalyDetector {
    fn default() -> Self {
        Self {
            z_threshold: 3.0,
            iqr_multiplier: 1.5,
        }
    }
}

impl AnomalyDetector {
    /// Create a new anomaly detector with custom parameters
    pub fn new(z_threshold: f64, iqr_multiplier: f64) -> Self {
        Self {
            z_threshold,
            iqr_multiplier,
        }
    }

    /// Detect anomalies using Z-score method
    pub fn detect_zscore(&self, ts: &TimeSeries) -> Result<Vec<Anomaly>> {
        if ts.len() < 3 {
            return Err(TelemetryError::InsufficientData(
                "Need at least 3 data points for Z-score detection".to_string(),
            ));
        }

        let stats = ts.statistics();
        let mut anomalies = Vec::new();

        for (i, &value) in ts.values.iter().enumerate() {
            let z_score = ((value - stats.mean) / stats.std_dev).abs();

            if z_score > self.z_threshold {
                anomalies.push(Anomaly {
                    index: i,
                    value,
                    anomaly_type: AnomalyType::Point,
                    score: z_score,
                });
            }
        }

        Ok(anomalies)
    }

    /// Detect anomalies using IQR (Interquartile Range) method
    pub fn detect_iqr(&self, ts: &TimeSeries) -> Result<Vec<Anomaly>> {
        if ts.len() < 4 {
            return Err(TelemetryError::InsufficientData(
                "Need at least 4 data points for IQR detection".to_string(),
            ));
        }

        let mut sorted = ts.values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let n = sorted.len();
        let q1 = sorted[n / 4];
        let q3 = sorted[3 * n / 4];
        let iqr = q3 - q1;

        let lower_bound = q1 - self.iqr_multiplier * iqr;
        let upper_bound = q3 + self.iqr_multiplier * iqr;

        let mut anomalies = Vec::new();

        for (i, &value) in ts.values.iter().enumerate() {
            if value < lower_bound || value > upper_bound {
                let score = if value < lower_bound {
                    (lower_bound - value) / iqr
                } else {
                    (value - upper_bound) / iqr
                };

                anomalies.push(Anomaly {
                    index: i,
                    value,
                    anomaly_type: AnomalyType::Point,
                    score,
                });
            }
        }

        Ok(anomalies)
    }

    /// Detect anomalies using moving average deviation
    pub fn detect_moving_average(&self, ts: &TimeSeries, window: usize) -> Result<Vec<Anomaly>> {
        let ma = ts.moving_average(window)?;
        let mut anomalies = Vec::new();

        // Calculate deviations from moving average
        let offset = window / 2;
        for (i, &ma_value) in ma.iter().enumerate() {
            let actual_idx = i + offset;
            if actual_idx >= ts.values.len() {
                break;
            }

            let deviation = (ts.values[actual_idx] - ma_value).abs();
            let relative_dev = deviation / ma_value.abs().max(1e-10);

            if relative_dev > 0.5 {
                // 50% deviation threshold
                anomalies.push(Anomaly {
                    index: actual_idx,
                    value: ts.values[actual_idx],
                    anomaly_type: AnomalyType::Contextual,
                    score: relative_dev,
                });
            }
        }

        Ok(anomalies)
    }

    /// Detect anomalies using multiple methods and aggregate results
    pub fn detect_ensemble(&self, ts: &TimeSeries) -> Result<Vec<Anomaly>> {
        let zscore_anomalies = self.detect_zscore(ts)?;
        let iqr_anomalies = self.detect_iqr(ts)?;

        // Combine and deduplicate anomalies
        let mut all_indices: Vec<usize> = zscore_anomalies
            .iter()
            .chain(iqr_anomalies.iter())
            .map(|a| a.index)
            .collect();

        all_indices.sort_unstable();
        all_indices.dedup();

        let mut anomalies = Vec::new();
        for idx in all_indices {
            let value = ts.values[idx];
            let zscore_score = zscore_anomalies
                .iter()
                .find(|a| a.index == idx)
                .map(|a| a.score)
                .unwrap_or(0.0);

            let iqr_score = iqr_anomalies
                .iter()
                .find(|a| a.index == idx)
                .map(|a| a.score)
                .unwrap_or(0.0);

            // Average the scores
            let score = (zscore_score + iqr_score) / 2.0;

            anomalies.push(Anomaly {
                index: idx,
                value,
                anomaly_type: AnomalyType::Point,
                score,
            });
        }

        Ok(anomalies)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zscore_detection() {
        // Create data with more normal values and a clear outlier
        let mut data = vec![10.0, 12.0, 11.0, 13.0, 12.0, 10.0, 11.0, 12.0];
        data.push(100.0); // Clear outlier
        data.extend(vec![11.0, 12.0, 10.0]);

        let ts = TimeSeries::new(data);
        let detector = AnomalyDetector::default();

        let anomalies = detector.detect_zscore(&ts).unwrap();
        assert!(!anomalies.is_empty(), "Should detect at least one anomaly");
        // The 100.0 value should be detected as an anomaly
        let has_large_value_anomaly = anomalies.iter().any(|a| a.value > 50.0);
        assert!(
            has_large_value_anomaly,
            "Should detect the outlier value of 100.0"
        );
    }
    #[test]
    fn test_iqr_detection() {
        let data = vec![1.0, 2.0, 3.0, 2.0, 1.0, 100.0, 2.0, 1.0];
        let ts = TimeSeries::new(data);
        let detector = AnomalyDetector::default();

        let anomalies = detector.detect_iqr(&ts).unwrap();
        assert!(!anomalies.is_empty());
    }
}
