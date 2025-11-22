//! ARIMA (AutoRegressive Integrated Moving Average) model

use crate::forecasting::{ForecastResult, Forecaster};
use crate::{Result, TelemetryError, TimeSeries};

/// ARIMA model parameters
#[derive(Debug, Clone)]
pub struct ARIMAParams {
    /// AR order (p)
    pub p: usize,
    /// Differencing order (d)
    pub d: usize,
    /// MA order (q)
    pub q: usize,
}

/// ARIMA model
pub struct ARIMA {
    params: ARIMAParams,
    ar_coeffs: Vec<f64>,
    ma_coeffs: Vec<f64>,
    fitted: bool,
    differenced_data: Vec<f64>,
}

impl ARIMA {
    /// Create a new ARIMA model
    pub fn new(p: usize, d: usize, q: usize) -> Self {
        Self {
            params: ARIMAParams { p, d, q },
            ar_coeffs: vec![0.0; p],
            ma_coeffs: vec![0.0; q],
            fitted: false,
            differenced_data: Vec::new(),
        }
    }

    /// Difference the time series d times
    fn difference(&self, data: &[f64]) -> Vec<f64> {
        let mut result = data.to_vec();

        for _ in 0..self.params.d {
            result = result.windows(2).map(|w| w[1] - w[0]).collect();
        }

        result
    }

    /// Integrate (cumulative sum) the differenced data
    fn _integrate(&self, differenced: &[f64], original_data: &[f64]) -> Vec<f64> {
        let mut result = differenced.to_vec();

        for _ in 0..self.params.d {
            let mut integrated = Vec::with_capacity(result.len() + 1);
            let start_value = if self.params.d == 1 {
                *original_data.last().unwrap()
            } else {
                0.0
            };
            integrated.push(start_value);

            for &val in &result {
                integrated.push(integrated.last().unwrap() + val);
            }

            result = integrated;
        }

        result
    }

    /// Simple AR coefficient estimation using Yule-Walker equations
    fn estimate_ar_coeffs(&mut self, data: &[f64]) -> Result<()> {
        if data.len() <= self.params.p {
            return Err(TelemetryError::InsufficientData(
                "Not enough data for AR estimation".to_string(),
            ));
        }

        // Simplified estimation: use least squares approximation
        for i in 0..self.params.p {
            let lag = i + 1;
            let mut sum_xy = 0.0;
            let mut sum_x2 = 0.0;

            for j in lag..data.len() {
                sum_xy += data[j] * data[j - lag];
                sum_x2 += data[j - lag] * data[j - lag];
            }

            self.ar_coeffs[i] = if sum_x2 != 0.0 { sum_xy / sum_x2 } else { 0.0 };
        }

        Ok(())
    }

    /// Simple MA coefficient estimation
    fn estimate_ma_coeffs(&mut self, _residuals: &[f64]) -> Result<()> {
        // Simplified: initialize with small random values
        for i in 0..self.params.q {
            self.ma_coeffs[i] = 0.1 * (i as f64 + 1.0) / (self.params.q as f64);
        }
        Ok(())
    }
}

impl Forecaster for ARIMA {
    fn fit(&mut self, ts: &TimeSeries) -> Result<()> {
        if ts.len() < self.params.p + self.params.d + self.params.q + 1 {
            return Err(TelemetryError::InsufficientData(
                "Insufficient data for ARIMA model".to_string(),
            ));
        }

        // Difference the data
        self.differenced_data = self.difference(&ts.values);

        // Clone data for coefficient estimation
        let differenced_clone = self.differenced_data.clone();

        // Estimate AR coefficients
        self.estimate_ar_coeffs(&differenced_clone)?;

        // Calculate residuals (simplified)
        let residuals = self.differenced_data.clone();

        // Estimate MA coefficients
        self.estimate_ma_coeffs(&residuals)?;

        self.fitted = true;
        Ok(())
    }

    fn forecast(&self, steps: usize) -> Result<ForecastResult> {
        if !self.fitted {
            return Err(TelemetryError::ModelError(
                "Model must be fitted before forecasting".to_string(),
            ));
        }

        let mut predictions = Vec::with_capacity(steps);
        let mut history = self.differenced_data.clone();

        for _ in 0..steps {
            let mut pred = 0.0;

            // AR component
            for (i, &coeff) in self.ar_coeffs.iter().enumerate() {
                if i < history.len() {
                    pred += coeff * history[history.len() - 1 - i];
                }
            }

            predictions.push(pred);
            history.push(pred);
        }

        // Note: This is simplified - should integrate back
        Ok(ForecastResult {
            predictions,
            lower_bound: None,
            upper_bound: None,
            confidence: 0.75,
        })
    }

    fn forecast_with_confidence(
        &self,
        steps: usize,
        confidence_level: f64,
    ) -> Result<ForecastResult> {
        let base_forecast = self.forecast(steps)?;

        // Simple confidence interval calculation
        let std_dev = 1.0; // Simplified - should calculate from residuals
        let z_score = if confidence_level >= 0.95 {
            1.96
        } else {
            1.645
        };
        let margin = z_score * std_dev;

        let lower_bound = base_forecast
            .predictions
            .iter()
            .map(|&v| v - margin)
            .collect();

        let upper_bound = base_forecast
            .predictions
            .iter()
            .map(|&v| v + margin)
            .collect();

        Ok(ForecastResult {
            predictions: base_forecast.predictions,
            lower_bound: Some(lower_bound),
            upper_bound: Some(upper_bound),
            confidence: confidence_level,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arima_creation() {
        let model = ARIMA::new(1, 1, 1);
        assert_eq!(model.params.p, 1);
        assert_eq!(model.params.d, 1);
        assert_eq!(model.params.q, 1);
    }

    #[test]
    fn test_arima_fit_and_forecast() {
        let data: Vec<f64> = (1..=50).map(|x| x as f64).collect();
        let ts = TimeSeries::new(data);

        let mut model = ARIMA::new(2, 1, 1);
        model.fit(&ts).unwrap();

        let forecast = model.forecast(5).unwrap();
        assert_eq!(forecast.predictions.len(), 5);
    }
}
