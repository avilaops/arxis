//! FFT (Fast Fourier Transform) implementation with SIMD optimization

use crate::core::{DataFrame, Series};
use crate::error::Result;
use rustfft::{num_complex::Complex, FftPlanner};
use std::f64::consts::PI;

/// Window types for FFT
#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    /// Rectangular window (no windowing)
    Rectangular,
    /// Hann window
    Hann,
    /// Hamming window
    Hamming,
    /// Blackman window
    Blackman,
    /// Tukey window
    Tukey(f64),
}

/// Standalone FFT function
pub fn fft(signal: &[f64], window: Option<WindowType>) -> Result<Vec<Complex<f64>>> {
    let n = signal.len();
    let mut complex_signal: Vec<Complex<f64>> =
        signal.iter().map(|&x| Complex::new(x, 0.0)).collect();

    // Apply window
    if let Some(w) = window {
        apply_window(&mut complex_signal, w);
    }

    let mut planner = FftPlanner::new();
    let fft_plan = planner.plan_fft_forward(n);
    fft_plan.process(&mut complex_signal);

    Ok(complex_signal)
}

/// Standalone power spectral density function
pub fn power_spectral_density(
    signal: &[f64],
    sample_rate: f64,
    window: Option<WindowType>,
) -> Result<Vec<f64>> {
    let spectrum = fft(signal, window)?;
    let n = signal.len();

    let psd: Vec<f64> = spectrum
        .iter()
        .map(|c| (c.norm().powi(2)) / n as f64)
        .collect();

    Ok(psd)
}

fn apply_window(signal: &mut [Complex<f64>], window: WindowType) {
    let n = signal.len();
    for (i, s) in signal.iter_mut().enumerate() {
        let w = match window {
            WindowType::Rectangular => 1.0,
            WindowType::Hann => 0.5 - 0.5 * (2.0 * PI * i as f64 / (n - 1) as f64).cos(),
            WindowType::Hamming => 0.54 - 0.46 * (2.0 * PI * i as f64 / (n - 1) as f64).cos(),
            WindowType::Blackman => {
                0.42 - 0.5 * (2.0 * PI * i as f64 / (n - 1) as f64).cos()
                    + 0.08 * (4.0 * PI * i as f64 / (n - 1) as f64).cos()
            }
            WindowType::Tukey(alpha) => {
                let n_taper = (alpha * (n - 1) as f64 / 2.0) as usize;
                if i < n_taper {
                    0.5 * (1.0 - (PI * i as f64 / n_taper as f64).cos())
                } else if i >= n - n_taper {
                    0.5 * (1.0 - (PI * (n - 1 - i) as f64 / n_taper as f64).cos())
                } else {
                    1.0
                }
            }
        };
        *s *= w;
    }
}

impl DataFrame {
    /// Compute FFT of a signal column
    ///
    /// # Arguments
    /// * `column` - Name of the column containing the signal
    /// * `window` - Window function to apply
    ///
    /// # Returns
    /// DataFrame with FFT spectrum (magnitude and phase)
    ///
    /// # Examples
    /// ```no_run
    /// # use avila_dataframe::prelude::*;
    /// # fn main() -> Result<()> {
    /// let df = DataFrame::new(vec![
    ///     Series::new("signal", vec![1.0, 2.0, 3.0, 4.0]),
    /// ])?;
    ///
    /// let spectrum = df.fft("signal", WindowType::Hann)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fft(&self, column: &str, window: WindowType) -> Result<Self> {
        let series = self.column(column)?;
        let n = series.len();

        // Get signal data
        let mut signal: Vec<Complex<f64>> = Vec::with_capacity(n);
        for i in 0..n {
            let value = series.get_f64(i)?;
            let windowed = value * window_function(i, n, window);
            signal.push(Complex::new(windowed, 0.0));
        }

        // Compute FFT
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(n);
        fft.process(&mut signal);

        // Extract magnitude and phase
        let magnitude: Vec<f64> = signal.iter().map(|c| c.norm()).collect();
        let phase: Vec<f64> = signal.iter().map(|c| c.arg()).collect();
        let frequency: Vec<f64> = (0..n).map(|i| i as f64 / n as f64).collect();

