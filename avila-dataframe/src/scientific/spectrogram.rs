use super::fft_pure::{fft_cooley_tukey, ifft, rfft};
use super::complex::Complex;

/// Tipos de janelas para windowing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowType {
    Rectangular,
    Hann,
    Hamming,
    Blackman,
    Kaiser(f64), // Parâmetro beta
}

/// Gera uma janela de um tipo específico
pub fn generate_window(size: usize, window_type: WindowType) -> Vec<f64> {
    match window_type {
        WindowType::Rectangular => vec![1.0; size],
        WindowType::Hann => hann_window(size),
        WindowType::Hamming => hamming_window(size),
        WindowType::Blackman => blackman_window(size),
        WindowType::Kaiser(beta) => kaiser_window(size, beta),
    }
}

/// Janela de Hann
/// w[n] = 0.5 * (1 - cos(2πn/(N-1)))
fn hann_window(size: usize) -> Vec<f64> {
    if size == 0 {
        return vec![];
    }
    if size == 1 {
        return vec![1.0];
    }

    (0..size)
        .map(|n| {
            0.5 * (1.0 - (2.0 * std::f64::consts::PI * n as f64 / (size - 1) as f64).cos())
        })
        .collect()
}

/// Janela de Hamming
/// w[n] = 0.54 - 0.46 * cos(2πn/(N-1))
fn hamming_window(size: usize) -> Vec<f64> {
    if size == 0 {
        return vec![];
    }
    if size == 1 {
        return vec![1.0];
    }

    (0..size)
        .map(|n| {
            0.54 - 0.46 * (2.0 * std::f64::consts::PI * n as f64 / (size - 1) as f64).cos()
        })
        .collect()
}

/// Janela de Blackman
/// w[n] = 0.42 - 0.5*cos(2πn/(N-1)) + 0.08*cos(4πn/(N-1))
fn blackman_window(size: usize) -> Vec<f64> {
    if size == 0 {
        return vec![];
    }
    if size == 1 {
        return vec![1.0];
    }

    (0..size)
        .map(|n| {
            let ratio = n as f64 / (size - 1) as f64;
            0.42 - 0.5 * (2.0 * std::f64::consts::PI * ratio).cos()
                + 0.08 * (4.0 * std::f64::consts::PI * ratio).cos()
        })
        .collect()
}

/// Janela de Kaiser
/// w[n] = I₀(β√(1-(2n/(N-1)-1)²)) / I₀(β)
/// onde I₀ é a função modificada de Bessel de primeira espécie e ordem zero
fn kaiser_window(size: usize, beta: f64) -> Vec<f64> {
    if size == 0 {
        return vec![];
    }
    if size == 1 {
        return vec![1.0];
    }

    let i0_beta = bessel_i0(beta);

    (0..size)
        .map(|n| {
            let alpha = (size - 1) as f64 / 2.0;
            let arg = beta * (1.0 - ((n as f64 - alpha) / alpha).powi(2)).sqrt();
            bessel_i0(arg) / i0_beta
        })
        .collect()
}

/// Função modificada de Bessel de primeira espécie e ordem zero (I₀)
/// Aproximação por série de potências
fn bessel_i0(x: f64) -> f64 {
    let mut sum = 1.0;
    let mut term = 1.0;
    let mut k = 1.0;

    // Convergência da série
    for _ in 0..50 {
        term *= (x / 2.0 / k).powi(2);
        sum += term;
        if term < 1e-12 * sum {
            break;
        }
        k += 1.0;
    }

    sum
}

