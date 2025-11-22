//! Scientific types for physics and astrophysics

use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

/// Quaternion for 4D rotations
///
/// Used in physics simulations, spacecraft orientation, and 3D graphics.
/// Components: w (scalar), x, y, z (vector)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Quaternion {
    /// Scalar component (real part)
    pub w: f64,
    /// X component (i)
    pub x: f64,
    /// Y component (j)
    pub y: f64,
    /// Z component (k)
    pub z: f64,
}

impl Quaternion {
    /// Create a new quaternion
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    /// Create identity quaternion (no rotation)
    pub fn identity() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    /// Create quaternion from axis and angle
    pub fn from_axis_angle(axis: [f64; 3], angle: f64) -> Self {
        let half_angle = angle / 2.0;
        let sin_half = half_angle.sin();
        let cos_half = half_angle.cos();

        // Normalize axis
        let len = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let norm_axis = [axis[0] / len, axis[1] / len, axis[2] / len];

        Self {
            w: cos_half,
            x: norm_axis[0] * sin_half,
            y: norm_axis[1] * sin_half,
            z: norm_axis[2] * sin_half,
        }
    }

    /// Get the magnitude (norm) of the quaternion
    pub fn magnitude(&self) -> f64 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Normalize the quaternion (magnitude = 1)
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag > 0.0 {
            Self {
                w: self.w / mag,
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            Self::identity()
        }
    }

    /// Get the conjugate (inverse rotation)
    pub fn conjugate(&self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Get the inverse
    pub fn inverse(&self) -> Self {
        let mag_sq = self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z;
        if mag_sq > 0.0 {
            let conj = self.conjugate();
            Self {
                w: conj.w / mag_sq,
                x: conj.x / mag_sq,
                y: conj.y / mag_sq,
                z: conj.z / mag_sq,
            }
        } else {
            Self::identity()
        }
    }

    /// Rotate a 3D vector by this quaternion
    pub fn rotate_vector(&self, v: [f64; 3]) -> [f64; 3] {
        let q = self.normalize();
        let v_quat = Quaternion::new(0.0, v[0], v[1], v[2]);
        let result = q * v_quat * q.conjugate();
        [result.x, result.y, result.z]
    }
}

impl Mul for Quaternion {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
        }
    }
}

impl Add for Quaternion {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            w: self.w + other.w,
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Quaternion {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            w: self.w - other.w,
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Complex number (for FFT, quantum mechanics)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex64 {
    /// Real part
    pub re: f64,
    /// Imaginary part
    pub im: f64,
}

impl Complex64 {
    /// Create a new complex number
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Create from polar coordinates (magnitude, phase)
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// Get magnitude
    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Get phase (angle)
    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Get complex conjugate
    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl Mul for Complex64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl Add for Complex64 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

/// Tensor4D for General Relativity (4x4 spacetime tensor)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Tensor4D {
    /// 4x4 matrix components [row][col]
    pub components: [[f64; 4]; 4],
}

impl Tensor4D {
    /// Create a new tensor with all zeros
    pub fn zeros() -> Self {
        Self {
            components: [[0.0; 4]; 4],
        }
    }

    /// Create identity tensor
    pub fn identity() -> Self {
        let mut tensor = Self::zeros();
        for i in 0..4 {
            tensor.components[i][i] = 1.0;
        }
        tensor
    }

    /// Create Minkowski metric (flat spacetime)
    pub fn minkowski() -> Self {
        let mut tensor = Self::zeros();
        tensor.components[0][0] = -1.0; // Time component (signature -+++)
        tensor.components[1][1] = 1.0;
        tensor.components[2][2] = 1.0;
        tensor.components[3][3] = 1.0;
        tensor
    }

    /// Create Schwarzschild metric (black hole)
    pub fn schwarzschild_metric(mass: f64, r: f64) -> Self {
        let rs = 2.0 * mass; // Schwarzschild radius (G=c=1)
        let mut tensor = Self::zeros();

        if r > rs {
            tensor.components[0][0] = -(1.0 - rs / r); // g_tt
            tensor.components[1][1] = 1.0 / (1.0 - rs / r); // g_rr
            tensor.components[2][2] = r * r; // g_θθ
            tensor.components[3][3] = r * r * r.sin().powi(2); // g_φφ (assuming θ=π/2)
        }

        tensor
    }

    /// Get component at (row, col)
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.components[row][col]
    }

    /// Set component at (row, col)
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.components[row][col] = value;
    }

