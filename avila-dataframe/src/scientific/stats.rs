//! Statistical tests and methods

use crate::core::{DataFrame, Series};
use crate::error::Result;

impl DataFrame {
    /// Kolmogorov-Smirnov test
    ///
    /// Tests if two distributions are the same
    ///
    /// # Returns
    /// Tuple of (statistic, p-value)
    pub fn kolmogorov_smirnov(&self, col1: &str, col2: &str) -> Result<(f64, f64)> {
        // TODO: Implement KS test
        Err(crate::error::AvilaError::not_implemented(
            "kolmogorov_smirnov",
        ))
    }

    /// Chi-square test for independence
    pub fn chi_square_test(&self, observed: &str, expected: &str) -> Result<(f64, f64)> {
        let obs = self.column(observed)?;
        let exp = self.column(expected)?;
        let n = obs.len();

        let mut chi_square = 0.0;
        for i in 0..n {
            let o = obs.get_f64(i)?;
            let e = exp.get_f64(i)?;
            if e > 0.0 {
                chi_square += (o - e).powi(2) / e;
            }
        }

        // TODO: Calculate p-value from chi-square distribution
        let p_value = 0.0; // Placeholder

        Ok((chi_square, p_value))
    }

    /// Anderson-Darling test for normality
    pub fn anderson_darling(&self, column: &str) -> Result<(f64, f64)> {
        // TODO: Implement Anderson-Darling test
        Err(crate::error::AvilaError::not_implemented(
            "anderson_darling",
        ))
    }

    /// Autocorrelation function
    pub fn autocorrelation(&self, column: &str, max_lag: usize) -> Result<Self> {
        let series = self.column(column)?;
        let n = series.len();
        let mean = series.mean()?;

        // Calculate variance
        let mut variance = 0.0;
        for i in 0..n {
            let diff = series.get_f64(i)? - mean;
            variance += diff * diff;
        }
        variance /= n as f64;

        // Calculate autocorrelation for each lag
        let mut acf = Vec::with_capacity(max_lag + 1);
        let lags: Vec<f64> = (0..=max_lag).map(|i| i as f64).collect();

        for lag in 0..=max_lag {
            let mut sum = 0.0;
            let count = n - lag;

            for i in 0..count {
                let x1 = series.get_f64(i)? - mean;
                let x2 = series.get_f64(i + lag)? - mean;
                sum += x1 * x2;
            }

            let correlation = sum / (count as f64 * variance);
            acf.push(correlation);
        }

        DataFrame::new(vec![Series::new("lag", lags), Series::new("acf", acf)])
    }

    /// Cross-correlation between two signals
    pub fn cross_correlation(&self, col1: &str, col2: &str, max_lag: usize) -> Result<Self> {
        let s1 = self.column(col1)?;
        let s2 = self.column(col2)?;
        let n = s1.len().min(s2.len());

        let mean1 = s1.mean()?;
        let mean2 = s2.mean()?;
        let std1 = s1.std()?;
        let std2 = s2.std()?;

        let mut ccf = Vec::with_capacity(2 * max_lag + 1);
        let mut lags_vec = Vec::with_capacity(2 * max_lag + 1);

        // Negative lags
        for lag in (1..=max_lag).rev() {
            let count = n - lag;
            let mut sum = 0.0;

            for i in 0..count {
                let x1 = (s1.get_f64(i + lag)? - mean1) / std1;
                let x2 = (s2.get_f64(i)? - mean2) / std2;
                sum += x1 * x2;
            }

            lags_vec.push(-(lag as f64));
            ccf.push(sum / count as f64);
        }

        // Zero and positive lags
        for lag in 0..=max_lag {
            let count = n - lag;
            let mut sum = 0.0;

            for i in 0..count {
                let x1 = (s1.get_f64(i)? - mean1) / std1;
                let x2 = (s2.get_f64(i + lag)? - mean2) / std2;
                sum += x1 * x2;
            }

            lags_vec.push(lag as f64);
            ccf.push(sum / count as f64);
        }

        DataFrame::new(vec![Series::new("lag", lags_vec), Series::new("ccf", ccf)])
    }

    /// Seasonal decomposition (additive model)
    pub fn seasonal_decompose(&self, column: &str, period: usize) -> Result<Self> {
        // TODO: Implement STL decomposition (Seasonal-Trend decomposition using LOESS)
        Err(crate::error::AvilaError::not_implemented(
            "seasonal_decompose",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_autocorrelation() {
        // Perfect positive correlation at lag 0
        let signal: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let df = DataFrame::new(vec![Series::new("signal", signal)]).unwrap();

        let acf = df.autocorrelation("signal", 2).unwrap();
        let acf_series = acf.column("acf").unwrap();

        // ACF at lag 0 should be 1.0
        assert_abs_diff_eq!(acf_series.get_f64(0).unwrap(), 1.0, epsilon = 0.001);
    }
}
