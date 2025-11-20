//! Caminhos de fótons e estruturas de interação

use crate::amplitude::{ComplexAmplitude, PhaseAccumulator};

/// Tipo de interação do fóton
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    /// Emissão de fóton (fonte)
    Emission,

    /// Propagação livre (vácuo/meio)
    Propagation,

    /// Reflexão especular
    Reflection,

    /// Refração (mudança de meio)
    Refraction,

    /// Espalhamento (Rayleigh, Mie, etc.)
    Scattering,

    /// Absorção (destruição do fóton)
    Absorption,

    /// Detecção (fim do caminho)
    Detection,
}

/// Vértice de interação (ponto onde fóton interage)
#[derive(Debug, Clone)]
pub struct Vertex {
    /// Posição 3D do vértice
    pub position: [f64; 3],

    /// Tempo da interação (opcional, para efeitos temporais)
    pub time: f64,

    /// Tipo de interação neste vértice
    pub interaction_type: InteractionType,

    /// Direção do fóton após o vértice
    pub direction: [f64; 3],

    /// Energia do fóton (ℏω)
    pub energy: f64,

    /// Índice de refração do meio após vértice
    pub refractive_index: f64,
}

impl Vertex {
    /// Cria novo vértice
    pub fn new(
        position: [f64; 3],
        time: f64,
        interaction_type: InteractionType,
        direction: [f64; 3],
        energy: f64,
    ) -> Self {
        Self {
            position,
            time,
            interaction_type,
            direction,
            energy,
            refractive_index: 1.0,
        }
    }

    /// Cria vértice de emissão (fonte de luz)
    pub fn emission(position: [f64; 3], direction: [f64; 3], energy: f64) -> Self {
        Self::new(position, 0.0, InteractionType::Emission, direction, energy)
    }

    /// Cria vértice de detecção (câmera/sensor)
    pub fn detection(position: [f64; 3], time: f64) -> Self {
        Self::new(
            position,
            time,
            InteractionType::Detection,
            [0.0, 0.0, 0.0],
            0.0,
        )
    }

    /// Distância euclidiana até outro vértice
    pub fn distance_to(&self, other: &Vertex) -> f64 {
        let dx = self.position[0] - other.position[0];
        let dy = self.position[1] - other.position[1];
        let dz = self.position[2] - other.position[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Comprimento de onda do fóton (λ = hc/E)
    pub fn wavelength(&self) -> f64 {
        use crate::{HBAR, SPEED_OF_LIGHT};
        2.0 * std::f64::consts::PI * HBAR * SPEED_OF_LIGHT / self.energy
    }
}

/// Interação entre dois vértices
#[derive(Debug, Clone)]
pub struct Interaction {
    /// Vértice inicial
    pub from: Vertex,

    /// Vértice final
    pub to: Vertex,

    /// Amplitude da interação (Feynman vertex)
    pub amplitude: ComplexAmplitude,

    /// Fase acumulada na propagação
    pub phase: f64,
}

impl Interaction {
    /// Cria nova interação
    pub fn new(from: Vertex, to: Vertex) -> Self {
        let distance = from.distance_to(&to);
        let wavelength = from.wavelength();

        // Calcula fase de propagação
        let mut phase_acc = PhaseAccumulator::new();
        phase_acc.add_propagation(distance, wavelength, from.refractive_index);

        Self {
            from,
            to,
            amplitude: ComplexAmplitude::unit(),
            phase: phase_acc.total_phase,
        }
    }

    /// Calcula amplitude da interação usando regras de Feynman
    pub fn compute_feynman_amplitude(&mut self) {
        use crate::FINE_STRUCTURE;

        // Amplitude base depende do tipo de interação
        let base_amplitude = match self.from.interaction_type {
            InteractionType::Emission => 1.0,
            InteractionType::Propagation => 1.0,
            InteractionType::Reflection => 0.9, // Reflexão tem perda
            InteractionType::Refraction => 0.95,
            InteractionType::Scattering => FINE_STRUCTURE.sqrt(), // α^(1/2)
            InteractionType::Absorption => 0.0,
            InteractionType::Detection => 1.0,
        };

        // Amplitude com fase
        self.amplitude = ComplexAmplitude::from_polar(base_amplitude, self.phase);
    }

    /// Probabilidade da interação
    pub fn probability(&self) -> f64 {
        self.amplitude.probability()
    }
}

/// Caminho completo de um fóton
///
/// Na formulação de path integral, somamos sobre todos os caminhos possíveis:
/// `A_total = Σ_caminhos A[caminho]`
#[derive(Debug, Clone)]
pub struct PhotonPath {
    /// Sequência de vértices (interações)
    pub vertices: Vec<Vertex>,

    /// Interações entre vértices consecutivos
    pub interactions: Vec<Interaction>,

    /// Amplitude total do caminho
    pub total_amplitude: ComplexAmplitude,

    /// Fase total acumulada
    pub total_phase: f64,

    /// Peso do caminho (para Monte Carlo sampling)
    pub weight: f64,
}

impl PhotonPath {
    /// Cria novo caminho vazio
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            interactions: Vec::new(),
            total_amplitude: ComplexAmplitude::unit(),
            total_phase: 0.0,
            weight: 1.0,
        }
    }

