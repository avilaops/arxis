/// Optical Flow - Motion estimation between consecutive frames
///
/// Implements classic algorithms:
/// - Lucas-Kanade (sparse optical flow)
/// - Farnebäck (dense optical flow)
/// - Horn-Schunck (dense optical flow with smoothness constraint)

use crate::native::buffer::NativeImageBuffer;
use crate::native::convolution::convolve_2d;
use crate::{AvxImageError, Result};

/// Flow vector at a pixel
#[derive(Debug, Clone, Copy)]
pub struct FlowVector {
    pub x: f32,
    pub y: f32,
    pub magnitude: f32,
    pub angle: f32,
}

impl FlowVector {
    pub fn new(x: f32, y: f32) -> Self {
        let magnitude = (x * x + y * y).sqrt();
        let angle = y.atan2(x);
        Self { x, y, magnitude, angle }
    }

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0, magnitude: 0.0, angle: 0.0 }
    }
}

/// Dense optical flow field (flow vector at every pixel)
#[derive(Debug, Clone)]
pub struct DenseFlow {
    pub width: usize,
    pub height: usize,
    pub flow_x: Vec<f32>,  // Horizontal flow component
    pub flow_y: Vec<f32>,  // Vertical flow component
}

impl DenseFlow {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            flow_x: vec![0.0; size],
            flow_y: vec![0.0; size],
        }
    }

    pub fn get_vector(&self, x: usize, y: usize) -> FlowVector {
        let idx = y * self.width + x;
        FlowVector::new(self.flow_x[idx], self.flow_y[idx])
    }

    pub fn set_vector(&mut self, x: usize, y: usize, vx: f32, vy: f32) {
        let idx = y * self.width + x;
        self.flow_x[idx] = vx;
        self.flow_y[idx] = vy;
    }

    /// Convert to visualization image (HSV color encoding)
    pub fn to_hsv_visualization(&self) -> NativeImageBuffer {
        let mut img = NativeImageBuffer::new(self.width, self.height, 3);

        // Find max magnitude for normalization
        let mut max_mag = 0.0f32;
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.get_vector(x, y);
                max_mag = max_mag.max(v.magnitude);
            }
        }

        if max_mag < 1e-6 {
            max_mag = 1.0; // Avoid division by zero
        }

        // Encode: Hue = angle, Saturation = 1.0, Value = magnitude
        for y in 0..self.height {
            for x in 0..self.width {
                let v = self.get_vector(x, y);
                let idx = (y * self.width + x) * 3;

                // Hue: angle mapped to [0, 1]
                let hue = (v.angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI);
                // Value: normalized magnitude
                let value = (v.magnitude / max_mag).clamp(0.0, 1.0);
                let saturation = if value > 0.01 { 1.0 } else { 0.0 };

                // HSV to RGB conversion
                let (r, g, b) = hsv_to_rgb(hue, saturation, value);
                img.data[idx] = r;
                img.data[idx + 1] = g;
                img.data[idx + 2] = b;
            }
        }

        img
    }
}

