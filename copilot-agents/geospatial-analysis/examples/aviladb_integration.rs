//! Example: Store and query geospatial data with AvilaDB
//!
//! This example demonstrates how to integrate geospatial analysis
//! with AvilaDB for persistent storage and querying.

use geospatial_analysis::{
    coords::WebMercator,
    distance::haversine_distance_m,
    indexing::{SpatialFeature, SpatialIndex},
};
use geo::Coord;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoreLocation {
    id: String,
    name: String,
    location: GeoPoint,
    store_type: String,
    opening_hours: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeoPoint {
    longitude: f64,
    latitude: f64,
}

impl StoreLocation {
    fn to_coord(&self) -> Coord<f64> {
        Coord {
            x: self.location.longitude,
            y: self.location.latitude,
        }
    }

    fn to_geojson(&self) -> serde_json::Value {
        json!({
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [self.location.longitude, self.location.latitude]
            },
            "properties": {
                "id": self.id,
                "name": self.name,
                "store_type": self.store_type,
                "opening_hours": self.opening_hours
            }
        })
    }
}

/// Example: Optimal store placement analysis for Portugal
fn example_store_placement() {
    use geospatial_analysis::optimization::{p_median_greedy, DemandPoint};

    println!("=== Optimal Store Placement Analysis ===\n");

    // Population centers in Portugal (simplified)
    let demand_points = vec![
        DemandPoint::new(
            Coord {
                x: -9.1393,
                y: 38.7223,
            },
            2_900_000.0,
        ), // Lisbon metro
        DemandPoint::new(
            Coord {
                x: -8.6291,
                y: 41.1579,
            },
            1_700_000.0,
        ), // Porto metro
        DemandPoint::new(
            Coord {
                x: -7.9304,
                y: 37.0194,
            },
            450_000.0,
        ), // Faro
        DemandPoint::new(
            Coord {
                x: -8.4103,
                y: 40.2033,
            },
            430_000.0,
        ), // Coimbra
        DemandPoint::new(
            Coord {
                x: -8.4261,
                y: 41.5518,
            },
            420_000.0,
        ), // Braga
    ];

    // Candidate locations for new stores
    let candidates = vec![
        Coord {
            x: -9.15,
            y: 38.75,
        }, // Lisbon area
        Coord {
            x: -8.60,
            y: 41.15,
        }, // Porto area
        Coord {
            x: -8.00,
            y: 37.00,
        }, // Algarve
        Coord {
            x: -8.40,
            y: 40.20,
        }, // Central Portugal
    ];

    // Find optimal 3 store locations
    match p_median_greedy(&demand_points, &candidates, 3) {
        Ok(optimal_locations) => {
            println!("Optimal locations for 3 stores:");
            for (i, loc) in optimal_locations.iter().enumerate() {
                println!("  Store {}: {:.4}°, {:.4}°", i + 1, loc.x, loc.y);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

/// Example: Find nearest stores to customer location
fn example_nearest_stores() {
    println!("\n=== Nearest Store Finder ===\n");

    // Build spatial index of stores
    let stores = vec![
        SpatialFeature::new(
            "store_lisbon_centro".to_string(),
            Coord {
                x: -9.1393,
                y: 38.7223,
            },
        ),
        SpatialFeature::new(
            "store_porto_baixa".to_string(),
            Coord {
                x: -8.6291,
                y: 41.1579,
            },
        ),
        SpatialFeature::new(
            "store_faro_marina".to_string(),
            Coord {
                x: -7.9304,
                y: 37.0194,
            },
        ),
    ];

    let index = SpatialIndex::from_features(stores);

    // Customer location (somewhere between Lisbon and Porto)
    let customer = Coord {
        x: -8.8,
        y: 39.5,
    };

    println!("Customer location: {:.4}°, {:.4}°", customer.x, customer.y);

    // Find 2 nearest stores
    let nearest = index.k_nearest_neighbors(&customer, 2);

    println!("\nNearest stores:");
    for (i, store) in nearest.iter().enumerate() {
        let distance = haversine_distance_m(&customer, &store.location).unwrap();
        println!(
            "  {}. {} - {:.1} km away",
            i + 1,
            store.id,
            distance / 1000.0
        );
    }
}

/// Example: Service area coverage analysis
fn example_coverage_analysis() {
    use geospatial_analysis::optimization::maximal_coverage;

    println!("\n=== Service Area Coverage Analysis ===\n");

    let population_centers = vec![
        DemandPoint::new(
            Coord {
                x: -9.1393,
                y: 38.7223,
            },
            2_900_000.0,
        ),
        DemandPoint::new(
            Coord {
                x: -8.6291,
                y: 41.1579,
            },
            1_700_000.0,
        ),
        DemandPoint::new(
            Coord {
                x: -7.9304,
                y: 37.0194,
            },
            450_000.0,
        ),
    ];

    let warehouse_candidates = vec![
        Coord {
            x: -9.1,
            y: 38.7,
        },
        Coord {
            x: -8.6,
            y: 41.1,
        },
        Coord {
            x: -8.0,
            y: 37.0,
        },
    ];

    // 50km delivery radius
    let delivery_radius = 50_000.0;

    match maximal_coverage(&population_centers, &warehouse_candidates, 2, delivery_radius) {
        Ok((warehouses, coverage)) => {
            println!(
                "With 2 warehouses (50km radius), coverage: {:.1}%",
                coverage * 100.0
            );
            println!("Warehouse locations:");
            for (i, wh) in warehouses.iter().enumerate() {
                println!("  Warehouse {}: {:.4}°, {:.4}°", i + 1, wh.x, wh.y);
            }
        }
        Err(e) => println!("Error: {}", e),
    }
}

/// Pseudo-code for AvilaDB integration
/// (Replace with actual AvilaDB SDK when available)
fn example_aviladb_storage() {
    println!("\n=== AvilaDB Storage Example (Pseudo-code) ===\n");

    println!("// Store spatial data in AvilaDB");
    println!(
        r#"
use aviladb::{{AvilaClient, Collection, Document}};

async fn store_locations(locations: Vec<StoreLocation>) -> Result<()> {{
    let client = AvilaClient::connect("avila://localhost:8000").await?;
    let db = client.database("retail_db").await?;
    let collection = db.collection("stores").await?;

    for location in locations {{
        let doc = Document::new()
            .set("id", location.id)
            .set("name", location.name)
            .set("location", json!({{
                "type": "Point",
                "coordinates": [location.location.longitude, location.location.latitude]
            }}))
            .set("store_type", location.store_type)
            .set("opening_hours", location.opening_hours);

        collection.insert(doc).await?;
    }}

    Ok(())
}}

// Query nearby stores using bounding box
async fn query_nearby_stores(
    collection: &Collection,
    center: Coord<f64>,
    radius_km: f64,
) -> Result<Vec<StoreLocation>> {{
    // Calculate approximate bounding box
    let lat_delta = radius_km / 111.0; // ~111km per degree latitude
    let lon_delta = radius_km / (111.0 * center.y.to_radians().cos());

    let query = format!(
        "SELECT * FROM c WHERE 
         c.location.coordinates[0] >= {{}} AND 
         c.location.coordinates[0] <= {{}} AND
         c.location.coordinates[1] >= {{}} AND
         c.location.coordinates[1] <= {{}}",
        center.x - lon_delta,
        center.x + lon_delta,
        center.y - lat_delta,
        center.y + lat_delta
    );

    let results = collection.query(&query).await?;
    
    // Post-filter with accurate distance calculation
    let mut nearby = Vec::new();
    for result in results {{
        let store: StoreLocation = serde_json::from_value(result)?;
        let distance = haversine_distance_m(&center, &store.to_coord())?;
        if distance <= radius_km * 1000.0 {{
            nearby.push(store);
        }}
    }}

    Ok(nearby)
}}
"#
    );
}

fn main() {
    println!("Geospatial Analysis with AvilaDB - Examples\n");
    println!("============================================\n");

    example_store_placement();
    example_nearest_stores();
    example_coverage_analysis();
    example_aviladb_storage();

    println!("\n✅ All examples completed!");
}
