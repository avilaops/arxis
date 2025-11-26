//! Map rendering system
//!
//! Combines projections, geometries, and rendering to create maps

use crate::{
    coords::{CartesianCoord, GeoBounds},
    geometry::{GeoCollection, GeoLine, GeoPoint, GeoPolygon},
    projection::Projection,
    render::{Color, Framebuffer, draw_line, draw_polyline, fill_polygon, fill_circle},
};

/// Style for rendering features
#[derive(Debug, Clone)]
pub struct Style {
    pub fill_color: Option<Color>,
    pub stroke_color: Option<Color>,
    pub stroke_width: u32,
    pub point_radius: u32,
}

impl Style {
    pub fn new() -> Self {
        Self {
            fill_color: Some(Color::from_hex(0xE0E0E0)),
            stroke_color: Some(Color::from_hex(0x404040)),
            stroke_width: 1,
            point_radius: 3,
        }
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn with_stroke(mut self, color: Color, width: u32) -> Self {
        self.stroke_color = Some(color);
        self.stroke_width = width;
        self
    }

    pub fn no_fill(mut self) -> Self {
        self.fill_color = None;
        self
    }

    pub fn no_stroke(mut self) -> Self {
        self.stroke_color = None;
        self
    }

    // Predefined styles
    pub fn land() -> Self {
        Self::new()
            .with_fill(Color::from_hex(0xF5F5DC))
            .with_stroke(Color::from_hex(0x808080), 1)
    }

    pub fn water() -> Self {
        Self::new()
            .with_fill(Color::from_hex(0xADD8E6))
            .with_stroke(Color::from_hex(0x4682B4), 1)
    }

    pub fn border() -> Self {
        Self::new()
            .no_fill()
            .with_stroke(Color::from_hex(0xFF0000), 2)
    }

    pub fn road() -> Self {
        Self::new()
            .no_fill()
            .with_stroke(Color::from_hex(0xFFFF00), 2)
    }

    pub fn city() -> Self {
        Self {
            fill_color: Some(Color::from_hex(0xFF0000)),
            stroke_color: Some(Color::from_hex(0x000000)),
            stroke_width: 1,
            point_radius: 5,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

/// Layer in a map
pub struct Layer {
    pub name: String,
    pub collection: GeoCollection,
    pub style: Style,
    pub visible: bool,
}

impl Layer {
    pub fn new(name: impl Into<String>, collection: GeoCollection, style: Style) -> Self {
        Self {
            name: name.into(),
            collection,
            style,
            visible: true,
        }
    }
}

/// Map renderer
pub struct Map {
    pub width: u32,
    pub height: u32,
    pub bounds: Option<GeoBounds>,
    pub background: Color,
    pub layers: Vec<Layer>,
}

impl Map {
    /// Create a new map
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            bounds: None,
            background: Color::from_hex(0xF0F0F0),
            layers: Vec::new(),
        }
    }

    /// Set map bounds (area to display)
    pub fn with_bounds(mut self, bounds: GeoBounds) -> Self {
        self.bounds = Some(bounds);
        self
    }

    /// Set background color
    pub fn with_background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    /// Add a layer
    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    /// Calculate bounds from all layers
    pub fn auto_bounds(&mut self) {
        let mut all_bounds = Vec::new();

        for layer in &self.layers {
            if let Some(bounds) = layer.collection.bounds() {
                all_bounds.push(bounds);
            }
        }

        if all_bounds.is_empty() {
            self.bounds = Some(GeoBounds::WORLD);
            return;
        }

        let min_lat = all_bounds.iter().map(|b| b.min_lat).fold(f64::INFINITY, f64::min);
        let max_lat = all_bounds.iter().map(|b| b.max_lat).fold(f64::NEG_INFINITY, f64::max);
        let min_lon = all_bounds.iter().map(|b| b.min_lon).fold(f64::INFINITY, f64::min);
        let max_lon = all_bounds.iter().map(|b| b.max_lon).fold(f64::NEG_INFINITY, f64::max);

        self.bounds = Some(GeoBounds::new(min_lat, max_lat, min_lon, max_lon));
    }

    /// Render map with given projection
    pub fn render<P: Projection>(&self, projection: &P) -> Framebuffer {
        let mut fb = Framebuffer::new(self.width, self.height, self.background);

        for layer in &self.layers {
            if !layer.visible {
                continue;
            }

            self.render_layer(&mut fb, layer, projection);
        }

        fb
    }

    fn render_layer<P: Projection>(&self, fb: &mut Framebuffer, layer: &Layer, projection: &P) {
        let w = self.width as f64;
        let h = self.height as f64;

        // Render polygons
        for polygon in &layer.collection.polygons {
            self.render_polygon(fb, polygon, &layer.style, projection, w, h);
        }

        // Render lines
        for line in &layer.collection.lines {
            self.render_line(fb, line, &layer.style, projection, w, h);
        }

        // Render points
        for point in &layer.collection.points {
            self.render_point(fb, point, &layer.style, projection, w, h);
        }
    }

    fn render_polygon<P: Projection>(
        &self,
        fb: &mut Framebuffer,
        polygon: &GeoPolygon,
        style: &Style,
        projection: &P,
        width: f64,
        height: f64,
    ) {
        // Project exterior ring
        let exterior: Vec<CartesianCoord> = polygon
            .exterior
            .iter()
            .map(|coord| projection.project(coord, width, height))
            .collect();

        // Fill
        if let Some(fill_color) = style.fill_color {
            fill_polygon(fb, &exterior, fill_color);
        }

        // Stroke
        if let Some(stroke_color) = style.stroke_color {
            for _ in 0..style.stroke_width {
                draw_polyline(fb, &exterior, stroke_color);
            }
        }

        // Render holes (as stroked outlines only)
        if let Some(stroke_color) = style.stroke_color {
            for hole in &polygon.holes {
                let hole_coords: Vec<CartesianCoord> = hole
                    .iter()
                    .map(|coord| projection.project(coord, width, height))
                    .collect();
                draw_polyline(fb, &hole_coords, stroke_color);
            }
        }
    }

    fn render_line<P: Projection>(
        &self,
        fb: &mut Framebuffer,
        line: &GeoLine,
        style: &Style,
        projection: &P,
        width: f64,
        height: f64,
    ) {
        if let Some(stroke_color) = style.stroke_color {
            let projected: Vec<CartesianCoord> = line
                .points
                .iter()
                .map(|coord| projection.project(coord, width, height))
                .collect();

            for _ in 0..style.stroke_width {
                draw_polyline(fb, &projected, stroke_color);
            }
        }
    }

    fn render_point<P: Projection>(
        &self,
        fb: &mut Framebuffer,
        point: &GeoPoint,
        style: &Style,
        projection: &P,
        width: f64,
        height: f64,
    ) {
        let cart = projection.project(&point.coord, width, height);
        let (x, y) = cart.to_i32();

        if let Some(fill_color) = style.fill_color {
            fill_circle(fb, x, y, style.point_radius as i32, fill_color);
        }

        if let Some(stroke_color) = style.stroke_color {
            use crate::render::draw_circle;
            draw_circle(fb, x, y, style.point_radius as i32, stroke_color);
        }
    }

    /// Save map as PPM file
    pub fn save_ppm<P: Projection>(&self, projection: &P, path: &str) -> std::io::Result<()> {
        let fb = self.render(projection);
        std::fs::write(path, fb.to_ppm())
    }

    /// Get rendered bytes (RGB)
    pub fn render_bytes<P: Projection>(&self, projection: &P) -> Vec<u8> {
        let fb = self.render(projection);
        fb.data
    }
}

/// Quick map builder for common scenarios
pub struct MapBuilder {
    width: u32,
    height: u32,
    bounds: Option<GeoBounds>,
}

impl MapBuilder {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            bounds: None,
        }
    }

