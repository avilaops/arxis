//! Multi-criteria decision analysis algorithms

pub mod ahp;
pub mod topsis;
pub mod electre;
pub mod maut;

pub use ahp::*;
pub use topsis::*;
pub use electre::*;
pub use maut::*;

use crate::models::LocationScore;

/// Rank locations by total score
pub fn rank_locations(scores: &mut [LocationScore]) {
    scores.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

    for (i, score) in scores.iter_mut().enumerate() {
        score.rank = Some(i + 1);
    }
}

/// Normalize scores to 0-100 range
pub fn normalize_scores(values: &[f64]) -> Vec<f64> {
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    if (max - min).abs() < 1e-10 {
        return vec![50.0; values.len()];
    }

    values
        .iter()
        .map(|&v| ((v - min) / (max - min)) * 100.0)
        .collect()
}

/// Normalize scores where lower is better
pub fn normalize_scores_inverse(values: &[f64]) -> Vec<f64> {
    let min = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    if (max - min).abs() < 1e-10 {
        return vec![50.0; values.len()];
    }

    values
        .iter()
        .map(|&v| ((max - v) / (max - min)) * 100.0)
        .collect()
}
