//! # Interpolation Module
//!
//! Métodos de interpolação para dados 4D:
//! - Interpolação linear
//! - Curvas de Bézier 4D
//! - B-splines 4D
//! - Splines cúbicas

pub mod bezier;
pub mod linear;
pub mod spline;

pub use bezier::{bezier_curve_4d, bezier_surface_4d, BezierCurve4D};
pub use linear::{lerp_4d, bilinear_4d, trilinear_4d, quadrilinear_4d};
pub use spline::{cubic_spline_4d, catmull_rom_4d, CubicSpline4D};
