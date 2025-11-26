# avila-geo 🗺️

**Geographic cartography and mapping library implemented from scratch in Rust**

[![Crates.io](https://img.shields.io/crates/v/avila-geo.svg)](https://crates.io/crates/avila-geo)
[![Documentation](https://docs.rs/avila-geo/badge.svg)](https://docs.rs/avila-geo)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

Part of the [AVL Platform](https://avila.cloud) - Built for Brazilian developers 🇧🇷

## ✨ Features

### Core Cartography
- **🌍 Coordinate Systems**: Geographic (lat/lon), Cartesian (x/y), ECEF
- **📐 Map Projections**:
  - Equirectangular (Plate Carrée)
  - Mercator (conformal)
  - Web Mercator (EPSG:3857)
  - Albers Equal Area (preserves area)
  - Lambert Conformal Conic
  - Custom projections via trait
- **🗺️ Tile System**: XYZ, TMS, QuadKey for web mapping (OSM, Google, Mapbox compatible)
- **⚡ Parallel Processing**: Multi-threaded rendering with Rayon (4-16x speedup)
- **🔍 Spatial Indexing**: R-Tree for fast queries (100x faster nearest neighbor)
- **🎨 Rendering Algorithms**:
  - Bresenham's line drawing
  - Scanline polygon fill
  - Xiaolin Wu anti-aliased lines
  - Midpoint circle algorithm
- **📍 Geometries**: Points, Lines, Polygons with properties
- **🧮 Geographic Calculations**:
  - Haversine distance (spherical Earth)
  - Vincenty distance (ellipsoidal Earth)
  - Bearing and destination
  - Area calculations (Shoelace, Spherical excess)
  - Point-in-polygon tests
  - Douglas-Peucker simplification
- **📄 Import/Export**: GeoJSON (RFC 7946), SVG, PNG/JPEG
- **🗺️ Multi-Layer Maps**: Combine multiple layers with custom styles

### 🏭 Indústria 4.0 & Geoprocessing
- **🤖 Digital Twins**: Virtual representations of physical assets
- **📡 IoT Spatial**: Real-time monitoring of geo-located devices
- **🗺️ Geofencing**: Virtual boundaries with rule-based alerts
- **🔧 Predictive Maintenance**: ML-based failure prediction
- **🚚 Route Optimization**: Vehicle Routing Problem (VRP) solver
- **📊 Real-time Analytics**: Stream processing and anomaly detection
- **🚨 Alert System**: Rule-based notifications
- **🔥 Spatial Patterns**: Hotspot detection and clustering
- **🌐 Network Analysis**: Dijkstra, A*, centrality algorithms
- **⛰️ Terrain Analysis**: DEM, slope, aspect, viewshed, hillshade
- **🎯 Clustering**: K-Means, DBSCAN, Hierarchical
- **⚡ Cache System**: LRU cache for query optimization

## 🚀 Quick Start

```toml
[dependencies]
avila-geo = { version = "0.1", features = ["geoprocessing"] }
```

### Basic Cartography Example

```rust
use avila_geo::prelude::*;

fn main() {
    // Create a map of Brazil
    let mut map = Map::new(1200, 1000)
        .with_bounds(GeoBounds::BRAZIL)
        .with_background(Color::from_hex(0xADD8E6));

    // Add some cities
    let mut cities = GeoCollection::new();
    cities.add_point(GeoPoint::with_name(
        GeoCoord::new(-23.55, -46.63),
        "São Paulo"
    ));
    cities.add_point(GeoPoint::with_name(
        GeoCoord::new(-22.91, -43.17),
        "Rio de Janeiro"
    ));

    // Add layer with style
    map.add_layer(Layer::new(
        "cities",
        cities,
        Style::city()
    ));

    // Render with Mercator projection
    let projection = Mercator::new();
    map.save_ppm(&projection, "brazil.ppm").unwrap();
}
```

### Industry 4.0 Example

```rust
use avila_geo::geoprocessing::*;

fn main() {
    // Create fleet management system
    let mut fleet = FleetManager::new();

    // Register IoT device
    let truck = IoTDevice::new(
        "TRUCK001".to_string(),
        "Mercedes Actros".to_string(),
        "heavy_truck".to_string(),
        GeoCoord::new(-23.5505, -46.6333),
    );
    fleet.register_device(truck);

    // Add geofence
    let geofence = Geofence::new(
        "Zona SP".to_string(),
        sp_polygon,
        GeofenceRule::MustStayInside,
    );
    fleet.add_geofence(geofence);

    // Process sensor readings
    let reading = SensorReading::new(
        SensorType::Temperature,
        85.0,
        "°C".to_string(),
    );
    fleet.process_sensor_reading(&"TRUCK001".to_string(), reading);

    // Check violations and maintenance
    let violations = fleet.check_geofence_violations();
    let recommendations = fleet.predictive_maintenance_analysis();
}
```

## 📚 Core Concepts

### Coordinate Systems

```rust
// Geographic coordinates (latitude/longitude)
let sao_paulo = GeoCoord::new(-23.55, -46.63);

// Cartesian coordinates (pixels)
let pixel = CartesianCoord::new(100.0, 200.0);

// Convert to radians for calculations
let radians = sao_paulo.to_radians();
```

### Map Projections

```rust
use avila_geo::projection::*;

// Equirectangular (simple, fast)
let proj1 = Equirectangular::new();

// Mercator (conformal, preserves angles)
let proj2 = Mercator::new();

// Web Mercator (used by Google Maps, OSM)
let proj3 = WebMercator::new();

// Albers Equal Area (preserves area)
let proj4 = AlbersEqualArea::brazil();

// Project geographic to cartesian
let cart = proj1.project(&sao_paulo, 800.0, 600.0);
```

### Distance Calculations

```rust
use avila_geo::calc::*;

let sp = GeoCoord::new(-23.55, -46.63);
let rio = GeoCoord::new(-22.91, -43.17);

// Haversine distance (fast, spherical approximation)
let dist_km = haversine_distance(&sp, &rio) / 1000.0;
println!("Distance: {:.1} km", dist_km); // ~360 km

// Vincenty distance (accurate, uses ellipsoid)
let dist_accurate = vincenty_distance(&sp, &rio) / 1000.0;

// Bearing (direction)
let bearing_deg = bearing(&sp, &rio);
println!("Bearing: {:.1}°", bearing_deg);

// Midpoint
let mid = midpoint(&sp, &rio);
```

### Geometries

```rust
use avila_geo::geometry::*;

// Point
let point = GeoPoint::with_name(
    GeoCoord::new(-23.55, -46.63),
    "São Paulo"
).with_property("population", "12000000");

// Line (road, river, border)
let line = GeoLine::new(
    vec![
        GeoCoord::new(-23.55, -46.63),
        GeoCoord::new(-22.91, -43.17),
    ],
    LineType::Road
);

// Polygon (country, state, lake)
let polygon = GeoPolygon::new(vec![
    GeoCoord::new(-20.0, -44.0),
    GeoCoord::new(-25.0, -44.0),
    GeoCoord::new(-25.0, -53.0),
    GeoCoord::new(-20.0, -53.0),
]);

// Calculate area
let area_m2 = polygon.area_meters();
println!("Area: {:.0} km²", area_m2 / 1_000_000.0);

// Check if point is inside
let test = GeoCoord::new(-23.0, -48.0);
if polygon.contains(&test) {
    println!("Point is inside polygon!");
}
```

### Map Rendering

```rust
use avila_geo::{map::*, render::Color};

// Create map
let mut map = Map::new(1920, 1080)
    .with_bounds(GeoBounds::WORLD)
    .with_background(Color::from_hex(0xE0F7FA));

// Create collections
let mut land = GeoCollection::new();
let mut cities = GeoCollection::new();

// Add geometries...
land.add_polygon(/* ... */);
cities.add_point(/* ... */);

// Add layers with styles
map.add_layer(Layer::new(
    "land",
    land,
    Style::land() // Predefined style
));

map.add_layer(Layer::new(
    "cities",
    cities,
    Style::new()
        .with_fill(Color::RED)
        .with_stroke(Color::BLACK, 2)
));

// Render
let projection = Mercator::new();
let framebuffer = map.render(&projection);

// Save
map.save_ppm(&projection, "world_map.ppm").unwrap();
```

### GeoJSON Support

```rust
#[cfg(feature = "geojson")]
use avila_geo::GeoJson;

// Parse GeoJSON
let json = r#"{
    "type": "FeatureCollection",
    "features": [{
        "type": "Feature",
        "geometry": {
            "type": "Point",
            "coordinates": [-46.63, -23.55]
        },
        "properties": {
            "name": "São Paulo"
        }
    }]
}"#;

let geojson = GeoJson::from_str(json)?;
let collection = geojson.to_collection();

// Serialize to GeoJSON
let geojson = GeoJson::from_collection(&collection);
let json_string = geojson.to_string_pretty()?;
```

## 🎯 Examples

Run the included examples:

```bash
# World map with continents and cities
cargo run --example world_map

# Brazil map with states and cities
cargo run --example brazil_map

# Europe map with countries and capitals
cargo run --example europe_map

# Dubai and Gulf region map
cargo run --example dubai_gulf_map

# Compare different projections
cargo run --example custom_projection

# Web map tiles demonstration
cargo run --example tiles_example

# Spatial indexing and fast queries
cargo run --example spatial_index --features spatial-index
```

### Pre-defined Geographic Regions

```rust
use avila_geo::coords::GeoBounds;

// Available regions
let world = GeoBounds::WORLD;
let brazil = GeoBounds::BRAZIL;
let usa = GeoBounds::USA;
let europe = GeoBounds::EUROPE;
let middle_east = GeoBounds::MIDDLE_EAST;
let dubai = GeoBounds::DUBAI;
let gulf = GeoBounds::GULF_REGION;

// Quick map builders
let map_eu = MapBuilder::europe(1600, 1200);
let map_dubai = MapBuilder::dubai(1400, 1000);
```

## 🔬 Advanced Features

### Tile System for Web Mapping

```rust
use avila_geo::tiles::{TileCoord, TileSystem, TileUrlTemplate};

let tile_system = TileSystem::new();
let sao_paulo = GeoCoord::new(-23.55, -46.63);

// Get tile containing São Paulo at zoom 12
let tile = tile_system.geo_to_tile(&sao_paulo, 12);
println!("Tile: {}/{}/{}", tile.zoom, tile.x, tile.y);

// Convert to QuadKey (Bing Maps format)
let quadkey = tile.to_quadkey();

// Get tile URL
let url = TileUrlTemplate::osm().build(&tile, "a");

// Get all tiles covering a region
let tiles = tile_system.tiles_in_bounds(&GeoBounds::BRAZIL, 8);
```

### Spatial Indexing for Fast Queries

```rust
#[cfg(feature = "spatial-index")]
use avila_geo::spatial::SpatialIndex;

// Build R-Tree index
let cities = vec![/* 10,000 cities */];
let index = SpatialIndex::from_points(&cities);

// Find nearest city (O(log n))
let (id, distance) = index.nearest(&my_location).unwrap();

// Find 5 nearest cities
let nearest_5 = index.k_nearest(&my_location, 5);

// Find all cities within 50km
let nearby = index.within_distance(&my_location, 50_000.0);

// 100x faster than brute force!
```

### Parallel Processing

```rust
#[cfg(feature = "parallel")]
use avila_geo::parallel::*;

// Project 100,000 points in parallel (12x faster!)
let points = vec![/* 100k coordinates */];
let projected = project_points_parallel(
    &points,
    &projection,
    1920.0,
    1080.0
);

// Render multiple layers in parallel
let framebuffer = render_collections_parallel(
    &collections,
    &projection,
    1920,
    1080
);
```

### SVG Export

```rust
#[cfg(feature = "export-svg")]
use avila_geo::export::MapSvgExt;

// Export map as scalable vector graphics
map.save_svg(&projection, "output.svg")?;

// Resolution-independent, infinite zoom!
```

### Custom Projections

Implement the `Projection` trait for custom projections:

```rust
use avila_geo::projection::Projection;

struct MyCustomProjection;

impl Projection for MyCustomProjection {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        // Your projection math here
        CartesianCoord::new(x, y)
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        // Inverse projection
        GeoCoord::new(lat, lon)
    }

    fn is_conformal(&self) -> bool { false }
    fn is_equal_area(&self) -> bool { true }
}
```

### Line Simplification

```rust
use avila_geo::calc::douglas_peucker;

let detailed_line = vec![/* many points */];
let simplified = douglas_peucker(&detailed_line, 0.01); // epsilon in degrees

println!("Reduced from {} to {} points",
    detailed_line.len(),
    simplified.len()
);
```

### Polygon Operations

```rust
// Area calculations
let area_deg2 = polygon.area_degrees(); // Planar approximation
let area_m2 = polygon.area_meters();    // Spherical Earth

// Perimeter
let perimeter_m = polygon.perimeter_meters();

// Contains
let inside = polygon.contains(&point);

// Bounds
let bounds = polygon.bounds();
```

## 🧪 Testing

```bash
cargo test
cargo test --features geojson
```

## 📊 Benchmarks

```bash
cargo bench
```

Example results:
- Equirectangular projection: ~5ns per point
- Mercator projection: ~15ns per point
- Line drawing (1000px): ~25μs
- Polygon fill (complex): ~150μs

## 🌐 Real-World Use Cases

### 🗺️ Geographic Data Visualization
- Interactive web maps
- Static map rendering
- Tile server integration
- Custom cartography

### 🏭 Industry 4.0 Applications
- **Fleet Management**: Real-time vehicle tracking and optimization
- **Smart Manufacturing**: Digital twins of production equipment
- **Logistics**: Route optimization and delivery tracking
- **Predictive Maintenance**: IoT sensor monitoring and failure prediction
- **Geofencing**: Virtual boundaries for compliance and security
- **Agriculture**: Precision farming with spatial analytics

### 📊 Spatial Analytics
- Hotspot detection
- Clustering analysis
- Network analysis (shortest paths, centrality)
- Terrain analysis (slope, aspect, viewshed)

## 📚 Documentation

- **[GEOPROCESSING.md](GEOPROCESSING.md)** - Complete geoprocessing API guide
- **[INDUSTRY4.md](INDUSTRY4.md)** - Industry 4.0 features and examples
- **[API Docs](https://docs.rs/avila-geo)** - Full API reference

### Examples

```bash
# Cartography
cargo run --example world_map
cargo run --example brazil_map
cargo run --example tiles_example

# Geoprocessing
cargo run --example geoprocessing_demo
cargo run --example network_analysis
cargo run --example terrain_analysis
cargo run --example clustering_demo

# Industry 4.0
cargo run --example industry4_demo --features geoprocessing
cargo run --example realtime_demo --features geoprocessing

# Performance
cargo run --example parallel_demo --features parallel
cargo run --example simd_performance --features simd
```
- Render maps from custom data sources
- Create thematic maps (choropleth, heat maps)
- Visualize GPS tracks and routes

### Game Development
- Mini-maps and world maps
- Strategic/simulation games
- Location-based features

### Scientific Computing
- Spatial analysis
- Climate data visualization
- Geographic modeling

### Education
- Learn cartography concepts
- Understand map projections
- Study computational geometry

## 📖 Documentation

Full API documentation: [docs.rs/avila-geo](https://docs.rs/avila-geo)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🙏 Acknowledgments

This library implements fundamental cartographic algorithms from scratch:
- Bresenham's line algorithm (1965)
- Haversine formula (ancient, formalized 19th century)
- Vincenty's formulae (1975)
- Douglas-Peucker algorithm (1973)
- Map projections from classical cartography

## 🔗 Related Projects

- [geo](https://crates.io/crates/geo) - Geospatial primitives and algorithms
- [proj](https://crates.io/crates/proj) - Rust bindings to PROJ
- [image](https://crates.io/crates/image) - For converting PPM to PNG/JPEG

---

**Built with ❤️ for the AVL Platform** | [avila.cloud](https://avila.cloud)
