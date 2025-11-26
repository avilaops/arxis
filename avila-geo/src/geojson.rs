//! GeoJSON parser and serializer
//!
//! Implements reading and writing of GeoJSON format (RFC 7946)

#[cfg(feature = "geojson")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "geojson")]
use crate::{
    coords::GeoCoord,
    geometry::{GeoCollection, GeoLine, GeoPoint, GeoPolygon, LineType},
};

#[cfg(feature = "geojson")]
use std::collections::HashMap;

#[cfg(feature = "geojson")]
#[derive(Debug, Deserialize, Serialize)]
pub struct GeoJson {
    #[serde(rename = "type")]
    pub geojson_type: String,
    pub features: Vec<Feature>,
}

#[cfg(feature = "geojson")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Feature {
    #[serde(rename = "type")]
    pub feature_type: String,
    pub geometry: Geometry,
    #[serde(default)]
    pub properties: serde_json::Value,
}

#[cfg(feature = "geojson")]
#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: String,
    pub coordinates: serde_json::Value,
}

#[cfg(feature = "geojson")]
impl GeoJson {
    /// Parse GeoJSON from string
    pub fn from_str(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Convert to GeoCollection
    pub fn to_collection(&self) -> GeoCollection {
        let mut collection = GeoCollection::new();

        for feature in &self.features {
            match feature.geometry.geometry_type.as_str() {
                "Point" => {
                    if let Some(point) = parse_point(&feature.geometry.coordinates) {
                        let mut geo_point = GeoPoint::new(point);
                        if let Some(props) = parse_properties(&feature.properties) {
                            geo_point.properties = props;
                        }
                        collection.add_point(geo_point);
                    }
                }
                "LineString" => {
                    if let Some(coords) = parse_line_string(&feature.geometry.coordinates) {
                        let mut line = GeoLine::new(coords, LineType::Custom);
                        if let Some(props) = parse_properties(&feature.properties) {
                            line.properties = props;
                        }
                        collection.add_line(line);
                    }
                }
                "Polygon" => {
                    if let Some((exterior, holes)) = parse_polygon(&feature.geometry.coordinates) {
                        let mut polygon = GeoPolygon::with_holes(exterior, holes);
                        if let Some(props) = parse_properties(&feature.properties) {
                            polygon.properties = props;
                        }
                        collection.add_polygon(polygon);
                    }
                }
                "MultiPoint" => {
                    if let Some(points) = parse_multi_point(&feature.geometry.coordinates) {
                        for point in points {
                            collection.add_point(GeoPoint::new(point));
                        }
                    }
                }
                "MultiLineString" => {
                    if let Some(lines) = parse_multi_line_string(&feature.geometry.coordinates) {
                        for coords in lines {
                            collection.add_line(GeoLine::new(coords, LineType::Custom));
                        }
                    }
                }
                "MultiPolygon" => {
                    if let Some(polygons) = parse_multi_polygon(&feature.geometry.coordinates) {
                        for (exterior, holes) in polygons {
                            collection.add_polygon(GeoPolygon::with_holes(exterior, holes));
                        }
                    }
                }
                _ => {}
            }
        }

        collection
    }

    /// Convert GeoCollection to GeoJSON
    pub fn from_collection(collection: &GeoCollection) -> Self {
        let mut features = Vec::new();

        for point in &collection.points {
            features.push(Feature {
                feature_type: "Feature".to_string(),
                geometry: Geometry {
                    geometry_type: "Point".to_string(),
                    coordinates: serde_json::json!([point.coord.lon, point.coord.lat]),
                },
                properties: properties_to_json(&point.properties),
            });
        }

        for line in &collection.lines {
            let coords: Vec<_> = line
                .points
                .iter()
                .map(|p| serde_json::json!([p.lon, p.lat]))
                .collect();
            features.push(Feature {
                feature_type: "Feature".to_string(),
                geometry: Geometry {
                    geometry_type: "LineString".to_string(),
                    coordinates: serde_json::Value::Array(coords),
                },
                properties: properties_to_json(&line.properties),
            });
        }

        for polygon in &collection.polygons {
            let mut rings = Vec::new();

            // Exterior ring
            let exterior: Vec<_> = polygon
                .exterior
                .iter()
                .map(|p| serde_json::json!([p.lon, p.lat]))
                .collect();
            rings.push(serde_json::Value::Array(exterior));

            // Holes
            for hole in &polygon.holes {
                let hole_coords: Vec<_> = hole
                    .iter()
                    .map(|p| serde_json::json!([p.lon, p.lat]))
                    .collect();
                rings.push(serde_json::Value::Array(hole_coords));
            }

            features.push(Feature {
                feature_type: "Feature".to_string(),
                geometry: Geometry {
                    geometry_type: "Polygon".to_string(),
                    coordinates: serde_json::Value::Array(rings),
                },
                properties: properties_to_json(&polygon.properties),
            });
        }

        GeoJson {
            geojson_type: "FeatureCollection".to_string(),
            features,
        }
    }

    /// Serialize to JSON string
    pub fn to_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Serialize to pretty JSON string
    pub fn to_string_pretty(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(feature = "geojson")]
fn parse_point(coords: &serde_json::Value) -> Option<GeoCoord> {
    if let Some(arr) = coords.as_array() {
        if arr.len() >= 2 {
            let lon = arr[0].as_f64()?;
            let lat = arr[1].as_f64()?;
            return Some(GeoCoord::new(lat, lon));
        }
    }
    None
}

#[cfg(feature = "geojson")]
fn parse_line_string(coords: &serde_json::Value) -> Option<Vec<GeoCoord>> {
    if let Some(arr) = coords.as_array() {
        let mut points = Vec::new();
        for point_val in arr {
            if let Some(point) = parse_point(point_val) {
                points.push(point);
            }
        }
        if !points.is_empty() {
            return Some(points);
        }
    }
    None
}

#[cfg(feature = "geojson")]
fn parse_polygon(coords: &serde_json::Value) -> Option<(Vec<GeoCoord>, Vec<Vec<GeoCoord>>)> {
    if let Some(arr) = coords.as_array() {
        if arr.is_empty() {
            return None;
        }

        let exterior = parse_line_string(&arr[0])?;
        let mut holes = Vec::new();

        for i in 1..arr.len() {
            if let Some(hole) = parse_line_string(&arr[i]) {
                holes.push(hole);
            }
        }

        return Some((exterior, holes));
    }
    None
}

#[cfg(feature = "geojson")]
fn parse_multi_point(coords: &serde_json::Value) -> Option<Vec<GeoCoord>> {
    parse_line_string(coords)
}

#[cfg(feature = "geojson")]
fn parse_multi_line_string(coords: &serde_json::Value) -> Option<Vec<Vec<GeoCoord>>> {
    if let Some(arr) = coords.as_array() {
        let mut lines = Vec::new();
        for line_val in arr {
            if let Some(line) = parse_line_string(line_val) {
                lines.push(line);
            }
        }
        if !lines.is_empty() {
            return Some(lines);
        }
    }
    None
}

#[cfg(feature = "geojson")]
fn parse_multi_polygon(coords: &serde_json::Value) -> Option<Vec<(Vec<GeoCoord>, Vec<Vec<GeoCoord>>)>> {
    if let Some(arr) = coords.as_array() {
        let mut polygons = Vec::new();
        for poly_val in arr {
            if let Some(polygon) = parse_polygon(poly_val) {
                polygons.push(polygon);
            }
        }
        if !polygons.is_empty() {
            return Some(polygons);
        }
    }
    None
}

#[cfg(feature = "geojson")]
fn parse_properties(props: &serde_json::Value) -> Option<HashMap<String, String>> {
    if let Some(obj) = props.as_object() {
        let mut map = HashMap::new();
        for (key, value) in obj {
            let val_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Number(n) => n.to_string(),
                serde_json::Value::Bool(b) => b.to_string(),
                _ => continue,
            };
            map.insert(key.clone(), val_str);
        }
        return Some(map);
    }
    None
}

#[cfg(feature = "geojson")]
fn properties_to_json(props: &HashMap<String, String>) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (key, value) in props {
        map.insert(key.clone(), serde_json::Value::String(value.clone()));
    }
    serde_json::Value::Object(map)
}

#[cfg(test)]
#[cfg(feature = "geojson")]
mod tests {
    use super::*;

    #[test]
    fn test_parse_point() {
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

        let geojson = GeoJson::from_str(json).unwrap();
        let collection = geojson.to_collection();

        assert_eq!(collection.points.len(), 1);
        assert_eq!(collection.points[0].coord.lat, -23.55);
        assert_eq!(collection.points[0].coord.lon, -46.63);
    }

    #[test]
    fn test_serialize() {
        let mut collection = GeoCollection::new();
        collection.add_point(GeoPoint::with_name(
            GeoCoord::new(-23.55, -46.63),
            "São Paulo",
        ));

        let geojson = GeoJson::from_collection(&collection);
        let json_str = geojson.to_string().unwrap();

        assert!(json_str.contains("Point"));
        assert!(json_str.contains("-23.55"));
    }
}
