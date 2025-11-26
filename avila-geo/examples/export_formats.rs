//! Example: Export maps to different formats
//!
//! Demonstrates SVG, PNG, and JPEG export capabilities.

use avila_geo::prelude::*;

fn main() {
    println!("=== Map Export Formats Example ===\n");

    // Create a sample map
    println!("Creating sample map of South America...");

    let mut map = Map::new(1600, 1200)
        .with_bounds(GeoBounds::new(-56.0, 12.0, -82.0, -34.0))
        .with_background(Color::from_hex(0xADD8E6)); // Light blue

    // Add some cities
    let mut cities = GeoCollection::new();

    cities.add_point(GeoPoint::with_name(GeoCoord::new(-23.55, -46.63), "São Paulo"));
    cities.add_point(GeoPoint::with_name(GeoCoord::new(-22.91, -43.17), "Rio de Janeiro"));
    cities.add_point(GeoPoint::with_name(GeoCoord::new(-15.78, -47.93), "Brasília"));
    cities.add_point(GeoPoint::with_name(GeoCoord::new(-12.97, -38.51), "Salvador"));
    cities.add_point(GeoPoint::with_name(GeoCoord::new(-34.60, -58.38), "Buenos Aires"));
    cities.add_point(GeoPoint::with_name(GeoCoord::new(-33.45, -70.67), "Santiago"));
    cities.add_point(GeoPoint::with_name(GeoCoord::new(-12.05, -77.03), "Lima"));
    cities.add_point(GeoPoint::with_name(GeoCoord::new(4.71, -74.07), "Bogotá"));

    map.add_layer(Layer::new(
        "cities",
        cities,
        Style::city(),
    ));

    // Add coastlines (simplified)
    let mut coast = GeoCollection::new();

    let coastline = vec![
        GeoCoord::new(12.0, -72.0),
        GeoCoord::new(10.0, -75.0),
        GeoCoord::new(0.0, -80.0),
        GeoCoord::new(-5.0, -81.0),
        GeoCoord::new(-18.0, -70.0),
        GeoCoord::new(-30.0, -71.0),
        GeoCoord::new(-40.0, -73.0),
        GeoCoord::new(-55.0, -68.0),
        GeoCoord::new(-55.0, -65.0),
        GeoCoord::new(-38.0, -57.0),
        GeoCoord::new(-23.0, -43.0),
        GeoCoord::new(-5.0, -35.0),
        GeoCoord::new(5.0, -50.0),
        GeoCoord::new(12.0, -60.0),
    ];

    coast.add_line(GeoLine::new(coastline, LineType::Coast));

    map.add_layer(Layer::new(
        "coast",
        coast,
        Style::new().with_stroke(Color::from_hex(0x2E7D32), 2),
    ));

    let projection = Mercator::new();

    // Export to different formats
    println!("\nExporting to different formats:");

    // 1. PPM (native format)
    println!("  1. PPM (native)...");
    map.save_ppm(&projection, "south_america.ppm")
        .unwrap_or_else(|e| eprintln!("     Error: {}", e));
    println!("     ✓ Saved south_america.ppm");

    // 2. SVG (vector format)
    #[cfg(feature = "export-svg")]
    {
        println!("  2. SVG (vector)...");
        map.save_svg(&projection, "south_america.svg")
            .unwrap_or_else(|e| eprintln!("     Error: {}", e));
        println!("     ✓ Saved south_america.svg");
        println!("     - Resolution independent");
        println!("     - Editable in Inkscape/Illustrator");
        println!("     - Small file size");
    }
    #[cfg(not(feature = "export-svg"))]
    {
        println!("  2. SVG - Not available (enable 'export-svg' feature)");
    }

    // 3. PNG (raster format)
    #[cfg(feature = "export-png")]
    {
        println!("  3. PNG (raster)...");
        map.save_png(&projection, "south_america.png")
            .unwrap_or_else(|e| eprintln!("     Error: {}", e));
        println!("     ✓ Saved south_america.png");
        println!("     - Lossless compression");
        println!("     - Web-friendly");
        println!("     - Transparency support");
    }
    #[cfg(not(feature = "export-png"))]
    {
        println!("  3. PNG - Not available (enable 'export-png' feature)");
    }

    // 4. JPEG (compressed format)
    #[cfg(feature = "export-png")]
    {
        println!("  4. JPEG (compressed)...");

        // High quality
        map.save_jpeg(&projection, "south_america_hq.jpg", 95)
            .unwrap_or_else(|e| eprintln!("     Error: {}", e));
        println!("     ✓ Saved south_america_hq.jpg (quality: 95)");

        // Medium quality
        map.save_jpeg(&projection, "south_america_mq.jpg", 75)
            .unwrap_or_else(|e| eprintln!("     Error: {}", e));
        println!("     ✓ Saved south_america_mq.jpg (quality: 75)");

        // Low quality
        map.save_jpeg(&projection, "south_america_lq.jpg", 50)
            .unwrap_or_else(|e| eprintln!("     Error: {}", e));
        println!("     ✓ Saved south_america_lq.jpg (quality: 50)");

        println!("     - Lossy compression");
        println!("     - Smaller file sizes");
        println!("     - Good for photos");
    }
    #[cfg(not(feature = "export-png"))]
    {
        println!("  4. JPEG - Not available (enable 'export-png' feature)");
    }

    // 5. In-memory bytes (for web servers)
    #[cfg(feature = "export-png")]
    {
        println!("  5. In-memory export (for web apps)...");

        let png_bytes = map.to_png_bytes(&projection)
            .unwrap_or_else(|e| {
                eprintln!("     Error: {}", e);
                Vec::new()
            });
        println!("     ✓ PNG bytes: {} KB", png_bytes.len() / 1024);

        let jpeg_bytes = map.to_jpeg_bytes(&projection, 85)
            .unwrap_or_else(|e| {
                eprintln!("     Error: {}", e);
                Vec::new()
            });
        println!("     ✓ JPEG bytes: {} KB", jpeg_bytes.len() / 1024);
        println!("     - Perfect for HTTP responses");
        println!("     - No disk I/O");
    }

    println!("\n=== Format Comparison ===");
    println!("  PPM:  Uncompressed, large, simple format");
    println!("  SVG:  Vector, scalable, small, editable");
    println!("  PNG:  Lossless, good for graphics, transparent");
    println!("  JPEG: Lossy, smallest size, good for photos");

    println!("\n=== Use Cases ===");
    println!("  Web display:      PNG or JPEG");
    println!("  Print/poster:     SVG or high-res PNG");
    println!("  Further editing:  SVG");
    println!("  Photo-realistic:  JPEG");
    println!("  Transparency:     PNG or SVG");
    println!("  Email/bandwidth:  JPEG (low quality)");

    println!("\n=== Export demonstration complete! ===");
}
