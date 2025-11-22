//! # Avila Linear Algebra
//!
//! Biblioteca de álgebra linear genuína em Rust para o ecossistema Avila.
//!
//! ## Filosofia
//!
//! - **100% Avila** - Sem dependências pesadas
//! - **Eficiente** - Performance sem sacrificar clareza
//! - **Educacional** - Código claro e bem documentado
//! - **Flexível** - Suporte para tipos genéricos
//!
//! ## Uso Rápido
//!
//! ```rust
//! use avila_linalg::{Vector3, Matrix3x3};
//!
//! let v1 = Vector3::new(1.0, 2.0, 3.0);
//! let v2 = Vector3::new(4.0, 5.0, 6.0);
//!
//! let dot = v1.dot(&v2);
//! let cross = v1.cross(&v2);
//! ```

#![deny(missing_docs)]
#![warn(clippy::all)]

pub mod matrix;
pub mod ops;
pub mod transform;
pub mod vector;

// Re-exports principais
pub use matrix::{Matrix2x2, Matrix3x3, Matrix4x4, MatrixMxN};
pub use vector::{Vector2, Vector3, Vector4, VectorN};

/// Módulo prelude para imports convenientes
pub mod prelude {
    pub use crate::matrix::*;
    pub use crate::ops::*;
    pub use crate::transform::*;
    pub use crate::vector::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }
}
