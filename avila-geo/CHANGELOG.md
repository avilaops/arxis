# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Topology Operations** (`src/topology.rs`)
  - Buffer operations (point, line, polygon) with configurable segments
  - Convex hull computation (Graham scan algorithm)
  - Bounding box and centroid calculations
  - Line intersection detection
  - Polygon clipping (Sutherland-Hodgman algorithm)
  - Minimum bounding circle (Welzl's algorithm)
- **AvilaDB Geospatial Integration** (`src/aviladb.rs`)
  - `GeoDocument` for geospatial data storage
  - `GeoQuery` builder for spatial queries
  - Query types: radius, bounding box, polygon contains, k-nearest neighbors
  - Property filtering with multiple operators
  - Grid-based spatial index (`GeoIndex`) for efficient queries
  - GeoJSON import/export for documents
  - Full integration with AvilaDB best practices
- **Caching System** (`src/cache.rs`)
  - Generic LRU cache with O(1) operations
  - Thread-safe `ConcurrentCache` wrapper
  - Specialized caches:
    - `TileCache` for tile data
    - `ProjectionCache` for coordinate transformations
    - `DistanceCache` for distance calculations
- **SIMD Vectorization** (`src/simd.rs`)
  - Portable SIMD for 4-8x speedup on batch operations
  - Vectorized Equirectangular projection
  - Vectorized Haversine distance calculations
  - Automatic fallback to scalar operations
- **Advanced Projections** (`src/projections_ext.rs`)
  - Robinson projection (National Geographic 1988-1998)
  - Winkel Tripel (National Geographic since 1998)
  - Mollweide (equal-area, elliptical)
  - UTM (Universal Transverse Mercator, 60 zones)
  - Stereographic (azimuthal, polar regions)
- **PNG/JPEG Export** (`src/image_export.rs`)
  - Export maps to PNG format (lossless)
  - Export maps to JPEG format with quality control
  - In-memory byte export for web servers
  - Extension traits for easy usage
- **Tile System** (`src/tiles.rs`)
  - Complete XYZ, TMS, and QuadKey tile coordinate systems
  - Tile URL templates for OSM, Google Maps, Mapbox
  - Optimal zoom calculation and tile range queries
  - Parent/children tile hierarchy navigation
- **Parallel Processing** (`src/parallel.rs`)
  - Rayon-based parallel projection (4-16x speedup)
  - Parallel rendering of geometry collections
  - Parallel distance calculations and simplification
- **Spatial Indexing** (`src/spatial.rs`)
  - R-Tree spatial index for O(log n) queries
  - Nearest neighbor and k-nearest neighbor searches
  - Range queries (within distance/bounds)
  - 10-100x speedup over brute force
- **SVG Export** (`src/export.rs`)
  - Vector-based map rendering
  - Scalable, resolution-independent output
  - Editable in Inkscape, Illustrator
- **New Geographic Regions**:
  - `GeoBounds::MIDDLE_EAST` - Middle East region bounds
  - `GeoBounds::DUBAI` - Dubai/UAE specific bounds
  - `GeoBounds::GULF_REGION` - Arabian/Persian Gulf region
  - `GeoBounds::EUROPE` - European region
- **Map Builders**:
  - `MapBuilder::middle_east()` - Quick Middle East map creation
  - `MapBuilder::dubai()` - Quick Dubai/UAE map creation
  - `MapBuilder::gulf_region()` - Quick Gulf region map creation
  - `MapBuilder::europe()` - Quick Europe map creation
- **Examples**:
  - `topology_operations.rs` - Buffer, convex hull, clipping, intersection
  - `aviladb_integration.rs` - Complete AvilaDB geospatial usage
  - `advanced_projections.rs` - Robinson, Winkel Tripel, Mollweide, UTM, Stereographic
  - `simd_performance.rs` - SIMD performance benchmarks
  - `export_formats.rs` - SVG, PNG, JPEG export demonstration
  - `europe_map.rs` - Europe with countries and capitals
  - `dubai_gulf_map.rs` - Dubai and Gulf region with landmarks
  - `tiles_example.rs` - Complete tile system demonstration
  - `spatial_index.rs` - Spatial queries with R-tree
- **Feature Flags**: `parallel`, `simd`, `spatial-index`, `export-svg`, `export-png`, `full`
- Comprehensive performance documentation in `ENHANCEMENTS.md`

### Changed
- Reorganized dependencies with optional features for better control
- Updated Cargo.toml with granular feature flags
- Enhanced prelude with all new modules (simd, projections_ext, image_export)
- Better separation of core vs. optional features

### Performance
- **12.5x** faster point projection with parallel processing
- **5.3x** faster multi-layer rendering with Rayon
- **100x** faster nearest neighbor queries with R-tree spatial index
- **4-8x** faster batch operations with SIMD (Equirectangular, distances)

## [0.1.0] - 2025-11-25

### Added
- **Coordinate Systems**: `GeoCoord`, `CartesianCoord`, `GeoBounds`
- **Map Projections**:
  - Equirectangular (Plate Carrée)
  - Mercator (conformal)
  - Web Mercator (EPSG:3857) with tile support
  - Albers Equal Area Conic
  - Lambert Conformal Conic
- **Rendering Algorithms**:
  - Bresenham's line drawing
  - Scanline polygon fill
  - Xiaolin Wu anti-aliased lines
  - Midpoint circle algorithm
- **Geometries**: `GeoPoint`, `GeoLine`, `GeoPolygon`, `GeoCollection`
- **Geographic Calculations**:
  - Haversine distance (spherical Earth)
  - Vincenty distance (ellipsoidal Earth, WGS84)
  - Bearing and destination calculations
  - Area calculations (Shoelace formula, Spherical excess)
  - Point-in-polygon test (ray casting)
  - Douglas-Peucker line simplification
  - Midpoint and interpolation
  - Cross-track distance
- **GeoJSON Support**: Parse and serialize GeoJSON (RFC 7946)
- **Map Rendering System**:
  - Multi-layer maps with custom styles
  - Predefined styles (land, water, borders, cities)
  - Export to PPM format
- **Examples**:
  - World map with continents and cities
  - Brazil map with states and cities
  - Custom projections comparison
- **Benchmarks**: Performance tests for projections and rendering
- **Tests**: Comprehensive unit and integration tests

### Documentation
- Complete API documentation with examples
- Detailed README with quick start guide
- Examples demonstrating all major features
- Inline code documentation with mathematical formulas

### Performance
- Zero-allocation coordinate conversions
- Fast integer-based line drawing
- Efficient scanline polygon fill
- Optimized projection calculations

[0.1.0]: https://github.com/avila-cloud/arxis/releases/tag/avila-geo-v0.1.0
