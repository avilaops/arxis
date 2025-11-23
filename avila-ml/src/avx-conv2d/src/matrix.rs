//! Operações de álgebra linear com matrizes 2D

use ndarray::Array2;
use num_traits::Num;

/// Matriz 2D genérica
#[derive(Debug, Clone)]
pub struct Matrix2D<T> {
    data: Array2<T>,
}

impl<T: Num + Copy> Matrix2D<T> {
    /// Cria nova matriz a partir de array
    pub fn from_array(data: Array2<T>) -> Self {
        Self { data }
    }

    /// Cria matriz de zeros
    pub fn zeros(rows: usize, cols: usize) -> Self
    where
        T: num_traits::Zero,
    {
        Self {
            data: Array2::zeros((rows, cols)),
        }
    }

    /// Cria matriz de uns
    pub fn ones(rows: usize, cols: usize) -> Self
    where
        T: num_traits::One,
    {
        Self {
            data: Array2::ones((rows, cols)),
        }
    }

    /// Cria matriz identidade
    pub fn identity(size: usize) -> Self
    where
        T: num_traits::Zero + num_traits::One,
    {
        Self {
            data: Array2::eye(size),
        }
    }

    /// Número de linhas
    pub fn rows(&self) -> usize {
        self.data.nrows()
    }

    /// Número de colunas
    pub fn cols(&self) -> usize {
        self.data.ncols()
    }

    /// Forma (linhas, colunas)
    pub fn shape(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    /// Obtém elemento
    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        self.data.get((row, col)).copied()
    }

    /// Define elemento
    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if let Some(elem) = self.data.get_mut((row, col)) {
            *elem = value;
        }
    }

    /// Acesso aos dados internos
    pub fn data(&self) -> &Array2<T> {
        &self.data
    }

    /// Transposta
    pub fn transpose(&self) -> Self {
        Self {
            data: self.data.t().to_owned(),
        }
    }
}

impl Matrix2D<f32> {
    /// Normaliza matriz (0.0 a 1.0)
    pub fn normalize(&self) -> Self {
        let min_val = self.data.iter().copied().fold(f32::INFINITY, f32::min);
        let max_val = self.data.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let range = max_val - min_val;

        if range == 0.0 {
            return self.clone();
        }

        let normalized = self.data.mapv(|x| (x - min_val) / range);
        Self::from_array(normalized)
    }

    /// Convolução 2D
    pub fn convolve(&self, kernel: &Matrix2D<f32>) -> Self {
        let (h, w) = self.shape();
        let (kh, kw) = kernel.shape();

        if kh > h || kw > w {
            return self.clone();
        }

        let out_h = h - kh + 1;
        let out_w = w - kw + 1;
        let mut result = Array2::zeros((out_h, out_w));

        for i in 0..out_h {
            for j in 0..out_w {
                let mut sum = 0.0;
                for ki in 0..kh {
                    for kj in 0..kw {
                        sum += self.data[[i + ki, j + kj]] * kernel.data[[ki, kj]];
                    }
                }
                result[[i, j]] = sum;
            }
        }

        Self::from_array(result)
    }

    /// Correlação cruzada 2D
    pub fn correlate(&self, kernel: &Matrix2D<f32>) -> Self {
        let flipped_kernel = kernel.flip_both();
        self.convolve(&flipped_kernel)
    }

    /// Flip horizontal e vertical
    fn flip_both(&self) -> Self {
        let (h, w) = self.shape();
        let mut flipped = Array2::zeros((h, w));

        for i in 0..h {
            for j in 0..w {
                flipped[[i, j]] = self.data[[h - 1 - i, w - 1 - j]];
            }
        }

        Self::from_array(flipped)
    }
}

/// Trait para operações matriciais
pub trait MatrixOps<T> {
    /// Soma elemento a elemento
    fn add(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    /// Subtração elemento a elemento
    fn sub(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    /// Multiplicação por escalar
    fn scale(&self, scalar: T) -> Self;

    /// Soma de todos os elementos
    fn sum(&self) -> T;

    /// Média de todos os elementos
    fn mean(&self) -> T;
}

impl MatrixOps<f32> for Matrix2D<f32> {
    fn add(&self, other: &Self) -> Option<Self> {
        if self.shape() != other.shape() {
            return None;
        }
        Some(Self::from_array(&self.data + &other.data))
    }

    fn sub(&self, other: &Self) -> Option<Self> {
        if self.shape() != other.shape() {
            return None;
        }
        Some(Self::from_array(&self.data - &other.data))
    }

    fn scale(&self, scalar: f32) -> Self {
        Self::from_array(&self.data * scalar)
    }

    fn sum(&self) -> f32 {
        self.data.iter().sum()
    }

    fn mean(&self) -> f32 {
        let total: f32 = self.sum();
        total / (self.rows() * self.cols()) as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let mat = Matrix2D::<f32>::zeros(3, 3);
        assert_eq!(mat.shape(), (3, 3));
    }

    #[test]
    fn test_identity() {
        let mat = Matrix2D::<f32>::identity(3);
        assert_eq!(mat.get(0, 0), Some(1.0));
        assert_eq!(mat.get(0, 1), Some(0.0));
        assert_eq!(mat.get(1, 1), Some(1.0));
    }

    #[test]
    fn test_transpose() {
        let mut mat = Matrix2D::<f32>::zeros(2, 3);
        mat.set(0, 0, 1.0);
        mat.set(0, 1, 2.0);
        mat.set(0, 2, 3.0);

        let transposed = mat.transpose();
        assert_eq!(transposed.shape(), (3, 2));
        assert_eq!(transposed.get(0, 0), Some(1.0));
        assert_eq!(transposed.get(1, 0), Some(2.0));
    }

    #[test]
    fn test_matrix_operations() {
        let mat1 = Matrix2D::from_array(Array2::from_shape_vec((2, 2), vec![1.0, 2.0, 3.0, 4.0]).unwrap());
        let mat2 = Matrix2D::from_array(Array2::from_shape_vec((2, 2), vec![5.0, 6.0, 7.0, 8.0]).unwrap());

        let sum = mat1.add(&mat2).unwrap();
        assert_eq!(sum.get(0, 0), Some(6.0));
        assert_eq!(sum.get(1, 1), Some(12.0));
    }

    #[test]
    fn test_convolution() {
        let mat = Matrix2D::from_array(Array2::from_shape_vec((4, 4), vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ]).unwrap());

        let kernel = Matrix2D::from_array(Array2::from_shape_vec((2, 2), vec![
            1.0, 0.0,
            0.0, 1.0,
        ]).unwrap());

        let result = mat.convolve(&kernel);
        assert_eq!(result.shape(), (3, 3));
    }

    #[test]
    fn test_normalize() {
        let mat = Matrix2D::from_array(Array2::from_shape_vec((2, 2), vec![0.0, 10.0, 20.0, 30.0]).unwrap());
        let normalized = mat.normalize();

        assert_eq!(normalized.get(0, 0), Some(0.0));
        assert_eq!(normalized.get(1, 1), Some(1.0));
    }
}
