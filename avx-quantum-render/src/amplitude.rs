//! Amplitude complexa e acumulação de fase quântica

use num_complex::Complex64;
use std::ops::{Add, Mul};

/// Amplitude complexa para caminhos quânticos
///
/// Representa a amplitude de probabilidade de um caminho quântico:
/// `A = |A| exp(iφ)`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexAmplitude {
    /// Amplitude complexa
    pub amplitude: Complex64,
}

impl ComplexAmplitude {
    /// Cria nova amplitude a partir de componentes real e imaginária
    pub fn new(real: f64, imag: f64) -> Self {
        Self {
            amplitude: Complex64::new(real, imag),
        }
    }

    /// Cria amplitude a partir de magnitude e fase
    ///
    /// `A = |A| exp(iφ)`
    pub fn from_polar(magnitude: f64, phase: f64) -> Self {
        Self {
            amplitude: Complex64::from_polar(magnitude, phase),
        }
    }

    /// Amplitude unitária (|A| = 1)
    pub fn unit() -> Self {
        Self::new(1.0, 0.0)
    }

    /// Amplitude zero
    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Magnitude (módulo) da amplitude
    pub fn magnitude(&self) -> f64 {
        self.amplitude.norm()
    }

    /// Fase da amplitude (arg)
    pub fn phase(&self) -> f64 {
        self.amplitude.arg()
    }

    /// Probabilidade = |A|²
    pub fn probability(&self) -> f64 {
        self.amplitude.norm_sqr()
    }

    /// Conjugado complexo
    pub fn conj(&self) -> Self {
        Self {
            amplitude: self.amplitude.conj(),
        }
    }

    /// Normaliza a amplitude (|A| = 1)
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 1e-10 {
            Self {
                amplitude: self.amplitude / mag,
            }
        } else {
            Self::zero()
        }
    }
}

impl Add for ComplexAmplitude {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            amplitude: self.amplitude + other.amplitude,
        }
    }
}

impl Mul for ComplexAmplitude {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            amplitude: self.amplitude * other.amplitude,
        }
    }
}

impl Mul<f64> for ComplexAmplitude {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            amplitude: self.amplitude * scalar,
        }
    }
}

/// Acumulador de fase quântica
///
/// Calcula a fase acumulada ao longo de um caminho:
/// `φ = S/ℏ = (1/ℏ)∫(n·ℏω - p·v)dt`
#[derive(Debug, Clone)]
pub struct PhaseAccumulator {
    /// Fase total acumulada (radianos)
    pub total_phase: f64,

    /// Comprimento óptico total (n·d)
    pub optical_length: f64,

    /// Índice de refração médio
    pub mean_refractive_index: f64,

    /// Comprimento de coerência (m)
    /// None = coerência infinita (laser ideal)
    pub coherence_length: Option<f64>,
}

impl PhaseAccumulator {
    /// Cria novo acumulador de fase
    pub fn new() -> Self {
        Self {
            total_phase: 0.0,
            optical_length: 0.0,
            mean_refractive_index: 1.0,
            coherence_length: None,
        }
    }

    /// Cria acumulador com comprimento de coerência especificado
    ///
    /// # Exemplos
    /// - Laser HeNe: ~30 cm
    /// - LED: ~10 μm
    /// - Luz solar: ~1 μm
    pub fn with_coherence_length(coherence_length: f64) -> Self {
        Self {
            total_phase: 0.0,
            optical_length: 0.0,
            mean_refractive_index: 1.0,
            coherence_length: Some(coherence_length),
        }
    }

    /// Adiciona contribuição de propagação livre
    ///
    /// Para propagação no vácuo/meio:
    /// `Δφ = k·d = (2π/λ)·n·d = (2πn/λ₀)·d`
    pub fn add_propagation(&mut self, distance: f64, wavelength: f64, refractive_index: f64) {
        let k = 2.0 * std::f64::consts::PI * refractive_index / wavelength;
        let phase_change = k * distance;

        self.total_phase += phase_change;
        self.optical_length += refractive_index * distance;

        // Atualiza média do índice de refração
        let segments = (self.optical_length / distance).max(1.0);
        self.mean_refractive_index =
            (self.mean_refractive_index * (segments - 1.0) + refractive_index) / segments;
    }