    pub fn bounds(mut self, bounds: GeoBounds) -> Self {
        self.bounds = Some(bounds);
        self
    }

    pub fn world(width: u32, height: u32) -> Map {
        Map::new(width, height).with_bounds(GeoBounds::WORLD)
    }

    pub fn brazil(width: u32, height: u32) -> Map {
        Map::new(width, height).with_bounds(GeoBounds::BRAZIL)
    }

    pub fn usa(width: u32, height: u32) -> Map {
        Map::new(width, height).with_bounds(GeoBounds::USA)
    }

    pub fn europe(width: u32, height: u32) -> Map {
        Map::new(width, height).with_bounds(GeoBounds::EUROPE)
    }

    pub fn middle_east(width: u32, height: u32) -> Map {
        Map::new(width, height).with_bounds(GeoBounds::MIDDLE_EAST)
    }

    pub fn dubai(width: u32, height: u32) -> Map {
        Map::new(width, height).with_bounds(GeoBounds::DUBAI)
    }

    pub fn gulf_region(width: u32, height: u32) -> Map {
        Map::new(width, height).with_bounds(GeoBounds::GULF_REGION)
    }

    pub fn build(self) -> Map {
        let mut map = Map::new(self.width, self.height);
        if let Some(bounds) = self.bounds {
            map.bounds = Some(bounds);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::Equirectangular;

    #[test]
    fn test_map_creation() {
        let map = Map::new(800, 600);
        assert_eq!(map.width, 800);
        assert_eq!(map.height, 600);
    }

    #[test]
    fn test_map_builder() {
        let map = MapBuilder::world(1024, 768);
        assert_eq!(map.bounds, Some(GeoBounds::WORLD));
    }

    #[test]
    fn test_render() {
        let map = Map::new(100, 100);
        let projection = Equirectangular::new();
        let fb = map.render(&projection);
        assert_eq!(fb.width, 100);
        assert_eq!(fb.height, 100);
    }
}
