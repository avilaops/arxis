//! Brazil map example
//!
//! Renders a map of Brazil with states and major cities

use avila_geo::{
    coords::{GeoCoord, GeoBounds},
    geometry::{shapes, GeoCollection, GeoLine, GeoPoint, GeoPolygon, LineType},
    map::{Layer, Map, Style},
    projection::{AlbersEqualArea, Equirectangular},
    render::Color,
};

fn main() {
    println!("🇧🇷 Rendering Brazil map...");

    // Create map focused on Brazil
    let mut map = Map::new(1200, 1000)
        .with_bounds(GeoBounds::BRAZIL)
        .with_background(Color::from_hex(0xE0F7FA)); // Light cyan

    // Brazil outline (simplified)
    let mut brazil_outline = GeoCollection::new();

    let brazil_border = vec![
        GeoCoord::new(5.27, -60.0),
        GeoCoord::new(5.27, -50.0),
        GeoCoord::new(-5.0, -35.0),
        GeoCoord::new(-8.0, -35.0),
        GeoCoord::new(-18.0, -39.0),
        GeoCoord::new(-23.0, -45.0),
        GeoCoord::new(-33.75, -53.0),
        GeoCoord::new(-30.0, -57.0),
        GeoCoord::new(-20.0, -58.0),
        GeoCoord::new(-10.0, -60.0),
        GeoCoord::new(-10.0, -70.0),
        GeoCoord::new(0.0, -70.0),
        GeoCoord::new(5.27, -60.0),
    ];

    brazil_outline.add_polygon(
        GeoPolygon::new(brazil_border)
            .with_property("name", "Brazil")
            .with_property("country", "BR"),
    );

    let brazil_style = Style::new()
        .with_fill(Color::from_hex(0xFFD700)) // Gold
        .with_stroke(Color::from_hex(0x009739), 3); // Brazilian green

    map.add_layer(Layer::new("brazil", brazil_outline, brazil_style));

    // Add some state divisions (simplified)
    let mut states = GeoCollection::new();

    // São Paulo state (simplified)
    let sp = vec![
        GeoCoord::new(-20.0, -44.0),
        GeoCoord::new(-20.0, -53.0),
        GeoCoord::new(-25.0, -53.0),
        GeoCoord::new(-25.0, -44.0),
        GeoCoord::new(-20.0, -44.0),
    ];
    states.add_polygon(GeoPolygon::new(sp).with_property("state", "SP"));

    // Rio de Janeiro state (simplified)
    let rj = vec![
        GeoCoord::new(-20.0, -41.0),
        GeoCoord::new(-20.0, -45.0),
        GeoCoord::new(-23.5, -45.0),
        GeoCoord::new(-23.5, -41.0),
        GeoCoord::new(-20.0, -41.0),
    ];
    states.add_polygon(GeoPolygon::new(rj).with_property("state", "RJ"));

    let state_style = Style::new()
        .no_fill()
        .with_stroke(Color::from_hex(0x555555), 1); // Gray borders

    map.add_layer(Layer::new("states", states, state_style));

    // Add major Brazilian cities
    let mut cities = GeoCollection::new();

    let brazilian_cities = vec![
        (-15.83, -47.86, "Brasília", "Capital"),
        (-23.55, -46.63, "São Paulo", "Largest city"),
        (-22.91, -43.17, "Rio de Janeiro", "Second largest"),
        (-12.97, -38.51, "Salvador", "Northeast capital"),
        (-30.03, -51.23, "Porto Alegre", "Southern capital"),
        (-25.43, -49.27, "Curitiba", "Southern capital"),
        (-8.05, -34.90, "Recife", "Northeast capital"),
        (-3.72, -38.53, "Fortaleza", "Northeast capital"),
        (-19.92, -43.94, "Belo Horizonte", "Southeast capital"),
        (-1.46, -48.50, "Belém", "Northern capital"),
    ];

    for (lat, lon, name, description) in brazilian_cities {
        cities.add_point(
            GeoPoint::with_name(GeoCoord::new(lat, lon), name)
                .with_property("type", description),
        );
    }

    let city_style = Style::new()
        .with_fill(Color::from_hex(0xFF0000)) // Red
        .with_stroke(Color::from_hex(0x000000), 1)
        .with_stroke(Color::BLACK, 1);

    // Different style for capital
    let capital_style = Style::new()
        .with_fill(Color::from_hex(0x0000FF)) // Blue
        .with_stroke(Color::from_hex(0xFFFFFF), 2);

    map.add_layer(Layer::new("cities", cities, city_style));

    // Add Amazon River (simplified)
    let mut rivers = GeoCollection::new();
    let amazon = vec![
        GeoCoord::new(-3.0, -73.0),
        GeoCoord::new(-3.5, -65.0),
        GeoCoord::new(-3.0, -58.0),
        GeoCoord::new(-2.0, -52.0),
        GeoCoord::new(-1.5, -48.0),
    ];
    rivers.add_line(GeoLine::new(amazon, LineType::River));

    let river_style = Style::new()
        .no_fill()
        .with_stroke(Color::from_hex(0x4682B4), 3); // Steel blue

    map.add_layer(Layer::new("rivers", rivers, river_style));

    // Render with Albers Equal Area projection (good for Brazil)
    println!("📐 Using Albers Equal Area projection...");
    let projection = AlbersEqualArea::brazil();

    // Save to file
    let output = "brazil_map.ppm";
    println!("💾 Saving to {}...", output);

    match map.save_ppm(&projection, output) {
        Ok(_) => println!("✅ Map saved successfully!"),
        Err(e) => eprintln!("❌ Error saving map: {}", e),
    }

    // Calculate some statistics
    println!("\n📊 Brazil Geographic Facts:");

    let brasilia = GeoCoord::new(-15.83, -47.86);
    let sao_paulo = GeoCoord::new(-23.55, -46.63);
    let rio = GeoCoord::new(-22.91, -43.17);

    use avila_geo::calc::haversine_distance;

    let distance_bsb_sp = haversine_distance(&brasilia, &sao_paulo);
    let distance_sp_rio = haversine_distance(&sao_paulo, &rio);

    println!("  - Brasília to São Paulo: {:.0} km", distance_bsb_sp / 1000.0);
    println!("  - São Paulo to Rio: {:.0} km", distance_sp_rio / 1000.0);
    println!("  - Brazil bounds: {:.2}°N to {:.2}°S, {:.2}°W to {:.2}°E",
        GeoBounds::BRAZIL.max_lat,
        -GeoBounds::BRAZIL.min_lat,
        -GeoBounds::BRAZIL.max_lon,
        -GeoBounds::BRAZIL.min_lon
    );
}
