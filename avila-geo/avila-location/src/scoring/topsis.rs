//! TOPSIS (Technique for Order Preference by Similarity to Ideal Solution)

use nalgebra::DMatrix;
use crate::{LocationError, Result};

/// TOPSIS decision maker
pub struct TOPSIS {
    /// Decision matrix (alternatives x criteria)
    pub decision_matrix: DMatrix<f64>,

    /// Criterion weights
    pub weights: Vec<f64>,

    /// Beneficial criteria (true) vs cost criteria (false)
    pub is_beneficial: Vec<bool>,
}

impl TOPSIS {
    pub fn new(
        decision_matrix: DMatrix<f64>,
        weights: Vec<f64>,
        is_beneficial: Vec<bool>,
    ) -> Result<Self> {
        let n_criteria = decision_matrix.ncols();

        if weights.len() != n_criteria {
            return Err(LocationError::AnalysisError(
                "Number of weights must match criteria".into()
            ));
        }

        if is_beneficial.len() != n_criteria {
            return Err(LocationError::AnalysisError(
                "Number of benefit indicators must match criteria".into()
            ));
        }

        let weight_sum: f64 = weights.iter().sum();
        if (weight_sum - 1.0).abs() > 0.01 {
            return Err(LocationError::AnalysisError(
                "Weights must sum to 1.0".into()
            ));
        }

        Ok(Self {
            decision_matrix,
            weights,
            is_beneficial,
        })
    }

    /// Calculate TOPSIS scores for all alternatives
    pub fn calculate_scores(&self) -> Vec<f64> {
        // Step 1: Normalize decision matrix
        let normalized = self.normalize_matrix();

        // Step 2: Apply weights
        let weighted = self.apply_weights(&normalized);

        // Step 3: Determine ideal and anti-ideal solutions
        let (ideal, anti_ideal) = self.find_ideal_solutions(&weighted);

        // Step 4: Calculate distances
        let n_alternatives = weighted.nrows();
        let mut distances_ideal = vec![0.0; n_alternatives];
        let mut distances_anti_ideal = vec![0.0; n_alternatives];

        for i in 0..n_alternatives {
            for j in 0..weighted.ncols() {
                distances_ideal[i] += (weighted[(i, j)] - ideal[j]).powi(2);
                distances_anti_ideal[i] += (weighted[(i, j)] - anti_ideal[j]).powi(2);
            }
            distances_ideal[i] = distances_ideal[i].sqrt();
            distances_anti_ideal[i] = distances_anti_ideal[i].sqrt();
        }

        // Step 5: Calculate relative closeness
        distances_ideal
            .iter()
            .zip(&distances_anti_ideal)
            .map(|(d_plus, d_minus)| {
                if d_plus + d_minus == 0.0 {
                    0.5
                } else {
                    d_minus / (d_plus + d_minus)
                }
            })
            .collect()
    }

    fn normalize_matrix(&self) -> DMatrix<f64> {
        let (n_alt, n_crit) = self.decision_matrix.shape();
        let mut normalized = DMatrix::zeros(n_alt, n_crit);

        for j in 0..n_crit {
            let col_sum_sq: f64 = (0..n_alt)
                .map(|i| self.decision_matrix[(i, j)].powi(2))
                .sum();

            let norm = col_sum_sq.sqrt();

            if norm > 0.0 {
                for i in 0..n_alt {
                    normalized[(i, j)] = self.decision_matrix[(i, j)] / norm;
                }
            }
        }

        normalized
    }

    fn apply_weights(&self, normalized: &DMatrix<f64>) -> DMatrix<f64> {
        let (n_alt, n_crit) = normalized.shape();
        let mut weighted = DMatrix::zeros(n_alt, n_crit);

        for i in 0..n_alt {
            for j in 0..n_crit {
                weighted[(i, j)] = normalized[(i, j)] * self.weights[j];
            }
        }

        weighted
    }

    fn find_ideal_solutions(&self, weighted: &DMatrix<f64>) -> (Vec<f64>, Vec<f64>) {
        let n_crit = weighted.ncols();
        let mut ideal = vec![0.0; n_crit];
        let mut anti_ideal = vec![0.0; n_crit];

        for j in 0..n_crit {
            let column: Vec<f64> = (0..weighted.nrows())
                .map(|i| weighted[(i, j)])
                .collect();

            let max_val = column.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let min_val = column.iter().cloned().fold(f64::INFINITY, f64::min);

            if self.is_beneficial[j] {
                ideal[j] = max_val;
                anti_ideal[j] = min_val;
            } else {
                ideal[j] = min_val;
                anti_ideal[j] = max_val;
            }
        }

        (ideal, anti_ideal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topsis() {
        let matrix = DMatrix::from_row_slice(3, 4, &[
            250.0, 16.0, 12.0, 5.0,
            200.0, 16.0, 8.0, 3.0,
            300.0, 32.0, 16.0, 4.0,
        ]);

        let weights = vec![0.25, 0.25, 0.25, 0.25];
        let is_beneficial = vec![false, true, true, true]; // Cost, then benefits

        let topsis = TOPSIS::new(matrix, weights, is_beneficial).unwrap();
        let scores = topsis.calculate_scores();

        assert_eq!(scores.len(), 3);
        assert!(scores.iter().all(|&s| s >= 0.0 && s <= 1.0));
    }
}
