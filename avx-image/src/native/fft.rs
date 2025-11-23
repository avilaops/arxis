//! Native FFT implementation
//!
//! Cooley-Tukey FFT algorithm - 100% pure Rust
//! No external dependencies, optimized for real-time processing

use std::f32::consts::PI;

/// Complex number
#[derive(Clone, Copy, Debug)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn zero() -> Self {
        Self { re: 0.0, im: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    pub fn phase(&self) -> f32 {
        self.im.atan2(self.re)
    }

    pub fn conj(&self) -> Complex {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

/// Cooley-Tukey FFT (radix-2, decimation-in-time)
pub fn fft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();

    // Base case
    if n <= 1 {
        return input.to_vec();
    }

    // Must be power of 2
    assert!(n.is_power_of_two(), "FFT input size must be power of 2");

    // Split into even and odd indices
    let mut even = Vec::with_capacity(n / 2);
    let mut odd = Vec::with_capacity(n / 2);

    for i in 0..n {
        if i % 2 == 0 {
            even.push(input[i]);
        } else {
            odd.push(input[i]);
        }
    }

    // Recursive FFT
    let fft_even = fft(&even);
    let fft_odd = fft(&odd);

    // Combine results
    let mut output = vec![Complex::zero(); n];

    for k in 0..n / 2 {
        let angle = -2.0 * PI * k as f32 / n as f32;
        let twiddle = Complex::new(angle.cos(), angle.sin());
        let t = twiddle * fft_odd[k];

        output[k] = fft_even[k] + t;
        output[k + n / 2] = fft_even[k] - t;
    }

    output
}

/// Inverse FFT
pub fn ifft(input: &[Complex]) -> Vec<Complex> {
    let n = input.len();

    // Conjugate input
    let conjugated: Vec<Complex> = input.iter().map(|c| c.conj()).collect();

    // Forward FFT
    let fft_result = fft(&conjugated);

    // Conjugate and normalize
    fft_result
        .iter()
        .map(|c| Complex::new(c.re / n as f32, -c.im / n as f32))
        .collect()
}

/// Real FFT (for real-valued input)
pub fn rfft(input: &[f32]) -> Vec<Complex> {
    let complex_input: Vec<Complex> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();

    let mut padded = complex_input;
    // Pad to next power of 2
    let n = padded.len().next_power_of_two();
    padded.resize(n, Complex::zero());

    fft(&padded)
}

/// 2D FFT (for images)
pub fn fft_2d(input: &[f32], width: usize, height: usize) -> Vec<Complex> {
    assert_eq!(input.len(), width * height);

    let mut result: Vec<Complex> = input.iter().map(|&x| Complex::new(x, 0.0)).collect();

    // FFT on rows
    let mut row_buf = vec![Complex::zero(); width];
    for y in 0..height {
        for x in 0..width {
            row_buf[x] = result[y * width + x];
        }

        // Pad to power of 2
        let n = width.next_power_of_two();
        row_buf.resize(n, Complex::zero());

        let row_fft = fft(&row_buf);

        for x in 0..width {
            result[y * width + x] = row_fft[x];
        }
    }

    // FFT on columns
    let mut col_buf = vec![Complex::zero(); height];
    for x in 0..width {
        for y in 0..height {
            col_buf[y] = result[y * width + x];
        }

        let n = height.next_power_of_two();
        col_buf.resize(n, Complex::zero());

        let col_fft = fft(&col_buf);

        for y in 0..height {
            result[y * width + x] = col_fft[y];
        }
    }

    result
}

/// 2D Inverse FFT
pub fn ifft_2d(input: &[Complex], width: usize, height: usize) -> Vec<f32> {
    let mut result = input.to_vec();

    // IFFT on rows
    let mut row_buf = vec![Complex::zero(); width];
    for y in 0..height {
        for x in 0..width {
            row_buf[x] = result[y * width + x];
        }

        let n = width.next_power_of_two();
        row_buf.resize(n, Complex::zero());

        let row_ifft = ifft(&row_buf);

        for x in 0..width {
            result[y * width + x] = row_ifft[x];
        }
    }

    // IFFT on columns
    let mut col_buf = vec![Complex::zero(); height];
    for x in 0..width {
        for y in 0..height {
            col_buf[y] = result[y * width + x];
        }

        let n = height.next_power_of_two();
        col_buf.resize(n, Complex::zero());

        let col_ifft = ifft(&col_buf);

        for y in 0..height {
            result[y * width + x] = col_ifft[y];
        }
    }

    // Extract real part
    result.iter().map(|c| c.re).collect()
}

/// Discrete Cosine Transform (DCT) - Type II
/// Used in JPEG compression
pub fn dct(input: &[f32]) -> Vec<f32> {
    let n = input.len();
    let mut output = vec![0.0; n];

    for k in 0..n {
        let mut sum = 0.0;
        for i in 0..n {
            let angle = PI * k as f32 * (2 * i + 1) as f32 / (2 * n) as f32;
            sum += input[i] * angle.cos();
        }

        let c = if k == 0 {
            (1.0 / n as f32).sqrt()
        } else {
            (2.0 / n as f32).sqrt()
        };

        output[k] = c * sum;
    }

    output
}

/// Inverse DCT
pub fn idct(input: &[f32]) -> Vec<f32> {
    let n = input.len();
    let mut output = vec![0.0; n];

    for i in 0..n {
        let mut sum = 0.0;

        for k in 0..n {
            let c = if k == 0 {
                (1.0 / n as f32).sqrt()
            } else {
                (2.0 / n as f32).sqrt()
            };

            let angle = PI * k as f32 * (2 * i + 1) as f32 / (2 * n) as f32;
            sum += c * input[k] * angle.cos();
        }

        output[i] = sum;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_operations() {
        let a = Complex::new(3.0, 4.0);
        let b = Complex::new(1.0, 2.0);

        let sum = a + b;
        assert_eq!(sum.re, 4.0);
        assert_eq!(sum.im, 6.0);

        let diff = a - b;
        assert_eq!(diff.re, 2.0);
        assert_eq!(diff.im, 2.0);

        let product = a * b;
        assert_eq!(product.re, -5.0);
        assert_eq!(product.im, 10.0);

        assert!((a.magnitude() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_fft_simple() {
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(1.0, 0.0),
        ];

        let output = fft(&input);

        // DC component should be 4.0
        assert!((output[0].re - 4.0).abs() < 0.001);
        assert!(output[0].im.abs() < 0.001);

        // Other components should be ~0
        for i in 1..4 {
            assert!(output[i].magnitude() < 0.001);
        }
    }

    #[test]
    fn test_fft_ifft_roundtrip() {
        let input = vec![
            Complex::new(1.0, 0.0),
            Complex::new(2.0, 0.0),
            Complex::new(3.0, 0.0),
            Complex::new(4.0, 0.0),
        ];

        let fft_result = fft(&input);
        let reconstructed = ifft(&fft_result);

        for i in 0..4 {
            assert!((input[i].re - reconstructed[i].re).abs() < 0.001);
            assert!((input[i].im - reconstructed[i].im).abs() < 0.001);
        }
    }

    #[test]
    fn test_dct_idct_roundtrip() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];

        let dct_result = dct(&input);
        let reconstructed = idct(&dct_result);

        for i in 0..8 {
            assert!((input[i] - reconstructed[i]).abs() < 0.001);
        }
    }
}
