/// Módulo de Física - Relatividade Geral e Física Teórica
///
/// Este módulo contém implementações de conceitos de física teórica,
/// incluindo relatividade geral, transformações de Lorentz e tensores físicos.
pub mod cosmology;
pub mod einstein;
pub mod geodesic;
pub mod gravitational_lensing;
pub mod gravitational_waves;
pub mod relativity;

pub use cosmology::{
    CosmicStructure, CosmologicalObservables, CosmologicalParameters, FLRWUniverse,
};
pub use einstein::{BlackHoleProperties, ChristoffelSymbols, EinsteinTensor, MetricTensor};
pub use geodesic::{
    GeodesicIntegrator, GravitationalEffects, OrbitCalculator, OrbitType, ParticleState,
};
pub use gravitational_lensing::{
    GravitationalLens, LensType, LensingStatistics, MicrolensingEvent, WeakLensing,
};
pub use gravitational_waves::{
    CompactBinary, Detector, GravitationalWave, Polarization, WaveformAnalysis,
};
pub use relativity::{LorentzTransform, MinkowskiMetric, RiemannTensor, StressEnergyTensor};
