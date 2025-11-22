//! Time series decomposition (trend, seasonality, residuals)

use crate::{Result, TelemetryError, TimeSeries};

/// Result of time series decomposition
#[derive(Debug, Clone)]
pub struct DecompositionResult {
    /// Trend component
    pub trend: Vec<f64>,
    /// Seasonal component
    pub seasonal: Vec<f64>,
    /// Residual component
    pub residual: Vec<f64>,
}

/// Type of decomposition
#[derive(Debug, Clone, Copy)]
pub enum DecompositionType {
    /// Additive: Y = T + S + R
    Additive,
    /// Multiplicative: Y = T * S * R
    Multiplicative,
}

/// Time series decomposer
pub struct Decomposer {
    decomposition_type: DecompositionType,
    period: usize,
}

impl Decomposer {
    /// Create a new decomposer
    pub fn new(decomposition_type: DecompositionType, period: usize) -> Result<Self> {
        if period == 0 {
            return Err(TelemetryError::InvalidParameter(
                "Period must be greater than 0".to_string(),
            ));
        }

        Ok(Self {
            decomposition_type,
            period,
        })
    }

    /// Decompose a time series
    pub fn decompose(&self, ts: &TimeSeries) -> Result<DecompositionResult> {
        if ts.len() < 2 * self.period {
            return Err(TelemetryError::InsufficientData(format!(
                "Need at least {} data points for period {}",
                2 * self.period,
                self.period
            )));
        }

        // Calculate trend using centered moving average
        let trend = self.calculate_trend(ts)?;

        // Detrend the series
        let detrended = self.detrend(&ts.values, &trend)?;

        // Calculate seasonal component
        let seasonal = self.calculate_seasonal(&detrended)?;

        // Calculate residuals
        let residual = self.calculate_residual(&ts.values, &trend, &seasonal)?;

        Ok(DecompositionResult {
            trend,
            seasonal,
            residual,
        })
    }

    /// Calculate trend component using moving average
    fn calculate_trend(&self, ts: &TimeSeries) -> Result<Vec<f64>> {
        let mut trend = Vec::with_capacity(ts.len());
        let half_window = self.period / 2;

        // Pad the beginning
        for _ in 0..half_window {
            trend.push(f64::NAN);
        }

        // Calculate centered moving average
        for i in half_window..(ts.len() - half_window) {
            let sum: f64 = ts.values[i - half_window..=i + half_window].iter().sum();
            trend.push(sum / self.period as f64);
        }

        // Pad the end
        for _ in 0..half_window {
            trend.push(f64::NAN);
        }

        Ok(trend)
    }

    /// Remove trend from series
    fn detrend(&self, values: &[f64], trend: &[f64]) -> Result<Vec<f64>> {
        let detrended: Vec<f64> = values
            .iter()
            .zip(trend.iter())
            .map(|(&v, &t)| {
                if t.is_nan() {
                    f64::NAN
                } else {
                    match self.decomposition_type {
                        DecompositionType::Additive => v - t,
                        DecompositionType::Multiplicative => {
                            if t == 0.0 {
                                f64::NAN
                            } else {
                                v / t
                            }
                        }
                    }
                }
            })
            .collect();

        Ok(detrended)
    }

    /// Calculate seasonal component
    fn calculate_seasonal(&self, detrended: &[f64]) -> Result<Vec<f64>> {
        // Average values for each position in the period
        let mut seasonal_avgs = vec![0.0; self.period];
        let mut counts = vec![0; self.period];

        for (i, &value) in detrended.iter().enumerate() {
            if !value.is_nan() {
                let pos = i % self.period;
                seasonal_avgs[pos] += value;
                counts[pos] += 1;
            }
        }

        for i in 0..self.period {
            if counts[i] > 0 {
                seasonal_avgs[i] /= counts[i] as f64;
            }
        }

        // Normalize seasonal component
        match self.decomposition_type {
            DecompositionType::Additive => {
                let mean: f64 = seasonal_avgs.iter().sum::<f64>() / self.period as f64;
                seasonal_avgs.iter_mut().for_each(|v| *v -= mean);
            }
            DecompositionType::Multiplicative => {
                let mean: f64 = seasonal_avgs.iter().sum::<f64>() / self.period as f64;
                if mean != 0.0 {
                    seasonal_avgs.iter_mut().for_each(|v| *v /= mean);
                }
            }
        }

        // Repeat seasonal pattern to match series length
        let mut seasonal = Vec::with_capacity(detrended.len());
        for i in 0..detrended.len() {
            seasonal.push(seasonal_avgs[i % self.period]);
        }

        Ok(seasonal)
    }

    /// Calculate residual component
    fn calculate_residual(
        &self,
        values: &[f64],
        trend: &[f64],
        seasonal: &[f64],
    ) -> Result<Vec<f64>> {
        let residual: Vec<f64> = values
            .iter()
            .zip(trend.iter())
            .zip(seasonal.iter())
            .map(|((&v, &t), &s)| {
                if t.is_nan() {
                    f64::NAN
                } else {
                    match self.decomposition_type {
                        DecompositionType::Additive => v - t - s,
                        DecompositionType::Multiplicative => {
                            if t == 0.0 || s == 0.0 {
                                f64::NAN
                            } else {
                                v / (t * s)
                            }
                        }
                    }
                }
            })
            .collect();

        Ok(residual)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decomposition() {
        // Create a simple seasonal series
        let mut data = Vec::new();
        for i in 0..20 {
            let trend = i as f64;
            let seasonal = (i % 4) as f64;
            data.push(trend + seasonal);
        }

        let ts = TimeSeries::new(data);
        let decomposer = Decomposer::new(DecompositionType::Additive, 4).unwrap();

        let result = decomposer.decompose(&ts).unwrap();

        assert_eq!(result.trend.len(), ts.len());
        assert_eq!(result.seasonal.len(), ts.len());
        assert_eq!(result.residual.len(), ts.len());
    }
}