        DataFrame::new(vec![
            Series::new("frequency", frequency),
            Series::new("magnitude", magnitude),
            Series::new("phase", phase),
        ])
    }

    /// Compute inverse FFT
    pub fn inverse_fft(&self, magnitude_col: &str, phase_col: &str) -> Result<Self> {
        let mag_series = self.column(magnitude_col)?;
        let phase_series = self.column(phase_col)?;
        let n = mag_series.len();

        // Reconstruct complex signal
        let mut signal: Vec<Complex<f64>> = Vec::with_capacity(n);
        for i in 0..n {
            let mag = mag_series.get_f64(i)?;
            let phase = phase_series.get_f64(i)?;
            signal.push(Complex::from_polar(mag, phase));
        }

        // Compute inverse FFT
        let mut planner = FftPlanner::new();
        let ifft = planner.plan_fft_inverse(n);
        ifft.process(&mut signal);

        // Extract real part and normalize
        let real: Vec<f64> = signal.iter().map(|c| c.re / n as f64).collect();

        DataFrame::new(vec![Series::new("signal", real)])
    }

    /// Compute power spectral density
    pub fn power_spectral_density(&self) -> Result<Self> {
        let mag = self.column("magnitude")?;
        let n = mag.len();

        let psd: Vec<f64> = (0..n)
            .map(|i| {
                let m = mag.get_f64(i).unwrap_or(0.0);
                m * m / n as f64
            })
            .collect();

        let freq = self.column("frequency")?.clone();

        DataFrame::new(vec![freq, Series::new("psd", psd)])
    }

    /// Compute spectrogram (Short-Time Fourier Transform)
    pub fn spectrogram(&self, column: &str, nperseg: usize, window: WindowType) -> Result<Self> {
        // TODO: Implement STFT with overlapping windows
        // For now, return placeholder
        Err(crate::error::AvilaError::not_implemented("spectrogram"))
    }
}

/// Apply window function
fn window_function(i: usize, n: usize, window: WindowType) -> f64 {
    match window {
        WindowType::Rectangular => 1.0,
        WindowType::Hann => 0.5 * (1.0 - (2.0 * PI * i as f64 / n as f64).cos()),
        WindowType::Hamming => 0.54 - 0.46 * (2.0 * PI * i as f64 / n as f64).cos(),
        WindowType::Blackman => {
            0.42 - 0.5 * (2.0 * PI * i as f64 / n as f64).cos()
                + 0.08 * (4.0 * PI * i as f64 / n as f64).cos()
        }
        WindowType::Tukey(alpha) => {
            let n_f = n as f64;
            let i_f = i as f64;
            if i_f < alpha * n_f / 2.0 {
                0.5 * (1.0 + (2.0 * PI * i_f / (alpha * n_f) - PI).cos())
            } else if i_f > n_f * (1.0 - alpha / 2.0) {
                0.5 * (1.0
                    + (2.0 * PI * (i_f - n_f * (1.0 - alpha / 2.0)) / (alpha * n_f) - PI).cos())
            } else {
                1.0
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_window_hann() {
        let n = 100;
        let mid = n / 2;

        // Hann window should be ~0.5 at the center
        let w = window_function(mid, n, WindowType::Hann);
        assert!(w > 0.99 && w <= 1.0);

        // Should be ~0 at edges
        let w0 = window_function(0, n, WindowType::Hann);
        assert!(w0 < 0.01);
    }

    #[test]
    fn test_fft_basic() {
        // Create a simple sine wave
        let n = 256;
        let freq = 10.0; // 10 Hz
        let signal: Vec<f64> = (0..n)
            .map(|i| (2.0 * PI * freq * i as f64 / n as f64).sin())
            .collect();

        let df = DataFrame::new(vec![Series::new("signal", signal)]).unwrap();

        let spectrum = df.fft("signal", WindowType::Hann).unwrap();

        // Should have peaks at frequency bins corresponding to 10 Hz
        assert_eq!(spectrum.height(), n);
    }
}
