//! # Avila Convexa3D
//!
//! Processamento de dados tridimensionais (3D) para vídeos e volumes.
//! Suporta análise temporal (vídeos) e espacial (volumes médicos, científicos).

pub mod video;
pub mod volume;
pub mod filters;
pub mod motion;
pub mod common;

pub use video::{Video, VideoFrame, VideoProcessor};
pub use volume::{Volume3D, VolumeProcessor};
pub use filters::{ConvolutionKernel3D, Filter3D};
pub use motion::{OpticalFlow, MotionDetector};
pub use common::{Point3D, Size3D, BoundingBox3D};

/// Versão da biblioteca
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
