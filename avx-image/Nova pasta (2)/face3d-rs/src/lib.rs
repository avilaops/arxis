//! # face3d-rs
//!
//! Biblioteca Rust para modelagem 3D de rostos, suportando:
//! - **3DMM** (3D Morphable Models) - Modelos paramétricos lineares
//! - **FLAME** - Faces Learned with an Articulated Model and Expressions
//! - **BFM** - Basel Face Model
//!
//! ## Exemplo básico
//!
//! ```no_run
//! use face3d_rs::models::MorphableModel;
//! use nalgebra as na;
//!
//! // Criar modelo 3DMM básico
//! let model = MorphableModel::new(
//!     na::DVector::zeros(300),
//!     na::DMatrix::zeros(300, 199),
//!     na::DMatrix::zeros(300, 199),
//!     na::DVector::zeros(300),
//!     vec![],
//! );
//! ```

pub mod error;
pub mod models;
pub mod utils;

#[cfg(feature = "scientific-io")]
pub mod io;

pub use error::{Face3dError, Result};
