//! Analytic Hierarchy Process (AHP) implementation

use nalgebra::DMatrix;
use crate::{LocationError, Result};

/// AHP decision maker
pub struct AHP {
    /// Pairwise comparison matrix
    pub comparison_matrix: DMatrix<f64>,

    /// Criterion names
    pub criteria: Vec<String>,
}

impl AHP {
    /// Create new AHP with pairwise comparisons
    /// Values: 1 = equal, 3 = moderate, 5 = strong, 7 = very strong, 9 = extreme
    pub fn new(comparison_matrix: DMatrix<f64>, criteria: Vec<String>) -> Result<Self> {
        if comparison_matrix.nrows() != comparison_matrix.ncols() {
            return Err(LocationError::AnalysisError(
                "Comparison matrix must be square".into()
            ));
        }

        if comparison_matrix.nrows() != criteria.len() {
            return Err(LocationError::AnalysisError(
                "Matrix size must match number of criteria".into()
            ));
        }

        Ok(Self {
            comparison_matrix,
            criteria,
        })
    }

    /// Calculate priority weights using eigenvalue method
    pub fn calculate_weights(&self) -> Vec<f64> {
        let n = self.comparison_matrix.nrows();

        // Normalize columns
        let mut normalized = self.comparison_matrix.clone();
        for j in 0..n {
            let col_sum: f64 = (0..n).map(|i| self.comparison_matrix[(i, j)]).sum();
            if col_sum > 0.0 {
                for i in 0..n {
                    normalized[(i, j)] /= col_sum;
                }
            }
        }

        // Calculate row averages (priority vector)
        let mut weights = Vec::with_capacity(n);
        for i in 0..n {
            let row_sum: f64 = (0..n).map(|j| normalized[(i, j)]).sum();
            weights.push(row_sum / n as f64);
        }

        weights
    }

    /// Calculate consistency ratio (CR < 0.1 is acceptable)
    pub fn consistency_ratio(&self) -> f64 {
        let n = self.comparison_matrix.nrows();
        let weights = self.calculate_weights();

        // Calculate weighted sum vector
        let mut weighted_sum = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                weighted_sum[i] += self.comparison_matrix[(i, j)] * weights[j];
            }
        }

        // Calculate lambda_max
        let lambda_max: f64 = weighted_sum
            .iter()
            .zip(&weights)
            .map(|(ws, w)| ws / w)
            .sum::<f64>() / n as f64;

        // Consistency index
        let ci = (lambda_max - n as f64) / (n as f64 - 1.0);

        // Random index (for n=3 to 10)
        let ri = match n {
            1 | 2 => 0.0,
            3 => 0.58,
            4 => 0.90,
            5 => 1.12,
            6 => 1.24,
            7 => 1.32,
            8 => 1.41,
            9 => 1.45,
            10 => 1.49,
            _ => 1.49,
        };

        if ri == 0.0 {
            0.0
        } else {
            ci / ri
        }
    }

    /// Score alternatives based on their criterion values
    pub fn score_alternatives(
        &self,
        alternatives: &[Vec<f64>],
    ) -> Vec<f64> {
        let weights = self.calculate_weights();

        alternatives
            .iter()
            .map(|alt_scores| {
                alt_scores
                    .iter()
                    .zip(&weights)
                    .map(|(score, weight)| score * weight)
                    .sum()
            })
            .collect()
    }
}

/// Build comparison matrix using pairwise preferences
pub struct AHPBuilder {
    size: usize,
    matrix: DMatrix<f64>,
}

impl AHPBuilder {
    pub fn new(num_criteria: usize) -> Self {
        Self {
            size: num_criteria,
            matrix: DMatrix::identity(num_criteria, num_criteria),
        }
    }

    /// Set comparison: criterion i is X times more important than criterion j
    pub fn compare(mut self, i: usize, j: usize, importance: f64) -> Self {
        if i < self.size && j < self.size {
            self.matrix[(i, j)] = importance;
            self.matrix[(j, i)] = 1.0 / importance;
        }
        self
    }

    pub fn build(self, criteria: Vec<String>) -> Result<AHP> {
        AHP::new(self.matrix, criteria)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ahp_weights() {
        let matrix = DMatrix::from_row_slice(3, 3, &[
            1.0, 3.0, 5.0,
            1.0/3.0, 1.0, 3.0,
            1.0/5.0, 1.0/3.0, 1.0,
        ]);

        let ahp = AHP::new(
            matrix,
            vec!["A".to_string(), "B".to_string(), "C".to_string()]
        ).unwrap();

        let weights = ahp.calculate_weights();
        let sum: f64 = weights.iter().sum();

        assert!((sum - 1.0).abs() < 0.01);
        assert!(weights[0] > weights[1]);
        assert!(weights[1] > weights[2]);
    }
}
