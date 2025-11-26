//! Topology Operations Example
//!
//! Demonstrates computational geometry operations:
//! - Buffer operations (point, line, polygon)
//! - Convex hull computation
//! - Bounding boxes and centroids
//! - Line intersection
//! - Polygon clipping
//!
//! Run with:
//! ```bash
//! cargo run --example topology_operations
//! ```

use avila_geo::prelude::*;
use avila_geo::topology::*;

fn main() {
    println!("=== Avila-Geo Topology Operations Demo ===\n");

    // Example 1: Buffer Operations
    println!("1. Buffer Operations");
    println!("-------------------");

    let sao_paulo = GeoCoord::new(-23.55, -46.63);
    println!("São Paulo: {:?}", sao_paulo);

    let buffer = buffer_point(&sao_paulo, 5000.0, 16);
    println!("Buffer (5km radius, 16 segments): {} points", buffer.exterior.len());
    println!("First 3 points: {:?}\n", &buffer.exterior[..3]);

    // Example 2: Line Buffer
    println!("2. Line Buffer");
    println!("-------------");

    let line = GeoLine::new(vec![
        GeoCoord::new(-23.55, -46.63), // São Paulo
        GeoCoord::new(-22.91, -43.17), // Rio de Janeiro
    ], LineType::Road);

    let line_buffer = buffer_line(&line, 1000.0, 8);
    println!("Line buffer (1km width): {} points", line_buffer.exterior.len());
    println!("Area: {:.2} km²\n", polygon_area(&line_buffer) / 1_000_000.0);

    // Example 3: Convex Hull
    println!("3. Convex Hull");
    println!("-------------");

    let cities = vec![
        GeoCoord::new(-23.55, -46.63), // São Paulo
        GeoCoord::new(-22.91, -43.17), // Rio de Janeiro
        GeoCoord::new(-15.78, -47.93), // Brasília
        GeoCoord::new(-12.97, -38.51), // Salvador
        GeoCoord::new(-8.05, -34.90),  // Recife
        GeoCoord::new(-3.73, -38.52),  // Fortaleza
        GeoCoord::new(-1.46, -48.50),  // Belém
        GeoCoord::new(-30.03, -51.23), // Porto Alegre
    ];

    println!("Major Brazilian cities: {}", cities.len());
    let hull = convex_hull(&cities);
    println!("Convex hull vertices: {}", hull.len());
    println!("Hull points:");
    for (i, point) in hull.iter().enumerate() {
        println!("  {}: {:?}", i + 1, point);
    }
    println!();

    // Example 4: Bounding Box
    println!("4. Bounding Box");
    println!("--------------");

    let (min, max) = bounding_box(&cities);
    println!("Brazil bounding box:");
    println!("  Southwest: {:?}", min);
    println!("  Northeast: {:?}", max);
    println!("  Width: {:.2} km", haversine_distance(
        &GeoCoord::new(min.lat, min.lon),
        &GeoCoord::new(min.lat, max.lon)
    ) / 1000.0);
    println!("  Height: {:.2} km\n", haversine_distance(
        &GeoCoord::new(min.lat, min.lon),
        &GeoCoord::new(max.lat, min.lon)
    ) / 1000.0);

    // Example 5: Centroid
    println!("5. Centroid");
    println!("----------");

    let brazil_approx = GeoPolygon::new(vec![
        GeoCoord::new(-33.75, -53.37), // South
        GeoCoord::new(-7.16, -34.79),  // Northeast
        GeoCoord::new(5.27, -60.64),   // North
        GeoCoord::new(-18.04, -73.98), // West
    ]);

    let center = centroid(&brazil_approx);
    println!("Brazil approximate centroid: {:?}", center);
    println!("(Actual geographic center: ~-14.2°, -51.9°)\n");

    // Example 6: Line Intersection
    println!("6. Line Intersection");
    println!("-------------------");

    // Two diagonal lines crossing
    let line1_start = GeoCoord::new(-23.0, -46.0);
    let line1_end = GeoCoord::new(-24.0, -47.0);
    let line2_start = GeoCoord::new(-23.0, -47.0);
    let line2_end = GeoCoord::new(-24.0, -46.0);

    match line_intersection(&line1_start, &line1_end, &line2_start, &line2_end) {
        Some(intersection) => {
            println!("Lines intersect at: {:?}", intersection);
            println!("Expected: approximately (-23.5, -46.5)");
        }
        None => println!("Lines do not intersect"),
    }
    println!();

    // Example 7: Polygon Clipping
    println!("7. Polygon Clipping (Sutherland-Hodgman)");
    println!("---------------------------------------");

    let polygon = GeoPolygon::new(vec![
        GeoCoord::new(-22.0, -45.0),
        GeoCoord::new(-22.0, -44.0),
        GeoCoord::new(-24.0, -44.0),
        GeoCoord::new(-24.0, -45.0),
    ]);

    let clip_min = GeoCoord::new(-23.5, -44.8);
    let clip_max = GeoCoord::new(-22.5, -44.2);

    println!("Original polygon: {} points", polygon.exterior.len());
    let clipped = clip_polygon(&polygon, &clip_min, &clip_max);
    println!("Clipped polygon: {} points", clipped.exterior.len());
    println!("Clipped vertices:");
    for (i, point) in clipped.exterior.iter().enumerate() {
        println!("  {}: {:?}", i + 1, point);
    }
    println!();

    // Example 8: Minimum Bounding Circle
    println!("8. Minimum Bounding Circle");
    println!("-------------------------");

    let (center, radius) = minimum_bounding_circle(&cities);
    println!("Circle center: {:?}", center);
    println!("Circle radius: {:.2} km", radius / 1000.0);
    println!("This circle encloses all major Brazilian cities\n");

    // Example 9: Practical Use Case - Service Area
    println!("9. Practical Use Case: Service Area Analysis");
    println!("--------------------------------------------");

    let warehouse = GeoCoord::new(-23.55, -46.63);
    let service_radius = 50_000.0; // 50 km

    let service_area = buffer_point(&warehouse, service_radius, 32);
    println!("Warehouse location: {:?}", warehouse);
    println!("Service radius: {} km", service_radius / 1000.0);
    println!("Service area polygon: {} points", service_area.exterior.len());

    // Check which cities are in service area
    let test_cities = vec![
        ("Guarulhos", GeoCoord::new(-23.46, -46.53)),
        ("Osasco", GeoCoord::new(-23.53, -46.79)),
        ("Santo André", GeoCoord::new(-23.66, -46.53)),
        ("Sorocaba", GeoCoord::new(-23.50, -47.45)), // Likely outside
    ];

    println!("\nCities within service area:");
    for (name, location) in &test_cities {
        let distance = haversine_distance(&warehouse, location);
        let in_area = distance <= service_radius;
        println!("  {} - {:.1} km {}",
            name,
            distance / 1000.0,
            if in_area { "✓" } else { "✗" }
        );
    }
    println!();

    // Example 10: Performance Comparison
    println!("10. Performance Benchmark");
    println!("------------------------");

    use std::time::Instant;

    let n = 1000;
    let start = Instant::now();
    for i in 0..n {
        let coord = GeoCoord::new(-23.55 + (i as f64 * 0.01), -46.63);
        buffer_point(&coord, 1000.0, 16);
    }
    let elapsed = start.elapsed();
    println!("Buffer {} points: {:?}", n, elapsed);
    println!("Average: {:.2} µs per operation", elapsed.as_micros() as f64 / n as f64);

    let start = Instant::now();
    for _ in 0..n {
        convex_hull(&cities);
    }
    let elapsed = start.elapsed();
    println!("Convex hull {} times: {:?}", n, elapsed);
    println!("Average: {:.2} µs per operation\n", elapsed.as_micros() as f64 / n as f64);

    println!("=== Demo Complete ===");
}

// Helper function for polygon area
fn polygon_area(polygon: &GeoPolygon) -> f64 {
    let mut area = 0.0;
    let n = polygon.exterior.len();

    for i in 0..n {
        let j = (i + 1) % n;
        let p1 = &polygon.exterior[i];
        let p2 = &polygon.exterior[j];

        area += (p2.lon - p1.lon).to_radians() *
                (2.0 + p1.lat.to_radians().sin() + p2.lat.to_radians().sin());
    }

    const EARTH_RADIUS: f64 = 6371008.8;
    (area.abs() * EARTH_RADIUS * EARTH_RADIUS / 2.0)
}
