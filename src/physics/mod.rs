/// Módulo de Física - Relatividade Geral e Física Teórica
///
/// Este módulo contém implementações de conceitos de física teórica,
/// incluindo relatividade geral, transformações de Lorentz e tensores físicos.

pub mod einstein;
pub mod relativity;

pub use einstein::{BlackHoleProperties, ChristoffelSymbols, EinsteinTensor, MetricTensor};
pub use relativity::{LorentzTransform, MinkowskiMetric, RiemannTensor, StressEnergyTensor};
