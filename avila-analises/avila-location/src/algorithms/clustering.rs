//! Clustering and market segmentation algorithms

use crate::models::Coordinate;
use crate::{LocationError, Result};
use nalgebra::{DMatrix, DVector};
use rand::Rng;
use rayon::prelude::*;

/// K-Means clustering for location grouping
pub struct KMeans {
    pub k: usize,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl KMeans {
    pub fn new(k: usize) -> Self {
        Self {
            k,
            max_iterations: 100,
            tolerance: 1e-4,
        }
    }

    /// Cluster coordinates into K groups
    pub fn fit(&self, points: &[Coordinate]) -> Result<Vec<usize>> {
        if points.len() < self.k {
            return Err(LocationError::InsufficientData(
                "Not enough points for K clusters".into()
            ));
        }

        let mut rng = rand::thread_rng();

        // Initialize centroids randomly
        let mut centroids: Vec<Coordinate> = (0..self.k)
            .map(|_| {
                let idx = rng.gen_range(0..points.len());
                points[idx]
            })
            .collect();

        let mut assignments = vec![0; points.len()];

        for iteration in 0..self.max_iterations {
            // Assignment step
            for (i, point) in points.iter().enumerate() {
                let mut min_dist = f64::INFINITY;
                let mut min_cluster = 0;

                for (j, centroid) in centroids.iter().enumerate() {
                    let dist = point.distance_to(centroid);
                    if dist < min_dist {
                        min_dist = dist;
                        min_cluster = j;
                    }
                }

                assignments[i] = min_cluster;
            }

            // Update step
            let mut new_centroids = vec![Coordinate::new(0.0, 0.0); self.k];
            let mut counts = vec![0; self.k];

            for (i, point) in points.iter().enumerate() {
                let cluster = assignments[i];
                new_centroids[cluster].latitude += point.latitude;
                new_centroids[cluster].longitude += point.longitude;
                counts[cluster] += 1;
            }

            let mut max_movement = 0.0;

            for k in 0..self.k {
                if counts[k] > 0 {
                    new_centroids[k].latitude /= counts[k] as f64;
                    new_centroids[k].longitude /= counts[k] as f64;

                    let movement = centroids[k].distance_to(&new_centroids[k]);
                    max_movement = max_movement.max(movement);
                }
            }

            centroids = new_centroids;

            if max_movement < self.tolerance {
                break;
            }
        }

        Ok(assignments)
    }
}

/// DBSCAN clustering for density-based location grouping
pub struct DBSCAN {
    /// Epsilon: Maximum distance for neighborhood
    pub eps: f64,

    /// Minimum points for core point
    pub min_pts: usize,
}

impl DBSCAN {
    pub fn new(eps: f64, min_pts: usize) -> Self {
        Self { eps, min_pts }
    }

    /// Cluster points, returns cluster assignment (-1 for noise)
    pub fn fit(&self, points: &[Coordinate]) -> Vec<i32> {
        let mut labels = vec![-1; points.len()];
        let mut cluster_id = 0;

        for i in 0..points.len() {
            if labels[i] != -1 {
                continue;
            }

            let neighbors = self.range_query(points, i);

            if neighbors.len() < self.min_pts {
                labels[i] = -1; // Noise
            } else {
                self.expand_cluster(points, &mut labels, i, neighbors, cluster_id);
                cluster_id += 1;
            }
        }

        labels
    }

    fn range_query(&self, points: &[Coordinate], idx: usize) -> Vec<usize> {
        points
            .iter()
            .enumerate()
            .filter(|(i, point)| {
                *i != idx && points[idx].distance_to(point) <= self.eps
            })
            .map(|(i, _)| i)
            .collect()
    }

    fn expand_cluster(
        &self,
        points: &[Coordinate],
        labels: &mut [i32],
        idx: usize,
        mut neighbors: Vec<usize>,
        cluster_id: i32,
    ) {
        labels[idx] = cluster_id;
        let mut i = 0;

        while i < neighbors.len() {
            let neighbor_idx = neighbors[i];

            if labels[neighbor_idx] == -1 {
                labels[neighbor_idx] = cluster_id;
            }

            if labels[neighbor_idx] == -1 {
                labels[neighbor_idx] = cluster_id;

                let neighbor_neighbors = self.range_query(points, neighbor_idx);
                if neighbor_neighbors.len() >= self.min_pts {
                    neighbors.extend(neighbor_neighbors);
                }
            }

            i += 1;
        }
    }
}

/// Market segmentation using RFM (Recency, Frequency, Monetary)
#[derive(Debug, Clone)]
pub struct RFMSegmentation {
    pub recency_score: f64,
    pub frequency_score: f64,
    pub monetary_score: f64,
}

impl RFMSegmentation {
    pub fn new(recency: f64, frequency: f64, monetary: f64) -> Self {
        Self {
            recency_score: recency,
            frequency_score: frequency,
            monetary_score: monetary,
        }
    }

    pub fn total_score(&self) -> f64 {
        (self.recency_score + self.frequency_score + self.monetary_score) / 3.0
    }

    pub fn segment(&self) -> &'static str {
        let total = self.total_score();

        if total >= 4.0 {
            "Champions"
        } else if total >= 3.5 {
            "Loyal Customers"
        } else if total >= 3.0 {
            "Potential Loyalists"
        } else if total >= 2.5 {
            "At Risk"
        } else if total >= 2.0 {
            "Need Attention"
        } else {
            "Lost"
        }
    }
}

/// Lead scoring algorithm
#[derive(Debug, Clone)]
pub struct LeadScore {
    /// Company characteristics
    pub company_size_score: f64,
    pub industry_fit_score: f64,
    pub tech_maturity_score: f64,
    pub budget_score: f64,

    /// Engagement metrics
    pub engagement_score: f64,
    pub urgency_score: f64,
}

impl LeadScore {
    pub fn total_score(&self) -> f64 {
        self.company_size_score * 0.20 +
        self.industry_fit_score * 0.25 +
        self.tech_maturity_score * 0.20 +
        self.budget_score * 0.15 +
        self.engagement_score * 0.15 +
        self.urgency_score * 0.05
    }

    pub fn classification(&self) -> &'static str {
        let score = self.total_score();

        if score >= 80.0 {
            "Hot Lead"
        } else if score >= 60.0 {
            "Warm Lead"
        } else if score >= 40.0 {
            "Cold Lead"
        } else {
            "Unqualified"
        }
    }
}

/// Customer Lifetime Value calculation
pub fn calculate_clv(
    avg_purchase_value: f64,
    purchase_frequency: f64,
    customer_lifespan_years: f64,
    discount_rate: f64,
) -> f64 {
    let annual_value = avg_purchase_value * purchase_frequency;
    let mut clv = 0.0;

    for year in 1..=(customer_lifespan_years as i32) {
        let discount_factor = (1.0 + discount_rate).powi(-year);
        clv += annual_value * discount_factor;
    }

    clv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kmeans() {
        let points = vec![
            Coordinate::new(0.0, 0.0),
            Coordinate::new(0.1, 0.1),
            Coordinate::new(5.0, 5.0),
            Coordinate::new(5.1, 5.1),
        ];

        let kmeans = KMeans::new(2);
        let clusters = kmeans.fit(&points).unwrap();

        assert_eq!(clusters[0], clusters[1]);
        assert_eq!(clusters[2], clusters[3]);
        assert_ne!(clusters[0], clusters[2]);
    }
}
