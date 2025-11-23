use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Número complexo genérico com componentes real e imaginário
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T> Complex<T>
where
    T: Copy,
{
    /// Cria um novo número complexo
    #[inline(always)]
    pub fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }

    /// Retorna a parte real
    #[inline(always)]
    pub fn real(&self) -> T {
        self.re
    }

    /// Retorna a parte imaginária
    #[inline(always)]
    pub fn imag(&self) -> T {
        self.im
    }
}

impl<T> Complex<T>
where
    T: Copy + Default,
{
    /// Retorna o número complexo zero (0 + 0i)
    #[inline(always)]
    pub fn zero() -> Self {
        Complex {
            re: T::default(),
            im: T::default(),
        }
    }
}

impl<T> Default for Complex<T>
where
    T: Default,
{
    fn default() -> Self {
        Complex {
            re: T::default(),
            im: T::default(),
        }
    }
}

impl Complex<f64> {
    /// Retorna o número complexo um (1 + 0i)
    #[inline(always)]
    pub fn one() -> Self {
        Complex { re: 1.0, im: 0.0 }
    }

    /// Retorna a unidade imaginária (0 + 1i)
    #[inline(always)]
    pub fn i() -> Self {
        Complex { re: 0.0, im: 1.0 }
    }

    /// Cria um número complexo a partir de coordenadas polares
    /// r * e^(iθ) = r * (cos(θ) + i*sin(θ))
    #[inline]
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Complex {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// Retorna o módulo (magnitude) do número complexo
    /// |z| = sqrt(re² + im²)
    #[inline]
    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Retorna o quadrado do módulo (evita sqrt)
    /// |z|² = re² + im²
    #[inline(always)]
    pub fn magnitude_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Retorna a fase (ângulo) do número complexo
    /// arg(z) = atan2(im, re)
    #[inline]
    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Retorna o conjugado complexo
    /// conj(a + bi) = a - bi
    #[inline(always)]
    pub fn conj(&self) -> Self {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }

    /// Exponencial complexa usando fórmula de Euler
    /// e^(a + bi) = e^a * (cos(b) + i*sin(b))
    #[inline]
    pub fn exp(&self) -> Self {
        let exp_re = self.re.exp();
        Complex {
            re: exp_re * self.im.cos(),
            im: exp_re * self.im.sin(),
        }
    }

    /// Retorna a norma L2
    #[inline]
    pub fn norm(&self) -> f64 {
        self.magnitude()
    }

    /// Normaliza o número complexo (retorna unitário)
    #[inline]
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            Complex {
                re: self.re / mag,
                im: self.im / mag,
            }
        }
    }
}

impl Complex<f32> {
    /// Retorna o número complexo um (1 + 0i)
    #[inline(always)]
    pub fn one() -> Self {
        Complex { re: 1.0, im: 0.0 }
    }

    /// Retorna a unidade imaginária (0 + 1i)
    #[inline(always)]
    pub fn i() -> Self {
        Complex { re: 0.0, im: 1.0 }
    }

    /// Cria um número complexo a partir de coordenadas polares
    #[inline]
    pub fn from_polar(r: f32, theta: f32) -> Self {
        Complex {
            re: r * theta.cos(),
            im: r * theta.sin(),
        }
    }

    /// Retorna o módulo do número complexo
    #[inline]
    pub fn magnitude(&self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Retorna o quadrado do módulo
    #[inline(always)]
    pub fn magnitude_squared(&self) -> f32 {
        self.re * self.re + self.im * self.im
    }

    /// Retorna a fase do número complexo
    #[inline]
    pub fn phase(&self) -> f32 {
        self.im.atan2(self.re)
    }

    /// Retorna o conjugado complexo
    #[inline(always)]
    pub fn conj(&self) -> Self {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }

    /// Exponencial complexa
    #[inline]
    pub fn exp(&self) -> Self {
        let exp_re = self.re.exp();
        Complex {
            re: exp_re * self.im.cos(),
            im: exp_re * self.im.sin(),
        }
    }

    /// Retorna a norma L2
    #[inline]
    pub fn norm(&self) -> f32 {
        self.magnitude()
    }

    /// Normaliza o número complexo
    #[inline]
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            *self
        } else {
            Complex {
                re: self.re / mag,
                im: self.im / mag,
            }
        }
    }
}

