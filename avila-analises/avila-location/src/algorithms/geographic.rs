//! Geographic optimization algorithms

use crate::models::{Coordinate, Location};
use crate::{LocationError, Result};
use nalgebra::{DMatrix, DVector};
use rayon::prelude::*;

/// Weber Problem: Find optimal location that minimizes total weighted distance to customers
pub struct WeberProblem {
    /// Customer locations
    pub customers: Vec<Coordinate>,

    /// Customer weights (importance/demand)
    pub weights: Vec<f64>,

    /// Maximum iterations for convergence
    pub max_iterations: usize,

    /// Convergence tolerance
    pub tolerance: f64,
}

impl WeberProblem {
    pub fn new(customers: Vec<Coordinate>, weights: Vec<f64>) -> Result<Self> {
        if customers.len() != weights.len() {
            return Err(LocationError::InvalidCoordinates(
                "Number of customers and weights must match".into()
            ));
        }

        Ok(Self {
            customers,
            weights,
            max_iterations: 1000,
            tolerance: 1e-6,
        })
    }

    /// Solve using Weiszfeld's algorithm
    pub fn solve(&self) -> Result<Coordinate> {
        if self.customers.is_empty() {
            return Err(LocationError::InsufficientData("No customers provided".into()));
        }

        // Initialize at centroid
        let mut x = self.customers.iter().map(|c| c.latitude).sum::<f64>() / self.customers.len() as f64;
        let mut y = self.customers.iter().map(|c| c.longitude).sum::<f64>() / self.customers.len() as f64;

        for _ in 0..self.max_iterations {
            let mut numerator_x = 0.0;
            let mut numerator_y = 0.0;
            let mut denominator = 0.0;

            for (i, customer) in self.customers.iter().enumerate() {
                let dx = customer.latitude - x;
                let dy = customer.longitude - y;
                let distance = (dx * dx + dy * dy).sqrt();

                if distance < self.tolerance {
                    continue;
                }

                let weight_over_dist = self.weights[i] / distance;
                numerator_x += customer.latitude * weight_over_dist;
                numerator_y += customer.longitude * weight_over_dist;
                denominator += weight_over_dist;
            }

            if denominator < self.tolerance {
                break;
            }

            let new_x = numerator_x / denominator;
            let new_y = numerator_y / denominator;

            let change = ((new_x - x).powi(2) + (new_y - y).powi(2)).sqrt();
            x = new_x;
            y = new_y;

            if change < self.tolerance {
                break;
            }
        }

        Ok(Coordinate::new(x, y))
    }

    /// Calculate total weighted distance for a given location
    pub fn total_cost(&self, location: &Coordinate) -> f64 {
        self.customers
            .par_iter()
            .zip(&self.weights)
            .map(|(customer, weight)| {
                location.distance_to(customer) * weight
            })
            .sum()
    }
}

/// P-Median Problem: Find P locations that minimize total distance to customers
pub struct PMedianProblem {
    /// Candidate locations
    pub candidates: Vec<Coordinate>,

    /// Customer locations
    pub customers: Vec<Coordinate>,

    /// Number of locations to select
    pub p: usize,

    /// Customer weights
    pub weights: Vec<f64>,
}

impl PMedianProblem {
    pub fn new(
        candidates: Vec<Coordinate>,
        customers: Vec<Coordinate>,
        p: usize,
        weights: Vec<f64>,
    ) -> Result<Self> {
        if customers.len() != weights.len() {
            return Err(LocationError::InvalidCoordinates(
                "Number of customers and weights must match".into()
            ));
        }

        if p > candidates.len() {
            return Err(LocationError::InvalidCoordinates(
                "P cannot exceed number of candidates".into()
            ));
        }

        Ok(Self {
            candidates,
            customers,
            p,
            weights,
        })
    }

    /// Solve using greedy algorithm
    pub fn solve_greedy(&self) -> Result<Vec<usize>> {
        let mut selected = Vec::new();
        let mut assigned_to: Vec<Option<usize>> = vec![None; self.customers.len()];

        for _ in 0..self.p {
            let mut best_candidate = 0;
            let mut best_improvement = f64::NEG_INFINITY;

            for (cand_idx, candidate) in self.candidates.iter().enumerate() {
                if selected.contains(&cand_idx) {
                    continue;
                }

                let mut improvement = 0.0;

                for (cust_idx, customer) in self.customers.iter().enumerate() {
                    let new_dist = candidate.distance_to(customer);
                    let old_dist = if let Some(assigned_idx) = assigned_to[cust_idx] {
                        self.candidates[assigned_idx].distance_to(customer)
                    } else {
                        f64::INFINITY
                    };

                    if new_dist < old_dist {
                        improvement += (old_dist - new_dist) * self.weights[cust_idx];
                    }
                }

                if improvement > best_improvement {
                    best_improvement = improvement;
                    best_candidate = cand_idx;
                }
            }

            selected.push(best_candidate);

            // Update assignments
            for (cust_idx, customer) in self.customers.iter().enumerate() {
                let current_dist = if let Some(assigned_idx) = assigned_to[cust_idx] {
                    self.candidates[assigned_idx].distance_to(customer)
                } else {
                    f64::INFINITY
                };

                let new_dist = self.candidates[best_candidate].distance_to(customer);
                if new_dist < current_dist {
                    assigned_to[cust_idx] = Some(best_candidate);
                }
            }
        }

        Ok(selected)
    }

    /// Calculate total weighted distance for selected locations
    pub fn total_cost(&self, selected: &[usize]) -> f64 {
        self.customers
            .par_iter()
            .enumerate()
            .map(|(i, customer)| {
                let min_dist = selected
                    .iter()
                    .map(|&idx| self.candidates[idx].distance_to(customer))
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(f64::INFINITY);

                min_dist * self.weights[i]
            })
            .sum()
    }
}

