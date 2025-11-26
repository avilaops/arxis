//! Example: Working with web map tiles
//!
//! Demonstrates the tile system for web mapping applications.
//! Compatible with OpenStreetMap, Google Maps, Mapbox, etc.

use avila_geo::prelude::*;

fn main() {
    println!("=== Web Map Tiles Example ===\n");

    // Create tile system
    let tile_system = TileSystem::new();

    // Example 1: Convert geographic coordinate to tile
    let sao_paulo = GeoCoord::new(-23.55, -46.63);

    println!("São Paulo coordinates: {:.2}, {:.2}", sao_paulo.lat, sao_paulo.lon);

    for zoom in [0, 4, 8, 12, 16] {
        let tile = tile_system.geo_to_tile(&sao_paulo, zoom);
        println!("  Zoom {}: Tile {}/{}/{}", zoom, tile.zoom, tile.x, tile.y);

        // Get tile bounds
        let bounds = tile.bounds();
        println!("    Bounds: ({:.4}, {:.4}) to ({:.4}, {:.4})",
            bounds.min_lat, bounds.min_lon,
            bounds.max_lat, bounds.max_lon
        );
    }

    println!();

    // Example 2: QuadKey (Bing Maps format)
    let tile = TileCoord::new(3, 5, 3);
    let quadkey = tile.to_quadkey();
    println!("Tile {}/{}/{} = QuadKey: {}", tile.zoom, tile.x, tile.y, quadkey);

    let tile2 = TileCoord::from_quadkey(&quadkey).unwrap();
    println!("QuadKey {} = Tile {}/{}/{}", quadkey, tile2.zoom, tile2.x, tile2.y);

    println!();

    // Example 3: Tile hierarchy
    let parent = tile.parent().unwrap();
    println!("Parent of {}: {}", tile, parent);

    let children = parent.children();
    println!("Children of {}:", parent);
    for child in &children {
        println!("  - {}", child);
    }

    println!();

    // Example 4: Get tiles covering a region
    let brazil_bounds = GeoBounds::BRAZIL;

    println!("Tiles covering Brazil at zoom 5:");
    let tiles = tile_system.tiles_in_bounds(&brazil_bounds, 5);
    println!("  Total tiles: {}", tiles.len());
    println!("  First 10 tiles:");
    for tile in tiles.iter().take(10) {
        println!("    - {}", tile);
    }

    println!();

    // Example 5: Optimal zoom level
    let viewport = (1920, 1080);
    let zoom = tile_system.optimal_zoom(&brazil_bounds, viewport.0, viewport.1);
    println!("Optimal zoom for Brazil in {}x{} viewport: {}",
        viewport.0, viewport.1, zoom
    );

    println!();

    // Example 6: Tile URL templates
    println!("Tile URL templates:");

    let osm = TileUrlTemplate::osm();
    let test_tile = TileCoord::new(100, 200, 10);
    println!("  OSM: {}", osm.build(&test_tile, "a"));

    let google = TileUrlTemplate::google("s"); // satellite
    println!("  Google: {}", google.build(&test_tile, "0"));

    println!();

    // Example 7: TMS (inverted Y axis)
    let xyz_tile = TileCoord::new(5, 10, 4);
    let tms_tile = xyz_tile.to_tms();
    println!("XYZ tile {} -> TMS tile {}", xyz_tile, tms_tile);

    println!();

    // Example 8: Calculate which tiles to download for offline use
    println!("Planning offline map download:");
    let city_bounds = GeoBounds::new(-23.7, -23.4, -46.8, -46.4); // São Paulo area

    for zoom in 10..=14 {
        let tiles = tile_system.tiles_in_bounds(&city_bounds, zoom);
        let size_mb = tiles.len() as f64 * 20.0 / 1024.0; // ~20KB per tile
        println!("  Zoom {}: {} tiles (~{:.1} MB)",
            zoom, tiles.len(), size_mb
        );
    }

    println!("\n=== Tile system demonstration complete! ===");
}