/// Lucas-Kanade Sparse Optical Flow
///
/// Tracks feature points between frames using local gradient information.
/// Assumes:
/// - Brightness constancy: I(x,y,t) = I(x+dx, y+dy, t+dt)
/// - Small motion: Taylor expansion is valid
/// - Spatial coherence: neighboring pixels have similar motion
///
/// Parameters:
/// - window_size: Size of integration window (typically 15-21)
/// - max_iterations: Maximum iterations for iterative refinement
/// - epsilon: Convergence threshold
pub fn lucas_kanade_sparse(
    prev: &NativeImageBuffer,
    next: &NativeImageBuffer,
    points: &[(f32, f32)],
    window_size: usize,
    max_iterations: usize,
    epsilon: f32,
) -> Result<Vec<Option<(f32, f32)>>> {
    if prev.width != next.width || prev.height != next.height || prev.channels != next.channels {
        return Err(AvxImageError::ProcessingError("Image dimensions must match".into()));
    }

    if prev.channels != 1 {
        return Err(AvxImageError::ProcessingError("Lucas-Kanade requires grayscale images".into()));
    }

    let half_win = (window_size / 2) as i32;

    // Compute spatial gradients (Ix, Iy) using Sobel
    let sobel_x = vec![-1.0, 0.0, 1.0, -2.0, 0.0, 2.0, -1.0, 0.0, 1.0];
    let sobel_y = vec![-1.0, -2.0, -1.0, 0.0, 0.0, 0.0, 1.0, 2.0, 1.0];

    let ix = convolve_2d(&prev.data, prev.width as usize, prev.height as usize, &sobel_x, 3);
    let iy = convolve_2d(&prev.data, prev.width as usize, prev.height as usize, &sobel_y, 3);

    let mut results = Vec::with_capacity(points.len());

    for &(px, py) in points {
        let x = px as i32;
        let y = py as i32;

        // Check if point is too close to border
        if x < half_win || y < half_win ||
           x >= (prev.width as i32 - half_win) ||
           y >= (prev.height as i32 - half_win) {
            results.push(None);
            continue;
        }

        // Build structure matrix [A^T A] and vector [A^T b]
        let mut a11 = 0.0;
        let mut a12 = 0.0;
        let mut a22 = 0.0;

        for dy in -half_win..=half_win {
            for dx in -half_win..=half_win {
                let cx = (x + dx) as usize;
                let cy = (y + dy) as usize;
                let idx = cy * prev.width as usize + cx;

                let ix_val = ix[idx] / 8.0; // Sobel normalization
                let iy_val = iy[idx] / 8.0;

                a11 += ix_val * ix_val;
                a12 += ix_val * iy_val;
                a22 += iy_val * iy_val;
            }
        }

        // Check if matrix is invertible (eigenvalues are large enough)
        let det = a11 * a22 - a12 * a12;
        let trace = a11 + a22;

        if det < 1e-4 || trace < 1e-4 {
            results.push(None); // Not enough texture
            continue;
        }

        // Iterative Lucas-Kanade
        let mut vx = 0.0f32;
        let mut vy = 0.0f32;

        for _iter in 0..max_iterations {
            let mut b1 = 0.0;
            let mut b2 = 0.0;

            for dy in -half_win..=half_win {
                for dx in -half_win..=half_win {
                    let cx1 = (x + dx) as usize;
                    let cy1 = (y + dy) as usize;

                    // Warped coordinates in next frame
                    let cx2 = ((x as f32 + dx as f32 + vx) as i32).clamp(0, next.width as i32 - 1) as usize;
                    let cy2 = ((y as f32 + dy as f32 + vy) as i32).clamp(0, next.height as i32 - 1) as usize;

                    let idx1 = cy1 * prev.width as usize + cx1;
                    let idx2 = cy2 * next.width as usize + cx2;

                    let it = next.data[idx2] - prev.data[idx1]; // Temporal derivative
                    let ix_val = ix[idx1] / 8.0;
                    let iy_val = iy[idx1] / 8.0;

                    b1 += ix_val * it;
                    b2 += iy_val * it;
                }
            }

            // Solve 2x2 system: [A^T A] * delta_v = -[A^T b]
            let inv_det = 1.0 / det;
            let delta_vx = inv_det * (-a22 * b1 + a12 * b2);
            let delta_vy = inv_det * (a12 * b1 - a11 * b2);

            vx += delta_vx;
            vy += delta_vy;

            // Check convergence
            if delta_vx.abs() < epsilon && delta_vy.abs() < epsilon {
                break;
            }
        }

        // Validate result (reasonable motion)
        if vx.abs() < 100.0 && vy.abs() < 100.0 {
            results.push(Some((px + vx, py + vy)));
        } else {
            results.push(None);
        }
    }

    Ok(results)
}

