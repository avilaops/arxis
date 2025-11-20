/// Avila Math - Mathematical Kernel for Avila Ecosystem
///
/// This library provides core mathematical primitives used across the Avila platform:
/// - Vectors (2D, 3D, 4D)
/// - Matrices (3×3, 4×4)
/// - Quaternions (3D and dual quaternions)
/// - Tensors (generalized N-dimensional arrays)
/// - 4D geometry (tesseracts, 24-cell, rotations)
///
/// # Features
///
/// - **Pure Rust** implementation
/// - **Zero-copy** operations where possible
/// - **Type-safe** APIs
/// - **SIMD-friendly** data layouts
/// - **Game engine** and **scientific computing** ready
///
/// # Examples
///
/// ```rust
/// use avila_math::geometry::Quat3D;
///
/// // Create a quaternion from components
/// let q = Quat3D::new(1.0, 0.0, 0.0, 0.0);
///
/// // Normalize it
/// let normalized = q.normalize();
/// assert!((normalized.norm() - 1.0).abs() < 1e-10);
/// ```
pub mod geometry;
pub mod tensor;

// Re-export commonly used types
pub use geometry::{DualQuat, Quat3D, SO4Rotation};
pub use tensor::{Tensor, Tensor4D};