    /// Cria caminho com vértices iniciais (emissão → detecção)
    pub fn from_endpoints(emission: Vertex, detection: Vertex) -> Self {
        let mut path = Self::new();
        path.vertices.push(emission);
        path.vertices.push(detection);
        path
    }

    /// Adiciona vértice ao caminho
    pub fn add_vertex(&mut self, vertex: Vertex) {
        if !self.vertices.is_empty() {
            // Cria interação entre último vértice e novo
            let from = self.vertices.last().unwrap().clone();
            let mut interaction = Interaction::new(from, vertex.clone());
            interaction.compute_feynman_amplitude();

            self.interactions.push(interaction);
        }

        self.vertices.push(vertex);
    }

    /// Calcula amplitude total do caminho
    ///
    /// Produto de amplitudes de todas as interações:
    /// `A_total = Π_i A_i`
    pub fn compute_total_amplitude(&mut self) {
        self.total_amplitude = ComplexAmplitude::unit();
        self.total_phase = 0.0;

        for interaction in &self.interactions {
            self.total_amplitude = self.total_amplitude * interaction.amplitude;
            self.total_phase += interaction.phase;
        }
    }

    /// Probabilidade do caminho (|A_total|²)
    pub fn probability(&self) -> f64 {
        self.total_amplitude.probability()
    }

    /// Número de interações no caminho
    pub fn num_interactions(&self) -> usize {
        self.interactions.len()
    }

    /// Comprimento óptico total do caminho
    pub fn optical_length(&self) -> f64 {
        self.interactions
            .iter()
            .map(|i| i.from.distance_to(&i.to) * i.from.refractive_index)
            .sum()
    }

    /// Emite diagnósticos de caminho para inspeção
    ///
    /// Retorna estrutura com métricas detalhadas:
    /// - Amplitude e fase de cada vértice
    /// - Probabilidades parciais
    /// - Tipos de interação
    /// - Comprimento óptico
    pub fn emit_diagnostics(&self) -> PathDiagnostics {
        let vertex_data: Vec<VertexDiagnostic> = self
            .vertices
            .iter()
            .map(|v| VertexDiagnostic {
                position: v.position,
                interaction_type: format!("{:?}", v.interaction_type),
                energy: v.energy,
                wavelength: v.wavelength(),
                refractive_index: v.refractive_index,
            })
            .collect();

        let interaction_data: Vec<InteractionDiagnostic> = self
            .interactions
            .iter()
            .map(|i| InteractionDiagnostic {
                amplitude_magnitude: i.amplitude.magnitude(),
                amplitude_phase: i.amplitude.phase(),
                probability: i.amplitude.probability(),
                distance: i.from.distance_to(&i.to),
            })
            .collect();

        PathDiagnostics {
            num_vertices: self.vertices.len(),
            num_interactions: self.interactions.len(),
            total_amplitude_magnitude: self.total_amplitude.magnitude(),
            total_phase: self.total_phase,
            total_probability: self.probability(),
            optical_length: self.optical_length(),
            weight: self.weight,
            vertices: vertex_data,
            interactions: interaction_data,
        }
    }