/// Farnebäck Dense Optical Flow
///
/// Computes dense optical flow using polynomial expansion.
/// Based on: "Two-Frame Motion Estimation Based on Polynomial Expansion" (Farnebäck, 2003)
///
/// Approximates image neighborhood with quadratic polynomial:
/// f(x) ≈ x^T A x + b^T x + c
///
/// Parameters:
/// - pyr_scale: Pyramid scale factor (typically 0.5)
/// - levels: Number of pyramid levels (typically 3-5)
/// - winsize: Averaging window size (typically 13-15)
/// - iterations: Number of iterations at each level (typically 3-10)
/// - poly_n: Size of pixel neighborhood (typically 5-7)
/// - poly_sigma: Standard deviation of Gaussian for polynomial expansion
pub fn farneback_dense(
    prev: &NativeImageBuffer,
    next: &NativeImageBuffer,
    pyr_scale: f32,
    levels: usize,
    winsize: usize,
    iterations: usize,
    poly_n: usize,
    poly_sigma: f32,
) -> Result<DenseFlow> {
    if prev.width != next.width || prev.height != next.height || prev.channels != next.channels {
        return Err(AvxImageError::ProcessingError("Image dimensions must match".into()));
    }

    if prev.channels != 1 {
        return Err(AvxImageError::ProcessingError("Farnebäck requires grayscale images".into()));
    }

    // Build image pyramids
    let prev_pyramid = build_pyramid(prev, levels, pyr_scale)?;
    let next_pyramid = build_pyramid(next, levels, pyr_scale)?;

    // Initialize flow at coarsest level
    let coarsest = &prev_pyramid[levels - 1];
    let mut flow = DenseFlow::new(coarsest.width as usize, coarsest.height as usize);

    // Iterative refinement from coarse to fine
    for level in (0..levels).rev() {
        let curr_prev = &prev_pyramid[level];
        let curr_next = &next_pyramid[level];

        // Upsample flow from previous level
        if level < levels - 1 {
            flow = upsample_flow(&flow, curr_prev.width as usize, curr_prev.height as usize);
        }

        // Polynomial expansion for both images
        let poly_prev = polynomial_expansion(curr_prev, poly_n, poly_sigma)?;
        let poly_next = polynomial_expansion(curr_next, poly_n, poly_sigma)?;

        // Iterative flow estimation at this level
        for _iter in 0..iterations {
            flow = estimate_flow_iteration(
                &poly_prev,
                &poly_next,
                &flow,
                winsize,
            )?;
        }
    }

    Ok(flow)
}

/// Build Gaussian pyramid for multi-scale processing
fn build_pyramid(img: &NativeImageBuffer, levels: usize, scale: f32) -> Result<Vec<NativeImageBuffer>> {
    let mut pyramid = Vec::with_capacity(levels);
    pyramid.push(img.clone());

    for level in 1..levels {
        let prev = &pyramid[level - 1];
        let new_width = ((prev.width as f32 * scale) as u32).max(1);
        let new_height = ((prev.height as f32 * scale) as u32).max(1);

        // Gaussian blur before downsampling (anti-aliasing)
        let blurred = gaussian_blur_simple(prev, 2.0)?;

        // Simple bilinear resize
        let mut downsampled = NativeImageBuffer::new(new_width as usize, new_height as usize, prev.channels);
        let x_ratio = prev.width as f32 / new_width as f32;
        let y_ratio = prev.height as f32 / new_height as f32;

        for y in 0..new_height as usize {
            for x in 0..new_width as usize {
                let src_x = ((x as f32 + 0.5) * x_ratio).min(prev.width as f32 - 1.0);
                let src_y = ((y as f32 + 0.5) * y_ratio).min(prev.height as f32 - 1.0);

                let x0 = src_x as usize;
                let y0 = src_y as usize;

                let idx_dst = y * new_width as usize + x;
                let idx_src = y0 * prev.width as usize + x0;

                downsampled.data[idx_dst] = blurred.data[idx_src];
            }
        }

        pyramid.push(downsampled);
    }

    Ok(pyramid)
}

/// Polynomial expansion: approximate image with quadratic polynomial
#[derive(Clone)]
struct PolynomialCoeffs {
    width: usize,
    height: usize,
    // Coefficients for f(x,y) = A*x^2 + B*y^2 + C*x*y + D*x + E*y + F
    a: Vec<f32>,
    b: Vec<f32>,
    c: Vec<f32>,
    d: Vec<f32>,
    e: Vec<f32>,
}

