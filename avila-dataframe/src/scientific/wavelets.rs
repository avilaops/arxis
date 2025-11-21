//! Wavelet transform implementation

use crate::core::{DataFrame, Series};
use crate::error::Result;
use std::f64::consts::PI;

/// Wavelet types
#[derive(Debug, Clone, Copy)]
pub enum WaveletType {
    /// Morlet wavelet (Gabor wavelet)
    Morlet,
    /// Mexican Hat (Ricker) wavelet
    MexicanHat,
    /// Daubechies wavelets
    Daubechies(usize),
    /// Haar wavelet
    Haar,
}

impl DataFrame {
    /// Continuous Wavelet Transform (CWT)
    ///
    /// # Arguments
    /// * `column` - Signal column name
    /// * `wavelet` - Type of wavelet to use
    /// * `scales` - Number of scales to compute
    ///
    /// # Examples
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// # fn main() -> Result<()> {
    /// let df = DataFrame::new(vec![
    ///     Series::new("strain_h", vec![1.0e-21, 1.5e-21, 1.2e-21]),
    /// ])?;
    ///
    /// let cwt = df.wavelet_transform("strain_h", WaveletType::Morlet, 128)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn wavelet_transform(
        &self,
        column: &str,
        wavelet: WaveletType,
        scales: usize,
    ) -> Result<Self> {
        let series = self.column(column)?;
        let n = series.len();

        // Get signal
        let signal: Vec<f64> = (0..n).map(|i| series.get_f64(i).unwrap_or(0.0)).collect();

        // Compute CWT at different scales
        let mut coefficients = Vec::with_capacity(n * scales);

        for scale in 1..=scales {
            let scale_f = scale as f64;
            for i in 0..n {
                let coef = cwt_at_position(&signal, i, scale_f, wavelet);
                coefficients.push(coef);
            }
        }

        // For now, return flattened coefficients
        // TODO: Return proper 2D structure (scale x time)
        DataFrame::new(vec![Series::new("cwt_coefficients", coefficients)])
    }

    /// Wavelet coherence between two signals
    pub fn wavelet_coherence(
        &self,
        signal1: &str,
        signal2: &str,
        wavelet: WaveletType,
        scales: usize,
    ) -> Result<Self> {
        // TODO: Implement wavelet coherence
        Err(crate::error::AvilaError::not_implemented(
            "wavelet_coherence",
        ))
    }

    /// Discrete Wavelet Transform (DWT)
    pub fn dwt(&self, column: &str, wavelet: WaveletType, level: usize) -> Result<Self> {
        // TODO: Implement DWT using filter banks
        Err(crate::error::AvilaError::not_implemented("dwt"))
    }
}

/// Compute CWT coefficient at a specific position and scale
fn cwt_at_position(signal: &[f64], pos: usize, scale: f64, wavelet: WaveletType) -> f64 {
    let n = signal.len();
    let mut coef = 0.0;

    for i in 0..n {
        let t = (i as f64 - pos as f64) / scale;
        let w = wavelet_function(t, wavelet);
        coef += signal[i] * w;
    }

    coef / scale.sqrt()
}

/// Wavelet function
fn wavelet_function(t: f64, wavelet: WaveletType) -> f64 {
    match wavelet {
        WaveletType::Morlet => {
            // Morlet wavelet: exp(-t²/2) * cos(5t)
            let omega = 5.0;
            (-t * t / 2.0).exp() * (omega * t).cos()
        }
        WaveletType::MexicanHat => {
            // Mexican Hat (Ricker): (1 - t²) * exp(-t²/2)
            let t2 = t * t;
            (1.0 - t2) * (-t2 / 2.0).exp()
        }
        WaveletType::Haar => {
            // Haar wavelet
            if t >= 0.0 && t < 0.5 {
                1.0
            } else if t >= 0.5 && t < 1.0 {
                -1.0
            } else {
                0.0
            }
        }
        WaveletType::Daubechies(_order) => {
            // TODO: Implement Daubechies wavelets
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_morlet_wavelet() {
        // Morlet at t=0 should be maximum
        let w0 = wavelet_function(0.0, WaveletType::Morlet);
        assert!(w0 > 0.9);

        // Should decay away from center
        let w5 = wavelet_function(5.0, WaveletType::Morlet);
        assert!(w5.abs() < 0.1);
    }

    #[test]
    fn test_mexican_hat() {
        let w0 = wavelet_function(0.0, WaveletType::MexicanHat);
        assert_eq!(w0, 1.0);
    }
}