    /// Calculate determinant (for volume elements)
    pub fn determinant(&self) -> f64 {
        // Simplified 4x4 determinant calculation
        // Full implementation would use cofactor expansion
        let m = &self.components;

        // Using first row expansion
        let mut det = 0.0;
        for j in 0..4 {
            let sign = if j % 2 == 0 { 1.0 } else { -1.0 };
            det += sign * m[0][j] * self.minor_3x3(0, j);
        }
        det
    }

    fn minor_3x3(&self, skip_row: usize, skip_col: usize) -> f64 {
        let mut m3x3 = [[0.0; 3]; 3];
        let mut i3 = 0;
        for i in 0..4 {
            if i == skip_row {
                continue;
            }
            let mut j3 = 0;
            for j in 0..4 {
                if j == skip_col {
                    continue;
                }
                m3x3[i3][j3] = self.components[i][j];
                j3 += 1;
            }
            i3 += 1;
        }

        // 3x3 determinant
        m3x3[0][0] * (m3x3[1][1] * m3x3[2][2] - m3x3[1][2] * m3x3[2][1])
            - m3x3[0][1] * (m3x3[1][0] * m3x3[2][2] - m3x3[1][2] * m3x3[2][0])
            + m3x3[0][2] * (m3x3[1][0] * m3x3[2][1] - m3x3[1][1] * m3x3[2][0])
    }
}

/// Spinor for particle physics (Dirac spinor)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Spinor {
    /// Upper component (spin up)
    pub up: Complex64,
    /// Lower component (spin down)
    pub down: Complex64,
}

impl Spinor {
    /// Create a new spinor
    pub fn new(up: Complex64, down: Complex64) -> Self {
        Self { up, down }
    }

    /// Create spin-up state
    pub fn spin_up() -> Self {
        Self {
            up: Complex64::new(1.0, 0.0),
            down: Complex64::new(0.0, 0.0),
        }
    }

    /// Create spin-down state
    pub fn spin_down() -> Self {
        Self {
            up: Complex64::new(0.0, 0.0),
            down: Complex64::new(1.0, 0.0),
        }
    }

    /// Get norm (probability amplitude)
    pub fn norm(&self) -> f64 {
        (self.up.magnitude().powi(2) + self.down.magnitude().powi(2)).sqrt()
    }

    /// Normalize spinor
    pub fn normalize(&self) -> Self {
        let n = self.norm();
        if n > 0.0 {
            Self {
                up: Complex64::new(self.up.re / n, self.up.im / n),
                down: Complex64::new(self.down.re / n, self.down.im / n),
            }
        } else {
            Self::spin_up()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quaternion_identity() {
        let q = Quaternion::identity();
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 0.0);
        assert_eq!(q.magnitude(), 1.0);
    }

    #[test]
    fn test_quaternion_multiplication() {
        let q1 = Quaternion::new(1.0, 0.0, 0.0, 0.0);
        let q2 = Quaternion::new(0.0, 1.0, 0.0, 0.0);
        let result = q1 * q2;
        assert_eq!(result.x, 1.0);
    }

    #[test]
    fn test_quaternion_normalize() {
        let q = Quaternion::new(2.0, 0.0, 0.0, 0.0);
        let normalized = q.normalize();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = Complex64::new(3.0, 4.0);
        assert_eq!(c.magnitude(), 5.0);
    }

    #[test]
    fn test_complex_multiplication() {
        let c1 = Complex64::new(1.0, 2.0);
        let c2 = Complex64::new(3.0, 4.0);
        let result = c1 * c2;
        assert_eq!(result.re, -5.0);
        assert_eq!(result.im, 10.0);
    }

    #[test]
    fn test_tensor4d_minkowski() {
        let metric = Tensor4D::minkowski();
        assert_eq!(metric.get(0, 0), -1.0);
        assert_eq!(metric.get(1, 1), 1.0);
        assert_eq!(metric.get(2, 2), 1.0);
        assert_eq!(metric.get(3, 3), 1.0);
    }

    #[test]
    fn test_tensor4d_schwarzschild() {
        let metric = Tensor4D::schwarzschild_metric(1.0, 10.0);
        assert!(metric.get(0, 0) < 0.0); // Time component negative
        assert!(metric.get(1, 1) > 0.0); // Spatial component positive
    }

    #[test]
    fn test_spinor_creation() {
        let spinor = Spinor::spin_up();
        assert_eq!(spinor.up.re, 1.0);
        assert_eq!(spinor.down.re, 0.0);
        assert!((spinor.norm() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_spinor_normalize() {
        let spinor = Spinor::new(
            Complex64::new(2.0, 0.0),
            Complex64::new(2.0, 0.0),
        );
        let normalized = spinor.normalize();
        assert!((normalized.norm() - 1.0).abs() < 1e-10);
    }
}
