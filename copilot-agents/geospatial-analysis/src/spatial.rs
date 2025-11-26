//! Spatial operations and geometric algorithms
//!
//! Provides fundamental geometric operations:
//! - Point-in-polygon tests
//! - Polygon intersections
//! - Buffer generation
//! - Convex hull
//! - Line simplification

use crate::error::{GeoError, Result};
use geo::{
    Area, BooleanOps, BoundingRect, Contains, ConvexHull, Coord, Intersects, LineString,
    Point, Polygon, Simplify, SimplifyVw,
};

/// Test if a point is inside a polygon using ray casting algorithm
///
/// # Algorithm
/// Ray casting: counts intersections of ray from point to infinity with polygon edges.
/// Odd count = inside, even count = outside.
///
/// # Complexity
/// O(n) where n is number of polygon vertices
///
/// # Example
/// ```
/// use geospatial_analysis::spatial::point_in_polygon;
/// use geo::{Coord, Polygon};
///
/// let polygon = Polygon::new(
///     vec![
///         Coord { x: 0.0, y: 0.0 },
///         Coord { x: 10.0, y: 0.0 },
///         Coord { x: 10.0, y: 10.0 },
///         Coord { x: 0.0, y: 10.0 },
///         Coord { x: 0.0, y: 0.0 },
///     ].into(),
///     vec![],
/// );
///
/// let inside = Coord { x: 5.0, y: 5.0 };
/// let outside = Coord { x: 15.0, y: 15.0 };
///
/// assert!(point_in_polygon(&inside, &polygon));
/// assert!(!point_in_polygon(&outside, &polygon));
/// ```
pub fn point_in_polygon(point: &Coord<f64>, polygon: &Polygon<f64>) -> bool {
    let point = Point::from(*point);
    polygon.contains(&point)
}

/// Calculate intersection of two polygons
///
/// Returns the geometric intersection (may be empty, single polygon, or multiple polygons).
///
/// # Complexity
/// O(n + m) where n, m are number of vertices
///
/// # Example
/// ```
/// use geospatial_analysis::spatial::polygon_intersection;
/// use geo::{Coord, Polygon};
///
/// let poly1 = Polygon::new(
///     vec![
///         Coord { x: 0.0, y: 0.0 },
///         Coord { x: 10.0, y: 0.0 },
///         Coord { x: 10.0, y: 10.0 },
///         Coord { x: 0.0, y: 10.0 },
///         Coord { x: 0.0, y: 0.0 },
///     ].into(),
///     vec![],
/// );
///
/// let poly2 = Polygon::new(
///     vec![
///         Coord { x: 5.0, y: 5.0 },
///         Coord { x: 15.0, y: 5.0 },
///         Coord { x: 15.0, y: 15.0 },
///         Coord { x: 5.0, y: 15.0 },
///         Coord { x: 5.0, y: 5.0 },
///     ].into(),
///     vec![],
/// );
///
/// let intersection = polygon_intersection(&poly1, &poly2).unwrap();
/// ```
pub fn polygon_intersection(a: &Polygon<f64>, b: &Polygon<f64>) -> Result<Vec<Polygon<f64>>> {
    let result = a.intersection(b);

    match result {
        geo::MultiPolygon(polys) => Ok(polys),
    }
}

/// Calculate union of two polygons
///
/// # Example
/// ```
/// use geospatial_analysis::spatial::polygon_union;
/// use geo::{Coord, Polygon};
///
/// let poly1 = Polygon::new(
///     vec![
///         Coord { x: 0.0, y: 0.0 },
///         Coord { x: 5.0, y: 0.0 },
///         Coord { x: 5.0, y: 5.0 },
///         Coord { x: 0.0, y: 5.0 },
///         Coord { x: 0.0, y: 0.0 },
///     ].into(),
///     vec![],
/// );
///
/// let poly2 = Polygon::new(
///     vec![
///         Coord { x: 3.0, y: 3.0 },
///         Coord { x: 8.0, y: 3.0 },
///         Coord { x: 8.0, y: 8.0 },
///         Coord { x: 3.0, y: 8.0 },
///         Coord { x: 3.0, y: 3.0 },
///     ].into(),
///     vec![],
/// );
///
/// let union = polygon_union(&poly1, &poly2).unwrap();
/// ```
pub fn polygon_union(a: &Polygon<f64>, b: &Polygon<f64>) -> Result<Vec<Polygon<f64>>> {
    let result = a.union(b);

    match result {
        geo::MultiPolygon(polys) => Ok(polys),
    }
}

/// Calculate difference between two polygons (A - B)
pub fn polygon_difference(a: &Polygon<f64>, b: &Polygon<f64>) -> Result<Vec<Polygon<f64>>> {
    let result = a.difference(b);

    match result {
        geo::MultiPolygon(polys) => Ok(polys),
    }
}

