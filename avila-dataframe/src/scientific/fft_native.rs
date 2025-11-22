//! FFT (Fast Fourier Transform) nativo
//!
//! Implementação 100% Rust para análise espectral de sinais,
//! ideal para astronomia, análise de ondas gravitacionais, e processamento de sinais.

use crate::error::{AvilaError, Result};
use rustfft::{num_complex::Complex, FftPlanner};
use std::f64::consts::PI;

/// Tipos de janelas para FFT
#[derive(Debug, Clone, Copy)]
pub enum WindowType {
    /// Sem janela (retângulo)
    None,
    /// Janela de Hann (suave, boa para maioria dos casos)
    Hann,
    /// Janela de Hamming
    Hamming,
    /// Janela de Blackman (melhor rejeição de lóbulos laterais)
    Blackman,
}

/// Aplica janela ao sinal
fn apply_window(signal: &[f64], window: WindowType) -> Vec<f64> {
    let n = signal.len();
    match window {
        WindowType::None => signal.to_vec(),
        WindowType::Hann => signal
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let w = 0.5 * (1.0 - (2.0 * PI * i as f64 / (n - 1) as f64).cos());
                x * w
            })
            .collect(),
        WindowType::Hamming => signal
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let w = 0.54 - 0.46 * (2.0 * PI * i as f64 / (n - 1) as f64).cos();
                x * w
            })
            .collect(),
        WindowType::Blackman => signal
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                let a0 = 0.42;
                let a1 = 0.5;
                let a2 = 0.08;
                let w = a0 - a1 * (2.0 * PI * i as f64 / (n - 1) as f64).cos()
                    + a2 * (4.0 * PI * i as f64 / (n - 1) as f64).cos();
                x * w
            })
            .collect(),
    }
}

/// Calcula FFT de um sinal real
///
/// # Argumentos
/// * `signal` - Vetor de valores reais (f64)
/// * `window` - Tipo de janela a aplicar (None, Hann, Hamming, Blackman)
///
/// # Retorna
/// Vetor de magnitudes do espectro (metade do tamanho do sinal)
///
/// # Exemplo
/// ```ignore
/// use avila_dataframe::scientific::fft::{fft, WindowType};
///
/// let signal = vec![1.0, 2.0, 1.0, -1.0, -2.0, -1.0];
/// let spectrum = fft(&signal, Some(WindowType::Hann))?;
/// ```
pub fn fft(signal: &[f64], window: Option<WindowType>) -> Result<Vec<f64>> {
    if signal.is_empty() {
        return Err(AvilaError::InvalidInput("Signal cannot be empty".into()));
    }

    // Aplica janela se especificado
    let windowed = if let Some(w) = window {
        apply_window(signal, w)
    } else {
        signal.to_vec()
    };

    // Converte para Complex
    let mut buffer: Vec<Complex<f64>> = windowed.iter().map(|&x| Complex::new(x, 0.0)).collect();

    // Calcula FFT
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(buffer.len());
    fft.process(&mut buffer);

    // Retorna apenas metade (frequências positivas) e calcula magnitude
    let half = buffer.len() / 2;
    Ok(buffer[..half]
        .iter()
        .map(|c| (c.re * c.re + c.im * c.im).sqrt())
        .collect())
}

/// Calcula Power Spectral Density (PSD)
///
/// # Argumentos
/// * `signal` - Vetor de valores reais
/// * `sample_rate` - Taxa de amostragem em Hz
/// * `window` - Tipo de janela
///
/// # Retorna
/// Vetor de valores PSD normalizados
///
/// # Exemplo
/// ```ignore
/// use avila_dataframe::scientific::fft::{power_spectral_density, WindowType};
///
/// let signal = vec![1.0, 2.0, 1.0, -1.0, -2.0, -1.0];
/// let psd = power_spectral_density(&signal, 1000.0, Some(WindowType::Hann))?;
/// ```
pub fn power_spectral_density(
    signal: &[f64],
    sample_rate: f64,
    window: Option<WindowType>,
) -> Result<Vec<f64>> {
    if sample_rate <= 0.0 {
        return Err(AvilaError::InvalidInput(
            "Sample rate must be positive".into(),
        ));
    }

    let spectrum = fft(signal, window)?;
    let n = signal.len();

    // Normaliza pela taxa de amostragem e tamanho
    Ok(spectrum
        .iter()
        .map(|&mag| {
            let power = mag * mag;
            2.0 * power / (n as f64 * sample_rate)
        })
        .collect())
}

/// Calcula vetor de frequências correspondente ao espectro
///
/// # Argumentos
/// * `n_samples` - Número de amostras no sinal original
/// * `sample_rate` - Taxa de amostragem em Hz
///
/// # Retorna
/// Vetor de frequências em Hz
///
/// # Exemplo
/// ```ignore
/// use avila_dataframe::scientific::fft::frequency_vector;
///
/// let freqs = frequency_vector(1024, 4096.0);
/// assert_eq!(freqs.len(), 512); // Metade das amostras
/// ```
pub fn frequency_vector(n_samples: usize, sample_rate: f64) -> Vec<f64> {
    let n_freq = n_samples / 2;
    let freq_resolution = sample_rate / n_samples as f64;

    (0..n_freq).map(|i| i as f64 * freq_resolution).collect()
}

/// Encontra o pico dominante no espectro
///
/// # Retorna
/// (índice, frequência, magnitude) do pico
pub fn find_peak(spectrum: &[f64], sample_rate: f64, n_samples: usize) -> (usize, f64, f64) {
    let mut max_idx = 0;
    let mut max_val = 0.0;

    for (i, &val) in spectrum.iter().enumerate() {
        if val > max_val {
            max_val = val;
            max_idx = i;
        }
    }

    let freq_resolution = sample_rate / n_samples as f64;
    let peak_freq = max_idx as f64 * freq_resolution;

    (max_idx, peak_freq, max_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_simple() {
        // Sinal simples: DC component
        let signal = vec![1.0; 16];
        let spectrum = fft(&signal, None).unwrap();

        // DC deve ser o maior componente
        assert!(spectrum[0] > spectrum[1]);
    }

    #[test]
    fn test_window_application() {
        let signal = vec![1.0; 10];
        let windowed = apply_window(&signal, WindowType::Hann);

        // Janela Hann deve ter valores menores nas bordas
        assert!(windowed[0] < 0.1);
        assert!(windowed[5] > 0.9);
    }

    #[test]
    fn test_frequency_vector() {
        let freqs = frequency_vector(100, 1000.0);
        assert_eq!(freqs.len(), 50);
        assert_eq!(freqs[0], 0.0);
        assert!((freqs[1] - 10.0).abs() < 1e-6);
    }
}
