//! Feature engineering for time series

use crate::{Result, TelemetryError, TimeSeries};

/// Feature extractor for time series data
pub struct FeatureExtractor;

impl FeatureExtractor {
    /// Create lag features
    pub fn create_lag_features(ts: &TimeSeries, lags: &[usize]) -> Result<Vec<Vec<f64>>> {
        if lags.is_empty() {
            return Err(TelemetryError::InvalidParameter(
                "Must specify at least one lag".to_string(),
            ));
        }

        let max_lag = *lags.iter().max().unwrap();
        if max_lag >= ts.len() {
            return Err(TelemetryError::InvalidParameter(
                "Lag is too large for the time series".to_string(),
            ));
        }

        let mut features = Vec::new();

        for &lag in lags {
            let mut lag_feature = Vec::new();
            for i in lag..ts.len() {
                lag_feature.push(ts.values[i - lag]);
            }
            features.push(lag_feature);
        }

        Ok(features)
    }

    /// Calculate rolling statistics (mean, std, min, max)
    pub fn rolling_statistics(ts: &TimeSeries, window: usize) -> Result<RollingStats> {
        if window == 0 {
            return Err(TelemetryError::InvalidParameter(
                "Window size must be greater than 0".to_string(),
            ));
        }

        if window > ts.len() {
            return Err(TelemetryError::InsufficientData(
                "Window size is larger than series length".to_string(),
            ));
        }

        let mut means = Vec::new();
        let mut stds = Vec::new();
        let mut mins = Vec::new();
        let mut maxs = Vec::new();

        for i in 0..=(ts.len() - window) {
            let window_data = &ts.values[i..i + window];

            let mean = window_data.iter().sum::<f64>() / window as f64;
            means.push(mean);

            let variance =
                window_data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / window as f64;
            stds.push(variance.sqrt());

            mins.push(
                *window_data
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            );

            maxs.push(
                *window_data
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap(),
            );
        }

        Ok(RollingStats {
            means,
            stds,
            mins,
            maxs,
        })
    }

    /// Extract trend features using linear regression
    pub fn trend_features(ts: &TimeSeries, window: usize) -> Result<Vec<f64>> {
        if window < 2 {
            return Err(TelemetryError::InvalidParameter(
                "Window size must be at least 2 for trend calculation".to_string(),
            ));
        }

        let mut trends = Vec::new();

        for i in 0..=(ts.len() - window) {
            let window_data = &ts.values[i..i + window];
            let slope = Self::calculate_slope(window_data);
            trends.push(slope);
        }

        Ok(trends)
    }

    /// Calculate slope using simple linear regression
    fn calculate_slope(data: &[f64]) -> f64 {
        let n = data.len() as f64;
        let x_mean = (n - 1.0) / 2.0;
        let y_mean = data.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for (i, &y) in data.iter().enumerate() {
            let x = i as f64;
            numerator += (x - x_mean) * (y - y_mean);
            denominator += (x - x_mean).powi(2);
        }

        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }

    /// Extract rate of change features
    pub fn rate_of_change(ts: &TimeSeries, periods: usize) -> Result<Vec<f64>> {
        if periods == 0 {
            return Err(TelemetryError::InvalidParameter(
                "Periods must be greater than 0".to_string(),
            ));
        }

        if periods >= ts.len() {
            return Err(TelemetryError::InsufficientData(
                "Periods is too large for the time series".to_string(),
            ));
        }

        let mut roc = Vec::new();

        for i in periods..ts.len() {
            let change = if ts.values[i - periods] == 0.0 {
                0.0
            } else {
                (ts.values[i] - ts.values[i - periods]) / ts.values[i - periods]
            };
            roc.push(change);
        }

        Ok(roc)
    }
}

/// Rolling statistics result
#[derive(Debug, Clone)]
pub struct RollingStats {
    pub means: Vec<f64>,
    pub stds: Vec<f64>,
    pub mins: Vec<f64>,
    pub maxs: Vec<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lag_features() {
        let ts = TimeSeries::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let features = FeatureExtractor::create_lag_features(&ts, &[1, 2]).unwrap();

        assert_eq!(features.len(), 2);
        assert_eq!(features[0], vec![1.0, 2.0, 3.0, 4.0]);
        assert_eq!(features[1], vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_rolling_statistics() {
        let ts = TimeSeries::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let stats = FeatureExtractor::rolling_statistics(&ts, 3).unwrap();

        assert_eq!(stats.means.len(), 3);
        assert_eq!(stats.means[0], 2.0);
    }
}