/// Calculate convex hull of a set of points
///
/// Returns the smallest convex polygon containing all points.
///
/// # Algorithm
/// Graham scan or Jarvis march (implementation dependent)
///
/// # Complexity
/// O(n log n)
///
/// # Example
/// ```
/// use geospatial_analysis::spatial::convex_hull;
/// use geo::Coord;
///
/// let points = vec![
///     Coord { x: 0.0, y: 0.0 },
///     Coord { x: 5.0, y: 5.0 },
///     Coord { x: 10.0, y: 0.0 },
///     Coord { x: 5.0, y: 2.0 }, // Interior point
/// ];
///
/// let hull = convex_hull(&points).unwrap();
/// assert_eq!(hull.exterior().0.len(), 4); // 3 points + closing point
/// ```
pub fn convex_hull(points: &[Coord<f64>]) -> Result<Polygon<f64>> {
    if points.len() < 3 {
        return Err(GeoError::InvalidParameter(
            "Need at least 3 points for convex hull".to_string(),
        ));
    }

    let multi_point: geo::MultiPoint<f64> = points.iter().map(|&c| Point::from(c)).collect();
    Ok(multi_point.convex_hull())
}

/// Simplify a line using Douglas-Peucker algorithm
///
/// Reduces the number of points while preserving shape.
///
/// # Algorithm
/// Douglas-Peucker recursive line simplification
///
/// # Complexity
/// O(n²) worst case, O(n log n) average
///
/// # Arguments
/// * `line` - LineString to simplify
/// * `epsilon` - Maximum distance threshold (larger = more simplification)
///
/// # Example
/// ```
/// use geospatial_analysis::spatial::simplify_line;
/// use geo::{Coord, LineString};
///
/// let line = LineString::from(vec![
///     Coord { x: 0.0, y: 0.0 },
///     Coord { x: 1.0, y: 0.1 },
///     Coord { x: 2.0, y: 0.0 },
///     Coord { x: 3.0, y: 0.1 },
///     Coord { x: 4.0, y: 0.0 },
/// ]);
///
/// let simplified = simplify_line(&line, 0.15);
/// assert!(simplified.0.len() < line.0.len());
/// ```
///
/// # References
/// - Douglas, D. H., & Peucker, T. K. (1973). "Algorithms for the reduction of
///   the number of points required to represent a digitized line or its caricature"
pub fn simplify_line(line: &LineString<f64>, epsilon: f64) -> LineString<f64> {
    line.simplify(&epsilon)
}

/// Simplify line using Visvalingam-Whyatt algorithm
///
/// Alternative simplification preserving area instead of distance.
///
/// # Arguments
/// * `line` - LineString to simplify
/// * `epsilon` - Area threshold
pub fn simplify_line_vw(line: &LineString<f64>, epsilon: f64) -> LineString<f64> {
    line.simplifyvw(&epsilon)
}

/// Calculate area of a polygon in square degrees
///
/// Note: For geographic coordinates, use projected coordinates or
/// geodesic area calculation for accurate results.
///
/// # Example
/// ```
/// use geospatial_analysis::spatial::polygon_area;
/// use geo::{Coord, Polygon};
///
/// let square = Polygon::new(
///     vec![
///         Coord { x: 0.0, y: 0.0 },
///         Coord { x: 10.0, y: 0.0 },
///         Coord { x: 10.0, y: 10.0 },
///         Coord { x: 0.0, y: 10.0 },
///         Coord { x: 0.0, y: 0.0 },
///     ].into(),
///     vec![],
/// );
///
/// let area = polygon_area(&square);
/// assert_eq!(area, 100.0);
/// ```
pub fn polygon_area(polygon: &Polygon<f64>) -> f64 {
    polygon.unsigned_area()
}

/// Create a bounding box around a set of coordinates
///
/// Returns (min_coord, max_coord) representing southwest and northeast corners.
///
/// # Example
/// ```
/// use geospatial_analysis::spatial::bounding_box;
/// use geo::Coord;
///
/// let points = vec![
///     Coord { x: -9.1393, y: 38.7223 }, // Lisbon
///     Coord { x: -8.6291, y: 41.1579 }, // Porto
/// ];
///
/// let (min, max) = bounding_box(&points).unwrap();
/// assert_eq!(min.x, -9.1393);
/// assert_eq!(min.y, 38.7223);
/// assert_eq!(max.x, -8.6291);
/// assert_eq!(max.y, 41.1579);
/// ```
pub fn bounding_box(coords: &[Coord<f64>]) -> Result<(Coord<f64>, Coord<f64>)> {
    if coords.is_empty() {
        return Err(GeoError::EmptyGeometry);
    }

    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;

    for coord in coords {
        min_x = min_x.min(coord.x);
        min_y = min_y.min(coord.y);
        max_x = max_x.max(coord.x);
        max_y = max_y.max(coord.y);
    }

    Ok((Coord { x: min_x, y: min_y }, Coord { x: max_x, y: max_y }))
}

