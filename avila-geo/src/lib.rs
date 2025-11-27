//! # avila-geo
//!
//! Geographic cartography and mapping library with zero external dependencies
//! (except optional serde/serde_json for GeoJSON support).
//!
//! ## Features
//!
//! - **Coordinate Systems**: Geographic (lat/lon) and Cartesian (x/y) coordinates
//! - **Map Projections**: Equirectangular, Mercator, Web Mercator, Albers, Lambert
//! - **Geometries**: Points, Lines, Polygons with properties
//! - **Rendering**: Bresenham lines, polygon fill, anti-aliasing
//! - **Calculations**: Haversine distance, bearing, areas, point-in-polygon
//! - **GeoJSON**: Parse and serialize GeoJSON format
//! - **Map Rendering**: Multi-layer maps with styles
//!
//! ## Quick Start
//!
//! ```rust
//! use avila_geo::{
//!     coords::GeoCoord,
//!     geometry::{GeoPoint, GeoPolygon, shapes},
//!     projection::Mercator,
//!     map::{Map, Layer, Style},
//! };
//!
//! // Create some geographic data
//! let brazil = shapes::rectangle(-33.75, 5.27, -73.99, -28.84);
//! let sao_paulo = GeoPoint::with_name(
//!     GeoCoord::new(-23.55, -46.63),
//!     "São Paulo"
//! );
//!
//! // Create a map
//! let mut map = Map::new(800, 600);
//! // Add layers and render...
//! ```
//!
//! ## Examples
//!
//! See the `examples/` directory for complete examples:
//! - `world_map.rs` - Render a world map
//! - `brazil_map.rs` - Brazil with cities
//! - `custom_projection.rs` - Custom projections

pub mod calc;
pub mod coords;
pub mod geometry;
pub mod map;
pub mod projection;
pub mod projections_ext;
pub mod render;
pub mod tiles;
pub mod cache;
pub mod topology;
pub mod aviladb;
pub mod parallel;
pub mod export;
pub mod image_export;
pub mod spatial_native; // Native R-Tree implementation
pub mod advanced_gis_features; // 30 Advanced GIS features

#[cfg(feature = "simd")]
pub mod simd;

#[cfg(feature = "spatial-index")]
pub mod spatial;

#[cfg(feature = "geojson")]
pub mod geojson;

#[cfg(feature = "geoprocessing")]
pub mod geoprocessing;

// Re-exports for convenience
pub use coords::{CartesianCoord, GeoCoord, GeoBounds};
pub use geometry::{GeoCollection, GeoLine, GeoPoint, GeoPolygon, LineType};
pub use map::{Layer, Map, MapBuilder, Style};
pub use projection::{
    AlbersEqualArea, Equirectangular, LambertConformalConic, Mercator, Projection, WebMercator,
};
pub use projections_ext::{Mollweide, Robinson, Stereographic, UTM, WinkelTripel};
pub use render::{Color, Framebuffer};
pub use tiles::{TileCoord, TileSystem, TileUrlTemplate};
pub use parallel::*;
pub use cache::*;
pub use topology::*;
pub use aviladb::*;
pub use spatial_native::*;

#[cfg(feature = "export-svg")]
pub use export::{MapSvgExt, SvgExporter};

#[cfg(feature = "export-png")]
pub use image_export::{FramebufferImageExt, ImageExporter, MapImageExt};

#[cfg(feature = "simd")]
pub use simd::*;

#[cfg(feature = "spatial-index")]
pub use spatial::{SpatialIndex, SpatialQuery};

#[cfg(feature = "geojson")]
pub use geojson::GeoJson;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Prelude module for common imports
pub mod prelude {
    pub use crate::calc::*;
    pub use crate::coords::*;
    pub use crate::geometry::*;
    pub use crate::map::*;
    pub use crate::projection::*;
    pub use crate::projections_ext::*;
    pub use crate::render::*;
    pub use crate::tiles::*;
    pub use crate::parallel::*;
    pub use crate::cache::*;
    pub use crate::topology::*;
    pub use crate::aviladb::*;

    #[cfg(feature = "export-svg")]
    pub use crate::export::*;

    #[cfg(feature = "export-png")]
    pub use crate::image_export::*;

    #[cfg(feature = "simd")]
    pub use crate::simd::*;

    #[cfg(feature = "spatial-index")]
    pub use crate::spatial::*;

    #[cfg(feature = "geojson")]
    pub use crate::geojson::*;

    #[cfg(feature = "geoprocessing")]
    pub use crate::geoprocessing::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_workflow() {
        // Create coordinates
        let coord = GeoCoord::new(-23.55, -46.63);

        // Create point
        let point = GeoPoint::with_name(coord, "São Paulo");
        assert_eq!(point.name(), Some("São Paulo"));

        // Project to cartesian
        let proj = Equirectangular::new();
        let cart = proj.project(&coord, 360.0, 180.0);

        // Should be in valid range
        assert!(cart.x >= 0.0 && cart.x <= 360.0);
        assert!(cart.y >= 0.0 && cart.y <= 180.0);
    }

    #[test]
    fn test_distance_calculation() {
        let p1 = GeoCoord::new(-23.55, -46.63); // São Paulo
        let p2 = GeoCoord::new(-22.91, -43.17); // Rio de Janeiro

        let distance = calc::haversine_distance(&p1, &p2);

        // Distance is approximately 360 km
        assert!(distance > 300000.0 && distance < 400000.0);
    }

    #[test]
    fn test_map_rendering() {
        let mut map = Map::new(100, 100);
        let collection = GeoCollection::new();
        let layer = Layer::new("test", collection, Style::default());

        map.add_layer(layer);

        let proj = Equirectangular::new();
        let fb = map.render(&proj);

        assert_eq!(fb.width, 100);
        assert_eq!(fb.height, 100);
    }
}
