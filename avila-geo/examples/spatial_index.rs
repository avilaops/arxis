//! Example: Spatial indexing for fast queries
//!
//! Demonstrates R-Tree spatial index for efficient nearest neighbor
//! and range queries on large geographic datasets.

#[cfg(feature = "spatial-index")]
use avila_geo::prelude::*;

#[cfg(feature = "spatial-index")]
fn main() {
    println!("=== Spatial Index Example ===\n");

    // Create dataset of Brazilian cities
    let cities = vec![
        GeoPoint::with_name(GeoCoord::new(-23.55, -46.63), "São Paulo"),
        GeoPoint::with_name(GeoCoord::new(-22.91, -43.17), "Rio de Janeiro"),
        GeoPoint::with_name(GeoCoord::new(-15.78, -47.93), "Brasília"),
        GeoPoint::with_name(GeoCoord::new(-12.97, -38.51), "Salvador"),
        GeoPoint::with_name(GeoCoord::new(-8.05, -34.90), "Recife"),
        GeoPoint::with_name(GeoCoord::new(-3.71, -38.54), "Fortaleza"),
        GeoPoint::with_name(GeoCoord::new(-1.45, -48.48), "Belém"),
        GeoPoint::with_name(GeoCoord::new(-25.42, -49.27), "Curitiba"),
        GeoPoint::with_name(GeoCoord::new(-30.03, -51.23), "Porto Alegre"),
        GeoPoint::with_name(GeoCoord::new(-19.92, -43.94), "Belo Horizonte"),
    ];

    println!("Building spatial index with {} cities...", cities.len());
    let index = SpatialIndex::from_points(&cities);
    println!("Index built with {} points\n", index.len());

    // Example 1: Find nearest city
    let query_point = GeoCoord::new(-23.5, -46.6); // Near São Paulo

    println!("Query: Nearest city to ({:.2}, {:.2})", query_point.lat, query_point.lon);
    if let Some((id, dist)) = index.nearest(&query_point) {
        println!("  Nearest: {} ({:.1} km)", cities[id].name().unwrap(), dist / 1000.0);
    }
    println!();

    // Example 2: Find k nearest neighbors
    let brasilia = GeoCoord::new(-15.78, -47.93);

    println!("Query: 3 nearest cities to Brasília");
    let k_nearest = index.k_nearest(&brasilia, 3);
    for (i, (id, dist)) in k_nearest.iter().enumerate() {
        println!("  {}. {} ({:.1} km)",
            i + 1,
            cities[*id].name().unwrap(),
            dist / 1000.0
        );
    }
    println!();

    // Example 3: Find cities within radius
    let radius_km = 500.0;
    let center = GeoCoord::new(-23.55, -46.63); // São Paulo

    println!("Query: Cities within {} km of São Paulo", radius_km);
    let within = index.within_distance(&center, radius_km * 1000.0);
    println!("  Found {} cities:", within.len());
    for (id, dist) in within {
        println!("    - {} ({:.1} km)", cities[id].name().unwrap(), dist / 1000.0);
    }
    println!();

    // Example 4: Find cities in bounding box
    let northeast_brazil = GeoBounds::new(-18.0, -3.0, -48.0, -34.0);

    println!("Query: Cities in Northeast Brazil region");
    let in_bounds = index.within_bounds(&northeast_brazil);
    println!("  Found {} cities:", in_bounds.len());
    for id in in_bounds {
        println!("    - {}", cities[id].name().unwrap());
    }
    println!();

    // Example 5: Performance comparison
    println!("Performance comparison (finding 5 nearest):");

    use std::time::Instant;

    // With index
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = index.k_nearest(&query_point, 5);
    }
    let with_index = start.elapsed();

    // Without index (brute force)
    let start = Instant::now();
    for _ in 0..1000 {
        let mut distances: Vec<_> = cities
            .iter()
            .map(|c| crate::calc::haversine_distance(&c.coord, &query_point))
            .enumerate()
            .collect();
        distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let _ = distances.into_iter().take(5).collect::<Vec<_>>();
    }
    let without_index = start.elapsed();

    println!("  With R-Tree index: {:?}", with_index);
    println!("  Without index (brute force): {:?}", without_index);
    println!("  Speedup: {:.1}x", without_index.as_secs_f64() / with_index.as_secs_f64());

    println!("\n=== Spatial indexing demonstration complete! ===");
}

#[cfg(not(feature = "spatial-index"))]
fn main() {
    println!("This example requires the 'spatial-index' feature.");
    println!("Run with: cargo run --example spatial_index --features spatial-index");
}