/// Maximal Coverage Location Problem: Maximize customers covered within service radius
pub struct MaximalCoverageProblem {
    /// Candidate locations
    pub candidates: Vec<Coordinate>,

    /// Customer locations
    pub customers: Vec<Coordinate>,

    /// Number of locations to select
    pub p: usize,

    /// Service radius (km)
    pub service_radius: f64,

    /// Customer weights
    pub weights: Vec<f64>,
}

impl MaximalCoverageProblem {
    pub fn new(
        candidates: Vec<Coordinate>,
        customers: Vec<Coordinate>,
        p: usize,
        service_radius: f64,
        weights: Vec<f64>,
    ) -> Result<Self> {
        if customers.len() != weights.len() {
            return Err(LocationError::InvalidCoordinates(
                "Number of customers and weights must match".into()
            ));
        }

        Ok(Self {
            candidates,
            customers,
            p,
            service_radius,
            weights,
        })
    }

    /// Solve using greedy algorithm
    pub fn solve_greedy(&self) -> Result<Vec<usize>> {
        let mut selected = Vec::new();
        let mut covered = vec![false; self.customers.len()];

        for _ in 0..self.p {
            let mut best_candidate = 0;
            let mut best_new_coverage = 0.0;

            for (cand_idx, candidate) in self.candidates.iter().enumerate() {
                if selected.contains(&cand_idx) {
                    continue;
                }

                let new_coverage: f64 = self.customers
                    .iter()
                    .enumerate()
                    .filter(|(i, customer)| {
                        !covered[*i] && candidate.distance_to(customer) <= self.service_radius
                    })
                    .map(|(i, _)| self.weights[i])
                    .sum();

                if new_coverage > best_new_coverage {
                    best_new_coverage = new_coverage;
                    best_candidate = cand_idx;
                }
            }

            selected.push(best_candidate);

            // Update covered customers
            let best_coord = &self.candidates[best_candidate];
            for (i, customer) in self.customers.iter().enumerate() {
                if best_coord.distance_to(customer) <= self.service_radius {
                    covered[i] = true;
                }
            }
        }

        Ok(selected)
    }

    /// Calculate total coverage for selected locations
    pub fn total_coverage(&self, selected: &[usize]) -> f64 {
        let mut covered = vec![false; self.customers.len()];

        for &idx in selected {
            let candidate = &self.candidates[idx];
            for (i, customer) in self.customers.iter().enumerate() {
                if candidate.distance_to(customer) <= self.service_radius {
                    covered[i] = true;
                }
            }
        }

        covered
            .iter()
            .enumerate()
            .filter(|(_, &is_covered)| is_covered)
            .map(|(i, _)| self.weights[i])
            .sum()
    }
}

/// Gravity Model: Calculate attractiveness of each location based on size and distance
pub struct GravityModel {
    /// Locations with their attractiveness (e.g., market size)
    pub locations: Vec<(Coordinate, f64)>,

    /// Distance decay parameter (typically 1.0 to 2.0)
    pub distance_decay: f64,
}

impl GravityModel {
    pub fn new(locations: Vec<(Coordinate, f64)>, distance_decay: f64) -> Self {
        Self {
            locations,
            distance_decay,
        }
    }

    /// Calculate gravity score for a point
    pub fn calculate_gravity(&self, point: &Coordinate) -> f64 {
        self.locations
            .par_iter()
            .map(|(location, attractiveness)| {
                let distance = point.distance_to(location).max(1.0); // Avoid division by zero
                attractiveness / distance.powf(self.distance_decay)
            })
            .sum()
    }

    /// Find location with highest gravity from candidates
    pub fn find_optimal(&self, candidates: &[Coordinate]) -> Option<(usize, f64)> {
        candidates
            .par_iter()
            .enumerate()
            .map(|(i, coord)| (i, self.calculate_gravity(coord)))
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
    }
}

/// Voronoi Diagram: Define service areas for each location
pub fn calculate_voronoi_region(
    location: &Coordinate,
    other_locations: &[Coordinate],
    test_point: &Coordinate,
) -> bool {
    let dist_to_location = location.distance_to(test_point);

    other_locations
        .iter()
        .all(|other| dist_to_location <= other.distance_to(test_point))
}

/// Center Problem: Minimize maximum distance to any customer
pub struct CenterProblem {
    /// Candidate locations
    pub candidates: Vec<Coordinate>,

    /// Customer locations
    pub customers: Vec<Coordinate>,
}

impl CenterProblem {
    pub fn new(candidates: Vec<Coordinate>, customers: Vec<Coordinate>) -> Self {
        Self {
            candidates,
            customers,
        }
    }

    /// Find location that minimizes maximum distance
    pub fn solve(&self) -> Option<(usize, f64)> {
        self.candidates
            .par_iter()
            .enumerate()
            .map(|(i, candidate)| {
                let max_dist = self.customers
                    .iter()
                    .map(|customer| candidate.distance_to(customer))
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0);

                (i, max_dist)
            })
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weber_problem() {
        let customers = vec![
            Coordinate::new(40.7128, -74.0060), // New York
            Coordinate::new(34.0522, -118.2437), // Los Angeles
            Coordinate::new(41.8781, -87.6298),  // Chicago
        ];
        let weights = vec![1.0, 1.0, 1.0];

        let problem = WeberProblem::new(customers, weights).unwrap();
        let solution = problem.solve().unwrap();

        assert!(solution.latitude > 30.0 && solution.latitude < 45.0);
        assert!(solution.longitude < -70.0 && solution.longitude > -120.0);
    }
}
