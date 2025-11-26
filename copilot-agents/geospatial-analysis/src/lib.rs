//! # Geospatial Analysis Engine
//!
//! High-performance geospatial analysis and location intelligence for Avelan Platform.
//!
//! ## Features
//!
//! - **Coordinate Systems**: WGS84, Web Mercator, UTM transformations
//! - **Spatial Indexing**: R-Tree, QuadTree, KD-Tree implementations
//! - **Distance Calculations**: Haversine, Vincenty, Euclidean
//! - **Spatial Operations**: Point-in-polygon, intersections, buffers
//! - **Location Optimization**: Weber problem, P-Median, MCLP
//! - **Network Analysis**: Shortest path, isochrones, centrality
//! - **Terrain Analysis**: Slope, aspect, viewshed, hillshade
//! - **Clustering**: DBSCAN, KDE, hot spot analysis
//!
//! ## Example
//!
//! ```rust
//! use geospatial_analysis::distance::haversine_distance;
//! use geo::Coord;
//!
//! let lisbon = Coord { x: -9.1393, y: 38.7223 };
//! let porto = Coord { x: -8.6291, y: 41.1579 };
//!
//! let distance_km = haversine_distance(&lisbon, &porto);
//! assert!((distance_km - 274.0).abs() < 1.0); // ~274 km
//! ```

pub mod coords;
pub mod distance;
pub mod error;
pub mod indexing;
pub mod optimization;
pub mod spatial;
pub mod network;
pub mod terrain;
pub mod clustering;

pub use error::{GeoError, Result};

// Re-export commonly used types
pub use geo::{Coord, Point, LineString, Polygon, MultiPolygon};
pub use geo_types::{Geometry, GeometryCollection};

/// Earth radius in kilometers (mean radius)
pub const EARTH_RADIUS_KM: f64 = 6371.0;

/// Earth radius in meters
pub const EARTH_RADIUS_M: f64 = 6371000.0;

/// Semi-major axis of WGS84 ellipsoid (meters)
pub const WGS84_A: f64 = 6378137.0;

/// Semi-minor axis of WGS84 ellipsoid (meters)
pub const WGS84_B: f64 = 6356752.314245;

/// Flattening of WGS84 ellipsoid
pub const WGS84_F: f64 = 1.0 / 298.257223563;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert!(EARTH_RADIUS_KM > 0.0);
        assert!(WGS84_A > WGS84_B);
        assert!(WGS84_F > 0.0 && WGS84_F < 1.0);
    }
}