/// Short-Time Fourier Transform (STFT)
///
/// Retorna: (spectrogram, frequencies, times)
/// - spectrogram: matriz [time_frames][frequency_bins] com magnitudes
/// - frequencies: vetor de frequências correspondentes aos bins
/// - times: vetor de tempos correspondentes aos frames
pub fn stft(
    signal: &[f64],
    window_size: usize,
    hop_size: usize,
    sample_rate: f64,
    window_type: WindowType,
) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    if signal.is_empty() || window_size == 0 || hop_size == 0 {
        return (vec![], vec![], vec![]);
    }

    let window = generate_window(window_size, window_type);
    let num_frames = (signal.len().saturating_sub(window_size)) / hop_size + 1;

    let mut spectrogram = Vec::with_capacity(num_frames);
    let mut times = Vec::with_capacity(num_frames);

    // Processar cada janela
    for frame_idx in 0..num_frames {
        let start = frame_idx * hop_size;
        let end = (start + window_size).min(signal.len());

        if end - start < window_size {
            // Última janela incompleta - fazer padding
            let mut windowed = vec![0.0; window_size];
            for (i, &sample) in signal[start..end].iter().enumerate() {
                windowed[i] = sample * window[i];
            }

            let spectrum = rfft(&windowed);
            let magnitudes: Vec<f64> = spectrum.iter().map(|z| z.magnitude()).collect();
            spectrogram.push(magnitudes);
        } else {
            // Janela completa
            let windowed: Vec<f64> = signal[start..end]
                .iter()
                .zip(window.iter())
                .map(|(&s, &w)| s * w)
                .collect();

            let spectrum = rfft(&windowed);
            let magnitudes: Vec<f64> = spectrum.iter().map(|z| z.magnitude()).collect();
            spectrogram.push(magnitudes);
        }

        times.push((start as f64) / sample_rate);
    }

    // Calcular frequências
    let freq_bin = sample_rate / window_size as f64;
    let frequencies: Vec<f64> = (0..=(window_size / 2))
        .map(|k| k as f64 * freq_bin)
        .collect();

    (spectrogram, frequencies, times)
}

/// Inverse Short-Time Fourier Transform (ISTFT)
///
/// Reconstrói o sinal a partir do espectrograma usando overlap-add
pub fn istft(
    spectrogram: &[Vec<Complex<f64>>],
    window_size: usize,
    hop_size: usize,
    window_type: WindowType,
) -> Vec<f64> {
    if spectrogram.is_empty() || window_size == 0 || hop_size == 0 {
        return vec![];
    }

    let num_frames = spectrogram.len();
    let output_length = (num_frames - 1) * hop_size + window_size;
    let mut output = vec![0.0; output_length];
    let mut window_sum = vec![0.0; output_length];

    let window = generate_window(window_size, window_type);

    // Overlap-add
    for (frame_idx, spectrum) in spectrogram.iter().enumerate() {
        // Reconstruir espectro completo (simetria hermitiana)
        let mut full_spectrum = vec![Complex::zero(); window_size];
        let len = spectrum.len().min(window_size / 2 + 1);
        full_spectrum[0..len].copy_from_slice(&spectrum[0..len]);

        for k in 1..(window_size / 2) {
            full_spectrum[window_size - k] = full_spectrum[k].conj();
        }

        // IFFT
        let time_signal = ifft(&full_spectrum);

        // Aplicar janela e adicionar ao output
        let start = frame_idx * hop_size;
        for (i, &val) in time_signal.iter().take(window_size).enumerate() {
            let pos = start + i;
            if pos < output_length {
                output[pos] += val.re * window[i];
                window_sum[pos] += window[i] * window[i];
            }
        }
    }

    // Normalizar dividindo pela soma das janelas
    for (i, &sum) in window_sum.iter().enumerate() {
        if sum > 1e-10 {
            output[i] /= sum;
        }
    }

    output
}

/// Calcula o espectrograma de potência (|STFT|²)
pub fn power_spectrogram(
    signal: &[f64],
    window_size: usize,
    hop_size: usize,
    sample_rate: f64,
    window_type: WindowType,
) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    let (spec, freqs, times) = stft(signal, window_size, hop_size, sample_rate, window_type);

    let power_spec: Vec<Vec<f64>> = spec
        .iter()
        .map(|frame| frame.iter().map(|&mag| mag * mag).collect())
        .collect();

    (power_spec, freqs, times)
}

/// Calcula o espectrograma em escala logarítmica (dB)
pub fn log_spectrogram(
    signal: &[f64],
    window_size: usize,
    hop_size: usize,
    sample_rate: f64,
    window_type: WindowType,
    ref_value: f64,
) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    let (spec, freqs, times) = stft(signal, window_size, hop_size, sample_rate, window_type);

    let log_spec: Vec<Vec<f64>> = spec
        .iter()
        .map(|frame| {
            frame
                .iter()
                .map(|&mag| {
                    let power = mag * mag;
                    10.0 * (power / ref_value).max(1e-10).log10()
                })
                .collect()
        })
        .collect();

    (log_spec, freqs, times)
}

