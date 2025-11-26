//! Example: Comparing different map projections
//!
//! Demonstrates Robinson, Winkel Tripel, Mollweide, UTM, and Stereographic
//! projections with visual comparison.

use avila_geo::prelude::*;

fn main() {
    println!("=== Advanced Projections Comparison ===\n");

    // Test coordinates
    let test_points = vec![
        ("Null Island", GeoCoord::new(0.0, 0.0)),
        ("São Paulo", GeoCoord::new(-23.55, -46.63)),
        ("London", GeoCoord::new(51.51, -0.13)),
        ("Tokyo", GeoCoord::new(35.68, 139.65)),
        ("North Pole", GeoCoord::new(89.0, 0.0)),
        ("South Pole", GeoCoord::new(-89.0, 0.0)),
    ];

    // 1. Robinson Projection
    println!("1. Robinson Projection (National Geographic 1988-1998)");
    println!("   Properties: Compromise, neither conformal nor equal-area");
    let robinson = Robinson::new();
    test_projection(&robinson, &test_points, "Robinson");
    println!();

    // 2. Winkel Tripel
    println!("2. Winkel Tripel (National Geographic since 1998)");
    println!("   Properties: Minimizes distortion in area, direction, and distance");
    let winkel = WinkelTripel::new();
    test_projection(&winkel, &test_points, "Winkel Tripel");
    println!();

    // 3. Mollweide
    println!("3. Mollweide (Homalographic)");
    println!("   Properties: Equal-area, elliptical shape");
    let mollweide = Mollweide::new();
    test_projection(&mollweide, &test_points, "Mollweide");
    println!();

    // 4. UTM
    println!("4. UTM (Universal Transverse Mercator)");
    println!("   Properties: Conformal, 60 zones of 6° each");

    // São Paulo is in UTM zone 23S
    let utm_sp = UTM::from_longitude(-46.63, -23.55);
    println!("   São Paulo zone: {} {}",
        utm_sp.zone,
        if utm_sp.northern { "N" } else { "S" }
    );

    let sao_paulo = GeoCoord::new(-23.55, -46.63);
    let cart = utm_sp.project(&sao_paulo, 800.0, 600.0);
    println!("   Projected: ({:.2}, {:.2})", cart.x, cart.y);
    println!();

    // 5. Stereographic (Polar projections)
    println!("5. Stereographic Projection");
    println!("   Properties: Conformal, azimuthal, excellent for poles");

    let north_stereo = Stereographic::north_pole();
    let south_stereo = Stereographic::south_pole();

    let arctic = GeoCoord::new(85.0, 0.0);
    let antarctic = GeoCoord::new(-85.0, 0.0);

    let arctic_proj = north_stereo.project(&arctic, 800.0, 600.0);
    let antarctic_proj = south_stereo.project(&antarctic, 800.0, 600.0);

    println!("   Arctic (85°N): ({:.2}, {:.2})", arctic_proj.x, arctic_proj.y);
    println!("   Antarctic (85°S): ({:.2}, {:.2})", antarctic_proj.x, antarctic_proj.y);
    println!();

    // World map comparison
    println!("6. Creating world maps with different projections...");

    let projections: Vec<(&str, Box<dyn Projection>)> = vec![
        ("robinson", Box::new(Robinson::new())),
        ("winkel_tripel", Box::new(WinkelTripel::new())),
        ("mollweide", Box::new(Mollweide::new())),
        ("mercator", Box::new(Mercator::new())),
        ("equirectangular", Box::new(Equirectangular::new())),
    ];

    for (name, proj) in projections {
        let mut map = Map::new(1920, 1080)
            .with_bounds(GeoBounds::WORLD)
            .with_background(Color::from_hex(0xE0F7FA));

        // Add simple grid
        let mut grid = GeoCollection::new();

        // Latitude lines every 30°
        for lat in (-60..=60).step_by(30) {
            let mut line_coords = Vec::new();
            for lon in -180..=180 {
                line_coords.push(GeoCoord::new(lat as f64, lon as f64));
            }
            grid.add_line(GeoLine::new(line_coords, LineType::Border));
        }

        // Longitude lines every 30°
        for lon in (-180..=180).step_by(30) {
            let mut line_coords = Vec::new();
            for lat in -90..=90 {
                line_coords.push(GeoCoord::new(lat as f64, lon as f64));
            }
            grid.add_line(GeoLine::new(line_coords, LineType::Border));
        }

        map.add_layer(Layer::new(
            "grid",
            grid,
            Style::new().with_stroke(Color::from_hex(0xCCCCCC), 1),
        ));

        let filename = format!("world_map_{}.ppm", name);
        map.save_ppm(proj.as_ref(), &filename)
            .unwrap_or_else(|_| eprintln!("Failed to save {}", filename));

        println!("   Saved: {}", filename);
    }

    println!("\n=== Advanced projections demonstration complete! ===");
    println!("Generated 5 world maps with different projections");
}

fn test_projection(proj: &dyn Projection, points: &[(&str, GeoCoord)], name: &str) {
    println!("   Testing {} projection:", name);

    for (label, coord) in points {
        let cart = proj.project(coord, 800.0, 600.0);
        let back = proj.unproject(&cart, 800.0, 600.0);

        let error = ((back.lat - coord.lat).powi(2) + (back.lon - coord.lon).powi(2)).sqrt();

        println!("     {:12} ({:6.2}, {:7.2}) -> ({:6.1}, {:6.1}) -> error: {:.4}°",
            label, coord.lat, coord.lon, cart.x, cart.y, error
        );
    }
}
