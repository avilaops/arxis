//! Advanced GIS Features - 30 Funcionalidades Nivel Esri ArcGIS
//!
//! Este módulo implementa funcionalidades avançadas de GIS comparáveis
//! às oferecidas pelo Esri ArcGIS, totalmente em Rust nativo.
//!
//! ## 30 Funcionalidades Implementadas
//!
//! ### Análise Espacial (1-10)
//! 1. **Buffer Analysis** - Zonas de influência
//! 2. **Overlay Analysis** - Intersect, Union, Difference, Clip
//! 3. **Proximity Analysis** - Near, Point Distance
//! 4. **Density Analysis** - Kernel Density, Point Density
//! 5. **Hot Spot Analysis** - Getis-Ord Gi*, Moran's I
//! 6. **Cluster Analysis** - DBSCAN, K-means espacial
//! 7. **Interpolation** - IDW, Kriging, Spline, Natural Neighbor
//! 8. **Surface Analysis** - Slope, Aspect, Hillshade, Viewshed
//! 9. **Hydrology** - Flow Direction, Flow Accumulation, Watersheds
//! 10. **Visibility Analysis** - Viewshed, Line of Sight
//!
//! ### Network Analysis (11-15)
//! 11. **Shortest Path** - Dijkstra, A*, Bidirectional
//! 12. **Service Area** - Drive-time polygons, Isochrones
//! 13. **Closest Facility** - Nearest hospital, fire station
//! 14. **OD Cost Matrix** - Origin-Destination matrix
//! 15. **Vehicle Routing** - TSP, VRP with constraints
//!
//! ### Raster Analysis (16-20)
//! 16. **Raster Calculator** - Map algebra operations
//! 17. **Zonal Statistics** - Statistics per zone
//! 18. **Focal Statistics** - Moving window operations
//! 19. **Raster Reclassification** - Value remapping
//! 20. **Cost Distance** - Weighted distance analysis
//!
//! ### Geocoding & Routing (21-25)
//! 21. **Geocoding** - Address to coordinates
//! 22. **Reverse Geocoding** - Coordinates to address
//! 23. **Batch Geocoding** - Multiple addresses
//! 24. **Turn-by-Turn Navigation** - Driving directions
//! 25. **Multi-Modal Routing** - Walk, bike, transit
//!
//! ### 3D & Temporal (26-30)
//! 26. **3D Terrain Generation** - DEM, TIN, mesh
//! 27. **3D Viewshed** - 3D visibility analysis
//! 28. **Space-Time Cubes** - Temporal patterns
//! 29. **Temporal Aggregation** - Time-series analysis
//! 30. **Emerging Hot Spots** - Mann-Kendall trend test

pub mod buffer;
pub mod overlay;
pub mod proximity;
pub mod density;
pub mod hotspot;
pub mod clustering_spatial;
pub mod interpolation;
pub mod surface;
pub mod hydrology;
pub mod visibility;
pub mod network;
pub mod service_area;
pub mod closest_facility;
pub mod od_matrix;
pub mod vehicle_routing;
pub mod raster_calc;
pub mod zonal_stats;
pub mod focal_stats;
pub mod reclassify;
pub mod cost_distance;
pub mod geocoding;
pub mod reverse_geocoding;
pub mod batch_geocoding;
pub mod turn_by_turn;
pub mod multimodal;
pub mod terrain_3d;
pub mod viewshed_3d;
pub mod space_time;
pub mod temporal_agg;
pub mod emerging_hotspots;

use crate::coords::{GeoCoord, GeoBounds};
use crate::geometry::{GeoPoint, GeoLine, GeoPolygon};

/// Re-exports for convenience
pub mod prelude {
    pub use super::buffer::*;
    pub use super::overlay::*;
    pub use super::proximity::*;
    pub use super::density::*;
    pub use super::hotspot::*;
    pub use super::clustering_spatial::*;
    pub use super::interpolation::*;
    pub use super::surface::*;
    pub use super::hydrology::*;
    pub use super::visibility::*;
    pub use super::network::*;
    pub use super::geocoding::*;
    pub use super::raster_calc::*;
    pub use super::terrain_3d::*;
}