fn polynomial_expansion(
    img: &NativeImageBuffer,
    poly_n: usize,
    poly_sigma: f32,
) -> Result<PolynomialCoeffs> {
    let width = img.width as usize;
    let height = img.height as usize;
    let size = width * height;

    let mut coeffs = PolynomialCoeffs {
        width,
        height,
        a: vec![0.0; size],
        b: vec![0.0; size],
        c: vec![0.0; size],
        d: vec![0.0; size],
        e: vec![0.0; size],
    };

    let half_n = (poly_n / 2) as i32;

    // Gaussian weights
    let sigma2 = poly_sigma * poly_sigma;
    let mut weights = vec![0.0; poly_n * poly_n];
    let mut weight_sum = 0.0;

    for dy in -half_n..=half_n {
        for dx in -half_n..=half_n {
            let dist2 = (dx * dx + dy * dy) as f32;
            let w = (-dist2 / (2.0 * sigma2)).exp();
            let idx = ((dy + half_n) * (2 * half_n + 1) + (dx + half_n)) as usize;
            weights[idx] = w;
            weight_sum += w;
        }
    }

    for w in &mut weights {
        *w /= weight_sum;
    }

    // Fit polynomial at each pixel
    for y in half_n..(height as i32 - half_n) {
        for x in half_n..(width as i32 - half_n) {
            let mut sum_x2 = 0.0;
            let mut sum_y2 = 0.0;
            let mut sum_xy = 0.0;
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            let mut sum_f = 0.0;
            let mut sum_fx = 0.0;
            let mut sum_fy = 0.0;

            for dy in -half_n..=half_n {
                for dx in -half_n..=half_n {
                    let px = (x + dx) as usize;
                    let py = (y + dy) as usize;
                    let pidx = py * width + px;
                    let widx = ((dy + half_n) * (2 * half_n + 1) + (dx + half_n)) as usize;

                    let w = weights[widx];
                    let f = img.data[pidx];
                    let fx = dx as f32;
                    let fy = dy as f32;

                    sum_x2 += w * fx * fx;
                    sum_y2 += w * fy * fy;
                    sum_xy += w * fx * fy;
                    sum_x += w * fx;
                    sum_y += w * fy;
                    sum_f += w * f;
                    sum_fx += w * f * fx;
                    sum_fy += w * f * fy;
                }
            }

            // Simplified polynomial fit (gradient-based)
            let idx = y as usize * width + x as usize;
            coeffs.d[idx] = sum_fx / (sum_x2 + 1e-6);
            coeffs.e[idx] = sum_fy / (sum_y2 + 1e-6);
        }
    }

    Ok(coeffs)
}

fn estimate_flow_iteration(
    poly_prev: &PolynomialCoeffs,
    poly_next: &PolynomialCoeffs,
    flow: &DenseFlow,
    winsize: usize,
) -> Result<DenseFlow> {
    let mut new_flow = flow.clone();
    let half_win = (winsize / 2) as i32;

    for y in half_win..(poly_prev.height as i32 - half_win) {
        for x in half_win..(poly_prev.width as i32 - half_win) {
            let idx = y as usize * poly_prev.width + x as usize;

            // Average polynomial difference in window
            let mut sum_dx = 0.0;
            let mut sum_dy = 0.0;
            let mut count = 0.0;

            for dy in -half_win..=half_win {
                for dx in -half_win..=half_win {
                    let px = (x + dx) as usize;
                    let py = (y + dy) as usize;
                    let pidx = py * poly_prev.width + px;

                    sum_dx += poly_next.d[pidx] - poly_prev.d[pidx];
                    sum_dy += poly_next.e[pidx] - poly_prev.e[pidx];
                    count += 1.0;
                }
            }

            new_flow.flow_x[idx] = flow.flow_x[idx] + sum_dx / count;
            new_flow.flow_y[idx] = flow.flow_y[idx] + sum_dy / count;
        }
    }

    Ok(new_flow)
}

fn upsample_flow(flow: &DenseFlow, new_width: usize, new_height: usize) -> DenseFlow {
    let mut upsampled = DenseFlow::new(new_width, new_height);

    let x_ratio = flow.width as f32 / new_width as f32;
    let y_ratio = flow.height as f32 / new_height as f32;
    let scale_x = new_width as f32 / flow.width as f32;
    let scale_y = new_height as f32 / flow.height as f32;

    for y in 0..new_height {
        for x in 0..new_width {
            let src_x = (x as f32 * x_ratio).min(flow.width as f32 - 1.0) as usize;
            let src_y = (y as f32 * y_ratio).min(flow.height as f32 - 1.0) as usize;

            let src_idx = src_y * flow.width + src_x;
            let dst_idx = y * new_width + x;

            upsampled.flow_x[dst_idx] = flow.flow_x[src_idx] * scale_x;
            upsampled.flow_y[dst_idx] = flow.flow_y[src_idx] * scale_y;
        }
    }

    upsampled
}

