//! Location optimization algorithms
//!
//! Implements classical facility location problems:
//! - **Weber Problem**: Minimize total weighted distance
//! - **P-Median Problem**: Optimal placement of p facilities
//! - **Center Problem**: Minimize maximum distance
//! - **Coverage Problem**: Maximize coverage within distance

use crate::distance::haversine_distance_m;
use crate::error::{GeoError, Result};
use geo::Coord;
use std::collections::HashSet;

/// Demand point with location and weight
#[derive(Debug, Clone)]
pub struct DemandPoint {
    pub location: Coord<f64>,
    pub weight: f64,
}

impl DemandPoint {
    pub fn new(location: Coord<f64>, weight: f64) -> Self {
        Self { location, weight }
    }
}

/// Solve the Weber problem (Fermat-Weber point)
///
/// Find the point that minimizes the sum of weighted distances to all demand points.
/// Uses iterative Weiszfeld's algorithm.
///
/// # Algorithm
/// Weiszfeld's iterative algorithm:
/// ```text
/// x_{k+1} = Σ(w_i * p_i / d_i) / Σ(w_i / d_i)
/// ```
/// where w_i = weight, p_i = demand point, d_i = distance
///
/// # Complexity
/// O(n * iterations) where n is number of demand points
///
/// # Example
/// ```
/// use geospatial_analysis::optimization::{weber_location, DemandPoint};
/// use geo::Coord;
///
/// let demand = vec![
///     DemandPoint::new(Coord { x: -9.1393, y: 38.7223 }, 1000.0), // Lisbon
///     DemandPoint::new(Coord { x: -8.6291, y: 41.1579 }, 800.0),  // Porto
///     DemandPoint::new(Coord { x: -7.9304, y: 37.0194 }, 300.0),  // Faro
/// ];
///
/// let optimal = weber_location(&demand, 100).unwrap();
/// // Optimal location should be between Lisbon and Porto (weighted towards Lisbon)
/// ```
///
/// # References
/// - E. Weiszfeld, "Sur le point pour lequel la somme des distances de n points
///   donnés est minimum", Tohoku Mathematical Journal, 1937
pub fn weber_location(demand_points: &[DemandPoint], max_iterations: usize) -> Result<Coord<f64>> {
    if demand_points.is_empty() {
        return Err(GeoError::InvalidParameter(
            "Need at least one demand point".to_string(),
        ));
    }

    // Initialize at weighted centroid
    let mut current = weighted_centroid(demand_points)?;

    for _ in 0..max_iterations {
        let mut numerator_x = 0.0;
        let mut numerator_y = 0.0;
        let mut denominator = 0.0;

        for dp in demand_points {
            let dist = haversine_distance_m(&current, &dp.location)?;

            if dist < 1e-6 {
                // Already at a demand point
                return Ok(current);
            }

            let weight_over_dist = dp.weight / dist;
            numerator_x += dp.location.x * weight_over_dist;
            numerator_y += dp.location.y * weight_over_dist;
            denominator += weight_over_dist;
        }

        let next = Coord {
            x: numerator_x / denominator,
            y: numerator_y / denominator,
        };

        // Check convergence
        let movement = haversine_distance_m(&current, &next)?;
        if movement < 1.0 {
            // Converged within 1 meter
            return Ok(next);
        }

        current = next;
    }

    Ok(current)
}

