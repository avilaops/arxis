//! Sistema completo de geoprocessamento
//!
//! Este módulo fornece funcionalidades avançadas de análise espacial, incluindo:
//! - Estruturas de dados espaciais (R-Tree, QuadTree)
//! - Operações topológicas e geométricas
//! - Análise espacial e interpolação
//! - Sistema de referência de coordenadas (CRS)
//! - Análise de redes (Network Analysis)
//! - Análise de terreno (Digital Elevation Model)
//! - Engine de geoprocessamento completo
//! - Processamento paralelo (Rayon)
//! - Clustering espacial (K-Means, DBSCAN, Hierarchical)
//! - Cache LRU para otimização
//! - Indústria 4.0: IoT Espacial e Digital Twins
//! - Real-time Analytics e Stream Processing

pub mod spatial;
pub mod operations;
pub mod analysis;
pub mod crs;
pub mod network;
pub mod terrain;
pub mod engine;
pub mod clustering;
pub mod cache;
pub mod industry4;
pub mod realtime;

#[cfg(feature = "parallel")]
pub mod parallel;

pub use spatial::*;
pub use operations::*;
pub use analysis::*;
pub use crs::*;
pub use network::*;
pub use terrain::*;
pub use engine::*;
pub use clustering::*;
pub use cache::*;
pub use industry4::*;
pub use realtime::*;

#[cfg(feature = "parallel")]
pub use parallel::*;
