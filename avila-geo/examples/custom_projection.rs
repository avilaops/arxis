//! Custom projection example
//!
//! Demonstrates creating custom projections and comparing different projections

use avila_geo::{
    coords::{CartesianCoord, GeoCoord, GeoBounds},
    geometry::{shapes, GeoCollection, GeoPoint},
    map::{Layer, Map, Style},
    projection::{Equirectangular, LambertConformalConic, Mercator, Projection, WebMercator},
    render::Color,
};

/// Custom sinusoidal projection (equal-area)
struct SinusoidalProjection;

impl Projection for SinusoidalProjection {
    fn project(&self, geo: &GeoCoord, width: f64, height: f64) -> CartesianCoord {
        let lon_rad = geo.lon.to_radians();
        let lat_rad = geo.lat.to_radians();

        let x = (lon_rad * lat_rad.cos() + std::f64::consts::PI) * (width / (2.0 * std::f64::consts::PI));
        let y = (std::f64::consts::PI / 2.0 - lat_rad) * (height / std::f64::consts::PI);

        CartesianCoord::new(x, y)
    }

    fn unproject(&self, cart: &CartesianCoord, width: f64, height: f64) -> GeoCoord {
        let lat_rad = std::f64::consts::PI / 2.0 - cart.y * std::f64::consts::PI / height;
        let lon_rad = (cart.x * 2.0 * std::f64::consts::PI / width - std::f64::consts::PI) / lat_rad.cos();

        GeoCoord::new_unchecked(lat_rad.to_degrees(), lon_rad.to_degrees())
    }

    fn is_equal_area(&self) -> bool {
        true
    }
}

fn create_sample_map() -> (GeoCollection, GeoCollection) {
    let mut land = GeoCollection::new();
    let mut cities = GeoCollection::new();

    // Simplified world continents
    let continents = vec![
        shapes::rectangle(-35.0, 70.0, -170.0, -30.0), // Americas
        shapes::rectangle(-35.0, 71.0, -10.0, 60.0),   // Europe/Africa
        shapes::rectangle(-45.0, 75.0, 60.0, 180.0),   // Asia/Oceania
    ];

    for continent in continents {
        land.add_polygon(continent);
    }

    // Major cities
    let world_cities = vec![
        (40.71, -74.01, "New York"),
        (51.51, -0.13, "London"),
        (35.68, 139.65, "Tokyo"),
        (-23.55, -46.63, "São Paulo"),
        (-33.87, 151.21, "Sydney"),
    ];

    for (lat, lon, name) in world_cities {
        cities.add_point(GeoPoint::with_name(GeoCoord::new(lat, lon), name));
    }

    (land, cities)
}

fn render_with_projection<P: Projection>(
    projection: &P,
    name: &str,
    land: &GeoCollection,
    cities: &GeoCollection,
) {
    println!("\n🗺️  Rendering with {} projection...", name);

    let mut map = Map::new(800, 600)
        .with_bounds(GeoBounds::WORLD)
        .with_background(Color::from_hex(0xADD8E6));

    // Land layer
    let land_style = Style::new()
        .with_fill(Color::from_hex(0x90EE90))
        .with_stroke(Color::from_hex(0x228B22), 2);
    map.add_layer(Layer::new("land", land.clone(), land_style));

    // Cities layer
    let city_style = Style::city();
    map.add_layer(Layer::new("cities", cities.clone(), city_style));

    // Render
    let filename = format!("{}_projection.ppm", name.to_lowercase().replace(' ', "_"));
    match map.save_ppm(projection, &filename) {
        Ok(_) => println!("   ✅ Saved to {}", filename),
        Err(e) => eprintln!("   ❌ Error: {}", e),
    }

    // Print projection properties
    println!("   Properties:");
    println!("     - Conformal: {}", projection.is_conformal());
    println!("     - Equal-area: {}", projection.is_equal_area());
}

fn compare_distances() {
    println!("\n📏 Distance Comparison:");
    println!("   Comparing different distance calculation methods\n");

    let nyc = GeoCoord::new(40.71, -74.01);
    let london = GeoCoord::new(51.51, -0.13);

    use avila_geo::calc::{haversine_distance, vincenty_distance};

    let haversine = haversine_distance(&nyc, &london);
    let vincenty = vincenty_distance(&nyc, &london);

    println!("   New York to London:");
    println!("     - Haversine: {:.2} km", haversine / 1000.0);
    println!("     - Vincenty:  {:.2} km", vincenty / 1000.0);
    println!("     - Difference: {:.2} km", (vincenty - haversine).abs() / 1000.0);

    let sao_paulo = GeoCoord::new(-23.55, -46.63);
    let rio = GeoCoord::new(-22.91, -43.17);

    let haversine2 = haversine_distance(&sao_paulo, &rio);
    let vincenty2 = vincenty_distance(&sao_paulo, &rio);

    println!("\n   São Paulo to Rio de Janeiro:");
    println!("     - Haversine: {:.2} km", haversine2 / 1000.0);
    println!("     - Vincenty:  {:.2} km", vincenty2 / 1000.0);
    println!("     - Difference: {:.2} km", (vincenty2 - haversine2).abs() / 1000.0);
}

fn demonstrate_calculations() {
    println!("\n🧮 Geographic Calculations:");

    let p1 = GeoCoord::new(-23.55, -46.63); // São Paulo
    let p2 = GeoCoord::new(-22.91, -43.17); // Rio de Janeiro

    use avila_geo::calc::{bearing, destination, midpoint};

    // Bearing
    let brg = bearing(&p1, &p2);
    println!("   Bearing from São Paulo to Rio: {:.1}°", brg);

    // Midpoint
    let mid = midpoint(&p1, &p2);
    println!("   Midpoint: {:.2}°S, {:.2}°W", -mid.lat, -mid.lon);

    // Destination
    let dest = destination(&p1, 100000.0, 90.0); // 100km east
    println!("   100km east of São Paulo: {:.2}°S, {:.2}°W", -dest.lat, -dest.lon);

    // Polygon area
    let square = vec![
        GeoCoord::new(0.0, 0.0),
        GeoCoord::new(0.0, 1.0),
        GeoCoord::new(1.0, 1.0),
        GeoCoord::new(1.0, 0.0),
    ];

    use avila_geo::calc::{shoelace_area, spherical_area};

    let planar_area = shoelace_area(&square);
    let spherical = spherical_area(&square);

    println!("\n   1°x1° square area:");
    println!("     - Planar (shoelace): {:.2} deg²", planar_area);
    println!("     - Spherical: {:.2} km²", spherical / 1_000_000.0);
}

fn main() {
    println!("🎨 Custom Projection Examples");
    println!("================================");

    let (land, cities) = create_sample_map();

    // Render with different projections
    render_with_projection(&Equirectangular::new(), "Equirectangular", &land, &cities);
    render_with_projection(&Mercator::new(), "Mercator", &land, &cities);
    render_with_projection(&WebMercator::new(), "Web Mercator", &land, &cities);
    render_with_projection(
        &LambertConformalConic::usa(),
        "Lambert Conformal",
        &land,
        &cities,
    );
    render_with_projection(&SinusoidalProjection, "Sinusoidal", &land, &cities);

    // Compare distance calculations
    compare_distances();

    // Demonstrate other calculations
    demonstrate_calculations();

    println!("\n✨ All projections rendered successfully!");
    println!("   Check the .ppm files for visual comparison.");
}
