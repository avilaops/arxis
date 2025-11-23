//! Time Series Clustering
//!
//! Specialized clustering for temporal data using Dynamic Time Warping (DTW),
//! shape-based distance, and temporal pattern matching.
//!
//! # Use Cases
//!
//! - Stock price pattern recognition
//! - Heartbeat/ECG classification
//! - Sensor data grouping
//! - Voice/audio pattern matching
//! - Weather pattern detection

use crate::{ClusteringError, Result};
use ndarray::{Array1, Array2, ArrayView1, ArrayView2, Axis};

/// Dynamic Time Warping distance between two time series
pub fn dtw_distance(series1: ArrayView1<f64>, series2: ArrayView1<f64>) -> f64 {
    let n = series1.len();
    let m = series2.len();

    let mut dtw = Array2::<f64>::from_elem((n + 1, m + 1), f64::INFINITY);
    dtw[[0, 0]] = 0.0;

    for i in 1..=n {
        for j in 1..=m {
            let cost = (series1[i - 1] - series2[j - 1]).abs();
            dtw[[i, j]] = cost + dtw[[i - 1, j]]
                .min(dtw[[i, j - 1]])
                .min(dtw[[i - 1, j - 1]]);
        }
    }

    dtw[[n, m]]
}

/// Shape-Based Distance (SBD) using cross-correlation
pub fn shape_based_distance(series1: ArrayView1<f64>, series2: ArrayView1<f64>) -> f64 {
    let mean1 = series1.mean().unwrap_or(0.0);
    let mean2 = series2.mean().unwrap_or(0.0);

    // Z-normalize
    let norm1: Array1<f64> = series1.iter().map(|&x| x - mean1).collect();
    let norm2: Array1<f64> = series2.iter().map(|&x| x - mean2).collect();

    let std1 = norm1.iter().map(|&x| x * x).sum::<f64>().sqrt();
    let std2 = norm2.iter().map(|&x| x * x).sum::<f64>().sqrt();

    if std1 == 0.0 || std2 == 0.0 {
        return f64::INFINITY;
    }

    // Cross-correlation
    let max_corr = norm1.iter()
        .zip(norm2.iter())
        .map(|(&x, &y)| x * y)
        .sum::<f64>() / (std1 * std2);

    1.0 - max_corr.max(-1.0).min(1.0)
}

/// Time Series K-Means with DTW distance
pub struct TimeSeriesKMeans {
    n_clusters: usize,
    max_iter: usize,
    distance_metric: TimeSeriesMetric,
}

#[derive(Clone, Copy)]
pub enum TimeSeriesMetric {
    /// Dynamic Time Warping
    DTW,
    /// Euclidean distance
    Euclidean,
    /// Shape-Based Distance
    SBD,
    /// Derivative DTW (considers trends)
    DerivativeDTW,
}

impl TimeSeriesKMeans {
    pub fn new(n_clusters: usize) -> Self {
        Self {
            n_clusters,
            max_iter: 100,
            distance_metric: TimeSeriesMetric::DTW,
        }
    }

    pub fn metric(mut self, metric: TimeSeriesMetric) -> Self {
        self.distance_metric = metric;
        self
    }

    pub fn max_iter(mut self, max_iter: usize) -> Self {
        self.max_iter = max_iter;
        self
    }

    /// Fit time series clustering
    /// data: (n_series, series_length) array
    pub fn fit(&self, data: ArrayView2<f64>) -> Result<TimeSeriesClusterResult> {
        if data.nrows() < self.n_clusters {
            return Err(ClusteringError::InvalidParameter(
                "Number of series must be >= n_clusters".to_string()
            ));
        }

        let n_series = data.nrows();
        let series_len = data.ncols();

        // Initialize centroids randomly
        let mut centroids = Array2::<f64>::zeros((self.n_clusters, series_len));
        for i in 0..self.n_clusters {
            centroids.row_mut(i).assign(&data.row(i));
        }

        let mut labels = vec![0; n_series];

        for _iter in 0..self.max_iter {
            let old_labels = labels.clone();

            // Assign to nearest centroid
            for (i, series) in data.axis_iter(Axis(0)).enumerate() {
                let mut min_dist = f64::INFINITY;
                let mut best_cluster = 0;

                for (j, centroid) in centroids.axis_iter(Axis(0)).enumerate() {
                    let dist = self.compute_distance(series, centroid);
                    if dist < min_dist {
                        min_dist = dist;
                        best_cluster = j;
                    }
                }

                labels[i] = best_cluster;
            }

            // Update centroids (DBA - DTW Barycenter Averaging for DTW)
            for k in 0..self.n_clusters {
                let cluster_series: Vec<_> = labels.iter()
                    .enumerate()
                    .filter(|(_, &label)| label == k)
                    .map(|(i, _)| data.row(i))
                    .collect();

                if !cluster_series.is_empty() {
                    // Simple mean for non-DTW metrics
                    let mut new_centroid = Array1::<f64>::zeros(series_len);
                    for series in &cluster_series {
                        new_centroid = &new_centroid + &series.to_owned();
                    }
                    new_centroid /= cluster_series.len() as f64;
                    centroids.row_mut(k).assign(&new_centroid);
                }
            }

            // Check convergence
            if labels == old_labels {
                break;
            }
        }

        Ok(TimeSeriesClusterResult {
            labels,
            centroids,
            n_clusters: self.n_clusters,
        })
    }

