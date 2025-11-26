//! MAUT (Multi-Attribute Utility Theory)

use crate::{LocationError, Result};

/// Utility function type
#[derive(Debug, Clone, Copy)]
pub enum UtilityFunction {
    /// Linear utility
    Linear,

    /// Exponential utility (risk averse)
    Exponential { risk_aversion: f64 },

    /// Logarithmic utility
    Logarithmic,

    /// Power utility
    Power { exponent: f64 },
}

impl UtilityFunction {
    /// Calculate utility for a normalized value [0, 1]
    pub fn utility(&self, value: f64) -> f64 {
        match self {
            UtilityFunction::Linear => value,

            UtilityFunction::Exponential { risk_aversion } => {
                if *risk_aversion == 0.0 {
                    value
                } else {
                    (1.0 - (-risk_aversion * value).exp()) / (1.0 - (-risk_aversion).exp())
                }
            }

            UtilityFunction::Logarithmic => {
                if value <= 0.0 {
                    0.0
                } else {
                    (1.0 + value).ln() / 2.0f64.ln()
                }
            }

            UtilityFunction::Power { exponent } => {
                value.powf(*exponent)
            }
        }
    }
}

/// MAUT decision maker
pub struct MAUT {
    /// Criterion utility functions
    pub utility_functions: Vec<UtilityFunction>,

    /// Criterion weights
    pub weights: Vec<f64>,

    /// Criterion names
    pub criteria: Vec<String>,
}

impl MAUT {
    pub fn new(
        utility_functions: Vec<UtilityFunction>,
        weights: Vec<f64>,
        criteria: Vec<String>,
    ) -> Result<Self> {
        if utility_functions.len() != weights.len() || weights.len() != criteria.len() {
            return Err(LocationError::AnalysisError(
                "Utility functions, weights, and criteria must have same length".into()
            ));
        }

        let weight_sum: f64 = weights.iter().sum();
        if (weight_sum - 1.0).abs() > 0.01 {
            return Err(LocationError::AnalysisError(
                "Weights must sum to 1.0".into()
            ));
        }

        Ok(Self {
            utility_functions,
            weights,
            criteria,
        })
    }

    /// Calculate total utility for an alternative
    /// Values should be normalized to [0, 1] range
    pub fn calculate_utility(&self, normalized_values: &[f64]) -> Result<f64> {
        if normalized_values.len() != self.weights.len() {
            return Err(LocationError::AnalysisError(
                "Number of values must match criteria".into()
            ));
        }

        let total_utility = normalized_values
            .iter()
            .zip(&self.utility_functions)
            .zip(&self.weights)
            .map(|((value, func), weight)| {
                func.utility(*value) * weight
            })
            .sum();

        Ok(total_utility)
    }

    /// Calculate utilities for multiple alternatives
    pub fn calculate_utilities(&self, alternatives: &[Vec<f64>]) -> Result<Vec<f64>> {
        alternatives
            .iter()
            .map(|alt| self.calculate_utility(alt))
            .collect()
    }

    /// Find best alternative
    pub fn find_best(&self, alternatives: &[Vec<f64>]) -> Result<(usize, f64)> {
        let utilities = self.calculate_utilities(alternatives)?;

        utilities
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .ok_or_else(|| LocationError::InsufficientData("No alternatives provided".into()))
    }
}

/// Additive value function (simplified MAUT)
pub struct AdditiveValue {
    pub weights: Vec<f64>,
}

impl AdditiveValue {
    pub fn new(weights: Vec<f64>) -> Result<Self> {
        let sum: f64 = weights.iter().sum();
        if (sum - 1.0).abs() > 0.01 {
            return Err(LocationError::AnalysisError(
                "Weights must sum to 1.0".into()
            ));
        }

        Ok(Self { weights })
    }

    /// Calculate value (assumes values are already normalized)
    pub fn calculate(&self, values: &[f64]) -> f64 {
        values
            .iter()
            .zip(&self.weights)
            .map(|(v, w)| v * w)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_utility_functions() {
        let linear = UtilityFunction::Linear;
        assert_eq!(linear.utility(0.5), 0.5);

        let exp = UtilityFunction::Exponential { risk_aversion: 1.0 };
        let util = exp.utility(0.5);
        assert!(util > 0.0 && util < 1.0);

        let log = UtilityFunction::Logarithmic;
        let util = log.utility(0.5);
        assert!(util > 0.0 && util < 1.0);
    }

    #[test]
    fn test_maut() {
        let functions = vec![
            UtilityFunction::Linear,
            UtilityFunction::Linear,
            UtilityFunction::Linear,
        ];
        let weights = vec![0.5, 0.3, 0.2];
        let criteria = vec!["A".to_string(), "B".to_string(), "C".to_string()];

        let maut = MAUT::new(functions, weights, criteria).unwrap();
        let utility = maut.calculate_utility(&[1.0, 0.5, 0.0]).unwrap();

        assert!((utility - 0.65).abs() < 0.01);
    }
}