// ============================================================================
// OPERAÇÕES ARITMÉTICAS
// ============================================================================

// Adição: (a + bi) + (c + di) = (a+c) + (b+d)i
impl<T> Add for Complex<T>
where
    T: Add<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign,
{
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

// Subtração: (a + bi) - (c + di) = (a-c) + (b-d)i
impl<T> Sub for Complex<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T> SubAssign for Complex<T>
where
    T: SubAssign,
{
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.re -= rhs.re;
        self.im -= rhs.im;
    }
}

// Multiplicação: (a + bi) * (c + di) = (ac - bd) + (ad + bc)i
impl<T> Mul for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl<T> MulAssign for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

// Divisão: (a + bi) / (c + di) = [(ac + bd) + (bc - ad)i] / (c² + d²)
impl<T> Div for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    type Output = Self;

    #[inline]
    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Complex {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl<T> DivAssign for Complex<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        let re = (self.re * rhs.re + self.im * rhs.im) / denom;
        let im = (self.im * rhs.re - self.re * rhs.im) / denom;
        self.re = re;
        self.im = im;
    }
}

// Negação: -(a + bi) = -a - bi
impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

// Multiplicação por escalar (f64)
impl Mul<f64> for Complex<f64> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self::Output {
        Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Mul<Complex<f64>> for f64 {
    type Output = Complex<f64>;

    #[inline(always)]
    fn mul(self, rhs: Complex<f64>) -> Self::Output {
        Complex {
            re: self * rhs.re,
            im: self * rhs.im,
        }
    }
}

// Multiplicação por escalar (f32)
impl Mul<f32> for Complex<f32> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f32) -> Self::Output {
        Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl Mul<Complex<f32>> for f32 {
    type Output = Complex<f32>;

    #[inline(always)]
    fn mul(self, rhs: Complex<f32>) -> Self::Output {
        Complex {
            re: self * rhs.re,
            im: self * rhs.im,
        }
    }
}

// Divisão por escalar
impl Div<f64> for Complex<f64> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self::Output {
        Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl Div<f32> for Complex<f32> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f32) -> Self::Output {
        Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

// ============================================================================
// CONVERSÕES E DISPLAY
// ============================================================================

impl<T> From<T> for Complex<T>
where
    T: Default,
{
    fn from(re: T) -> Self {
        Complex {
            re,
            im: T::default(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Complex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_creation() {
        let z = Complex::new(3.0, 4.0);
        assert_eq!(z.re, 3.0);
        assert_eq!(z.im, 4.0);
    }

    #[test]
    fn test_complex_zero_one() {
        let zero = Complex::<f64>::zero();
        assert_eq!(zero.re, 0.0);
        assert_eq!(zero.im, 0.0);

        let one = Complex::one();
        assert_eq!(one.re, 1.0);
        assert_eq!(one.im, 0.0);

        let i = Complex::i();
        assert_eq!(i.re, 0.0);
        assert_eq!(i.im, 1.0);
    }

    #[test]
    fn test_complex_from_polar() {
        let z = Complex::from_polar(1.0, std::f64::consts::PI / 2.0);
        assert!((z.re).abs() < 1e-10);
        assert!((z.im - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_magnitude() {
        let z = Complex::new(3.0, 4.0);
        assert_eq!(z.magnitude(), 5.0);
        assert_eq!(z.magnitude_squared(), 25.0);
    }

    #[test]
    fn test_complex_phase() {
        let z = Complex::new(1.0, 1.0);
        assert!((z.phase() - std::f64::consts::PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_conj() {
        let z = Complex::new(3.0, 4.0);
        let conj = z.conj();
        assert_eq!(conj.re, 3.0);
        assert_eq!(conj.im, -4.0);
    }

    #[test]
    fn test_complex_exp() {
        // e^(iπ) = -1
        let z = Complex::new(0.0, std::f64::consts::PI);
        let result = z.exp();
        assert!((result.re + 1.0).abs() < 1e-10);
        assert!(result.im.abs() < 1e-10);
    }

    #[test]
    fn test_complex_add() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);
        let sum = z1 + z2;
        assert_eq!(sum.re, 4.0);
        assert_eq!(sum.im, 6.0);
    }

    #[test]
    fn test_complex_add_assign() {
        let mut z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);
        z1 += z2;
        assert_eq!(z1.re, 4.0);
        assert_eq!(z1.im, 6.0);
    }

    #[test]
    fn test_complex_sub() {
        let z1 = Complex::new(5.0, 6.0);
        let z2 = Complex::new(2.0, 3.0);
        let diff = z1 - z2;
        assert_eq!(diff.re, 3.0);
        assert_eq!(diff.im, 3.0);
    }

    #[test]
    fn test_complex_sub_assign() {
        let mut z1 = Complex::new(5.0, 6.0);
        let z2 = Complex::new(2.0, 3.0);
        z1 -= z2;
        assert_eq!(z1.re, 3.0);
        assert_eq!(z1.im, 3.0);
    }

    #[test]
    fn test_complex_mul() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);
        let product = z1 * z2;
        // (1+2i)(3+4i) = 3 + 4i + 6i + 8i² = 3 + 10i - 8 = -5 + 10i
        assert_eq!(product.re, -5.0);
        assert_eq!(product.im, 10.0);
    }

    #[test]
    fn test_complex_mul_assign() {
        let mut z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);
        z1 *= z2;
        assert_eq!(z1.re, -5.0);
        assert_eq!(z1.im, 10.0);
    }

    #[test]
    fn test_complex_div() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);
        let quotient = z1 / z2;
        // (1+2i)/(3+4i) = (1+2i)(3-4i)/(9+16) = (3-4i+6i-8i²)/25 = (11+2i)/25
        assert!((quotient.re - 11.0 / 25.0).abs() < 1e-10);
        assert!((quotient.im - 2.0 / 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_div_assign() {
        let mut z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);
        z1 /= z2;
        assert!((z1.re - 11.0 / 25.0).abs() < 1e-10);
        assert!((z1.im - 2.0 / 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_neg() {
        let z = Complex::new(3.0, 4.0);
        let neg = -z;
        assert_eq!(neg.re, -3.0);
        assert_eq!(neg.im, -4.0);
    }

    #[test]
    fn test_complex_scalar_mul() {
        let z = Complex::new(2.0, 3.0);
        let scaled = z * 2.0;
        assert_eq!(scaled.re, 4.0);
        assert_eq!(scaled.im, 6.0);

        let scaled2 = 2.0 * z;
        assert_eq!(scaled2.re, 4.0);
        assert_eq!(scaled2.im, 6.0);
    }

    #[test]
    fn test_complex_scalar_div() {
        let z = Complex::new(4.0, 6.0);
        let scaled = z / 2.0;
        assert_eq!(scaled.re, 2.0);
        assert_eq!(scaled.im, 3.0);
    }

    #[test]
    fn test_complex_normalize() {
        let z = Complex::new(3.0, 4.0);
        let norm = z.normalize();
        assert!((norm.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_identity_mul() {
        let z = Complex::new(2.0, 3.0);
        let one = Complex::one();
        let result = z * one;
        assert_eq!(result.re, z.re);
        assert_eq!(result.im, z.im);
    }

    #[test]
    fn test_complex_i_squared() {
        let i = Complex::i();
        let i_squared = i * i;
        // i² = -1
        assert!((i_squared.re + 1.0).abs() < 1e-10);
        assert!(i_squared.im.abs() < 1e-10);
    }

    #[test]
    fn test_complex_conjugate_product() {
        let z = Complex::new(3.0, 4.0);
        let product = z * z.conj();
        // z * conj(z) = |z|²
        assert!((product.re - 25.0).abs() < 1e-10);
        assert!(product.im.abs() < 1e-10);
    }

    #[test]
    fn test_complex_f32() {
        let z1 = Complex::<f32>::new(1.0, 2.0);
        let z2 = Complex::<f32>::new(3.0, 4.0);
        let sum = z1 + z2;
        assert_eq!(sum.re, 4.0);
        assert_eq!(sum.im, 6.0);

        let product = z1 * z2;
        assert_eq!(product.re, -5.0);
        assert_eq!(product.im, 10.0);
    }
}
