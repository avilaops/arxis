//! Dubai and Gulf region map example
//!
//! Renders a map of Dubai, UAE, and the Arabian Gulf region

use avila_geo::{
    coords::{GeoCoord, GeoBounds},
    geometry::{shapes, GeoCollection, GeoLine, GeoPoint, GeoPolygon, LineType},
    map::{Layer, Map, Style},
    projection::Mercator,
    render::Color,
};

fn main() {
    println!("🏜️ Rendering Dubai & Gulf Region map...");

    // Create map focused on Gulf region
    let mut map = Map::new(1400, 1000)
        .with_bounds(GeoBounds::GULF_REGION)
        .with_background(Color::from_hex(0xE1F5FE)); // Light blue (Gulf waters)

    // Gulf countries (simplified)
    let mut countries = GeoCollection::new();

    // UAE (simplified)
    let uae = vec![
        GeoCoord::new(26.0, 51.5),
        GeoCoord::new(22.5, 51.5),
        GeoCoord::new(22.5, 56.5),
        GeoCoord::new(25.0, 56.5),
        GeoCoord::new(26.0, 56.0),
        GeoCoord::new(26.0, 51.5),
    ];
    countries.add_polygon(
        GeoPolygon::new(uae)
            .with_property("name", "United Arab Emirates")
            .with_property("capital", "Abu Dhabi"),
    );

    // Saudi Arabia (eastern part, simplified)
    let saudi_arabia = vec![
        GeoCoord::new(30.5, 47.0),
        GeoCoord::new(22.0, 47.0),
        GeoCoord::new(22.0, 52.0),
        GeoCoord::new(26.0, 51.5),
        GeoCoord::new(28.0, 49.0),
        GeoCoord::new(30.5, 47.0),
    ];
    countries.add_polygon(
        GeoPolygon::new(saudi_arabia)
            .with_property("name", "Saudi Arabia")
            .with_property("capital", "Riyadh"),
    );

    // Qatar (simplified)
    let qatar = vec![
        GeoCoord::new(26.2, 50.7),
        GeoCoord::new(24.5, 50.7),
        GeoCoord::new(24.5, 51.6),
        GeoCoord::new(26.2, 51.6),
        GeoCoord::new(26.2, 50.7),
    ];
    countries.add_polygon(
        GeoPolygon::new(qatar)
            .with_property("name", "Qatar")
            .with_property("capital", "Doha"),
    );

    // Bahrain (simplified, very small)
    let bahrain = vec![
        GeoCoord::new(26.3, 50.4),
        GeoCoord::new(25.8, 50.4),
        GeoCoord::new(25.8, 50.8),
        GeoCoord::new(26.3, 50.8),
        GeoCoord::new(26.3, 50.4),
    ];
    countries.add_polygon(
        GeoPolygon::new(bahrain)
            .with_property("name", "Bahrain")
            .with_property("capital", "Manama"),
    );

    // Kuwait (simplified)
    let kuwait = vec![
        GeoCoord::new(30.1, 47.5),
        GeoCoord::new(28.5, 47.5),
        GeoCoord::new(28.5, 48.5),
        GeoCoord::new(30.1, 48.5),
        GeoCoord::new(30.1, 47.5),
    ];
    countries.add_polygon(
        GeoPolygon::new(kuwait)
            .with_property("name", "Kuwait")
            .with_property("capital", "Kuwait City"),
    );

    // Oman (northern part, simplified)
    let oman = vec![
        GeoCoord::new(26.5, 56.0),
        GeoCoord::new(23.0, 56.5),
        GeoCoord::new(22.0, 59.0),
        GeoCoord::new(23.5, 59.5),
        GeoCoord::new(26.5, 57.0),
        GeoCoord::new(26.5, 56.0),
    ];
    countries.add_polygon(
        GeoPolygon::new(oman)
            .with_property("name", "Oman")
            .with_property("capital", "Muscat"),
    );

    let country_style = Style::new()
        .with_fill(Color::from_hex(0xFFF9C4)) // Light yellow (desert)
        .with_stroke(Color::from_hex(0xF57C00), 2); // Orange border

    map.add_layer(Layer::new("countries", countries, country_style));

    // Major cities in the Gulf
    let mut cities = GeoCollection::new();

    let gulf_cities = vec![
        // UAE
        (25.20, 55.27, "Dubai", "UAE - Largest city"),
        (24.47, 54.37, "Abu Dhabi", "UAE - Capital"),
        (25.34, 55.42, "Sharjah", "UAE - 3rd largest"),
        (25.43, 56.18, "Ras Al Khaimah", "UAE - Northern emirate"),
        (24.37, 54.50, "Al Ain", "UAE - Oasis city"),

        // Other Gulf capitals
        (25.28, 51.53, "Doha", "Qatar - Capital"),
        (26.23, 50.59, "Manama", "Bahrain - Capital"),
        (29.31, 47.48, "Kuwait City", "Kuwait - Capital"),
        (24.71, 46.67, "Riyadh", "Saudi Arabia - Capital"),
        (26.24, 50.57, "Dammam", "Saudi Arabia - Eastern"),
        (23.61, 58.59, "Muscat", "Oman - Capital"),
    ];

    for (lat, lon, name, description) in gulf_cities {
        cities.add_point(
            GeoPoint::with_name(GeoCoord::new(lat, lon), name)
                .with_property("description", description),
        );
    }

    let city_style = Style::new()
        .with_fill(Color::from_hex(0xE65100)) // Deep orange
        .with_stroke(Color::from_hex(0x000000), 1);

    map.add_layer(Layer::new("cities", cities, city_style));

    // Add Dubai landmarks (zoomed in detail)
    let mut landmarks = GeoCollection::new();

    let dubai_landmarks = vec![
        (25.197, 55.274, "Burj Khalifa"),
        (25.204, 55.270, "Dubai Mall"),
        (25.141, 55.185, "Palm Jumeirah"),
        (25.080, 55.139, "Burj Al Arab"),
        (25.252, 55.365, "Dubai International Airport"),
    ];

    for (lat, lon, name) in dubai_landmarks {
        landmarks.add_point(
            GeoPoint::with_name(GeoCoord::new(lat, lon), name)
                .with_property("type", "landmark"),
        );
    }

    let landmark_style = Style::new()
        .with_fill(Color::from_hex(0x0277BD)) // Blue
        .with_stroke(Color::from_hex(0xFFFFFF), 2);

    map.add_layer(Layer::new("landmarks", landmarks, landmark_style));

    // Add coastline highlights
    let mut coastlines = GeoCollection::new();

    // UAE coastline (simplified)
    let uae_coast = vec![
        GeoCoord::new(26.0, 56.0),
        GeoCoord::new(25.5, 55.5),
        GeoCoord::new(25.0, 55.0),
        GeoCoord::new(24.5, 54.5),
        GeoCoord::new(24.0, 54.0),
        GeoCoord::new(23.5, 53.5),
    ];
    coastlines.add_line(GeoLine::new(uae_coast, LineType::Coastline));

    let coastline_style = Style::new()
        .no_fill()
        .with_stroke(Color::from_hex(0x01579B), 2); // Dark blue

    map.add_layer(Layer::new("coastlines", coastlines, coastline_style));

    // Render with Mercator projection
    println!("📐 Using Mercator projection...");
    let projection = Mercator::new();

    // Save to file
    let output = "dubai_gulf_map.ppm";
    println!("💾 Saving to {}...", output);

    match map.save_ppm(&projection, output) {
        Ok(_) => println!("✅ Map saved successfully!"),
        Err(e) => eprintln!("❌ Error saving map: {}", e),
    }

    // Calculate some statistics
    println!("\n📊 Gulf Region Geographic Facts:");

    let dubai = GeoCoord::new(25.20, 55.27);
    let abu_dhabi = GeoCoord::new(24.47, 54.37);
    let doha = GeoCoord::new(25.28, 51.53);
    let riyadh = GeoCoord::new(24.71, 46.67);
    let muscat = GeoCoord::new(23.61, 58.59);

    use avila_geo::calc::{bearing, haversine_distance};

    let dubai_abu_dhabi = haversine_distance(&dubai, &abu_dhabi);
    let dubai_doha = haversine_distance(&dubai, &doha);
    let dubai_muscat = haversine_distance(&dubai, &muscat);
    let dubai_riyadh = haversine_distance(&dubai, &riyadh);

    println!("  - Dubai to Abu Dhabi: {:.0} km", dubai_abu_dhabi / 1000.0);
    println!("  - Dubai to Doha: {:.0} km", dubai_doha / 1000.0);
    println!("  - Dubai to Muscat: {:.0} km", dubai_muscat / 1000.0);
    println!("  - Dubai to Riyadh: {:.0} km", dubai_riyadh / 1000.0);

    let bearing_to_doha = bearing(&dubai, &doha);
    println!("\n  - Bearing from Dubai to Doha: {:.1}°", bearing_to_doha);

    println!("\n  - Dubai coordinates: {:.2}°N, {:.2}°E", dubai.lat, dubai.lon);
    println!(
        "  - Gulf region bounds: {:.2}°N to {:.2}°N, {:.2}°E to {:.2}°E",
        GeoBounds::GULF_REGION.min_lat,
        GeoBounds::GULF_REGION.max_lat,
        GeoBounds::GULF_REGION.min_lon,
        GeoBounds::GULF_REGION.max_lon
    );

    println!("\n🌍 Fun Fact: Dubai is at similar latitude to southern Brazil!");
    println!("  Dubai: {:.2}°N ≈ São Paulo: {:.2}°S (mirrored)", dubai.lat, -(-23.55));
}