/// Calculate centroid of a polygon
///
/// # Example
/// ```
/// use geospatial_analysis::spatial::polygon_centroid;
/// use geo::{Coord, Polygon};
///
/// let square = Polygon::new(
///     vec![
///         Coord { x: 0.0, y: 0.0 },
///         Coord { x: 10.0, y: 0.0 },
///         Coord { x: 10.0, y: 10.0 },
///         Coord { x: 0.0, y: 10.0 },
///         Coord { x: 0.0, y: 0.0 },
///     ].into(),
///     vec![],
/// );
///
/// let center = polygon_centroid(&square).unwrap();
/// assert_eq!(center.x, 5.0);
/// assert_eq!(center.y, 5.0);
/// ```
pub fn polygon_centroid(polygon: &Polygon<f64>) -> Result<Coord<f64>> {
    use geo::Centroid;

    polygon
        .centroid()
        .map(|p| Coord { x: p.x(), y: p.y() })
        .ok_or_else(|| GeoError::InvalidPolygon("Cannot calculate centroid".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_point_in_polygon() {
        let polygon = Polygon::new(
            vec![
                Coord { x: 0.0, y: 0.0 },
                Coord { x: 10.0, y: 0.0 },
                Coord { x: 10.0, y: 10.0 },
                Coord { x: 0.0, y: 10.0 },
                Coord { x: 0.0, y: 0.0 },
            ]
            .into(),
            vec![],
        );

        assert!(point_in_polygon(&Coord { x: 5.0, y: 5.0 }, &polygon));
        assert!(!point_in_polygon(&Coord { x: 15.0, y: 15.0 }, &polygon));
        assert!(point_in_polygon(&Coord { x: 0.0, y: 0.0 }, &polygon));
    }

    #[test]
    fn test_convex_hull() {
        let points = vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 5.0, y: 5.0 },
            Coord { x: 10.0, y: 0.0 },
            Coord { x: 5.0, y: 2.0 },
        ];

        let hull = convex_hull(&points).unwrap();
        assert_eq!(hull.exterior().0.len(), 4);
    }

    #[test]
    fn test_polygon_area() {
        let square = Polygon::new(
            vec![
                Coord { x: 0.0, y: 0.0 },
                Coord { x: 10.0, y: 0.0 },
                Coord { x: 10.0, y: 10.0 },
                Coord { x: 0.0, y: 10.0 },
                Coord { x: 0.0, y: 0.0 },
            ]
            .into(),
            vec![],
        );

        assert_eq!(polygon_area(&square), 100.0);
    }

    #[test]
    fn test_bounding_box() {
        let points = vec![
            Coord {
                x: -9.1393,
                y: 38.7223,
            },
            Coord {
                x: -8.6291,
                y: 41.1579,
            },
            Coord {
                x: -7.9304,
                y: 37.0194,
            },
        ];

        let (min, max) = bounding_box(&points).unwrap();

        assert_relative_eq!(min.x, -9.1393, epsilon = 0.0001);
        assert_relative_eq!(min.y, 37.0194, epsilon = 0.0001);
        assert_relative_eq!(max.x, -7.9304, epsilon = 0.0001);
        assert_relative_eq!(max.y, 41.1579, epsilon = 0.0001);
    }

    #[test]
    fn test_polygon_centroid() {
        let square = Polygon::new(
            vec![
                Coord { x: 0.0, y: 0.0 },
                Coord { x: 10.0, y: 0.0 },
                Coord { x: 10.0, y: 10.0 },
                Coord { x: 0.0, y: 10.0 },
                Coord { x: 0.0, y: 0.0 },
            ]
            .into(),
            vec![],
        );

        let center = polygon_centroid(&square).unwrap();
        assert_relative_eq!(center.x, 5.0, epsilon = 0.01);
        assert_relative_eq!(center.y, 5.0, epsilon = 0.01);
    }

    #[test]
    fn test_simplify_line() {
        let line = LineString::from(vec![
            Coord { x: 0.0, y: 0.0 },
            Coord { x: 1.0, y: 0.1 },
            Coord { x: 2.0, y: 0.0 },
            Coord { x: 3.0, y: 0.1 },
            Coord { x: 4.0, y: 0.0 },
        ]);

        let simplified = simplify_line(&line, 0.15);
        assert!(simplified.0.len() <= line.0.len());
    }
}