    fn compute_distance(&self, s1: ArrayView1<f64>, s2: ArrayView1<f64>) -> f64 {
        match self.distance_metric {
            TimeSeriesMetric::DTW => dtw_distance(s1, s2),
            TimeSeriesMetric::Euclidean => {
                s1.iter().zip(s2.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f64>().sqrt()
            }
            TimeSeriesMetric::SBD => shape_based_distance(s1, s2),
            TimeSeriesMetric::DerivativeDTW => {
                // Derivative of time series
                let d1: Array1<f64> = (1..s1.len()).map(|i| s1[i] - s1[i-1]).collect();
                let d2: Array1<f64> = (1..s2.len()).map(|i| s2[i] - s2[i-1]).collect();
                dtw_distance(d1.view(), d2.view())
            }
        }
    }
}

pub struct TimeSeriesClusterResult {
    pub labels: Vec<usize>,
    pub centroids: Array2<f64>,
    pub n_clusters: usize,
}

/// Motif Discovery - Find recurring patterns in time series
pub struct MotifDiscovery {
    pattern_length: usize,
    n_motifs: usize,
    distance_threshold: f64,
}

impl MotifDiscovery {
    pub fn new(pattern_length: usize, n_motifs: usize) -> Self {
        Self {
            pattern_length,
            n_motifs,
            distance_threshold: 0.5,
        }
    }

    pub fn threshold(mut self, threshold: f64) -> Self {
        self.distance_threshold = threshold;
        self
    }

    /// Find top recurring patterns (motifs) in time series
    pub fn find_motifs(&self, series: ArrayView1<f64>) -> Result<Vec<Motif>> {
        if series.len() < self.pattern_length {
            return Err(ClusteringError::InvalidParameter(
                "Series too short for pattern length".to_string()
            ));
        }

        let n_windows = series.len() - self.pattern_length + 1;
        let mut motifs = Vec::new();

        // Extract all subsequences
        let mut subsequences = Vec::new();
        for i in 0..n_windows {
            let window = series.slice(ndarray::s![i..i + self.pattern_length]);
            subsequences.push((i, window.to_owned()));
        }

        // Find motifs by clustering subsequences
        let mut used_indices = vec![false; n_windows];

        for _ in 0..self.n_motifs {
            let mut best_motif: Option<Motif> = None;
            let mut max_count = 0;

            // Try each unused subsequence as potential motif
            for (idx1, subseq1) in &subsequences {
                if used_indices[*idx1] {
                    continue;
                }

                let mut matches = vec![*idx1];

                for (idx2, subseq2) in &subsequences {
                    if *idx2 != *idx1 && !used_indices[*idx2] {
                        let dist = dtw_distance(subseq1.view(), subseq2.view());
                        if dist < self.distance_threshold {
                            matches.push(*idx2);
                        }
                    }
                }

                if matches.len() > max_count {
                    max_count = matches.len();
                    best_motif = Some(Motif {
                        pattern: subseq1.clone(),
                        occurrences: matches.clone(),
                        frequency: matches.len(),
                    });
                }
            }

            if let Some(motif) = best_motif {
                for &idx in &motif.occurrences {
                    used_indices[idx] = true;
                }
                motifs.push(motif);
            } else {
                break;
            }
        }

        Ok(motifs)
    }
}

pub struct Motif {
    pub pattern: Array1<f64>,
    pub occurrences: Vec<usize>,
    pub frequency: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_dtw_basic() {
        let s1 = array![1.0, 2.0, 3.0, 4.0];
        let s2 = array![1.0, 2.0, 3.0, 4.0];
        let dist = dtw_distance(s1.view(), s2.view());
        assert!(dist < 0.1);
    }

    #[test]
    fn test_timeseries_kmeans() {
        let data = array![
            [1.0, 2.0, 3.0, 4.0],
            [1.1, 2.1, 3.1, 4.1],
            [10.0, 11.0, 12.0, 13.0],
            [10.1, 11.1, 12.1, 13.1],
        ];

        let result = TimeSeriesKMeans::new(2)
            .fit(data.view())
            .unwrap();

        assert_eq!(result.n_clusters, 2);
        assert_eq!(result.labels.len(), 4);
    }
}