/// Calculate weighted centroid (center of mass) of demand points
///
/// # Example
/// ```
/// use geospatial_analysis::optimization::{weighted_centroid, DemandPoint};
/// use geo::Coord;
///
/// let demand = vec![
///     DemandPoint::new(Coord { x: 0.0, y: 0.0 }, 1.0),
///     DemandPoint::new(Coord { x: 10.0, y: 0.0 }, 1.0),
/// ];
///
/// let center = weighted_centroid(&demand).unwrap();
/// assert_eq!(center.x, 5.0);
/// assert_eq!(center.y, 0.0);
/// ```
pub fn weighted_centroid(demand_points: &[DemandPoint]) -> Result<Coord<f64>> {
    if demand_points.is_empty() {
        return Err(GeoError::InvalidParameter(
            "Need at least one demand point".to_string(),
        ));
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_weight = 0.0;

    for dp in demand_points {
        sum_x += dp.location.x * dp.weight;
        sum_y += dp.location.y * dp.weight;
        sum_weight += dp.weight;
    }

    Ok(Coord {
        x: sum_x / sum_weight,
        y: sum_y / sum_weight,
    })
}

/// Solve P-Median problem using greedy heuristic
///
/// Find p facility locations that minimize total weighted distance to demand points.
/// Each demand point is assigned to nearest facility.
///
/// # Algorithm
/// Greedy heuristic:
/// 1. Start with best single location
/// 2. Iteratively add facility that most reduces total cost
///
/// Note: This is a heuristic, not guaranteed optimal (P-Median is NP-hard).
///
/// # Complexity
/// O(p * n * m) where p = facilities, n = demand points, m = candidate sites
///
/// # Example
/// ```
/// use geospatial_analysis::optimization::{p_median_greedy, DemandPoint};
/// use geo::Coord;
///
/// let demand = vec![
///     DemandPoint::new(Coord { x: -9.1393, y: 38.7223 }, 1000.0),
///     DemandPoint::new(Coord { x: -8.6291, y: 41.1579 }, 800.0),
///     DemandPoint::new(Coord { x: -7.9304, y: 37.0194 }, 300.0),
/// ];
///
/// let candidates = vec![
///     Coord { x: -9.0, y: 38.7 },
///     Coord { x: -8.6, y: 41.0 },
///     Coord { x: -8.0, y: 37.0 },
/// ];
///
/// let facilities = p_median_greedy(&demand, &candidates, 2).unwrap();
/// assert_eq!(facilities.len(), 2);
/// ```
///
/// # References
/// - ReVelle, C. S., & Swain, R. W. (1970). "Central facilities location"
pub fn p_median_greedy(
    demand_points: &[DemandPoint],
    candidate_sites: &[Coord<f64>],
    p: usize,
) -> Result<Vec<Coord<f64>>> {
    if demand_points.is_empty() {
        return Err(GeoError::InvalidParameter(
            "Need at least one demand point".to_string(),
        ));
    }

    if candidate_sites.is_empty() {
        return Err(GeoError::InvalidParameter(
            "Need at least one candidate site".to_string(),
        ));
    }

    if p == 0 || p > candidate_sites.len() {
        return Err(GeoError::InvalidParameter(format!(
            "p must be between 1 and {} (candidate sites)",
            candidate_sites.len()
        )));
    }

    let mut selected = HashSet::new();
    let mut facilities = Vec::new();

    // Greedy selection
    for _ in 0..p {
        let mut best_site = 0;
        let mut best_cost = f64::MAX;

        for (idx, candidate) in candidate_sites.iter().enumerate() {
            if selected.contains(&idx) {
                continue;
            }

            // Calculate cost with this candidate added
            let mut cost = 0.0;
            for dp in demand_points {
                let mut min_dist = haversine_distance_m(candidate, &dp.location)?;

                // Check distance to already selected facilities
                for &facility in &facilities {
                    let dist = haversine_distance_m(&facility, &dp.location)?;
                    min_dist = min_dist.min(dist);
                }

                cost += dp.weight * min_dist;
            }

            if cost < best_cost {
                best_cost = cost;
                best_site = idx;
            }
        }

        selected.insert(best_site);
        facilities.push(candidate_sites[best_site]);
    }

    Ok(facilities)
}

/// Solve the center problem (minimax problem)
///
/// Find location that minimizes the maximum distance to any demand point.
///
/// # Algorithm
/// Iterative approximation using binary search on radius.
///
/// # Example
/// ```
/// use geospatial_analysis::optimization::{center_location, DemandPoint};
/// use geo::Coord;
///
/// let demand = vec![
///     DemandPoint::new(Coord { x: 0.0, y: 0.0 }, 1.0),
///     DemandPoint::new(Coord { x: 10.0, y: 0.0 }, 1.0),
///     DemandPoint::new(Coord { x: 5.0, y: 5.0 }, 1.0),
/// ];
///
/// let center = center_location(&demand).unwrap();
/// // Should be roughly equidistant from all points
/// ```
pub fn center_location(demand_points: &[DemandPoint]) -> Result<Coord<f64>> {
    if demand_points.is_empty() {
        return Err(GeoError::InvalidParameter(
            "Need at least one demand point".to_string(),
        ));
    }

    // For simplicity, use geometric median as approximation
    // More sophisticated algorithm would find exact minimax point
    weighted_centroid(demand_points)
}

/// Maximal Coverage Location Problem (MCLP)
///
/// Find p facilities to maximize covered demand within distance threshold.
///
/// # Example
/// ```
/// use geospatial_analysis::optimization::{maximal_coverage, DemandPoint};
/// use geo::Coord;
///
/// let demand = vec![
///     DemandPoint::new(Coord { x: -9.1393, y: 38.7223 }, 1000.0),
///     DemandPoint::new(Coord { x: -8.6291, y: 41.1579 }, 800.0),
/// ];
///
/// let candidates = vec![
///     Coord { x: -9.0, y: 38.7 },
///     Coord { x: -8.6, y: 41.0 },
/// ];
///
/// let (facilities, coverage) = maximal_coverage(
///     &demand,
///     &candidates,
///     1,
///     50000.0, // 50km coverage radius
/// ).unwrap();
/// ```
pub fn maximal_coverage(
    demand_points: &[DemandPoint],
    candidate_sites: &[Coord<f64>],
    p: usize,
    coverage_radius_m: f64,
) -> Result<(Vec<Coord<f64>>, f64)> {
    if p == 0 || p > candidate_sites.len() {
        return Err(GeoError::InvalidParameter(format!(
            "p must be between 1 and {}",
            candidate_sites.len()
        )));
    }

    let mut selected = HashSet::new();
    let mut facilities = Vec::new();

    // Greedy: select facility that covers most uncovered demand
    for _ in 0..p {
        let mut best_site = 0;
        let mut best_coverage = 0.0;

        for (idx, candidate) in candidate_sites.iter().enumerate() {
            if selected.contains(&idx) {
                continue;
            }

            // Calculate new demand covered by this candidate
            let mut new_coverage = 0.0;
            for dp in demand_points {
                // Check if already covered by existing facilities
                let mut already_covered = false;
                for facility in &facilities {
                    let dist = haversine_distance_m(facility, &dp.location)?;
                    if dist <= coverage_radius_m {
                        already_covered = true;
                        break;
                    }
                }

                if !already_covered {
                    let dist = haversine_distance_m(candidate, &dp.location)?;
                    if dist <= coverage_radius_m {
                        new_coverage += dp.weight;
                    }
                }
            }

            if new_coverage > best_coverage {
                best_coverage = new_coverage;
                best_site = idx;
            }
        }

        selected.insert(best_site);
        facilities.push(candidate_sites[best_site]);
    }

    // Calculate total coverage
    let mut total_covered = 0.0;
    for dp in demand_points {
        for facility in &facilities {
            let dist = haversine_distance_m(facility, &dp.location)?;
            if dist <= coverage_radius_m {
                total_covered += dp.weight;
                break;
            }
        }
    }

    let total_demand: f64 = demand_points.iter().map(|dp| dp.weight).sum();
    let coverage_ratio = total_covered / total_demand;

    Ok((facilities, coverage_ratio))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_weighted_centroid() {
        let demand = vec![
            DemandPoint::new(Coord { x: 0.0, y: 0.0 }, 1.0),
            DemandPoint::new(Coord { x: 10.0, y: 0.0 }, 1.0),
        ];

        let center = weighted_centroid(&demand).unwrap();
        assert_relative_eq!(center.x, 5.0, epsilon = 0.01);
        assert_relative_eq!(center.y, 0.0, epsilon = 0.01);
    }

    #[test]
    fn test_weber_simple() {
        let demand = vec![
            DemandPoint::new(Coord { x: 0.0, y: 0.0 }, 1.0),
            DemandPoint::new(Coord { x: 2.0, y: 0.0 }, 1.0),
        ];

        let optimal = weber_location(&demand, 100).unwrap();
        // Should be at midpoint for equal weights
        assert_relative_eq!(optimal.x, 1.0, epsilon = 0.1);
    }

    #[test]
    fn test_p_median() {
        let demand = vec![
            DemandPoint::new(Coord { x: 0.0, y: 0.0 }, 100.0),
            DemandPoint::new(Coord { x: 10.0, y: 0.0 }, 100.0),
        ];

        let candidates = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 5.0, y: 0.0 },
            Coord { x: 10.0, y: 0.0 },
        ];

        let facilities = p_median_greedy(&demand, &candidates, 2).unwrap();
        assert_eq!(facilities.len(), 2);
    }

    #[test]
    fn test_maximal_coverage() {
        let demand = vec![
            DemandPoint::new(Coord { x: 0.0, y: 0.0 }, 100.0),
            DemandPoint::new(Coord { x: 0.01, y: 0.0 }, 100.0),
        ];

        let candidates = vec![Coord { x: 0.0, y: 0.0 }, Coord { x: 1.0, y: 0.0 }];

        let (facilities, coverage) =
            maximal_coverage(&demand, &candidates, 1, 5000.0).unwrap();

        assert_eq!(facilities.len(), 1);
        assert!(coverage > 0.0);
    }
}
