//! # AVL Cloud Platform
//!
//! Suite completa Rust para computação de alto desempenho, machine learning,
//! databases e cloud services otimizada para Brasil e LATAM.
//!
//! ## 🚀 Componentes
//!
//! ### Data Science & ML
//! - **compress**: Compressão SIMD (LZ4, Zstd, Snappy)
//! - **clustering**: Algoritmos de clustering avançados
//! - **math**: Operações matemáticas de alta performance
//! - **linalg**: Álgebra linear otimizada
//! - **tokenizers**: Tokenização para NLP (BPE, WordPiece, Unigram)
//!
//! ### Database
//! - **db**: AvilaDB - NoSQL distribuído multi-região
//!
//! ### Cloud Services
//! - **auth**: Autenticação e autorização
//! - **console**: Console web de gerenciamento
//! - **queue**: Sistema de filas e mensagens
//! - **storage**: Object storage S3-compatible
//! - **secrets**: Gerenciamento de secrets
//! - **observability**: Métricas e monitoramento
//!
//! ### Runtime
//! - **http**: Framework HTTP de alta performance
//! - **events**: Sistema de eventos pub/sub
//! - **cli**: CLI para gerenciamento
//!
//! ## 📦 Quick Start
//!
//! ```toml
//! [dependencies]
//! avila = "0.2"
//! ```
//!
//! Para features específicas:
//!
//! ```toml
//! [dependencies]
//! avila = { version = "0.2", features = ["full"] }
//! ```
//!
//! ## 🎯 Feature Bundles
//!
//! - `default` - Essenciais: compress, math, http, db
//! - `science` - Computação científica
//! - `ai` - Machine learning
//! - `cloud` - Serviços cloud completos
//! - `runtime` - Runtime e networking
//! - `full` - Todos os componentes

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

// Data Science & ML
#[cfg(feature = "compress")]
#[cfg_attr(docsrs, doc(cfg(feature = "compress")))]
pub use avila_compress as compress;

#[cfg(feature = "clustering")]
#[cfg_attr(docsrs, doc(cfg(feature = "clustering")))]
pub use avila_clustering as clustering;

#[cfg(feature = "math")]
#[cfg_attr(docsrs, doc(cfg(feature = "math")))]
pub use avila_math as math;

#[cfg(feature = "linalg")]
#[cfg_attr(docsrs, doc(cfg(feature = "linalg")))]
pub use avila_linalg as linalg;

#[cfg(feature = "arrow")]
#[cfg_attr(docsrs, doc(cfg(feature = "arrow")))]
pub use avila_arrow as arrow;

#[cfg(feature = "telemetry")]
#[cfg_attr(docsrs, doc(cfg(feature = "telemetry")))]
pub use avila_telemetry as telemetry;

#[cfg(feature = "tokenizers")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokenizers")))]
pub use avila_tokenizers as tokenizers;

// Database
#[cfg(feature = "db")]
#[cfg_attr(docsrs, doc(cfg(feature = "db")))]
pub use aviladb as db;

// Cloud Services
#[cfg(feature = "auth")]
#[cfg_attr(docsrs, doc(cfg(feature = "auth")))]
pub use avl_auth as auth;

#[cfg(feature = "console")]
#[cfg_attr(docsrs, doc(cfg(feature = "console")))]
pub use avl_console as console;

#[cfg(feature = "observability")]
#[cfg_attr(docsrs, doc(cfg(feature = "observability")))]
pub use avl_observability as observability;

#[cfg(feature = "queue")]
#[cfg_attr(docsrs, doc(cfg(feature = "queue")))]
pub use avl_queue as queue;

#[cfg(feature = "secrets")]
#[cfg_attr(docsrs, doc(cfg(feature = "secrets")))]
pub use avl_secrets as secrets;

#[cfg(feature = "storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
pub use avl_storage as storage;

// Runtime & Networking
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
pub use avx_http as http;

#[cfg(feature = "cli")]
#[cfg_attr(docsrs, doc(cfg(feature = "cli")))]
pub use avx_cli as cli;

#[cfg(feature = "config")]
#[cfg_attr(docsrs, doc(cfg(feature = "config")))]
pub use avx_config as config;

#[cfg(feature = "events")]
#[cfg_attr(docsrs, doc(cfg(feature = "events")))]
pub use avx_events as events;

#[cfg(feature = "avx-telemetry-feature")]
#[cfg_attr(docsrs, doc(cfg(feature = "avx-telemetry-feature")))]
pub use avx_telemetry;

/// Versão da plataforma AVL
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Informações da plataforma
pub mod platform {
    /// Nome da plataforma
    pub const NAME: &str = "AVL Cloud Platform";
    
    /// Website oficial
    pub const WEBSITE: &str = "https://avila.cloud";
    
    /// Documentação
    pub const DOCS: &str = "https://docs.avila.cloud";
    
    /// Região primária
    pub const PRIMARY_REGION: &str = "Brazil (São Paulo)";
    
    /// Latência típica no Brasil
    pub const LATENCY_BRAZIL: &str = "< 10ms";
    
    /// Status
    pub const STATUS: &str = "Production Ready 🚀";
}
