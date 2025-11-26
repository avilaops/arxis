//! ELECTRE (Elimination and Choice Translating Reality)

use nalgebra::DMatrix;
use crate::{LocationError, Result};

/// ELECTRE III decision maker
pub struct ELECTRE {
    /// Decision matrix (alternatives x criteria)
    pub decision_matrix: DMatrix<f64>,

    /// Criterion weights
    pub weights: Vec<f64>,

    /// Beneficial criteria (true) vs cost criteria (false)
    pub is_beneficial: Vec<bool>,

    /// Indifference threshold
    pub indifference_threshold: f64,

    /// Preference threshold
    pub preference_threshold: f64,
}

impl ELECTRE {
    pub fn new(
        decision_matrix: DMatrix<f64>,
        weights: Vec<f64>,
        is_beneficial: Vec<bool>,
    ) -> Result<Self> {
        let n_criteria = decision_matrix.ncols();

        if weights.len() != n_criteria || is_beneficial.len() != n_criteria {
            return Err(LocationError::AnalysisError(
                "Weights and benefit indicators must match criteria count".into()
            ));
        }

        Ok(Self {
            decision_matrix,
            weights,
            is_beneficial,
            indifference_threshold: 0.1,
            preference_threshold: 0.3,
        })
    }

    /// Calculate concordance matrix
    pub fn concordance_matrix(&self) -> DMatrix<f64> {
        let n = self.decision_matrix.nrows();
        let mut concordance = DMatrix::zeros(n, n);

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let mut concordance_sum = 0.0;

                    for k in 0..self.decision_matrix.ncols() {
                        let val_i = self.decision_matrix[(i, k)];
                        let val_j = self.decision_matrix[(j, k)];

                        let diff = if self.is_beneficial[k] {
                            val_i - val_j
                        } else {
                            val_j - val_i
                        };

                        if diff >= -self.indifference_threshold {
                            concordance_sum += self.weights[k];
                        } else if diff >= -self.preference_threshold {
                            let ratio = (self.preference_threshold + diff) /
                                       (self.preference_threshold - self.indifference_threshold);
                            concordance_sum += self.weights[k] * ratio;
                        }
                    }

                    concordance[(i, j)] = concordance_sum;
                }
            }
        }

        concordance
    }

    /// Calculate discordance matrix
    pub fn discordance_matrix(&self) -> DMatrix<f64> {
        let n = self.decision_matrix.nrows();
        let mut discordance = DMatrix::zeros(n, n);

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let mut max_discordance = 0.0;

                    for k in 0..self.decision_matrix.ncols() {
                        let val_i = self.decision_matrix[(i, k)];
                        let val_j = self.decision_matrix[(j, k)];

                        let diff = if self.is_beneficial[k] {
                            val_j - val_i
                        } else {
                            val_i - val_j
                        };

                        if diff > self.preference_threshold {
                            let disc = (diff - self.preference_threshold) /
                                      (1.0 - self.preference_threshold);
                            max_discordance = max_discordance.max(disc);
                        }
                    }

                    discordance[(i, j)] = max_discordance;
                }
            }
        }

        discordance
    }

    /// Calculate credibility matrix
    pub fn credibility_matrix(&self) -> DMatrix<f64> {
        let concordance = self.concordance_matrix();
        let discordance = self.discordance_matrix();
        let n = concordance.nrows();

        let mut credibility = concordance.clone();

        for i in 0..n {
            for j in 0..n {
                if concordance[(i, j)] < discordance[(i, j)] {
                    credibility[(i, j)] = concordance[(i, j)] *
                        ((1.0 - discordance[(i, j)]) / (1.0 - concordance[(i, j)]));
                }
            }
        }

        credibility
    }

    /// Rank alternatives based on credibility
    pub fn rank_alternatives(&self) -> Vec<usize> {
        let credibility = self.credibility_matrix();
        let n = credibility.nrows();

        let mut scores = vec![0.0; n];

        for i in 0..n {
            for j in 0..n {
                scores[i] += credibility[(i, j)] - credibility[(j, i)];
            }
        }

        let mut ranking: Vec<usize> = (0..n).collect();
        ranking.sort_by(|&a, &b| scores[b].partial_cmp(&scores[a]).unwrap());

        ranking
    }
}
