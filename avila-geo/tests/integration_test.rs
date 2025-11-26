//! Integration tests for avila-geo

use avila_geo::prelude::*;

#[test]
fn test_basic_coordinate_creation() {
    let coord = GeoCoord::new(-23.55, -46.63);
    assert_eq!(coord.lat, -23.55);
    assert_eq!(coord.lon, -46.63);
}

#[test]
fn test_haversine_distance() {
    let sp = GeoCoord::new(-23.55, -46.63);
    let rio = GeoCoord::new(-22.91, -43.17);

    let distance = avila_geo::calc::haversine_distance(&sp, &rio);

    // Distance should be approximately 360 km
    assert!(distance > 300_000.0);
    assert!(distance < 400_000.0);
}

#[test]
fn test_point_in_polygon() {
    let polygon = vec![
        GeoCoord::new(0.0, 0.0),
        GeoCoord::new(0.0, 10.0),
        GeoCoord::new(10.0, 10.0),
        GeoCoord::new(10.0, 0.0),
    ];

    let inside = GeoCoord::new(5.0, 5.0);
    let outside = GeoCoord::new(15.0, 15.0);

    assert!(avila_geo::calc::point_in_polygon(&inside, &polygon));
    assert!(!avila_geo::calc::point_in_polygon(&outside, &polygon));
}

#[test]
fn test_projection_roundtrip() {
    let coord = GeoCoord::new(-23.55, -46.63);
    let proj = Equirectangular::new();

    let cart = proj.project(&coord, 360.0, 180.0);
    let back = proj.unproject(&cart, 360.0, 180.0);

    assert!((back.lat - coord.lat).abs() < 0.01);
    assert!((back.lon - coord.lon).abs() < 0.01);
}

#[test]
fn test_map_rendering() {
    let mut map = Map::new(100, 100);

    let mut collection = GeoCollection::new();
    collection.add_point(GeoPoint::with_name(
        GeoCoord::new(0.0, 0.0),
        "Test Point"
    ));

    let layer = Layer::new("test", collection, Style::default());
    map.add_layer(layer);

    let proj = Equirectangular::new();
    let fb = map.render(&proj);

    assert_eq!(fb.width, 100);
    assert_eq!(fb.height, 100);
}

#[test]
fn test_polygon_area() {
    // 1 degree square at equator
    let square = vec![
        GeoCoord::new(0.0, 0.0),
        GeoCoord::new(0.0, 1.0),
        GeoCoord::new(1.0, 1.0),
        GeoCoord::new(1.0, 0.0),
    ];

    let area = avila_geo::calc::shoelace_area(&square);
    assert!((area - 1.0).abs() < 0.01);
}

#[test]
fn test_line_simplification() {
    let line = vec![
        GeoCoord::new(0.0, 0.0),
        GeoCoord::new(0.5, 0.01),
        GeoCoord::new(1.0, 0.0),
        GeoCoord::new(1.5, 0.01),
        GeoCoord::new(2.0, 0.0),
    ];

    let simplified = avila_geo::calc::douglas_peucker(&line, 0.1);
    assert!(simplified.len() < line.len());
    assert!(simplified.len() >= 2);
}

#[test]
fn test_bearing_calculation() {
    let north = GeoCoord::new(0.0, 0.0);
    let south = GeoCoord::new(-1.0, 0.0);

    let bearing = avila_geo::calc::bearing(&north, &south);
    // Should be approximately 180° (south)
    assert!((bearing - 180.0).abs() < 5.0);
}

#[cfg(feature = "geojson")]
#[test]
fn test_geojson_parsing() {
    let json = r#"{
        "type": "FeatureCollection",
        "features": [{
            "type": "Feature",
            "geometry": {
                "type": "Point",
                "coordinates": [-46.63, -23.55]
            },
            "properties": {
                "name": "São Paulo"
            }
        }]
    }"#;

    let geojson = avila_geo::GeoJson::from_str(json).unwrap();
    let collection = geojson.to_collection();

    assert_eq!(collection.points.len(), 1);
    assert_eq!(collection.points[0].coord.lat, -23.55);
}

#[test]
fn test_europe_bounds() {
    let london = GeoCoord::new(51.51, -0.13);
    let paris = GeoCoord::new(48.86, 2.35);
    let athens = GeoCoord::new(37.98, 23.73);

    assert!(GeoBounds::EUROPE.contains(&london));
    assert!(GeoBounds::EUROPE.contains(&paris));
    assert!(GeoBounds::EUROPE.contains(&athens));
}

#[test]
fn test_dubai_bounds() {
    let dubai = GeoCoord::new(25.20, 55.27);
    let abu_dhabi = GeoCoord::new(24.47, 54.37);

    assert!(GeoBounds::DUBAI.contains(&dubai));
    assert!(GeoBounds::DUBAI.contains(&abu_dhabi));
    assert!(GeoBounds::MIDDLE_EAST.contains(&dubai));
    assert!(GeoBounds::GULF_REGION.contains(&dubai));
}

#[test]
fn test_middle_east_distances() {
    let dubai = GeoCoord::new(25.20, 55.27);
    let doha = GeoCoord::new(25.28, 51.53);

    let distance = avila_geo::calc::haversine_distance(&dubai, &doha);

    // Distance should be approximately 380 km
    assert!(distance > 350_000.0);
    assert!(distance < 400_000.0);
}

#[test]
fn test_europe_distances() {
    let london = GeoCoord::new(51.51, -0.13);
    let paris = GeoCoord::new(48.86, 2.35);

    let distance = avila_geo::calc::haversine_distance(&london, &paris);

    // Distance should be approximately 340 km
    assert!(distance > 300_000.0);
    assert!(distance < 380_000.0);
}

#[test]
fn test_map_builder_regions() {
    let map_europe = MapBuilder::europe(1600, 1200);
    assert_eq!(map_europe.bounds, Some(GeoBounds::EUROPE));

    let map_dubai = MapBuilder::dubai(1400, 1000);
    assert_eq!(map_dubai.bounds, Some(GeoBounds::DUBAI));

    let map_gulf = MapBuilder::gulf_region(1400, 1000);
    assert_eq!(map_gulf.bounds, Some(GeoBounds::GULF_REGION));
}
