//! Tile system for web mapping
//!
//! Implements XYZ, TMS, and QuadTree tile systems used by
//! Leaflet, OpenLayers, Google Maps, Bing Maps, etc.

use crate::coords::{CartesianCoord, GeoCoord, GeoBounds};
use std::fmt;

/// Tile coordinates (x, y, zoom)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TileCoord {
    pub x: u32,
    pub y: u32,
    pub zoom: u8,
}

impl TileCoord {
    pub fn new(x: u32, y: u32, zoom: u8) -> Self {
        Self { x, y, zoom }
    }

    /// Get number of tiles at this zoom level
    pub fn tile_count(&self) -> u32 {
        2u32.pow(self.zoom as u32)
    }

    /// Get geographic bounds of this tile
    pub fn bounds(&self) -> GeoBounds {
        let n = self.tile_count() as f64;

        let min_lon = (self.x as f64 / n) * 360.0 - 180.0;
        let max_lon = ((self.x + 1) as f64 / n) * 360.0 - 180.0;

        let min_lat = Self::tile_y_to_lat(self.y + 1, self.zoom);
        let max_lat = Self::tile_y_to_lat(self.y, self.zoom);

        GeoBounds::new(min_lat, max_lat, min_lon, max_lon)
    }

    /// Convert tile Y to latitude (Web Mercator)
    fn tile_y_to_lat(y: u32, zoom: u8) -> f64 {
        let n = 2f64.powi(zoom as i32);
        let lat_rad = ((1.0 - 2.0 * y as f64 / n) * std::f64::consts::PI).sinh().atan();
        lat_rad.to_degrees()
    }

    /// Get center coordinate of this tile
    pub fn center(&self) -> GeoCoord {
        let bounds = self.bounds();
        GeoCoord::new(
            (bounds.min_lat + bounds.max_lat) / 2.0,
            (bounds.min_lon + bounds.max_lon) / 2.0,
        )
    }

    /// Get parent tile (zoom - 1)
    pub fn parent(&self) -> Option<Self> {
        if self.zoom == 0 {
            return None;
        }
        Some(TileCoord::new(
            self.x / 2,
            self.y / 2,
            self.zoom - 1,
        ))
    }

    /// Get 4 children tiles (zoom + 1)
    pub fn children(&self) -> [TileCoord; 4] {
        let zoom = self.zoom + 1;
        let x = self.x * 2;
        let y = self.y * 2;

        [
            TileCoord::new(x, y, zoom),
            TileCoord::new(x + 1, y, zoom),
            TileCoord::new(x, y + 1, zoom),
            TileCoord::new(x + 1, y + 1, zoom),
        ]
    }

    /// Convert to QuadKey (Bing Maps format)
    pub fn to_quadkey(&self) -> String {
        let mut quadkey = String::with_capacity(self.zoom as usize);

        for i in (0..self.zoom).rev() {
            let mut digit = 0;
            let mask = 1 << i;

            if (self.x & mask) != 0 {
                digit += 1;
            }
            if (self.y & mask) != 0 {
                digit += 2;
            }

            quadkey.push_char(std::char::from_digit(digit, 10).unwrap());
        }

        quadkey
    }

    /// Parse from QuadKey
    pub fn from_quadkey(quadkey: &str) -> Result<Self, TileError> {
        let zoom = quadkey.len() as u8;
        let mut x = 0u32;
        let mut y = 0u32;

        for (i, c) in quadkey.chars().enumerate() {
            let digit = c.to_digit(10).ok_or(TileError::InvalidQuadKey)?;
            if digit > 3 {
                return Err(TileError::InvalidQuadKey);
            }

            let mask = 1 << (zoom - 1 - i as u8);

            if digit & 1 != 0 {
                x |= mask;
            }
            if digit & 2 != 0 {
                y |= mask;
            }
        }

        Ok(TileCoord::new(x, y, zoom))
    }

    /// Convert to TMS (y-axis inverted)
    pub fn to_tms(&self) -> TileCoord {
        let tile_count = self.tile_count();
        TileCoord::new(self.x, tile_count - 1 - self.y, self.zoom)
    }

    /// Convert from TMS to XYZ
    pub fn from_tms(x: u32, y: u32, zoom: u8) -> Self {
        let tile_count = 2u32.pow(zoom as u32);
        TileCoord::new(x, tile_count - 1 - y, zoom)
    }
}

impl fmt::Display for TileCoord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.zoom, self.x, self.y)
    }
}

/// Tile system for converting between geographic and tile coordinates
#[derive(Debug, Clone, Copy)]
pub struct TileSystem {
    /// Tile size in pixels (typically 256 or 512)
    pub tile_size: u32,
}

impl TileSystem {
    /// Create with standard 256x256 tiles
    pub fn new() -> Self {
        Self { tile_size: 256 }
    }

    /// Create with custom tile size
    pub fn with_size(tile_size: u32) -> Self {
        Self { tile_size }
    }