    /// Valida o caminho (regras físicas)
    pub fn is_valid(&self) -> bool {
        if self.vertices.len() < 2 {
            return false;
        }

        // Primeiro vértice deve ser emissão
        if self.vertices[0].interaction_type != InteractionType::Emission {
            return false;
        }

        // Último vértice deve ser detecção
        if self.vertices.last().unwrap().interaction_type != InteractionType::Detection {
            return false;
        }

        // Não pode ter absorção antes de detecção
        for v in &self.vertices[..self.vertices.len() - 1] {
            if v.interaction_type == InteractionType::Absorption {
                return false;
            }
        }

        true
    }
}

impl Default for PhotonPath {
    fn default() -> Self {
        Self::new()
    }
}

/// Diagnósticos de um vértice
#[derive(Debug, Clone)]
pub struct VertexDiagnostic {
    /// Posição 3D do vértice
    pub position: [f64; 3],
    /// Tipo de interação (string para serialização)
    pub interaction_type: String,
    /// Energia do fóton (J)
    pub energy: f64,
    /// Comprimento de onda (m)
    pub wavelength: f64,
    /// Índice de refração do meio
    pub refractive_index: f64,
}

/// Diagnósticos de uma interação
#[derive(Debug, Clone)]
pub struct InteractionDiagnostic {
    /// Magnitude da amplitude complexa
    pub amplitude_magnitude: f64,
    /// Fase da amplitude (radianos)
    pub amplitude_phase: f64,
    /// Probabilidade da interação
    pub probability: f64,
    /// Distância percorrida (m)
    pub distance: f64,
}

/// Diagnósticos completos de um caminho de fóton
#[derive(Debug, Clone)]
pub struct PathDiagnostics {
    /// Número de vértices no caminho
    pub num_vertices: usize,
    /// Número de interações
    pub num_interactions: usize,
    /// Magnitude da amplitude total
    pub total_amplitude_magnitude: f64,
    /// Fase total acumulada (radianos)
    pub total_phase: f64,
    /// Probabilidade total do caminho
    pub total_probability: f64,
    /// Comprimento óptico total (m)
    pub optical_length: f64,
    /// Peso do caminho (importance sampling)
    pub weight: f64,
    /// Dados de cada vértice
    pub vertices: Vec<VertexDiagnostic>,
    /// Dados de cada interação
    pub interactions: Vec<InteractionDiagnostic>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_creation() {
        let v = Vertex::emission([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3e-19);
        assert_eq!(v.interaction_type, InteractionType::Emission);
    }

    #[test]
    fn test_vertex_distance() {
        let v1 = Vertex::emission([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3e-19);
        let v2 = Vertex::detection([3.0, 4.0, 0.0], 1e-9);

        assert!((v1.distance_to(&v2) - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_interaction_creation() {
        let v1 = Vertex::emission([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3e-19);
        let v2 = Vertex::detection([1.0, 0.0, 0.0], 1e-9);

        let interaction = Interaction::new(v1, v2);
        assert!(interaction.phase != 0.0);
    }

    #[test]
    fn test_photon_path() {
        let mut path = PhotonPath::new();

        let v1 = Vertex::emission([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], 3e-19);
        let v2 = Vertex::detection([1.0, 0.0, 0.0], 1e-9);

        path.add_vertex(v1);
        path.add_vertex(v2);
        path.compute_total_amplitude();

        assert!(path.is_valid());
        assert_eq!(path.num_interactions(), 1);
    }

    #[test]
    fn test_invalid_path() {
        let mut path = PhotonPath::new();
        path.add_vertex(Vertex::detection([0.0, 0.0, 0.0], 0.0));

        assert!(!path.is_valid());
    }
}