/// Calcula o mel-espectrograma
/// Converte espectrograma linear para escala Mel
pub fn mel_spectrogram(
    signal: &[f64],
    window_size: usize,
    hop_size: usize,
    sample_rate: f64,
    window_type: WindowType,
    n_mels: usize,
) -> (Vec<Vec<f64>>, Vec<f64>, Vec<f64>) {
    let (spec, freqs, times) = stft(signal, window_size, hop_size, sample_rate, window_type);

    // Criar banco de filtros Mel
    let mel_filters = create_mel_filterbank(n_mels, freqs.len(), sample_rate);

    // Aplicar filtros Mel
    let mel_spec: Vec<Vec<f64>> = spec
        .iter()
        .map(|frame| {
            let mut mel_frame = vec![0.0; n_mels];
            for (mel_idx, filter) in mel_filters.iter().enumerate() {
                for (freq_idx, &filter_val) in filter.iter().enumerate() {
                    if freq_idx < frame.len() {
                        mel_frame[mel_idx] += frame[freq_idx] * filter_val;
                    }
                }
            }
            mel_frame
        })
        .collect();

    // Frequências Mel
    let mel_freqs: Vec<f64> = (0..n_mels)
        .map(|i| mel_to_hz(hz_to_mel(0.0) + (i as f64 / n_mels as f64) * hz_to_mel(sample_rate / 2.0)))
        .collect();

    (mel_spec, mel_freqs, times)
}

/// Converte frequência em Hz para escala Mel
fn hz_to_mel(hz: f64) -> f64 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

/// Converte escala Mel para frequência em Hz
fn mel_to_hz(mel: f64) -> f64 {
    700.0 * (10.0_f64.powf(mel / 2595.0) - 1.0)
}

