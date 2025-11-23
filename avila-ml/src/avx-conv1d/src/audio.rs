//! Processamento de áudio sequencial
//!
//! Análise de sinais de áudio 1D com features temporais e espectrais

use crate::common::ConvolutionKernel;
use ndarray::{Array1, Array2, ArrayView1};
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

/// Processador de áudio
#[derive(Debug, Clone)]
pub struct AudioProcessor {
    /// Taxa de amostragem padrão
    pub sample_rate: f32,
    /// Tamanho da janela para FFT
    pub window_size: usize,
    /// Hop size para análise temporal
    pub hop_size: usize,
}

impl AudioProcessor {
    /// Cria novo processador de áudio
    pub fn new(sample_rate: f32, window_size: usize, hop_size: usize) -> Self {
        Self {
            sample_rate,
            window_size,
            hop_size,
        }
    }

    /// Configuração padrão (44.1kHz, janela de 2048)
    pub fn default() -> Self {
        Self::new(44100.0, 2048, 512)
    }

    /// Normaliza sinal de áudio
    pub fn normalize(&self, signal: &Array1<f32>) -> Array1<f32> {
        let max_val = signal.iter().map(|x| x.abs()).fold(0.0f32, f32::max);
        if max_val > 0.0 {
            signal / max_val
        } else {
            signal.clone()
        }
    }

    /// Aplica janela de Hanning
    pub fn hanning_window(&self, size: usize) -> Array1<f32> {
        Array1::from_shape_fn(size, |i| {
            0.5 * (1.0 - (2.0 * PI * i as f32 / (size - 1) as f32).cos())
        })
    }

    /// Calcula energia do sinal
    pub fn energy(&self, signal: &ArrayView1<f32>) -> f32 {
        signal.iter().map(|x| x * x).sum::<f32>() / signal.len() as f32
    }

    /// Calcula zero-crossing rate
    pub fn zero_crossing_rate(&self, signal: &ArrayView1<f32>) -> f32 {
        let mut crossings = 0;
        for i in 1..signal.len() {
            if (signal[i] >= 0.0 && signal[i - 1] < 0.0) ||
               (signal[i] < 0.0 && signal[i - 1] >= 0.0) {
                crossings += 1;
            }
        }
        crossings as f32 / signal.len() as f32
    }

    /// Calcula RMS (Root Mean Square)
    pub fn rms(&self, signal: &ArrayView1<f32>) -> f32 {
        self.energy(signal).sqrt()
    }

    /// Calcula espectrograma simples (magnitude)
    pub fn spectrogram(&self, signal: &Array1<f32>) -> Array2<f32> {
        let num_frames = (signal.len() - self.window_size) / self.hop_size + 1;
        let freq_bins = self.window_size / 2 + 1;

        let mut spec = Array2::zeros((freq_bins, num_frames));
        let window = self.hanning_window(self.window_size);

        for frame_idx in 0..num_frames {
            let start = frame_idx * self.hop_size;
            let end = start + self.window_size;

            if end <= signal.len() {
                let frame = signal.slice(ndarray::s![start..end]).to_owned();
                let windowed = &frame * &window;

                // FFT simplificado (magnitudes)
                for k in 0..freq_bins {
                    let mut real = 0.0;
                    let mut imag = 0.0;

                    for (n, &sample) in windowed.iter().enumerate() {
                        let angle = -2.0 * PI * k as f32 * n as f32 / self.window_size as f32;
                        real += sample * angle.cos();
                        imag += sample * angle.sin();
                    }

                    spec[[k, frame_idx]] = (real * real + imag * imag).sqrt();
                }
            }
        }

        spec
    }

    /// Extrai features de áudio
    pub fn extract_features(&self, signal: &Array1<f32>) -> AudioFeatures {
        let signal_view = signal.view();

        // Features temporais
        let energy = self.energy(&signal_view);
        let zcr = self.zero_crossing_rate(&signal_view);
        let rms = self.rms(&signal_view);

        // Estatísticas básicas
        let mean = signal.mean().unwrap_or(0.0);
        let std = signal.std(0.0);
        let max_amplitude = signal.iter().map(|x| x.abs()).fold(0.0f32, f32::max);

        // Envelope
        let envelope = self.compute_envelope(signal);

        // Mel-frequency cepstral coefficients (simplificado)
        let mfcc = self.compute_mfcc_simple(signal, 13);

        AudioFeatures {
            sample_rate: self.sample_rate,
            duration: signal.len() as f32 / self.sample_rate,
            energy,
            zcr,
            rms,
            mean,
            std,
            max_amplitude,
            envelope_mean: envelope.mean().unwrap_or(0.0),
            mfcc,
        }
    }

    /// Calcula envelope do sinal
    pub fn compute_envelope(&self, signal: &Array1<f32>) -> Array1<f32> {
        let kernel = ConvolutionKernel::gaussian(128, 20.0);
        let abs_signal = signal.mapv(|x| x.abs());
        kernel.convolve(&abs_signal.view())
    }

