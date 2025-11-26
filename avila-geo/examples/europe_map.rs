//! Europe map example
//!
//! Renders a map of Europe with countries and major cities

use avila_geo::{
    coords::{GeoCoord, GeoBounds},
    geometry::{GeoCollection, GeoLine, GeoPoint, GeoPolygon, LineType},
    map::{Layer, Map, Style},
    projection::LambertConformalConic,
    render::Color,
};

fn main() {
    println!("🇪🇺 Rendering Europe map...");

    // Create map focused on Europe
    let mut map = Map::new(1600, 1200)
        .with_bounds(GeoBounds::EUROPE)
        .with_background(Color::from_hex(0xE0F7FA)); // Light cyan

    // European countries (simplified)
    let mut countries = GeoCollection::new();

    // France (simplified)
    let france = vec![
        GeoCoord::new(51.0, 2.5),
        GeoCoord::new(43.0, -1.5),
        GeoCoord::new(42.5, 3.0),
        GeoCoord::new(43.0, 7.0),
        GeoCoord::new(49.0, 8.0),
        GeoCoord::new(51.0, 2.5),
    ];
    countries.add_polygon(
        GeoPolygon::new(france)
            .with_property("name", "France")
            .with_property("capital", "Paris"),
    );

    // Germany (simplified)
    let germany = vec![
        GeoCoord::new(55.0, 8.0),
        GeoCoord::new(47.5, 7.5),
        GeoCoord::new(47.5, 13.0),
        GeoCoord::new(54.5, 14.0),
        GeoCoord::new(55.0, 8.0),
    ];
    countries.add_polygon(
        GeoPolygon::new(germany)
            .with_property("name", "Germany")
            .with_property("capital", "Berlin"),
    );

    // Italy (simplified)
    let italy = vec![
        GeoCoord::new(47.0, 7.0),
        GeoCoord::new(45.5, 13.5),
        GeoCoord::new(41.0, 16.0),
        GeoCoord::new(37.0, 15.0),
        GeoCoord::new(38.0, 8.5),
        GeoCoord::new(44.0, 7.5),
        GeoCoord::new(47.0, 7.0),
    ];
    countries.add_polygon(
        GeoPolygon::new(italy)
            .with_property("name", "Italy")
            .with_property("capital", "Rome"),
    );

    // Spain (simplified)
    let spain = vec![
        GeoCoord::new(43.5, -9.0),
        GeoCoord::new(36.0, -6.0),
        GeoCoord::new(36.0, 3.0),
        GeoCoord::new(42.5, 3.0),
        GeoCoord::new(43.5, -9.0),
    ];
    countries.add_polygon(
        GeoPolygon::new(spain)
            .with_property("name", "Spain")
            .with_property("capital", "Madrid"),
    );

    // United Kingdom (simplified)
    let uk = vec![
        GeoCoord::new(50.0, -5.5),
        GeoCoord::new(49.0, 2.0),
        GeoCoord::new(55.0, 2.0),
        GeoCoord::new(58.5, -3.0),
        GeoCoord::new(55.0, -7.0),
        GeoCoord::new(50.0, -5.5),
    ];
    countries.add_polygon(
        GeoPolygon::new(uk)
            .with_property("name", "United Kingdom")
            .with_property("capital", "London"),
    );

    // Poland (simplified)
    let poland = vec![
        GeoCoord::new(54.5, 14.0),
        GeoCoord::new(49.0, 14.5),
        GeoCoord::new(49.0, 24.0),
        GeoCoord::new(54.5, 23.0),
        GeoCoord::new(54.5, 14.0),
    ];
    countries.add_polygon(
        GeoPolygon::new(poland)
            .with_property("name", "Poland")
            .with_property("capital", "Warsaw"),
    );

    // Greece (simplified)
    let greece = vec![
        GeoCoord::new(41.5, 19.5),
        GeoCoord::new(35.0, 23.0),
        GeoCoord::new(35.0, 28.0),
        GeoCoord::new(41.0, 26.5),
        GeoCoord::new(41.5, 19.5),
    ];
    countries.add_polygon(
        GeoPolygon::new(greece)
            .with_property("name", "Greece")
            .with_property("capital", "Athens"),
    );

    let country_style = Style::new()
        .with_fill(Color::from_hex(0xE8F5E9)) // Light green
        .with_stroke(Color::from_hex(0x2E7D32), 2); // Dark green border

    map.add_layer(Layer::new("countries", countries, country_style));

    // Add major European cities
    let mut cities = GeoCollection::new();

    let european_cities = vec![
        (51.51, -0.13, "London", "UK"),
        (48.86, 2.35, "Paris", "France"),
        (52.52, 13.40, "Berlin", "Germany"),
        (40.42, -3.70, "Madrid", "Spain"),
        (41.90, 12.50, "Rome", "Italy"),
        (52.23, 21.01, "Warsaw", "Poland"),
        (50.08, 14.44, "Prague", "Czechia"),
        (59.33, 18.07, "Stockholm", "Sweden"),
        (55.68, 12.57, "Copenhagen", "Denmark"),
        (41.01, 28.98, "Istanbul", "Turkey"),
        (37.98, 23.73, "Athens", "Greece"),
        (59.91, 10.75, "Oslo", "Norway"),
        (60.17, 24.94, "Helsinki", "Finland"),
        (47.50, 19.04, "Budapest", "Hungary"),
        (50.45, 30.52, "Kyiv", "Ukraine"),
        (44.79, 20.45, "Belgrade", "Serbia"),
        (45.46, 9.19, "Milan", "Italy"),
        (55.75, 37.62, "Moscow", "Russia"),
    ];

    for (lat, lon, name, country) in european_cities {
        cities.add_point(
            GeoPoint::with_name(GeoCoord::new(lat, lon), name)
                .with_property("country", country)
                .with_property("type", "capital"),
        );
    }

    let city_style = Style::new()
        .with_fill(Color::from_hex(0xD32F2F)) // Red
        .with_stroke(Color::from_hex(0x000000), 1);

    map.add_layer(Layer::new("cities", cities, city_style));

    // Add major rivers (simplified)
    let mut rivers = GeoCollection::new();

    // Danube River (simplified)
    let danube = vec![
        GeoCoord::new(48.5, 8.5),
        GeoCoord::new(48.0, 12.0),
        GeoCoord::new(47.5, 19.0),
        GeoCoord::new(44.5, 26.0),
        GeoCoord::new(45.0, 29.5),
    ];
    rivers.add_line(GeoLine::new(danube, LineType::River));

    // Rhine River (simplified)
    let rhine = vec![
        GeoCoord::new(47.5, 9.0),
        GeoCoord::new(50.0, 7.5),
        GeoCoord::new(51.5, 6.0),
        GeoCoord::new(52.0, 4.5),
    ];
    rivers.add_line(GeoLine::new(rhine, LineType::River));

    let river_style = Style::new()
        .no_fill()
        .with_stroke(Color::from_hex(0x1976D2), 3); // Blue

    map.add_layer(Layer::new("rivers", rivers, river_style));

    // Render with Lambert Conformal Conic projection (good for Europe)
    println!("📐 Using Lambert Conformal Conic projection...");
    let projection = LambertConformalConic::new(45.0, 55.0, 10.0, 50.0);

    // Save to file
    let output = "europe_map.ppm";
    println!("💾 Saving to {}...", output);

    match map.save_ppm(&projection, output) {
        Ok(_) => println!("✅ Map saved successfully!"),
        Err(e) => eprintln!("❌ Error saving map: {}", e),
    }

    // Calculate some statistics
    println!("\n📊 European Geographic Facts:");

    let london = GeoCoord::new(51.51, -0.13);
    let paris = GeoCoord::new(48.86, 2.35);
    let berlin = GeoCoord::new(52.52, 13.40);
    let rome = GeoCoord::new(41.90, 12.50);

    use avila_geo::calc::haversine_distance;

    let london_paris = haversine_distance(&london, &paris);
    let paris_berlin = haversine_distance(&paris, &berlin);
    let berlin_rome = haversine_distance(&berlin, &rome);

    println!("  - London to Paris: {:.0} km", london_paris / 1000.0);
    println!("  - Paris to Berlin: {:.0} km", paris_berlin / 1000.0);
    println!("  - Berlin to Rome: {:.0} km", berlin_rome / 1000.0);
    println!("  - Europe bounds: {:.2}°N to {:.2}°N, {:.2}°W to {:.2}°E",
        GeoBounds::EUROPE.min_lat,
        GeoBounds::EUROPE.max_lat,
        -GeoBounds::EUROPE.min_lon,
        GeoBounds::EUROPE.max_lon
    );
}