/// Cria banco de filtros Mel triangulares
fn create_mel_filterbank(n_mels: usize, n_freqs: usize, sample_rate: f64) -> Vec<Vec<f64>> {
    let mut filters = vec![vec![0.0; n_freqs]; n_mels];

    let mel_min = hz_to_mel(0.0);
    let mel_max = hz_to_mel(sample_rate / 2.0);
    let mel_step = (mel_max - mel_min) / (n_mels + 1) as f64;

    // Pontos centrais dos filtros em escala Mel
    let mel_centers: Vec<f64> = (0..=n_mels + 1)
        .map(|i| mel_min + i as f64 * mel_step)
        .collect();

    let hz_centers: Vec<f64> = mel_centers.iter().map(|&m| mel_to_hz(m)).collect();

    // Frequências dos bins
    let freq_bin = sample_rate / ((n_freqs - 1) * 2) as f64;
    let hz_bins: Vec<f64> = (0..n_freqs).map(|i| i as f64 * freq_bin).collect();

    // Construir filtros triangulares
    for mel_idx in 0..n_mels {
        let left = hz_centers[mel_idx];
        let center = hz_centers[mel_idx + 1];
        let right = hz_centers[mel_idx + 2];

        for (freq_idx, &freq) in hz_bins.iter().enumerate() {
            if freq >= left && freq <= center {
                filters[mel_idx][freq_idx] = (freq - left) / (center - left);
            } else if freq > center && freq <= right {
                filters[mel_idx][freq_idx] = (right - freq) / (right - center);
            }
        }
    }

    filters
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_hann_window() {
        let window = hann_window(4);
        assert_eq!(window.len(), 4);
        assert!(window[0].abs() < EPSILON); // Início próximo de 0
        assert!(window[3].abs() < EPSILON); // Fim próximo de 0
        assert!(window[1] > 0.0 && window[1] < 1.0);
        assert!(window[2] > 0.0 && window[2] < 1.0);
    }

    #[test]
    fn test_hamming_window() {
        let window = hamming_window(4);
        assert_eq!(window.len(), 4);
        // Hamming não vai exatamente a zero nas bordas
        assert!(window[0] > 0.0 && window[0] < 0.2);
        assert!(window[3] > 0.0 && window[3] < 0.2);
    }

    #[test]
    fn test_blackman_window() {
        let window = blackman_window(4);
        assert_eq!(window.len(), 4);
        assert!(window[0].abs() < 0.1);
        assert!(window[3].abs() < 0.1);
    }

    #[test]
    fn test_kaiser_window() {
        let window = kaiser_window(8, 5.0);
        assert_eq!(window.len(), 8);
        // Kaiser deve ter valor máximo no centro
        let max_val = window.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        assert!((window[3] - max_val).abs() < EPSILON || (window[4] - max_val).abs() < EPSILON);
    }

    #[test]
    fn test_bessel_i0() {
        // I₀(0) = 1
        assert!((bessel_i0(0.0) - 1.0).abs() < EPSILON);
        // I₀(x) deve ser sempre positivo e crescente
        assert!(bessel_i0(1.0) > 1.0);
        assert!(bessel_i0(2.0) > bessel_i0(1.0));
    }

    #[test]
    fn test_stft_dimensions() {
        let signal = vec![0.0; 1000];
        let window_size = 256;
        let hop_size = 128;
        let sample_rate = 44100.0;

        let (spec, freqs, times) = stft(&signal, window_size, hop_size, sample_rate, WindowType::Hann);

        assert_eq!(freqs.len(), window_size / 2 + 1);
        assert!(times.len() > 0);
        assert_eq!(spec.len(), times.len());
        if !spec.is_empty() {
            assert_eq!(spec[0].len(), freqs.len());
        }
    }

    #[test]
    fn test_stft_simple_signal() {
        // Sinal constante
        let signal = vec![1.0; 100];
        let (spec, _, _) = stft(&signal, 32, 16, 1000.0, WindowType::Rectangular);

        assert!(!spec.is_empty());
        // DC bin deve ter maior energia
        assert!(spec[0][0] > 0.0);
    }

    #[test]
    fn test_stft_istft_reconstruction() {
        let signal = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0];
        let window_size = 4;
        let hop_size = 2;

        // STFT
        let (spec_mag, _, _) = stft(&signal, window_size, hop_size, 100.0, WindowType::Hann);

        // Converter magnitudes para complex (fase zero - não é perfeito mas serve para teste)
        let spec_complex: Vec<Vec<Complex<f64>>> = spec_mag
            .iter()
            .map(|frame| frame.iter().map(|&mag| Complex::new(mag, 0.0)).collect())
            .collect();

        // ISTFT
        let reconstructed = istft(&spec_complex, window_size, hop_size, WindowType::Hann);

        // Verificar que o tamanho é razoável
        assert!(reconstructed.len() >= signal.len());
    }

    #[test]
    fn test_power_spectrogram() {
        let signal = vec![1.0; 100];
        let (power_spec, _, _) = power_spectrogram(&signal, 32, 16, 1000.0, WindowType::Hann);

        assert!(!power_spec.is_empty());
        // Verificar que valores são não-negativos
        for frame in &power_spec {
            for &val in frame {
                assert!(val >= 0.0);
            }
        }
    }

    #[test]
    fn test_log_spectrogram() {
        let signal = vec![1.0; 100];
        let (log_spec, _, _) = log_spectrogram(&signal, 32, 16, 1000.0, WindowType::Hann, 1.0);

        assert!(!log_spec.is_empty());
        // Valores em dB podem ser negativos
    }

    #[test]
    fn test_hz_mel_conversion() {
        let hz = 1000.0;
        let mel = hz_to_mel(hz);
        let hz_back = mel_to_hz(mel);
        assert!((hz - hz_back).abs() < 0.01);
    }

    #[test]
    fn test_mel_spectrogram() {
        let signal = vec![1.0; 100];
        let n_mels = 40;
        let (mel_spec, mel_freqs, _) = mel_spectrogram(
            &signal,
            32,
            16,
            1000.0,
            WindowType::Hann,
            n_mels,
        );

        assert_eq!(mel_freqs.len(), n_mels);
        if !mel_spec.is_empty() {
            assert_eq!(mel_spec[0].len(), n_mels);
        }
    }

    #[test]
    fn test_mel_filterbank() {
        let filters = create_mel_filterbank(10, 128, 1000.0);
        assert_eq!(filters.len(), 10);
        assert_eq!(filters[0].len(), 128);

        // Cada filtro deve ter valores não-negativos
        for filter in &filters {
            for &val in filter {
                assert!(val >= 0.0);
            }
        }
    }

    #[test]
    fn test_window_types() {
        let size = 64;
        let windows = vec![
            WindowType::Rectangular,
            WindowType::Hann,
            WindowType::Hamming,
            WindowType::Blackman,
            WindowType::Kaiser(5.0),
        ];

        for window_type in windows {
            let window = generate_window(size, window_type);
            assert_eq!(window.len(), size);
            // Verificar que valores são não-negativos
            for &val in &window {
                assert!(val >= 0.0);
            }
        }
    }
}
