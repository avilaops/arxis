//! AvilaDB Geospatial Integration Example
//!
//! Demonstrates how to use avila-geo with AvilaDB for
//! geospatial data storage and queries.
//!
//! Features:
//! - GeoJSON document storage
//! - Spatial queries (radius, bounding box, polygon)
//! - Property filtering
//! - Nearest neighbor search
//! - Spatial indexing
//!
//! Run with:
//! ```bash
//! cargo run --example aviladb_integration
//! ```

use avila_geo::prelude::*;
use avila_geo::aviladb::*;

fn main() {
    println!("=== AvilaDB Geospatial Integration Demo ===\n");

    // Example 1: Create GeoDocuments
    println!("1. Creating Geographic Documents");
    println!("--------------------------------");

    let brazilian_cities = vec![
        GeoDocument::new()
            .with_id("city_saopaulo")
            .with_location(GeoCoord::new(-23.55, -46.63))
            .with_property("name", "São Paulo")
            .with_property("state", "SP")
            .with_property("population", 12_300_000)
            .with_property("category", "metropolis"),

        GeoDocument::new()
            .with_id("city_rio")
            .with_location(GeoCoord::new(-22.91, -43.17))
            .with_property("name", "Rio de Janeiro")
            .with_property("state", "RJ")
            .with_property("population", 6_700_000)
            .with_property("category", "metropolis"),

        GeoDocument::new()
            .with_id("city_brasilia")
            .with_location(GeoCoord::new(-15.78, -47.93))
            .with_property("name", "Brasília")
            .with_property("state", "DF")
            .with_property("population", 3_000_000)
            .with_property("category", "capital"),

        GeoDocument::new()
            .with_id("city_salvador")
            .with_location(GeoCoord::new(-12.97, -38.51))
            .with_property("name", "Salvador")
            .with_property("state", "BA")
            .with_property("population", 2_900_000)
            .with_property("category", "metropolis"),

        GeoDocument::new()
            .with_id("city_fortaleza")
            .with_location(GeoCoord::new(-3.73, -38.52))
            .with_property("name", "Fortaleza")
            .with_property("state", "CE")
            .with_property("population", 2_700_000)
            .with_property("category", "metropolis"),

        GeoDocument::new()
            .with_id("city_bh")
            .with_location(GeoCoord::new(-19.92, -43.94))
            .with_property("name", "Belo Horizonte")
            .with_property("state", "MG")
            .with_property("population", 2_500_000)
            .with_property("category", "metropolis"),

        GeoDocument::new()
            .with_id("city_curitiba")
            .with_location(GeoCoord::new(-25.43, -49.27))
            .with_property("name", "Curitiba")
            .with_property("state", "PR")
            .with_property("population", 1_900_000)
            .with_property("category", "metropolis"),

        GeoDocument::new()
            .with_id("city_recife")
            .with_location(GeoCoord::new(-8.05, -34.90))
            .with_property("name", "Recife")
            .with_property("state", "PE")
            .with_property("population", 1_600_000)
            .with_property("category", "metropolis"),
    ];

    println!("Created {} city documents", brazilian_cities.len());
    for doc in brazilian_cities.iter().take(3) {
        println!("  - {} ({}) - Population: {}",
            doc.properties["name"],
            doc.properties["state"],
            doc.properties["population"]
        );
    }
    println!("  ... and {} more\n", brazilian_cities.len() - 3);

    // Example 2: Radius Query
    println!("2. Radius Query - Cities within 300km of São Paulo");
    println!("-------------------------------------------------");

    let sp_location = GeoCoord::new(-23.55, -46.63);
    let radius_query = GeoQuery::within_radius(sp_location, 300_000.0);

    let results = radius_query.execute(&brazilian_cities);
    println!("Found {} cities:", results.len());
    for doc in &results {
        let dist = haversine_distance(&sp_location, &doc.coordinates[0]);
        println!("  - {} - {:.1} km away",
            doc.properties["name"],
            dist / 1000.0
        );
    }
    println!();

    // Example 3: Bounding Box Query
    println!("3. Bounding Box Query - Southeast Region");
    println!("----------------------------------------");

    let southeast_bounds = GeoBounds {
        min_lat: -25.0,
        max_lat: -15.0,
        min_lon: -50.0,
        max_lon: -40.0,
    };

    let bbox_query = GeoQuery::within_bounds(southeast_bounds);
    let results = bbox_query.execute(&brazilian_cities);

    println!("Cities in Southeast region:");
    for doc in &results {
        println!("  - {} ({}) - {}",
            doc.properties["name"],
            doc.properties["state"],
            doc.coordinates[0]
        );
    }
    println!();

    // Example 4: Property Filtering
    println!("4. Property Filtering - Large Metropolises");
    println!("------------------------------------------");

    let large_cities_query = GeoQuery::within_bounds(GeoBounds {
        min_lat: -90.0,
        max_lat: 90.0,
        min_lon: -180.0,
        max_lon: 180.0,
    }).with_filter(PropertyFilter {
        field: "population".to_string(),
        operator: FilterOperator::GreaterThan,
        value: serde_json::json!(5_000_000),
    });

    let results = large_cities_query.execute(&brazilian_cities);
    println!("Cities with population > 5M:");
    for doc in &results {
        println!("  - {} - {} inhabitants",
            doc.properties["name"],
            doc.properties["population"]
        );
    }
    println!();

    // Example 5: Nearest Neighbors Query
    println!("5. Nearest Neighbors - 3 closest cities to Brasília");
    println!("---------------------------------------------------");

    let brasilia = GeoCoord::new(-15.78, -47.93);
    let knn_query = GeoQuery::nearest_neighbors(brasilia, 3);

    let results = knn_query.execute(&brazilian_cities);
    println!("3 nearest cities to Brasília:");
    for (i, doc) in results.iter().enumerate() {
        let dist = haversine_distance(&brasilia, &doc.coordinates[0]);
        println!("  {}. {} - {:.1} km away",
            i + 1,
            doc.properties["name"],
            dist / 1000.0
        );
    }
    println!();

    // Example 6: Polygon Query - Coastal Region
    println!("6. Polygon Query - Coastal Cities");
    println!("---------------------------------");

    let coastal_polygon = vec![
        GeoCoord::new(-25.0, -50.0),
        GeoCoord::new(-25.0, -34.0),
        GeoCoord::new(-3.0, -34.0),
        GeoCoord::new(-3.0, -36.0),
        GeoCoord::new(-10.0, -36.0),
        GeoCoord::new(-10.0, -50.0),
    ];

    let polygon_query = GeoQuery::within_polygon(coastal_polygon);
    let results = polygon_query.execute(&brazilian_cities);

    println!("Cities in coastal region:");
    for doc in &results {
        println!("  - {} ({}) on the coast",
            doc.properties["name"],
            doc.properties["state"]
        );
    }
    println!();

    // Example 7: Spatial Index Performance
    println!("7. Spatial Index Performance");
    println!("---------------------------");

    use std::time::Instant;

    let mut index = GeoIndex::new(1.0);
    for (i, doc) in brazilian_cities.iter().enumerate() {
        index.insert(i, &doc.coordinates[0]);
    }
    println!("Indexed {} documents", brazilian_cities.len());

    let test_bounds = GeoBounds {
        min_lat: -24.0,
        max_lat: -22.0,
        min_lon: -47.0,
        max_lon: -43.0,
    };

    // Without index
    let start = Instant::now();
    for _ in 0..10_000 {
        let query = GeoQuery::within_bounds(test_bounds);
        let _ = query.execute(&brazilian_cities);
    }
    let without_index = start.elapsed();

    // With index
    let start = Instant::now();
    for _ in 0..10_000 {
        let _ = index.query_bounds(&test_bounds);
    }
    let with_index = start.elapsed();

    println!("10,000 queries without index: {:?}", without_index);
    println!("10,000 queries with index: {:?}", with_index);
    println!("Speedup: {:.2}x\n", without_index.as_secs_f64() / with_index.as_secs_f64());

    // Example 8: GeoJSON Export
    println!("8. GeoJSON Export");
    println!("----------------");

    println!("Exporting documents to GeoJSON format:");
    let sao_paulo = &brazilian_cities[0];
    let geojson = sao_paulo.to_geojson();
    println!("{}", serde_json::to_string_pretty(&geojson).unwrap());
    println!();

    // Example 9: Complex Query - Metropolises near coast
    println!("9. Complex Query - Large coastal cities");
    println!("---------------------------------------");

    let complex_query = GeoQuery::within_polygon(vec![
        GeoCoord::new(-25.0, -50.0),
        GeoCoord::new(-25.0, -34.0),
        GeoCoord::new(-3.0, -34.0),
        GeoCoord::new(-3.0, -50.0),
    ])
    .with_filter(PropertyFilter {
        field: "population".to_string(),
        operator: FilterOperator::GreaterThan,
        value: serde_json::json!(2_000_000),
    })
    .with_filter(PropertyFilter {
        field: "category".to_string(),
        operator: FilterOperator::Equal,
        value: serde_json::json!("metropolis"),
    });

    let results = complex_query.execute(&brazilian_cities);
    println!("Large coastal metropolises:");
    for doc in &results {
        println!("  - {} - {} people",
            doc.properties["name"],
            doc.properties["population"]
        );
    }
    println!();

    // Example 10: Real-world Use Case - Store Locator
    println!("10. Use Case: Store Locator Service");
    println!("-----------------------------------");

    let user_location = GeoCoord::new(-23.60, -46.70); // User in west SP
    println!("User location: {:?}", user_location);

    let nearby_query = GeoQuery::nearest_neighbors(user_location, 3)
        .with_filter(PropertyFilter {
            field: "category".to_string(),
            operator: FilterOperator::Equal,
            value: serde_json::json!("metropolis"),
        });

    let results = nearby_query.execute(&brazilian_cities);
    println!("\nClosest 3 stores (in metropolises):");
    for (i, doc) in results.iter().enumerate() {
        let dist = haversine_distance(&user_location, &doc.coordinates[0]);
        let eta_minutes = (dist / 1000.0) * 1.5; // Estimate: 1.5 min per km
        println!("  {}. {} store", i + 1, doc.properties["name"]);
        println!("     Distance: {:.1} km", dist / 1000.0);
        println!("     ETA: {:.0} minutes", eta_minutes);
    }
    println!();

    // Summary Statistics
    println!("=== Summary ===");
    println!("Total documents: {}", brazilian_cities.len());
    println!("Total population: {} million",
        brazilian_cities.iter()
            .map(|d| d.properties["population"].as_i64().unwrap())
            .sum::<i64>() / 1_000_000
    );

    let (min, max) = bounding_box(&brazilian_cities.iter()
        .flat_map(|d| d.coordinates.clone())
        .collect::<Vec<_>>());
    println!("Geographic extent:");
    println!("  Southwest: {:?}", min);
    println!("  Northeast: {:?}", max);
    println!("  Coverage: {:.0} × {:.0} km",
        haversine_distance(&GeoCoord::new(min.lat, min.lon), &GeoCoord::new(min.lat, max.lon)) / 1000.0,
        haversine_distance(&GeoCoord::new(min.lat, min.lon), &GeoCoord::new(max.lat, min.lon)) / 1000.0
    );

    println!("\n=== Demo Complete ===");
}
