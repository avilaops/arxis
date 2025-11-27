//! Native mathematical operations
//!
//! Pure Rust implementations of fundamental math operations
//! optimized with SIMD where available.

use std::f32::consts::PI;

/// Fast inverse square root (Quake III algorithm)
#[inline]
pub fn fast_inv_sqrt(x: f32) -> f32 {
    let i = x.to_bits();
    let i = 0x5f3759df - (i >> 1);
    let y = f32::from_bits(i);
    y * (1.5 - 0.5 * x * y * y)
}

/// Clamp value between min and max
#[inline]
pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Linear interpolation
#[inline]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Bilinear interpolation for 2D data
#[inline]
pub fn bilinear_interp(
    v00: f32,
    v10: f32,
    v01: f32,
    v11: f32,
    tx: f32,
    ty: f32,
) -> f32 {
    let a = lerp(v00, v10, tx);
    let b = lerp(v01, v11, tx);
    lerp(a, b, ty)
}

/// Gaussian function
#[inline]
pub fn gaussian(x: f32, sigma: f32) -> f32 {
    let variance = sigma * sigma;
    (-(x * x) / (2.0 * variance)).exp() / (2.0 * PI * variance).sqrt()
}

/// Gaussian 2D
#[inline]
pub fn gaussian_2d(x: f32, y: f32, sigma: f32) -> f32 {
    let variance = sigma * sigma;
    let r2 = x * x + y * y;
    (-(r2) / (2.0 * variance)).exp() / (2.0 * PI * variance)
}

/// Create Gaussian kernel (1D)
pub fn gaussian_kernel(size: usize, sigma: f32) -> Vec<f32> {
    let mut kernel = Vec::with_capacity(size);
    let center = (size / 2) as f32;
    let mut sum = 0.0;

    for i in 0..size {
        let x = i as f32 - center;
        let value = gaussian(x, sigma);
        kernel.push(value);
        sum += value;
    }

    // Normalize
    for value in &mut kernel {
        *value /= sum;
    }

    kernel
}

/// Create Gaussian kernel (2D)
pub fn gaussian_kernel_2d(size: usize, sigma: f32) -> Vec<f32> {
    let mut kernel = Vec::with_capacity(size * size);
    let center = (size / 2) as f32;
    let mut sum = 0.0;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            let value = gaussian_2d(dx, dy, sigma);
            kernel.push(value);
            sum += value;
        }
    }

    // Normalize
    for value in &mut kernel {
        *value /= sum;
    }

    kernel
}

/// Fast approximation of atan2 (for orientation computation)
#[inline]
pub fn fast_atan2(y: f32, x: f32) -> f32 {
    const PI: f32 = std::f32::consts::PI;

    if x == 0.0 && y == 0.0 {
        return 0.0;
    }

    let abs_y = y.abs();
    let abs_x = x.abs();

    let a = if abs_x > abs_y {
        abs_y / abs_x
    } else {
        abs_x / abs_y
    };

    // Polynomial approximation
    let s = a * a;
    let mut r = ((-0.0464964749 * s + 0.15931422) * s - 0.327622764) * s * a + a;

    if abs_y > abs_x {
        r = PI / 2.0 - r;
    }

    if x < 0.0 {
        r = PI - r;
    }

    if y < 0.0 {
        r = -r;
    }

    r
}

/// Sobel kernel for edge detection
pub fn sobel_x() -> [f32; 9] {
    [-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0]
}

pub fn sobel_y() -> [f32; 9] {
    [-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0]
}

/// Scharr kernels (better rotational symmetry)
pub fn scharr_x() -> [f32; 9] {
    [-3.0, 0.0, 3.0, -10.0, 0.0, 10.0, -3.0, 0.0, 3.0]
}

pub fn scharr_y() -> [f32; 9] {
    [-3.0, -10.0, -3.0, 0.0, 0.0, 0.0, 3.0, 10.0, 3.0]
}

/// Laplacian kernel
pub fn laplacian() -> [f32; 9] {
    [0.0, 1.0, 0.0, 1.0, -4.0, 1.0, 0.0, 1.0, 0.0]
}

/// Sharpen kernel
pub fn sharpen() -> [f32; 9] {
    [0.0, -1.0, 0.0, -1.0, 5.0, -1.0, 0.0, -1.0, 0.0]
}

/// Box blur kernel
pub fn box_blur(size: usize) -> Vec<f32> {
    let value = 1.0 / (size * size) as f32;
    vec![value; size * size]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_inv_sqrt() {
        let x = 4.0;
        let result = fast_inv_sqrt(x);
        let expected = 1.0 / x.sqrt();
        assert!((result - expected).abs() < 0.01);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_lerp() {
        assert_eq!(lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);
    }

    #[test]
    fn test_gaussian_kernel() {
        let kernel = gaussian_kernel(5, 1.0);
        assert_eq!(kernel.len(), 5);
        // Sum should be 1.0 (normalized)
        let sum: f32 = kernel.iter().sum();
        assert!((sum - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_gaussian_kernel_2d() {
        let kernel = gaussian_kernel_2d(3, 1.0);
        assert_eq!(kernel.len(), 9);
        let sum: f32 = kernel.iter().sum();
        assert!((sum - 1.0).abs() < 0.0001);
    }
}