    /// Adiciona mudança de fase por reflexão/refração
    ///
    /// Reflexão: φ_reflected = φ_incident + π (mudança de fase)
    /// Refração: depende de Fresnel coefficients
    pub fn add_interface_phase(&mut self, phase_shift: f64) {
        self.total_phase += phase_shift;
    }

    /// Adiciona fase de interação com material
    ///
    /// Inclui absorção, espalhamento, etc.
    pub fn add_interaction_phase(&mut self, phase_shift: f64) {
        self.total_phase += phase_shift;
    }

    /// Retorna amplitude correspondente à fase acumulada
    ///
    /// `A = exp(i·φ)`
    ///
    /// Se comprimento de coerência está definido, atenua baseado
    /// no comprimento óptico total
    pub fn to_amplitude(&self) -> ComplexAmplitude {
        let magnitude = self.coherence_attenuation();
        ComplexAmplitude::from_polar(magnitude, self.total_phase)
    }

    /// Retorna amplitude com magnitude ajustada
    ///
    /// `A = |A|·exp(i·φ)`
    pub fn to_amplitude_with_magnitude(&self, magnitude: f64) -> ComplexAmplitude {
        let attenuated_mag = magnitude * self.coherence_attenuation();
        ComplexAmplitude::from_polar(attenuated_mag, self.total_phase)
    }

    /// Calcula atenuação por perda de coerência
    ///
    /// Para caminhos maiores que o comprimento de coerência,
    /// a amplitude é atenuada exponencialmente:
    /// `A(d) = exp(-d/L_c)`
    fn coherence_attenuation(&self) -> f64 {
        if let Some(l_c) = self.coherence_length {
            if l_c > 0.0 {
                return (-self.optical_length / l_c).exp();
            }
        }
        1.0 // Sem atenuação se coerência infinita
    }

    /// Reseta o acumulador
    pub fn reset(&mut self) {
        self.total_phase = 0.0;
        self.optical_length = 0.0;
        self.mean_refractive_index = 1.0;
        // Mantém coherence_length
    }
}

impl Default for PhaseAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_complex_amplitude_creation() {
        let amp = ComplexAmplitude::new(3.0, 4.0);
        assert_abs_diff_eq!(amp.magnitude(), 5.0, epsilon = 1e-10);
    }

    #[test]
    fn test_amplitude_probability() {
        let amp = ComplexAmplitude::from_polar(2.0, std::f64::consts::PI / 4.0);
        assert_abs_diff_eq!(amp.probability(), 4.0, epsilon = 1e-10);
    }

    #[test]
    fn test_amplitude_addition() {
        let a1 = ComplexAmplitude::new(1.0, 0.0);
        let a2 = ComplexAmplitude::new(0.0, 1.0);
        let sum = a1 + a2;

        assert_abs_diff_eq!(sum.magnitude(), std::f64::consts::SQRT_2, epsilon = 1e-10);
    }

    #[test]
    fn test_phase_accumulation() {
        let mut phase = PhaseAccumulator::new();

        // Propagar 500nm por 1μm no vácuo
        let wavelength = 500e-9; // m
        let distance = 1e-6; // m

        phase.add_propagation(distance, wavelength, 1.0);

        // Fase esperada: (2π/λ)·d
        let expected = 2.0 * std::f64::consts::PI * distance / wavelength;
        assert_abs_diff_eq!(phase.total_phase, expected, epsilon = 1e-6);
    }

    #[test]
    fn test_phase_to_amplitude() {
        let mut phase = PhaseAccumulator::new();
        phase.total_phase = std::f64::consts::PI / 2.0;

        let amp = phase.to_amplitude();
        assert_abs_diff_eq!(amp.amplitude.re, 0.0, epsilon = 1e-10);
        assert_abs_diff_eq!(amp.amplitude.im, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_amplitude_normalization() {
        let amp = ComplexAmplitude::new(3.0, 4.0);
        let normalized = amp.normalize();

        assert_abs_diff_eq!(normalized.magnitude(), 1.0, epsilon = 1e-10);
    }
}
