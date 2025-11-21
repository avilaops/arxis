//! # Linear Algebra Module
//!
//! Advanced linear algebra operations including:
//! - SVD (Singular Value Decomposition)
//! - Eigenvalue decomposition
//! - QR decomposition
//! - LU decomposition
//! - Matrix inversions

pub mod decomposition;
pub mod eigen;
pub mod solve;

pub use decomposition::{lu_decomposition, qr_decomposition, svd};
pub use eigen::{eigenvalues, eigenvectors, power_iteration};
pub use solve::{solve_linear_system, solve_triangular};
