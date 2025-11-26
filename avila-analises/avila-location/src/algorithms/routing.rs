//! Routing and accessibility algorithms

use crate::models::Coordinate;
use crate::{LocationError, Result};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

/// Traveling Salesman Problem solver (nearest neighbor heuristic)
pub struct TSPSolver {
    pub locations: Vec<Coordinate>,
}

impl TSPSolver {
    pub fn new(locations: Vec<Coordinate>) -> Self {
        Self { locations }
    }

    /// Solve using nearest neighbor heuristic
    pub fn solve_nearest_neighbor(&self, start_idx: usize) -> Result<Vec<usize>> {
        if self.locations.is_empty() {
            return Err(LocationError::InsufficientData("No locations provided".into()));
        }

        let n = self.locations.len();
        let mut route = Vec::with_capacity(n);
        let mut visited = vec![false; n];

        let mut current = start_idx;
        route.push(current);
        visited[current] = true;

        for _ in 1..n {
            let mut nearest = 0;
            let mut min_dist = f64::INFINITY;

            for (i, location) in self.locations.iter().enumerate() {
                if !visited[i] {
                    let dist = self.locations[current].distance_to(location);
                    if dist < min_dist {
                        min_dist = dist;
                        nearest = i;
                    }
                }
            }

            route.push(nearest);
            visited[nearest] = true;
            current = nearest;
        }

        Ok(route)
    }

    /// Calculate total route distance
    pub fn route_distance(&self, route: &[usize]) -> f64 {
        let mut total = 0.0;

        for i in 0..route.len() - 1 {
            total += self.locations[route[i]].distance_to(&self.locations[route[i + 1]]);
        }

        // Return to start
        if !route.is_empty() {
            total += self.locations[*route.last().unwrap()]
                .distance_to(&self.locations[route[0]]);
        }

        total
    }

    /// 2-opt optimization
    pub fn optimize_2opt(&self, route: &mut Vec<usize>, max_iterations: usize) {
        let n = route.len();
        let mut improved = true;
        let mut iteration = 0;

        while improved && iteration < max_iterations {
            improved = false;

            for i in 1..n - 1 {
                for j in i + 1..n {
                    let delta = self.calculate_2opt_delta(route, i, j);

                    if delta < -1e-6 {
                        // Reverse segment [i, j]
                        route[i..=j].reverse();
                        improved = true;
                    }
                }
            }

            iteration += 1;
        }
    }

    fn calculate_2opt_delta(&self, route: &[usize], i: usize, j: usize) -> f64 {
        let a = route[i - 1];
        let b = route[i];
        let c = route[j];
        let d = if j + 1 < route.len() { route[j + 1] } else { route[0] };

        let current = self.locations[a].distance_to(&self.locations[b])
            + self.locations[c].distance_to(&self.locations[d]);

        let new = self.locations[a].distance_to(&self.locations[c])
            + self.locations[b].distance_to(&self.locations[d]);

        new - current
    }
}

/// Isochrone analysis: Calculate reachable area within time/distance
pub struct IsochroneAnalysis {
    /// Origin point
    pub origin: Coordinate,

    /// Maximum distance (km)
    pub max_distance: f64,

    /// Distance intervals
    pub intervals: Vec<f64>,
}

impl IsochroneAnalysis {
    pub fn new(origin: Coordinate, max_distance: f64, num_intervals: usize) -> Self {
        let intervals = (1..=num_intervals)
            .map(|i| (i as f64 / num_intervals as f64) * max_distance)
            .collect();

        Self {
            origin,
            max_distance,
            intervals,
        }
    }

    /// Classify points by isochrone interval
    pub fn classify_points(&self, points: &[Coordinate]) -> Vec<usize> {
        points
            .iter()
            .map(|point| {
                let dist = self.origin.distance_to(point);

                self.intervals
                    .iter()
                    .position(|&interval| dist <= interval)
                    .unwrap_or(self.intervals.len())
            })
            .collect()
    }

    /// Calculate coverage statistics
    pub fn coverage_stats(&self, points: &[Coordinate]) -> Vec<usize> {
        let classifications = self.classify_points(points);
        let mut counts = vec![0; self.intervals.len() + 1];

        for classification in classifications {
            counts[classification] += 1;
        }

        counts
    }
}

/// Time-Distance Matrix calculation
pub struct TimeDistanceMatrix {
    pub locations: Vec<Coordinate>,
    pub distance_matrix: Vec<Vec<f64>>,
}

impl TimeDistanceMatrix {
    pub fn new(locations: Vec<Coordinate>) -> Self {
        let n = locations.len();
        let mut distance_matrix = vec![vec![0.0; n]; n];

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    distance_matrix[i][j] = locations[i].distance_to(&locations[j]);
                }
            }
        }

        Self {
            locations,
            distance_matrix,
        }
    }

    /// Find closest location to a point
    pub fn find_closest(&self, point: &Coordinate) -> Option<(usize, f64)> {
        self.locations
            .iter()
            .enumerate()
            .map(|(i, loc)| (i, loc.distance_to(point)))
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
    }

    /// Calculate average distance from a location to all others
    pub fn average_distance(&self, location_idx: usize) -> f64 {
        let sum: f64 = self.distance_matrix[location_idx].iter().sum();
        sum / (self.locations.len() - 1) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tsp() {
        let locations = vec![
            Coordinate::new(0.0, 0.0),
            Coordinate::new(1.0, 0.0),
            Coordinate::new(1.0, 1.0),
            Coordinate::new(0.0, 1.0),
        ];

        let tsp = TSPSolver::new(locations);
        let route = tsp.solve_nearest_neighbor(0).unwrap();

        assert_eq!(route.len(), 4);
        assert_eq!(route[0], 0);
    }
}