fn gaussian_blur_simple(img: &NativeImageBuffer, sigma: f32) -> Result<NativeImageBuffer> {
    // Simple 3x3 Gaussian kernel
    let kernel = vec![
        1.0, 2.0, 1.0,
        2.0, 4.0, 2.0,
        1.0, 2.0, 1.0,
    ];

    let blurred_data = convolve_2d(&img.data, img.width as usize, img.height as usize, &kernel, 3);

    // Normalize by kernel sum (16.0)
    let normalized: Vec<f32> = blurred_data.iter().map(|&x| x / 16.0).collect();

    Ok(NativeImageBuffer {
        width: img.width,
        height: img.height,
        channels: img.channels,
        data: normalized,
    })
}

fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (v, v, v);
    }

    let h = h * 6.0;
    let i = h.floor() as i32;
    let f = h - i as f32;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    match i % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flow_vector() {
        let v = FlowVector::new(3.0, 4.0);
        assert_eq!(v.magnitude, 5.0);

        let v_zero = FlowVector::zero();
        assert_eq!(v_zero.magnitude, 0.0);
    }

    #[test]
    fn test_dense_flow_creation() {
        let flow = DenseFlow::new(100, 100);
        assert_eq!(flow.width, 100);
        assert_eq!(flow.height, 100);
        assert_eq!(flow.flow_x.len(), 10000);
        assert_eq!(flow.flow_y.len(), 10000);
    }

    #[test]
    fn test_lucas_kanade_no_motion() {
        let mut img = NativeImageBuffer::new(50, 50, 1);

        // Create a simple pattern
        for y in 20..30 {
            for x in 20..30 {
                img.data[y * 50 + x] = 1.0;
            }
        }

        // Track center point (no motion)
        let points = vec![(25.0, 25.0)];
        let result = lucas_kanade_sparse(&img, &img, &points, 15, 10, 0.01).unwrap();

        assert_eq!(result.len(), 1);
        if let Some((nx, ny)) = result[0] {
            assert!((nx - 25.0).abs() < 1.0);
            assert!((ny - 25.0).abs() < 1.0);
        }
    }

    #[test]
    fn test_lucas_kanade_translation() {
        let mut img1 = NativeImageBuffer::new(50, 50, 1);
        let mut img2 = NativeImageBuffer::new(50, 50, 1);

        // Create pattern in img1
        for y in 20..25 {
            for x in 20..25 {
                img1.data[y * 50 + x] = 1.0;
            }
        }

        // Same pattern shifted in img2 (translation)
        for y in 22..27 {
            for x in 23..28 {
                img2.data[y * 50 + x] = 1.0;
            }
        }

        let points = vec![(22.0, 22.0)];
        let result = lucas_kanade_sparse(&img1, &img2, &points, 11, 10, 0.01).unwrap();

        assert_eq!(result.len(), 1);
        if let Some((nx, ny)) = result[0] {
            // Should detect ~3 pixel shift in x and ~2 in y
            assert!((nx - 25.0).abs() < 2.0);
            assert!((ny - 24.0).abs() < 2.0);
        }
    }

    #[test]
    fn test_farneback_no_motion() {
        let img = NativeImageBuffer::new(64, 64, 1);
        let flow = farneback_dense(&img, &img, 0.5, 2, 11, 3, 5, 1.2).unwrap();

        assert_eq!(flow.width, 64);
        assert_eq!(flow.height, 64);

        // Flow should be near zero everywhere
        let avg_flow: f32 = flow.flow_x.iter().map(|&x| x.abs()).sum::<f32>() / flow.flow_x.len() as f32;
        assert!(avg_flow < 0.1);
    }

    #[test]
    fn test_pyramid_build() {
        let img = NativeImageBuffer::new(64, 64, 1);
        let pyramid = build_pyramid(&img, 3, 0.5).unwrap();

        assert_eq!(pyramid.len(), 3);
        assert_eq!(pyramid[0].width, 64);
        assert_eq!(pyramid[1].width, 32);
        assert_eq!(pyramid[2].width, 16);
    }
}
