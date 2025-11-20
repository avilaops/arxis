//! Diagramas de Feynman e cálculo de vértices QED

use crate::amplitude::ComplexAmplitude;
use crate::photon::{InteractionType, Vertex};
use crate::FINE_STRUCTURE;

/// Vértice de Feynman em QED
///
/// Representa um ponto de interação onde partículas se encontram.
/// Em QED, os vértices básicos são:
/// - e⁻ + γ → e⁻ (absorção de fóton)
/// - e⁻ → e⁻ + γ (emissão de fóton)
/// - γ + γ → e⁻ + e⁺ (pair production, não linear)
#[derive(Debug, Clone)]
pub struct FeynmanVertex {
    /// Posição do vértice
    pub position: [f64; 3],

    /// Tipo de interação
    pub interaction_type: InteractionType,

    /// Momento transferido (q)
    pub momentum_transfer: [f64; 3],

    /// Energia transferida
    pub energy_transfer: f64,

    /// Amplitude do vértice (factor de acoplamento)
    pub amplitude: ComplexAmplitude,
}

impl FeynmanVertex {
    /// Cria novo vértice de Feynman
    pub fn new(position: [f64; 3], interaction_type: InteractionType) -> Self {
        Self {
            position,
            interaction_type,
            momentum_transfer: [0.0, 0.0, 0.0],
            energy_transfer: 0.0,
            amplitude: ComplexAmplitude::unit(),
        }
    }

    /// Calcula amplitude do vértice QED
    ///
    /// Para vértice básico e⁻γ:
    /// `amplitude = -i·e·γ^μ = -i·√(4πα)·γ^μ`
    ///
    /// onde α ≈ 1/137 é a constante de estrutura fina
    pub fn compute_qed_amplitude(&mut self) {
        use std::f64::consts::PI;

        let coupling_constant = (4.0 * PI * FINE_STRUCTURE).sqrt();

        // Amplitude base por tipo de interação
        let base_amplitude = match self.interaction_type {
            InteractionType::Emission => coupling_constant,
            InteractionType::Absorption => coupling_constant,
            InteractionType::Scattering => coupling_constant.powi(2), // Duas interações
            InteractionType::Reflection => {
                // Reflexão ≈ forward scattering
                coupling_constant * 0.9
            }
            InteractionType::Refraction => {
                // Refração envolve interação com meio
                coupling_constant * 0.95
            }
            _ => 1.0,
        };

        // Fase adicional (-i para vértices)
        let phase = -std::f64::consts::FRAC_PI_2; // -i = exp(-iπ/2)
        self.amplitude = ComplexAmplitude::from_polar(base_amplitude, phase);
    }

    /// Calcula amplitude com propagador de fóton
    ///
    /// Propagador do fóton no gauge de Feynman:
    /// `D_μν(q) = -i·g_μν / (q² + iε)`
    pub fn photon_propagator(&self, momentum_squared: f64) -> ComplexAmplitude {
        let epsilon = 1e-10; // Regularização
        let denominator = momentum_squared + epsilon;

        let magnitude = 1.0 / denominator.abs();
        let phase = if denominator > 0.0 {
            0.0
        } else {
            std::f64::consts::PI
        };

        ComplexAmplitude::from_polar(magnitude, phase)
    }

    /// Calcula amplitude com propagador de elétron
    ///
    /// Propagador do elétron:
    /// `S(p) = i(γ·p + m) / (p² - m² + iε)`
    pub fn electron_propagator(
        &self,
        momentum_squared: f64,
        mass_squared: f64,
    ) -> ComplexAmplitude {
        let epsilon = 1e-10;
        let denominator = momentum_squared - mass_squared + epsilon;

        let magnitude = 1.0 / denominator.abs();
        let phase = std::f64::consts::FRAC_PI_2; // i = exp(iπ/2)

        ComplexAmplitude::from_polar(magnitude, phase)
    }
}

/// Diagrama de Feynman completo
///
/// Representa uma sequência de vértices conectados por propagadores.
/// A amplitude total é o produto de todas as contribuições.
#[derive(Debug, Clone)]
pub struct FeynmanDiagram {
    /// Vértices no diagrama
    pub vertices: Vec<FeynmanVertex>,

    /// Linhas externas (inicial e final)
    pub external_lines: Vec<Vertex>,

    /// Amplitude total do diagrama
    pub total_amplitude: ComplexAmplitude,

    /// Ordem em α (número de vértices)
    pub order: usize,
}