    /// MFCC simplificado
    fn compute_mfcc_simple(&self, signal: &Array1<f32>, num_coeffs: usize) -> Vec<f32> {
        // Implementação simplificada para demonstração
        let spec = self.spectrogram(signal);
        let mel_spec = self.mel_filterbank(&spec, 40);

        // DCT simplificado nos filtros mel
        let mut mfcc = vec![0.0; num_coeffs];
        for i in 0..num_coeffs.min(mel_spec.len()) {
            mfcc[i] = mel_spec[i].ln().max(-10.0); // Log com clipping
        }

        mfcc
    }

    /// Banco de filtros Mel simplificado
    fn mel_filterbank(&self, spectrogram: &Array2<f32>, num_filters: usize) -> Vec<f32> {
        let freq_bins = spectrogram.nrows();
        let num_frames = spectrogram.ncols();

        let mut mel_features = vec![0.0; num_filters];

        for i in 0..num_filters {
            let start_bin = (i * freq_bins) / (num_filters + 1);
            let end_bin = ((i + 2) * freq_bins) / (num_filters + 1);

            let mut sum = 0.0;
            let mut count = 0;

            for bin in start_bin..end_bin.min(freq_bins) {
                for frame in 0..num_frames {
                    sum += spectrogram[[bin, frame]];
                    count += 1;
                }
            }

            mel_features[i] = if count > 0 { sum / count as f32 } else { 0.0 };
        }

        mel_features
    }

    /// Aplica convolução temporal com kernel
    pub fn temporal_convolution(&self, signal: &Array1<f32>, kernel: &ConvolutionKernel) -> Array1<f32> {
        kernel.convolve(&signal.view())
    }

    /// Gera sinal de onda senoidal
    pub fn generate_sine_wave(&self, frequency: f32, duration: f32, amplitude: f32) -> Array1<f32> {
        let num_samples = (duration * self.sample_rate) as usize;
        Array1::from_shape_fn(num_samples, |i| {
            amplitude * (2.0 * PI * frequency * i as f32 / self.sample_rate).sin()
        })
    }

    /// Adiciona ruído branco
    pub fn add_white_noise(&self, signal: &Array1<f32>, noise_level: f32) -> Array1<f32> {
        signal.mapv(|x| {
            let noise = (x * 12345.6789).sin() * noise_level;
            x + noise
        })
    }
}

/// Features extraídas de áudio
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFeatures {
    /// Taxa de amostragem
    pub sample_rate: f32,
    /// Duração em segundos
    pub duration: f32,
    /// Energia do sinal
    pub energy: f32,
    /// Zero-crossing rate
    pub zcr: f32,
    /// RMS
    pub rms: f32,
    /// Média
    pub mean: f32,
    /// Desvio padrão
    pub std: f32,
    /// Amplitude máxima
    pub max_amplitude: f32,
    /// Média do envelope
    pub envelope_mean: f32,
    /// MFCC coefficients
    pub mfcc: Vec<f32>,
}

impl AudioFeatures {
    /// Converte para vetor de features
    pub fn to_vector(&self) -> Vec<f32> {
        let mut features = vec![
            self.duration,
            self.energy,
            self.zcr,
            self.rms,
            self.mean,
            self.std,
            self.max_amplitude,
            self.envelope_mean,
        ];
        features.extend(&self.mfcc);
        features
    }

    /// Serializa para JSON
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_processor() {
        let processor = AudioProcessor::default();
        assert_eq!(processor.sample_rate, 44100.0);
    }

    #[test]
    fn test_generate_sine_wave() {
        let processor = AudioProcessor::default();
        let signal = processor.generate_sine_wave(440.0, 1.0, 1.0);

        assert_eq!(signal.len(), 44100);
        assert!(signal[0].abs() < 0.1); // Começa próximo de zero
    }

    #[test]
    fn test_normalization() {
        let processor = AudioProcessor::default();
        let signal = Array1::from(vec![1.0, 2.0, 3.0, 4.0]);
        let normalized = processor.normalize(&signal);

        assert!((normalized.iter().map(|x| x.abs()).fold(0.0f32, f32::max) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_energy_calculation() {
        let processor = AudioProcessor::default();
        let signal = Array1::from(vec![1.0, 2.0, 3.0, 4.0]);
        let energy = processor.energy(&signal.view());

        assert!((energy - 7.5).abs() < 0.001); // (1+4+9+16)/4 = 7.5
    }

    #[test]
    fn test_feature_extraction() {
        let processor = AudioProcessor::default();
        let signal = processor.generate_sine_wave(440.0, 0.1, 0.5);
        let features = processor.extract_features(&signal);

        assert_eq!(features.sample_rate, 44100.0);
        assert!((features.duration - 0.1).abs() < 0.001);
        assert!(features.energy > 0.0);
        assert!(features.mfcc.len() > 0);
    }

    #[test]
    fn test_zero_crossing_rate() {
        let processor = AudioProcessor::default();
        let signal = processor.generate_sine_wave(440.0, 0.01, 1.0);
        let zcr = processor.zero_crossing_rate(&signal.view());

        assert!(zcr > 0.0); // Sinal senoidal deve ter cruzamentos
    }
}