    /// Convert geographic coordinate to tile coordinate
    pub fn geo_to_tile(&self, geo: &GeoCoord, zoom: u8) -> TileCoord {
        let n = 2f64.powi(zoom as i32);

        let x = ((geo.lon + 180.0) / 360.0 * n).floor() as u32;

        let lat_rad = geo.lat.to_radians();
        let y = ((1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0 * n).floor() as u32;

        TileCoord::new(
            x.min(n as u32 - 1),
            y.min(n as u32 - 1),
            zoom,
        )
    }

    /// Convert geographic coordinate to pixel coordinate within a tile
    pub fn geo_to_pixel(&self, geo: &GeoCoord, zoom: u8) -> CartesianCoord {
        let n = 2f64.powi(zoom as i32);
        let tile_size = self.tile_size as f64;

        let x = (geo.lon + 180.0) / 360.0 * n * tile_size;

        let lat_rad = geo.lat.to_radians();
        let y = (1.0 - (lat_rad.tan() + 1.0 / lat_rad.cos()).ln() / std::f64::consts::PI) / 2.0 * n * tile_size;

        CartesianCoord::new(x, y)
    }

    /// Get tiles covering a geographic bounds at given zoom
    pub fn tiles_in_bounds(&self, bounds: &GeoBounds, zoom: u8) -> Vec<TileCoord> {
        let nw = GeoCoord::new(bounds.max_lat, bounds.min_lon);
        let se = GeoCoord::new(bounds.min_lat, bounds.max_lon);

        let tile_nw = self.geo_to_tile(&nw, zoom);
        let tile_se = self.geo_to_tile(&se, zoom);

        let mut tiles = Vec::new();

        for x in tile_nw.x..=tile_se.x {
            for y in tile_nw.y..=tile_se.y {
                tiles.push(TileCoord::new(x, y, zoom));
            }
        }

        tiles
    }

    /// Get optimal zoom level for a bounds and viewport size
    pub fn optimal_zoom(&self, bounds: &GeoBounds, width: u32, height: u32) -> u8 {
        let lat_range = bounds.max_lat - bounds.min_lat;
        let lon_range = bounds.max_lon - bounds.min_lon;

        // Calculate zoom based on longitude range
        let zoom_lon = ((width as f64 / self.tile_size as f64) * 360.0 / lon_range).log2();

        // Calculate zoom based on latitude range (Web Mercator)
        let lat_rad_max = bounds.max_lat.to_radians();
        let lat_rad_min = bounds.min_lat.to_radians();
        let merc_range = (lat_rad_max.tan() + 1.0 / lat_rad_max.cos()).ln()
                       - (lat_rad_min.tan() + 1.0 / lat_rad_min.cos()).ln();
        let zoom_lat = ((height as f64 / self.tile_size as f64) * (2.0 * std::f64::consts::PI) / merc_range).log2();

        zoom_lon.min(zoom_lat).floor() as u8
    }
}

impl Default for TileSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Tile URL template builder
#[derive(Debug, Clone)]
pub struct TileUrlTemplate {
    template: String,
}

impl TileUrlTemplate {
    /// Create from template string
    ///
    /// Placeholders:
    /// - {z} = zoom level
    /// - {x} = tile x
    /// - {y} = tile y
    /// - {s} = subdomain (a, b, c)
    /// - {q} = quadkey
    pub fn new(template: impl Into<String>) -> Self {
        Self {
            template: template.into(),
        }
    }

    /// OpenStreetMap
    pub fn osm() -> Self {
        Self::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png")
    }

    /// Mapbox
    pub fn mapbox(style: &str, access_token: &str) -> Self {
        Self::new(format!(
            "https://api.mapbox.com/styles/v1/mapbox/{}/tiles/{{z}}/{{x}}/{{y}}?access_token={}",
            style, access_token
        ))
    }

    /// Google Maps
    pub fn google(map_type: &str) -> Self {
        Self::new(format!("https://mt{{s}}.google.com/vt/lyrs={}&x={{x}}&y={{y}}&z={{z}}", map_type))
    }

    /// Build URL for specific tile
    pub fn build(&self, tile: &TileCoord, subdomain: &str) -> String {
        self.template
            .replace("{z}", &tile.zoom.to_string())
            .replace("{x}", &tile.x.to_string())
            .replace("{y}", &tile.y.to_string())
            .replace("{s}", subdomain)
            .replace("{q}", &tile.to_quadkey())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileError {
    InvalidQuadKey,
    InvalidZoom,
    OutOfBounds,
}

impl fmt::Display for TileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TileError::InvalidQuadKey => write!(f, "Invalid QuadKey format"),
            TileError::InvalidZoom => write!(f, "Zoom level out of valid range (0-30)"),
            TileError::OutOfBounds => write!(f, "Tile coordinates out of bounds"),
        }
    }
}

impl std::error::Error for TileError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_bounds() {
        let tile = TileCoord::new(0, 0, 0);
        let bounds = tile.bounds();

        // Zoom 0 tile should cover whole world
        assert!((bounds.min_lat + 85.051).abs() < 1.0);
        assert!((bounds.max_lat - 85.051).abs() < 1.0);
    }

    #[test]
    fn test_quadkey() {
        let tile = TileCoord::new(3, 5, 3);
        let quadkey = tile.to_quadkey();
        assert_eq!(quadkey, "213");

        let tile2 = TileCoord::from_quadkey(&quadkey).unwrap();
        assert_eq!(tile, tile2);
    }

    #[test]
    fn test_parent_children() {
        let tile = TileCoord::new(4, 6, 4);
        let parent = tile.parent().unwrap();

        assert_eq!(parent.x, 2);
        assert_eq!(parent.y, 3);
        assert_eq!(parent.zoom, 3);

        let children = parent.children();
        assert!(children.contains(&tile));
    }

    #[test]
    fn test_geo_to_tile() {
        let sys = TileSystem::new();
        let null_island = GeoCoord::new(0.0, 0.0);

        let tile = sys.geo_to_tile(&null_island, 0);
        assert_eq!(tile.x, 0);
        assert_eq!(tile.y, 0);
    }

    #[test]
    fn test_tiles_in_bounds() {
        let sys = TileSystem::new();
        let bounds = GeoBounds::new(-10.0, 10.0, -10.0, 10.0);

        let tiles = sys.tiles_in_bounds(&bounds, 2);
        assert!(!tiles.is_empty());
    }
}
