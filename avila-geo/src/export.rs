//! SVG export for vector-based map rendering
//!
//! Export maps as Scalable Vector Graphics (SVG) for high-quality,
//! resolution-independent output.

#[cfg(feature = "export-svg")]
use svg::{
    node::element::{Circle, Group, Path, Polygon, Polyline, Rectangle, Text},
    Document,
};

use crate::coords::GeoCoord;
use crate::geometry::{GeoCollection, GeoLine, GeoPoint, GeoPolygon, LineType};
use crate::map::{Layer, Map, Style};
use crate::projection::Projection;
use crate::render::Color;
use std::io::Write;

/// SVG exporter for maps
pub struct SvgExporter {
    width: u32,
    height: u32,
    view_box: Option<(f64, f64, f64, f64)>,
}

impl SvgExporter {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            view_box: None,
        }
    }

    pub fn with_view_box(mut self, x: f64, y: f64, w: f64, h: f64) -> Self {
        self.view_box = Some((x, y, w, h));
        self
    }

    /// Export map to SVG
    #[cfg(feature = "export-svg")]
    pub fn export(&self, map: &Map, projection: &dyn Projection) -> Document {
        let mut doc = Document::new()
            .set("width", self.width)
            .set("height", self.height);

        if let Some((x, y, w, h)) = self.view_box {
            doc = doc.set("viewBox", format!("{} {} {} {}", x, y, w, h));
        }

        // Background
        if let Some(bg) = map.background {
            let rect = Rectangle::new()
                .set("width", "100%")
                .set("height", "100%")
                .set("fill", color_to_hex(bg));
            doc = doc.add(rect);
        }

        // Layers
        for layer in &map.layers {
            let group = self.layer_to_group(layer, projection, map.width as f64, map.height as f64);
            doc = doc.add(group);
        }

        doc
    }

    #[cfg(feature = "export-svg")]
    fn layer_to_group(&self, layer: &Layer, projection: &dyn Projection, w: f64, h: f64) -> Group {
        let mut group = Group::new().set("id", layer.name.clone());

        // Polygons (render first, so they're behind)
        for polygon in &layer.collection.polygons {
            let element = self.polygon_to_svg(polygon, &layer.style, projection, w, h);
            group = group.add(element);
        }

        // Lines
        for line in &layer.collection.lines {
            let element = self.line_to_svg(line, &layer.style, projection, w, h);
            group = group.add(element);
        }

        // Points (render last, so they're on top)
        for point in &layer.collection.points {
            let element = self.point_to_svg(point, &layer.style, projection, w, h);
            group = group.add(element);
        }

        group
    }

    #[cfg(feature = "export-svg")]
    fn point_to_svg(&self, point: &GeoPoint, style: &Style, proj: &dyn Projection, w: f64, h: f64) -> Group {
        let cart = proj.project(&point.coord, w, h);
        let mut group = Group::new();

        // Circle marker
        let mut circle = Circle::new()
            .set("cx", cart.x)
            .set("cy", cart.y)
            .set("r", style.point_radius.unwrap_or(3) as f64);

        if let Some(fill) = style.fill_color {
            circle = circle.set("fill", color_to_hex(fill));
        }

        if let Some(stroke) = style.stroke_color {
            circle = circle
                .set("stroke", color_to_hex(stroke))
                .set("stroke-width", style.stroke_width.unwrap_or(1));
        }

        group = group.add(circle);

        // Label
        if let Some(name) = point.name() {
            let text = Text::new()
                .set("x", cart.x + 5.0)
                .set("y", cart.y - 5.0)
                .set("font-size", "12")
                .set("font-family", "sans-serif")
                .add(svg::node::Text::new(name));
            group = group.add(text);
        }

        group
    }

    #[cfg(feature = "export-svg")]
    fn line_to_svg(&self, line: &GeoLine, style: &Style, proj: &dyn Projection, w: f64, h: f64) -> Polyline {
        let points: Vec<String> = line
            .coords
            .iter()
            .map(|c| {
                let cart = proj.project(c, w, h);
                format!("{},{}", cart.x, cart.y)
            })
            .collect();

        let mut polyline = Polyline::new()
            .set("points", points.join(" "))
            .set("fill", "none");

        if let Some(stroke) = style.stroke_color {
            polyline = polyline
                .set("stroke", color_to_hex(stroke))
                .set("stroke-width", style.stroke_width.unwrap_or(1));
        }

        // Line style
        match line.line_type {
            LineType::Border => {
                polyline = polyline.set("stroke-dasharray", "5,5");
            }
            LineType::River => {
                polyline = polyline.set("stroke-linecap", "round");
            }
            _ => {}
        }

        polyline
    }

    #[cfg(feature = "export-svg")]
    fn polygon_to_svg(&self, polygon: &GeoPolygon, style: &Style, proj: &dyn Projection, w: f64, h: f64) -> Polygon {
        let points: Vec<String> = polygon
            .exterior
            .iter()
            .map(|c| {
                let cart = proj.project(c, w, h);
                format!("{},{}", cart.x, cart.y)
            })
            .collect();

        let mut poly = Polygon::new().set("points", points.join(" "));

        if let Some(fill) = style.fill_color {
            poly = poly.set("fill", color_to_hex(fill));
        } else {
            poly = poly.set("fill", "none");
        }

        if let Some(stroke) = style.stroke_color {
            poly = poly
                .set("stroke", color_to_hex(stroke))
                .set("stroke-width", style.stroke_width.unwrap_or(1));
        }

        poly
    }

    /// Save to file
    #[cfg(feature = "export-svg")]
    pub fn save(&self, map: &Map, projection: &dyn Projection, path: &str) -> std::io::Result<()> {
        let doc = self.export(map, projection);
        svg::save(path, &doc).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
}

#[cfg(feature = "export-svg")]
fn color_to_hex(color: Color) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)
}

/// Extension trait for Map to easily export to SVG
pub trait MapSvgExt {
    #[cfg(feature = "export-svg")]
    fn to_svg(&self, projection: &dyn Projection) -> Document;

    #[cfg(feature = "export-svg")]
    fn save_svg(&self, projection: &dyn Projection, path: &str) -> std::io::Result<()>;
}

impl MapSvgExt for Map {
    #[cfg(feature = "export-svg")]
    fn to_svg(&self, projection: &dyn Projection) -> Document {
        let exporter = SvgExporter::new(self.width, self.height);
        exporter.export(self, projection)
    }

    #[cfg(feature = "export-svg")]
    fn save_svg(&self, projection: &dyn Projection, path: &str) -> std::io::Result<()> {
        let exporter = SvgExporter::new(self.width, self.height);
        exporter.save(self, projection, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::coords::GeoBounds;
    use crate::projection::Mercator;

    #[test]
    #[cfg(feature = "export-svg")]
    fn test_svg_export() {
        let mut map = Map::new(800, 600);
        let collection = GeoCollection::new();
        let layer = Layer::new("test", collection, Style::default());
        map.add_layer(layer);

        let exporter = SvgExporter::new(800, 600);
        let proj = Mercator::new();
        let _doc = exporter.export(&map, &proj);
    }
}
