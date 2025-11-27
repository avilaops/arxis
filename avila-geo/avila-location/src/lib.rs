//! # Avila Location Intelligence
//!
//! Sistema completo de inteligência geoespacial e análise de mercado para
//! seleção de localização ideal para empresas de tecnologia.
//!
//! ## Funcionalidades
//!
//! - **Análise Geográfica**: Weber Problem, P-Median, MCLP, Voronoi Diagrams
//! - **Análise de Mercado**: Clustering, segmentação, lead scoring
//! - **Análise Competitiva**: Porter's Five Forces, SWOT, densidade de concorrência
//! - **Análise Financeira**: NPV, IRR, Break-Even, Monte Carlo, otimização fiscal
//! - **Scoring Multi-Critério**: AHP, TOPSIS, ELECTRE, MAUT
//!
//! ## Regiões Suportadas
//!
//! - **Portugal**: Todas as regiões (Lisboa, Porto, Braga, Coimbra, Aveiro, Faro, etc.)
//! - **Dubai/UAE**: Dubai, Abu Dhabi, Sharjah, Free Zones

pub mod models;
pub mod algorithms;
pub mod data;
pub mod scoring;
pub mod visualization;
pub mod analysis;

pub use models::*;
pub use algorithms::*;
pub use scoring::*;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocationError {
    #[error("Invalid coordinates: {0}")]
    InvalidCoordinates(String),

    #[error("Optimization failed: {0}")]
    OptimizationFailed(String),

    #[error("Insufficient data: {0}")]
    InsufficientData(String),

    #[error("Analysis error: {0}")]
    AnalysisError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, LocationError>;
