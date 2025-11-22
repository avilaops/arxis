//! Signal processing operations

use crate::core::{DataFrame, Series};
use crate::error::Result;

/// Resampling methods
#[derive(Debug, Clone, Copy)]
pub enum ResampleMethod {
    /// Linear interpolation
    Linear,
    /// Nearest neighbor
    Nearest,
    /// Cubic spline
    Cubic,
    /// FFT-based sinc interpolation
    Sinc,
}

/// Filter types
#[derive(Debug, Clone, Copy)]
pub enum FilterType {
    /// Butterworth filter
    Butterworth { cutoff: f64, order: usize },
    /// Chebyshev Type I
    Chebyshev1 {
        cutoff: f64,
        order: usize,
        ripple: f64,
    },
    /// Bessel filter
    Bessel { cutoff: f64, order: usize },
    /// Moving average
    MovingAverage { window: usize },
}

impl DataFrame {
    /// Apply Butterworth filter
    ///
    /// # Arguments
    /// * `column` - Signal column
    /// * `cutoff` - Cutoff frequency (normalized 0-1)
    /// * `order` - Filter order
    ///
    /// # Examples
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// # fn main() -> Result<()> {
    /// let df = DataFrame::new(vec![
    ///     Series::new("signal", vec![1.0, 2.0, 3.0, 2.0, 1.0]),
    /// ])?;
    ///
    /// let filtered = df.filter_butterworth("signal", 0.1, 4)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn filter_butterworth(&self, column: &str, cutoff: f64, order: usize) -> Result<Self> {
        // TODO: Implement IIR Butterworth filter
        // For now, simple moving average as placeholder
        self.moving_average(column, order)
    }

    /// Resample signal to new sampling rate
    pub fn resample(&self, column: &str, new_rate: f64, method: ResampleMethod) -> Result<Self> {
        // TODO: Implement proper resampling with interpolation
        Err(crate::error::AvilaError::not_implemented("resample"))
    }

    /// Apply rolling window operation
    pub fn rolling_window(&self, column: &str, window: usize) -> RollingWindow {
        RollingWindow {
            df: self.clone(),
            column: column.to_string(),
            window,
        }
    }

    /// Detrend signal (remove linear trend)
    pub fn detrend(&self, column: &str) -> Result<Self> {
        let series = self.column(column)?;
        let n = series.len();

        // Calculate linear trend
        let mean_x = (n - 1) as f64 / 2.0;
        let mut mean_y = 0.0;
        for i in 0..n {
            mean_y += series.get_f64(i)?;
        }
        mean_y /= n as f64;

        // Calculate slope
        let mut numerator = 0.0;
        let mut denominator = 0.0;
        for i in 0..n {
            let x = i as f64 - mean_x;
            let y = series.get_f64(i)? - mean_y;
            numerator += x * y;
            denominator += x * x;
        }
        let slope = numerator / denominator;
        let intercept = mean_y - slope * mean_x;

        // Remove trend
        let detrended: Vec<f64> = (0..n)
            .map(|i| {
                let y = series.get_f64(i).unwrap_or(0.0);
                let trend = slope * i as f64 + intercept;
                y - trend
            })
            .collect();

        let mut result = self.clone();
        result = result.with_column(Series::new(column, detrended))?;
        Ok(result)
    }

    /// Simple moving average (helper)
    fn moving_average(&self, column: &str, window: usize) -> Result<Self> {
        let series = self.column(column)?;
        let n = series.len();
        let half_window = window / 2;

        let smoothed: Vec<f64> = (0..n)
            .map(|i| {
                let start = i.saturating_sub(half_window);
                let end = (i + half_window + 1).min(n);
                let mut sum = 0.0;
                let mut count = 0;

                for j in start..end {
                    if let Ok(val) = series.get_f64(j) {
                        sum += val;
                        count += 1;
                    }
                }

                if count > 0 {
                    sum / count as f64
                } else {
                    0.0
                }
            })
            .collect();

        let mut result = self.clone();
        result = result.with_column(Series::new(column, smoothed))?;
        Ok(result)
    }
}

/// Rolling window builder
pub struct RollingWindow {
    df: DataFrame,
    column: String,
    window: usize,
}

impl RollingWindow {
    /// Apply function to rolling windows
    pub fn apply<F>(&self, func: F) -> Result<DataFrame>
    where
        F: Fn(&[f64]) -> f64,
    {
        let series = self.df.column(&self.column)?;
        let n = series.len();

        let mut result = Vec::with_capacity(n);

        for i in 0..n {
            let start = i.saturating_sub(self.window / 2);
            let end = (i + self.window / 2 + 1).min(n);

            let window_data: Vec<f64> = (start..end)
                .filter_map(|j| series.get_f64(j).ok())
                .collect();

            result.push(func(&window_data));
        }

        let mut df = self.df.clone();
        df = df.with_column(Series::new(&format!("{}_rolled", self.column), result))?;
        Ok(df)
    }

    /// Rolling mean
    pub fn mean(&self) -> Result<DataFrame> {
        self.apply(|window| window.iter().sum::<f64>() / window.len() as f64)
    }

    /// Rolling standard deviation
    pub fn std(&self) -> Result<DataFrame> {
        self.apply(|window| {
            let mean = window.iter().sum::<f64>() / window.len() as f64;
            let variance =
                window.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / window.len() as f64;
            variance.sqrt()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detrend() {
        // Create signal with linear trend
        let signal: Vec<f64> = (0..100).map(|i| i as f64 * 2.0 + 10.0).collect();
        let df = DataFrame::new(vec![Series::new("signal", signal)]).unwrap();

        let detrended = df.detrend("signal").unwrap();
        let result_series = detrended.column("signal").unwrap();

        // After detrending, mean should be close to 0
        let mean = result_series.mean().unwrap();
        assert!(mean.abs() < 1.0);
    }
}