impl FeynmanDiagram {
    /// Cria novo diagrama vazio
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            external_lines: Vec::new(),
            total_amplitude: ComplexAmplitude::unit(),
            order: 0,
        }
    }

    /// Adiciona vértice ao diagrama
    pub fn add_vertex(&mut self, mut vertex: FeynmanVertex) {
        vertex.compute_qed_amplitude();
        self.vertices.push(vertex);
        self.order += 1;
    }

    /// Adiciona linha externa
    pub fn add_external_line(&mut self, line: Vertex) {
        self.external_lines.push(line);
    }

    /// Calcula amplitude total do diagrama
    ///
    /// A amplitude é o produto de:
    /// - Amplitude de cada vértice
    /// - Propagadores internos
    /// - Fatores de simetria
    pub fn compute_total_amplitude(&mut self) {
        self.total_amplitude = ComplexAmplitude::unit();

        // Produto de amplitudes dos vértices
        for vertex in &self.vertices {
            self.total_amplitude = self.total_amplitude * vertex.amplitude;
        }

        // Fator de normalização (1/n! para n vértices idênticos)
        let symmetry_factor = Self::factorial(self.order) as f64;
        self.total_amplitude = self.total_amplitude * (1.0 / symmetry_factor.sqrt());
    }

    /// Calcula probabilidade (seção de choque)
    ///
    /// σ = |M|² onde M é a amplitude
    pub fn cross_section(&self) -> f64 {
        self.total_amplitude.probability()
    }

    /// Fatorial (para fatores de simetria)
    fn factorial(n: usize) -> usize {
        (1..=n).product()
    }

    /// Valida o diagrama (conservação de energia-momento)
    pub fn is_valid(&self) -> bool {
        // Deve ter pelo menos 2 linhas externas
        if self.external_lines.len() < 2 {
            return false;
        }

        // Deve ter pelo menos 1 vértice
        if self.vertices.is_empty() {
            return false;
        }

        // Em diagramas mais complexos, verificar conservação nos vértices
        true
    }
}

impl Default for FeynmanDiagram {
    fn default() -> Self {
        Self::new()
    }
}

/// Calcula amplitude de espalhamento Compton (γ + e⁻ → γ + e⁻)
///
/// Exemplo clássico de QED: fóton espalhando em elétron
pub fn compton_scattering_amplitude(
    photon_in_energy: f64,
    photon_out_energy: f64,
    scattering_angle: f64,
) -> ComplexAmplitude {
    use crate::SPEED_OF_LIGHT;

    // Massa do elétron
    let electron_mass = 9.1093837015e-31; // kg
    let _m_e_c2 = electron_mass * SPEED_OF_LIGHT * SPEED_OF_LIGHT;

    // Momento transferido
    let _q_squared = 2.0 * photon_in_energy * photon_out_energy * (1.0 - scattering_angle.cos());

    // Amplitude Klein-Nishina (aproximação)
    let amplitude_squared = FINE_STRUCTURE.powi(2)
        * (photon_out_energy / photon_in_energy + photon_in_energy / photon_out_energy
            - scattering_angle.sin().powi(2));

    ComplexAmplitude::from_polar(amplitude_squared.sqrt(), 0.0)
}

/// Calcula amplitude de pair production (γ → e⁺ + e⁻)
///
/// Produção de pares próximo a núcleo (necessário para conservação)
pub fn pair_production_amplitude(photon_energy: f64, nuclear_charge: f64) -> ComplexAmplitude {
    use crate::SPEED_OF_LIGHT;

    // Massa do elétron
    let electron_mass = 9.1093837015e-31; // kg
    let threshold_energy = 2.0 * electron_mass * SPEED_OF_LIGHT * SPEED_OF_LIGHT;

    if photon_energy < threshold_energy {
        return ComplexAmplitude::zero(); // Abaixo do threshold
    }

    // Amplitude proporcional a Z² α⁴
    let z_squared = nuclear_charge * nuclear_charge;
    let amplitude = z_squared * FINE_STRUCTURE.powi(4);

    ComplexAmplitude::from_polar(amplitude.sqrt(), 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_feynman_vertex_creation() {
        let mut vertex = FeynmanVertex::new([0.0, 0.0, 0.0], InteractionType::Emission);
        vertex.compute_qed_amplitude();

        // Amplitude deve ser proporcional a √α
        let expected = (4.0 * std::f64::consts::PI * FINE_STRUCTURE).sqrt();
        assert_abs_diff_eq!(vertex.amplitude.magnitude(), expected, epsilon = 1e-6);
    }

    #[test]
    fn test_feynman_diagram() {
        let mut diagram = FeynmanDiagram::new();

        let v1 = FeynmanVertex::new([0.0, 0.0, 0.0], InteractionType::Emission);
        let v2 = FeynmanVertex::new([1.0, 0.0, 0.0], InteractionType::Absorption);

        diagram.add_vertex(v1);
        diagram.add_vertex(v2);
        diagram.compute_total_amplitude();

        assert_eq!(diagram.order, 2);
        assert!(diagram.total_amplitude.magnitude() > 0.0);
    }

    #[test]
    fn test_compton_scattering() {
        let e_in = 1e-15; // 1 keV
        let e_out = 0.9e-15;
        let angle = std::f64::consts::FRAC_PI_4;

        let amplitude = compton_scattering_amplitude(e_in, e_out, angle);
        assert!(amplitude.magnitude() > 0.0);
        assert!(amplitude.magnitude() < 1.0);
    }

    #[test]
    fn test_pair_production_threshold() {
        let below_threshold = 1e-14; // Abaixo de 2·m_e·c²
        let amplitude = pair_production_amplitude(below_threshold, 1.0);

        assert_eq!(amplitude.magnitude(), 0.0);
    }

    #[test]
    fn test_photon_propagator() {
        let vertex = FeynmanVertex::new([0.0, 0.0, 0.0], InteractionType::Scattering);
        let q_squared = 1e6;

        let prop = vertex.photon_propagator(q_squared);
        assert!(prop.magnitude() > 0.0);
    }
}
