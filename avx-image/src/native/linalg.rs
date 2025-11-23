//! Native linear algebra operations
//!
//! Pure Rust implementation of matrix and vector operations

use crate::native::simd;

/// 2D Matrix (row-major storage)
#[derive(Clone, Debug)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>,
}

impl Matrix {
    /// Create new matrix filled with zeros
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Create identity matrix
    pub fn identity(size: usize) -> Self {
        let mut mat = Self::zeros(size, size);
        for i in 0..size {
            mat.data[i * size + i] = 1.0;
        }
        mat
    }

    /// Get element at (row, col)
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f32 {
        self.data[row * self.cols + col]
    }

    /// Set element at (row, col)
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f32) {
        self.data[row * self.cols + col] = value;
    }

    /// Matrix multiplication (naive, O(n³))
    pub fn matmul(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.cols != other.rows {
            return Err("Matrix dimensions incompatible for multiplication");
        }

        let mut result = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }

        Ok(result)
    }

    /// Transpose matrix
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);

        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }

        result
    }

    /// Matrix-vector multiplication
    pub fn matvec(&self, vec: &[f32]) -> Result<Vec<f32>, &'static str> {
        if vec.len() != self.cols {
            return Err("Vector length must match matrix columns");
        }

        let mut result = vec![0.0; self.rows];

        for i in 0..self.rows {
            let mut sum = 0.0;
            for j in 0..self.cols {
                sum += self.get(i, j) * vec[j];
            }
            result[i] = sum;
        }

        Ok(result)
    }

    /// Solve Ax = b using Gaussian elimination (for small matrices)
    pub fn solve(&self, b: &[f32]) -> Result<Vec<f32>, &'static str> {
        if self.rows != self.cols {
            return Err("Matrix must be square");
        }

        if b.len() != self.rows {
            return Err("Vector length must match matrix size");
        }

        // Create augmented matrix [A|b]
        let mut aug = Matrix::zeros(self.rows, self.cols + 1);
        for i in 0..self.rows {
            for j in 0..self.cols {
                aug.set(i, j, self.get(i, j));
            }
            aug.set(i, self.cols, b[i]);
        }

        // Forward elimination
        for col in 0..self.cols {
            // Find pivot
            let mut max_row = col;
            let mut max_val = aug.get(col, col).abs();

            for row in col + 1..self.rows {
                let val = aug.get(row, col).abs();
                if val > max_val {
                    max_val = val;
                    max_row = row;
                }
            }

            if max_val < 1e-10 {
                return Err("Matrix is singular");
            }

            // Swap rows
            if max_row != col {
                for j in 0..=self.cols {
                    let temp = aug.get(col, j);
                    aug.set(col, j, aug.get(max_row, j));
                    aug.set(max_row, j, temp);
                }
            }

            // Eliminate below
            for row in col + 1..self.rows {
                let factor = aug.get(row, col) / aug.get(col, col);
                for j in col..=self.cols {
                    let val = aug.get(row, j) - factor * aug.get(col, j);
                    aug.set(row, j, val);
                }
            }
        }

        // Back substitution
        let mut x = vec![0.0; self.rows];
        for i in (0..self.rows).rev() {
            let mut sum = aug.get(i, self.cols);
            for j in i + 1..self.cols {
                sum -= aug.get(i, j) * x[j];
            }
            x[i] = sum / aug.get(i, i);
        }

        Ok(x)
    }
}

/// Vector operations
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_creation() {
        let mat = Matrix::zeros(3, 4);
        assert_eq!(mat.rows, 3);
        assert_eq!(mat.cols, 4);
        assert_eq!(mat.data.len(), 12);
    }

    #[test]
    fn test_identity_matrix() {
        let mat = Matrix::identity(3);
        assert_eq!(mat.get(0, 0), 1.0);
        assert_eq!(mat.get(1, 1), 1.0);
        assert_eq!(mat.get(2, 2), 1.0);
        assert_eq!(mat.get(0, 1), 0.0);
    }

    #[test]
    fn test_matrix_transpose() {
        let mut mat = Matrix::zeros(2, 3);
        mat.set(0, 0, 1.0);
        mat.set(0, 1, 2.0);
        mat.set(0, 2, 3.0);
        mat.set(1, 0, 4.0);
        mat.set(1, 1, 5.0);
        mat.set(1, 2, 6.0);

        let t = mat.transpose();
        assert_eq!(t.rows, 3);
        assert_eq!(t.cols, 2);
        assert_eq!(t.get(0, 0), 1.0);
        assert_eq!(t.get(1, 0), 2.0);
        assert_eq!(t.get(2, 0), 3.0);
    }

    #[test]
    fn test_vec3_operations() {
        let v1 = Vec3::new(1.0, 0.0, 0.0);
        let v2 = Vec3::new(0.0, 1.0, 0.0);

        assert_eq!(v1.dot(&v2), 0.0);

        let cross = v1.cross(&v2);
        assert_eq!(cross.x, 0.0);
        assert_eq!(cross.y, 0.0);
        assert_eq!(cross.z, 1.0);

        assert_eq!(v1.length(), 1.0);
    }
}
