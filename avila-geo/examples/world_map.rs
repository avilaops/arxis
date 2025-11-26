//! World map example
//!
//! Renders a simple world map with country borders

use avila_geo::{
    coords::{GeoCoord, GeoBounds},
    geometry::{shapes, GeoCollection, GeoLine, GeoPolygon, LineType},
    map::{Layer, Map, Style},
    projection::Mercator,
    render::Color,
};

fn main() {
    println!("🌍 Rendering world map...");

    // Create map
    let mut map = Map::new(1920, 1080)
        .with_bounds(GeoBounds::WORLD)
        .with_background(Color::from_hex(0xADD8E6)); // Light blue ocean

    // Create continents (simplified shapes)
    let mut continents = GeoCollection::new();

    // South America (approximation)
    let south_america = vec![
        GeoCoord::new(12.0, -81.0),
        GeoCoord::new(-5.0, -75.0),
        GeoCoord::new(-20.0, -70.0),
        GeoCoord::new(-35.0, -65.0),
        GeoCoord::new(-55.0, -68.0),
        GeoCoord::new(-55.0, -75.0),
        GeoCoord::new(-35.0, -75.0),
        GeoCoord::new(-20.0, -80.0),
        GeoCoord::new(-5.0, -82.0),
        GeoCoord::new(12.0, -81.0),
    ];
    continents.add_polygon(
        GeoPolygon::new(south_america)
            .with_property("name", "South America")
            .with_property("continent", "Americas"),
    );

    // North America (simplified)
    let north_america = vec![
        GeoCoord::new(25.0, -80.0),
        GeoCoord::new(15.0, -90.0),
        GeoCoord::new(15.0, -110.0),
        GeoCoord::new(30.0, -120.0),
        GeoCoord::new(50.0, -130.0),
        GeoCoord::new(70.0, -140.0),
        GeoCoord::new(70.0, -60.0),
        GeoCoord::new(45.0, -60.0),
        GeoCoord::new(25.0, -80.0),
    ];
    continents.add_polygon(GeoPolygon::new(north_america).with_property("name", "North America"));

    // Africa (simplified)
    let africa = vec![
        GeoCoord::new(37.0, -10.0),
        GeoCoord::new(15.0, -17.0),
        GeoCoord::new(-35.0, 20.0),
        GeoCoord::new(-35.0, 52.0),
        GeoCoord::new(12.0, 52.0),
        GeoCoord::new(37.0, 32.0),
        GeoCoord::new(37.0, -10.0),
    ];
    continents.add_polygon(GeoPolygon::new(africa).with_property("name", "Africa"));

    // Europe (simplified)
    let europe = vec![
        GeoCoord::new(36.0, -10.0),
        GeoCoord::new(71.0, 25.0),
        GeoCoord::new(71.0, 60.0),
        GeoCoord::new(36.0, 40.0),
        GeoCoord::new(36.0, -10.0),
    ];
    continents.add_polygon(GeoPolygon::new(europe).with_property("name", "Europe"));

    // Asia (simplified)
    let asia = vec![
        GeoCoord::new(70.0, 60.0),
        GeoCoord::new(70.0, 170.0),
        GeoCoord::new(50.0, 140.0),
        GeoCoord::new(-10.0, 140.0),
        GeoCoord::new(-10.0, 95.0),
        GeoCoord::new(10.0, 60.0),
        GeoCoord::new(40.0, 60.0),
        GeoCoord::new(70.0, 60.0),
    ];
    continents.add_polygon(GeoPolygon::new(asia).with_property("name", "Asia"));

    // Australia (simplified)
    let australia = vec![
        GeoCoord::new(-10.0, 113.0),
        GeoCoord::new(-10.0, 154.0),
        GeoCoord::new(-39.0, 150.0),
        GeoCoord::new(-39.0, 113.0),
        GeoCoord::new(-10.0, 113.0),
    ];
    continents.add_polygon(GeoPolygon::new(australia).with_property("name", "Australia"));

    // Add continents layer
    let land_style = Style::new()
        .with_fill(Color::from_hex(0x90EE90)) // Light green
        .with_stroke(Color::from_hex(0x228B22), 2); // Forest green border

    map.add_layer(Layer::new("continents", continents, land_style));

    // Add equator line
    let mut lines = GeoCollection::new();
    let equator = vec![
        GeoCoord::new(0.0, -180.0),
        GeoCoord::new(0.0, 180.0),
    ];
    lines.add_line(GeoLine::new(equator, LineType::Custom));

    let line_style = Style::new()
        .no_fill()
        .with_stroke(Color::from_hex(0xFF0000), 2); // Red equator

    map.add_layer(Layer::new("equator", lines, line_style));

    // Add major cities
    let mut cities = GeoCollection::new();
    let major_cities = vec![
        (-23.55, -46.63, "São Paulo"),
        (-22.91, -43.17, "Rio de Janeiro"),
        (40.71, -74.01, "New York"),
        (51.51, -0.13, "London"),
        (48.86, 2.35, "Paris"),
        (35.68, 139.65, "Tokyo"),
        (-33.87, 151.21, "Sydney"),
        (1.35, 103.82, "Singapore"),
    ];

    for (lat, lon, name) in major_cities {
        cities.add_point(
            avila_geo::geometry::GeoPoint::with_name(GeoCoord::new(lat, lon), name),
        );
    }

    let city_style = Style::city();
    map.add_layer(Layer::new("cities", cities, city_style));

    // Render with Mercator projection
    println!("📐 Using Mercator projection...");
    let projection = Mercator::new();

    // Save to file
    let output = "world_map.ppm";
    println!("💾 Saving to {}...", output);

    match map.save_ppm(&projection, output) {
        Ok(_) => println!("✅ Map saved successfully!"),
        Err(e) => eprintln!("❌ Error saving map: {}", e),
    }

    println!("📊 Map stats:");
    println!("  - Size: {}x{} pixels", map.width, map.height);
    println!("  - Layers: {}", map.layers.len());
    println!("  - Projection: Mercator");
    println!("  - Bounds: World (-180° to 180°, -85° to 85°)");
}
