//! Forecasting models and algorithms

use crate::{Result, TelemetryError, TimeSeries};

/// Result of a forecast operation
#[derive(Debug, Clone)]
pub struct ForecastResult {
    /// Predicted values
    pub predictions: Vec<f64>,
    /// Lower confidence bound (if available)
    pub lower_bound: Option<Vec<f64>>,
    /// Upper confidence bound (if available)
    pub upper_bound: Option<Vec<f64>>,
    /// Model confidence/accuracy metrics
    pub confidence: f64,
}

/// Forecaster trait for different forecasting models
pub trait Forecaster {
    /// Fit the model to historical data
    fn fit(&mut self, ts: &TimeSeries) -> Result<()>;

    /// Forecast n steps ahead
    fn forecast(&self, steps: usize) -> Result<ForecastResult>;

    /// Forecast with confidence intervals
    fn forecast_with_confidence(
        &self,
        steps: usize,
        confidence_level: f64,
    ) -> Result<ForecastResult>;
}

/// Simple exponential smoothing forecaster
pub struct ExponentialSmoothing {
    alpha: f64,
    last_value: Option<f64>,
    fitted: bool,
}

impl ExponentialSmoothing {
    /// Create a new exponential smoothing model
    pub fn new(alpha: f64) -> Result<Self> {
        if alpha <= 0.0 || alpha > 1.0 {
            return Err(TelemetryError::InvalidParameter(
                "Alpha must be between 0 and 1".to_string(),
            ));
        }

        Ok(Self {
            alpha,
            last_value: None,
            fitted: false,
        })
    }
}

impl Forecaster for ExponentialSmoothing {
    fn fit(&mut self, ts: &TimeSeries) -> Result<()> {
        if ts.is_empty() {
            return Err(TelemetryError::InsufficientData(
                "Cannot fit on empty time series".to_string(),
            ));
        }

        // Calculate the final smoothed value
        let mut smoothed = ts.values[0];
        for &value in ts.values.iter().skip(1) {
            smoothed = self.alpha * value + (1.0 - self.alpha) * smoothed;
        }

        self.last_value = Some(smoothed);
        self.fitted = true;
        Ok(())
    }

    fn forecast(&self, steps: usize) -> Result<ForecastResult> {
        if !self.fitted {
            return Err(TelemetryError::ModelError(
                "Model must be fitted before forecasting".to_string(),
            ));
        }

        let last_value = self.last_value.unwrap();
        let predictions = vec![last_value; steps];

        Ok(ForecastResult {
            predictions,
            lower_bound: None,
            upper_bound: None,
            confidence: 0.7,
        })
    }

    fn forecast_with_confidence(
        &self,
        steps: usize,
        confidence_level: f64,
    ) -> Result<ForecastResult> {
        let base_forecast = self.forecast(steps)?;
        let last_value = self.last_value.unwrap();

        // Simple confidence interval based on confidence level
        let margin = last_value * (1.0 - confidence_level) * 0.5;

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

/// Moving average forecaster
pub struct MovingAverageForecaster {
    window: usize,
    history: Option<Vec<f64>>,
}

impl MovingAverageForecaster {
    /// Create a new moving average forecaster
    pub fn new(window: usize) -> Result<Self> {
        if window == 0 {
            return Err(TelemetryError::InvalidParameter(
                "Window size must be greater than 0".to_string(),
            ));
        }

        Ok(Self {
            window,
            history: None,
        })
    }
}

impl Forecaster for MovingAverageForecaster {
    fn fit(&mut self, ts: &TimeSeries) -> Result<()> {
        if ts.len() < self.window {
            return Err(TelemetryError::InsufficientData(format!(
                "Need at least {} data points",
                self.window
            )));
        }

        self.history = Some(ts.values.clone());
        Ok(())
    }

    fn forecast(&self, steps: usize) -> Result<ForecastResult> {
        let history = self.history.as_ref().ok_or_else(|| {
            TelemetryError::ModelError("Model must be fitted before forecasting".to_string())
        })?;

        // Use last 'window' values to predict
        let last_values = &history[history.len() - self.window..];
        let avg: f64 = last_values.iter().sum::<f64>() / self.window as f64;

        let predictions = vec![avg; steps];

        Ok(ForecastResult {
            predictions,
            lower_bound: None,
            upper_bound: None,
            confidence: 0.6,
        })
    }

    fn forecast_with_confidence(
        &self,
        steps: usize,
        confidence_level: f64,
    ) -> Result<ForecastResult> {
        let base_forecast = self.forecast(steps)?;
        let history = self.history.as_ref().unwrap();

        // Calculate std dev of recent values
        let last_values = &history[history.len() - self.window..];
        let mean: f64 = last_values.iter().sum::<f64>() / self.window as f64;
        let variance: f64 =
            last_values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / self.window as f64;
        let std_dev = variance.sqrt();

        // Use std dev for confidence intervals
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
    fn test_exponential_smoothing() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ts = TimeSeries::new(data);

        let mut model = ExponentialSmoothing::new(0.5).unwrap();
        model.fit(&ts).unwrap();

        let forecast = model.forecast(3).unwrap();
        assert_eq!(forecast.predictions.len(), 3);
    }

    #[test]
    fn test_moving_average_forecaster() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ts = TimeSeries::new(data);

        let mut model = MovingAverageForecaster::new(3).unwrap();
        model.fit(&ts).unwrap();

        let forecast = model.forecast(2).unwrap();
        assert_eq!(forecast.predictions.len(), 2);
    }
}
