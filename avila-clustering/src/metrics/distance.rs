//! Distance metrics for clustering

use crate::{ClusteringError, Result};
use ndarray::ArrayView1;

/// Distance metric for clustering
#[derive(Debug, Clone)]
pub enum Metric {
    /// Euclidean distance
    Euclidean,
    /// Manhattan (L1) distance
    Manhattan,
    /// Chebyshev distance
    Chebyshev,
    /// Minkowski distance with parameter p
    Minkowski(f64),
    /// Cosine distance
    Cosine,
    /// Correlation distance
    Correlation,

    // Scientific metrics
    /// Mahalanobis distance with covariance matrix
    Mahalanobis(Vec<Vec<f64>>),
    /// Geodesic distance on manifold
    Geodesic(Manifold),
    /// Spectral angle (astronomy)
    SpectralAngle,
    /// Dynamic Time Warping
    DynamicTimeWarping,
    /// Haversine (great-circle distance)
    Haversine,

    // String metrics
    /// Hamming distance
    Hamming,
    /// Levenshtein distance
    Levenshtein,
    /// Jaro-Winkler distance
    JaroWinkler,

    // Probabilistic
    /// Kullback-Leibler divergence
    KullbackLeibler,
    /// Jensen-Shannon divergence
    JensenShannon,
    /// Wasserstein distance
    Wasserstein,
    /// Hellinger distance
    Hellinger,

    /// Custom distance function
    Custom(fn(&ArrayView1<f64>, &ArrayView1<f64>) -> f64),
}

impl Metric {
    /// Compute distance between two points
    pub fn distance(&self, x: &ArrayView1<f64>, y: &ArrayView1<f64>) -> Result<f64> {
        match self {
            Metric::Euclidean => Ok(euclidean_distance(x, y)),
            Metric::Manhattan => Ok(manhattan_distance(x, y)),
            Metric::Cosine => Ok(cosine_distance(x, y)),
            Metric::Custom(f) => Ok(f(x, y)),
            _ => unimplemented!("Metric::distance for {:?}", self),
        }
    }
}

/// Euclidean distance
pub fn euclidean_distance(x: &ArrayView1<f64>, y: &ArrayView1<f64>) -> f64 {
    x.iter()
        .zip(y.iter())
        .map(|(a, b)| (a - b).powi(2))
        .sum::<f64>()
        .sqrt()
}

/// Manhattan distance
pub fn manhattan_distance(x: &ArrayView1<f64>, y: &ArrayView1<f64>) -> f64 {
    x.iter().zip(y.iter()).map(|(a, b)| (a - b).abs()).sum()
}

/// Cosine distance
pub fn cosine_distance(x: &ArrayView1<f64>, y: &ArrayView1<f64>) -> f64 {
    let dot_product: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    let norm_x: f64 = x.iter().map(|a| a.powi(2)).sum::<f64>().sqrt();
    let norm_y: f64 = y.iter().map(|b| b.powi(2)).sum::<f64>().sqrt();

    if norm_x == 0.0 || norm_y == 0.0 {
        return 1.0;
    }

    1.0 - (dot_product / (norm_x * norm_y))
}

/// Manifold for geodesic distances
#[derive(Debug, Clone)]
pub enum Manifold {
    /// Schwarzschild spacetime (non-rotating black hole)
    Schwarzschild { mass: f64 },
    /// Kerr spacetime (rotating black hole)
    Kerr { mass: f64, spin: f64 },
    /// Flat Euclidean space
    Euclidean,
}

impl Manifold {
    /// Compute geodesic distance on the manifold
    pub fn geodesic_distance(&self, x: &ArrayView1<f64>, y: &ArrayView1<f64>) -> Result<f64> {
        match self {
            Manifold::Euclidean => Ok(euclidean_distance(x, y)),
            _ => unimplemented!("Manifold::geodesic_distance for {:?}", self),
        }
    }
}
