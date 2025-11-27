//! Clustering algorithms for spatial data

use geo::Coord;
use std::collections::{HashSet, VecDeque};

/// DBSCAN (Density-Based Spatial Clustering of Applications with Noise)
///
/// # Arguments
/// * `points` - Array of 2D coordinates
/// * `epsilon` - Maximum distance between two points to be considered neighbors
/// * `min_points` - Minimum number of points required to form a dense region
///
/// # Returns
/// Vector of clusters, where each cluster is a vector of point indices
pub fn dbscan(points: &[Coord<f64>], epsilon: f64, min_points: usize) -> Vec<Vec<usize>> {
    if points.is_empty() {
        return vec![];
    }

    let mut clusters = Vec::new();
    let mut visited = HashSet::new();

    for i in 0..points.len() {
        if visited.contains(&i) {
            continue;
        }

        visited.insert(i);
        let neighbors = region_query(points, i, epsilon);

        if neighbors.len() < min_points {
            continue; // Noise point
        }

        // Start new cluster
        let mut cluster = Vec::new();
        let mut queue = VecDeque::from(neighbors.clone());

        while let Some(neighbor) = queue.pop_front() {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                let neighbor_neighbors = region_query(points, neighbor, epsilon);

                if neighbor_neighbors.len() >= min_points {
                    for &nn in &neighbor_neighbors {
                        if !queue.contains(&nn) && !cluster.contains(&nn) {
                            queue.push_back(nn);
                        }
                    }
                }
            }

            if !cluster.contains(&neighbor) {
                cluster.push(neighbor);
            }
        }

        clusters.push(cluster);
    }

    clusters
}

/// Find all points within epsilon distance
fn region_query(points: &[Coord<f64>], index: usize, epsilon: f64) -> Vec<usize> {
    let point = points[index];
    let epsilon_sq = epsilon * epsilon;

    points
        .iter()
        .enumerate()
        .filter(|(_, p)| {
            let dx = p.x - point.x;
            let dy = p.y - point.y;
            dx * dx + dy * dy <= epsilon_sq
        })
        .map(|(i, _)| i)
        .collect()
}

/// K-means clustering
pub fn kmeans(points: &[Coord<f64>], k: usize, max_iterations: usize) -> Vec<Vec<usize>> {
    if points.is_empty() || k == 0 {
        return vec![];
    }

    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();

    // Initialize centroids randomly
    let mut centroids: Vec<Coord<f64>> = points
        .choose_multiple(&mut rng, k.min(points.len()))
        .copied()
        .collect();

    let mut assignments = vec![0; points.len()];

    for _ in 0..max_iterations {
        let mut changed = false;

        // Assign points to nearest centroid
        for (i, point) in points.iter().enumerate() {
            let nearest = centroids
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| {
                    let dist_a = (point.x - a.x).powi(2) + (point.y - a.y).powi(2);
                    let dist_b = (point.x - b.x).powi(2) + (point.y - b.y).powi(2);
                    dist_a.partial_cmp(&dist_b).unwrap()
                })
                .map(|(idx, _)| idx)
                .unwrap();

            if assignments[i] != nearest {
                assignments[i] = nearest;
                changed = true;
            }
        }

        if !changed {
            break;
        }

        // Update centroids
        let mut sums = vec![Coord { x: 0.0, y: 0.0 }; k];
        let mut counts = vec![0; k];

        for (i, &cluster_id) in assignments.iter().enumerate() {
            sums[cluster_id].x += points[i].x;
            sums[cluster_id].y += points[i].y;
            counts[cluster_id] += 1;
        }

        for i in 0..k {
            if counts[i] > 0 {
                centroids[i].x = sums[i].x / counts[i] as f64;
                centroids[i].y = sums[i].y / counts[i] as f64;
            }
        }
    }

    // Convert to cluster format
    let mut clusters = vec![Vec::new(); k];
    for (i, &cluster_id) in assignments.iter().enumerate() {
        clusters[cluster_id].push(i);
    }

    clusters.into_iter().filter(|c| !c.is_empty()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbscan() {
        let points = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.1, y: 0.1 },
            Coord { x: 5.0, y: 5.0 },
            Coord { x: 5.1, y: 5.0 },
        ];

        let clusters = dbscan(&points, 0.5, 2);
        assert!(!clusters.is_empty());
    }

    #[test]
    fn test_kmeans() {
        let points = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 0.1, y: 0.1 },
            Coord { x: 5.0, y: 5.0 },
            Coord { x: 5.1, y: 5.0 },
        ];

        let clusters = kmeans(&points, 2, 100);
        assert_eq!(clusters.len(), 2);
    }
}
