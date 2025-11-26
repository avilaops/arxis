//! Sistema de Referência de Coordenadas (CRS)
//!
//! Este módulo implementa:
//! - Diferentes sistemas de coordenadas (WGS84, Web Mercator, UTM)
//! - Transformações entre sistemas de coordenadas
//! - Projeções cartográficas

use crate::coords::GeoCoord;
use std::f64::consts::PI;

/// Raio equatorial da Terra (WGS84) em metros
pub const WGS84_EQUATORIAL_RADIUS: f64 = 6378137.0;

/// Raio polar da Terra (WGS84) em metros
pub const WGS84_POLAR_RADIUS: f64 = 6356752.314245;

/// Achatamento da Terra (WGS84)
pub const WGS84_FLATTENING: f64 = 1.0 / 298.257223563;

/// Excentricidade ao quadrado (WGS84)
pub const WGS84_ECCENTRICITY_SQUARED: f64 = 0.00669437999014;

/// Sistemas de coordenadas suportados
#[derive(Debug, Clone, PartialEq)]
pub enum CoordinateSystem {
    /// WGS84 - Sistema GPS padrão (latitude/longitude em graus)
    WGS84,
    /// Web Mercator - Usado por Google Maps, OpenStreetMap
    WebMercator,
    /// Universal Transverse Mercator - zona específica
    UTM { zone: u8, north: bool },
    /// Sistema customizado
    Custom(String),
}

/// Transformador de coordenadas
pub struct CoordinateTransformer;

impl CoordinateTransformer {
    /// Transforma coordenadas entre sistemas
    pub fn transform(
        coord: &GeoCoord,
        from: &CoordinateSystem,
        to: &CoordinateSystem,
    ) -> GeoCoord {
        match (from, to) {
            (CoordinateSystem::WGS84, CoordinateSystem::WGS84) => *coord,
            (CoordinateSystem::WGS84, CoordinateSystem::WebMercator) => {
                let (x, y) = Self::wgs84_to_web_mercator(coord);
                GeoCoord { lat: y, lon: x }
            }
            (CoordinateSystem::WebMercator, CoordinateSystem::WGS84) => {
                Self::web_mercator_to_wgs84(coord.lon, coord.lat)
            }
            (CoordinateSystem::WGS84, CoordinateSystem::UTM { zone, north }) => {
                let (x, y) = Self::wgs84_to_utm(coord, *zone, *north);
                GeoCoord { lat: y, lon: x }
            }
            (CoordinateSystem::UTM { zone, north }, CoordinateSystem::WGS84) => {
                Self::utm_to_wgs84(coord.lon, coord.lat, *zone, *north)
            }
            _ => *coord, // Fallback
        }
    }

    /// Converte WGS84 (lat/lon) para Web Mercator (x/y em metros)
    pub fn wgs84_to_web_mercator(coord: &GeoCoord) -> (f64, f64) {
        let lon_rad = coord.lon.to_radians();
        let lat_rad = coord.lat.to_radians();

        let x = WGS84_EQUATORIAL_RADIUS * lon_rad;
        let y = WGS84_EQUATORIAL_RADIUS * ((PI / 4.0 + lat_rad / 2.0).tan().ln());

        (x, y)
    }

    /// Converte Web Mercator (x/y em metros) para WGS84 (lat/lon)
    pub fn web_mercator_to_wgs84(x: f64, y: f64) -> GeoCoord {
        let lon = (x / WGS84_EQUATORIAL_RADIUS).to_degrees();
        let lat = (2.0 * (y / WGS84_EQUATORIAL_RADIUS).exp().atan() - PI / 2.0).to_degrees();

        GeoCoord { lat, lon }
    }

    /// Converte WGS84 para UTM
    ///
    /// # Argumentos
    /// * `coord` - Coordenada em WGS84
    /// * `zone` - Zona UTM (1-60)
    /// * `north` - true para hemisfério norte, false para sul
    ///
    /// # Retorna
    /// (Easting, Northing) em metros
    pub fn wgs84_to_utm(coord: &GeoCoord, zone: u8, north: bool) -> (f64, f64) {
        let lat_rad = coord.lat.to_radians();
        let lon_rad = coord.lon.to_radians();

        let k0 = 0.9996; // Fator de escala
        let e = WGS84_ECCENTRICITY_SQUARED.sqrt();
        let e2 = WGS84_ECCENTRICITY_SQUARED;

        let lon0 = ((zone as f64 - 1.0) * 6.0 - 180.0 + 3.0).to_radians();
        let lon_diff = lon_rad - lon0;

        let n = WGS84_EQUATORIAL_RADIUS / (1.0 - e2 * lat_rad.sin().powi(2)).sqrt();
        let t = lat_rad.tan().powi(2);
        let c = e2 * lat_rad.cos().powi(2) / (1.0 - e2);
        let a = lon_diff * lat_rad.cos();

        let m = WGS84_EQUATORIAL_RADIUS
            * ((1.0 - e2 / 4.0 - 3.0 * e2.powi(2) / 64.0 - 5.0 * e2.powi(3) / 256.0) * lat_rad
                - (3.0 * e2 / 8.0 + 3.0 * e2.powi(2) / 32.0 + 45.0 * e2.powi(3) / 1024.0)
                    * (2.0 * lat_rad).sin()
                + (15.0 * e2.powi(2) / 256.0 + 45.0 * e2.powi(3) / 1024.0)
                    * (4.0 * lat_rad).sin()
                - (35.0 * e2.powi(3) / 3072.0) * (6.0 * lat_rad).sin());

        let easting = k0 * n
            * (a + (1.0 - t + c) * a.powi(3) / 6.0
                + (5.0 - 18.0 * t + t.powi(2) + 72.0 * c - 58.0 * e2) * a.powi(5) / 120.0)
            + 500000.0;

        let mut northing = k0
            * (m + n
                * lat_rad.tan()
                * (a.powi(2) / 2.0
                    + (5.0 - t + 9.0 * c + 4.0 * c.powi(2)) * a.powi(4) / 24.0
                    + (61.0 - 58.0 * t + t.powi(2) + 600.0 * c - 330.0 * e2) * a.powi(6)
                        / 720.0));

        if !north {
            northing += 10000000.0;
        }

        (easting, northing)
    }

    /// Converte UTM para WGS84
    ///
    /// # Argumentos
    /// * `easting` - Coordenada Easting em metros
    /// * `northing` - Coordenada Northing em metros
    /// * `zone` - Zona UTM (1-60)
    /// * `north` - true para hemisfério norte, false para sul
    ///
    /// # Retorna
    /// Coordenada em WGS84 (lat/lon)
    pub fn utm_to_wgs84(easting: f64, northing: f64, zone: u8, north: bool) -> GeoCoord {
        let k0 = 0.9996;
        let e = WGS84_ECCENTRICITY_SQUARED.sqrt();
        let e2 = WGS84_ECCENTRICITY_SQUARED;
        let e1 = (1.0 - (1.0 - e2).sqrt()) / (1.0 + (1.0 - e2).sqrt());

        let x = easting - 500000.0;
        let y = if north {
            northing
        } else {
            northing - 10000000.0
        };

        let m = y / k0;
        let mu = m
            / (WGS84_EQUATORIAL_RADIUS
                * (1.0 - e2 / 4.0 - 3.0 * e2.powi(2) / 64.0 - 5.0 * e2.powi(3) / 256.0));

        let phi1 = mu
            + (3.0 * e1 / 2.0 - 27.0 * e1.powi(3) / 32.0) * (2.0 * mu).sin()
            + (21.0 * e1.powi(2) / 16.0 - 55.0 * e1.powi(4) / 32.0) * (4.0 * mu).sin()
            + (151.0 * e1.powi(3) / 96.0) * (6.0 * mu).sin();

        let n1 = WGS84_EQUATORIAL_RADIUS / (1.0 - e2 * phi1.sin().powi(2)).sqrt();
        let t1 = phi1.tan().powi(2);
        let c1 = e2 * phi1.cos().powi(2) / (1.0 - e2);
        let r1 = WGS84_EQUATORIAL_RADIUS * (1.0 - e2)
            / (1.0 - e2 * phi1.sin().powi(2)).powf(1.5);
        let d = x / (n1 * k0);

        let lat = phi1
            - (n1 * phi1.tan() / r1)
                * (d.powi(2) / 2.0
                    - (5.0 + 3.0 * t1 + 10.0 * c1 - 4.0 * c1.powi(2) - 9.0 * e2) * d.powi(4)
                        / 24.0
                    + (61.0 + 90.0 * t1 + 298.0 * c1 + 45.0 * t1.powi(2) - 252.0 * e2
                        - 3.0 * c1.powi(2))
                        * d.powi(6)
                        / 720.0);

        let lon = ((zone as f64 - 1.0) * 6.0 - 180.0 + 3.0).to_radians()
            + (d - (1.0 + 2.0 * t1 + c1) * d.powi(3) / 6.0
                + (5.0 - 2.0 * c1 + 28.0 * t1 - 3.0 * c1.powi(2) + 8.0 * e2 + 24.0 * t1.powi(2))
                    * d.powi(5)
                    / 120.0)
                / phi1.cos();

        GeoCoord {
            lat: lat.to_degrees(),
            lon: lon.to_degrees(),
        }
    }

    /// Determina automaticamente a zona UTM para uma coordenada
    pub fn get_utm_zone(coord: &GeoCoord) -> (u8, bool) {
        let zone = (((coord.lon + 180.0) / 6.0).floor() as u8 % 60) + 1;
        let north = coord.lat >= 0.0;
        (zone, north)
    }

    /// Converte para Web Mercator com limites (usado em tiles)
    pub fn wgs84_to_web_mercator_tile(coord: &GeoCoord, zoom: u8) -> (u32, u32) {
        let lat_rad = coord.lat.to_radians();
        let n = 2_u32.pow(zoom as u32) as f64;

        let x = ((coord.lon + 180.0) / 360.0 * n) as u32;
        let y = ((1.0 - lat_rad.tan().asinh() / PI) / 2.0 * n) as u32;

        (x, y)
    }

    /// Converte tile para WGS84
    pub fn web_mercator_tile_to_wgs84(x: u32, y: u32, zoom: u8) -> GeoCoord {
        let n = 2_f64.powi(zoom as i32);
        let lon = x as f64 / n * 360.0 - 180.0;
        let lat_rad = ((PI * (1.0 - 2.0 * y as f64 / n)).sinh()).atan();

        GeoCoord {
            lat: lat_rad.to_degrees(),
            lon,
        }
    }
}

/// Informações sobre zona UTM
#[derive(Debug, Clone)]
pub struct UTMZoneInfo {
    pub zone: u8,
    pub north: bool,
    pub central_meridian: f64,
    pub false_easting: f64,
    pub false_northing: f64,
}

impl UTMZoneInfo {
    /// Cria informações sobre uma zona UTM
    pub fn new(zone: u8, north: bool) -> Self {
        Self {
            zone,
            north,
            central_meridian: (zone as f64 - 1.0) * 6.0 - 180.0 + 3.0,
            false_easting: 500000.0,
            false_northing: if north { 0.0 } else { 10000000.0 },
        }
    }

    /// Retorna os limites da zona em WGS84
    pub fn bounds(&self) -> (f64, f64, f64, f64) {
        let lon_min = (self.zone as f64 - 1.0) * 6.0 - 180.0;
        let lon_max = lon_min + 6.0;
        let lat_min = if self.north { 0.0 } else { -80.0 };
        let lat_max = if self.north { 84.0 } else { 0.0 };

        (lon_min, lat_min, lon_max, lat_max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wgs84_to_web_mercator_roundtrip() {
        let original = GeoCoord::new(-23.5505, -46.6333); // São Paulo

        let (x, y) = CoordinateTransformer::wgs84_to_web_mercator(&original);
        let converted = CoordinateTransformer::web_mercator_to_wgs84(x, y);

        assert!((original.lat - converted.lat).abs() < 1e-6);
        assert!((original.lon - converted.lon).abs() < 1e-6);
    }

    #[test]
    fn test_utm_zone_detection() {
        let sp = GeoCoord::new(-23.5505, -46.6333); // São Paulo
        let (zone, north) = CoordinateTransformer::get_utm_zone(&sp);

        assert_eq!(zone, 23);
        assert!(!north);
    }

    #[test]
    fn test_wgs84_to_utm_roundtrip() {
        let original = GeoCoord::new(-23.5505, -46.6333); // São Paulo
        let (zone, north) = CoordinateTransformer::get_utm_zone(&original);

        let (easting, northing) = CoordinateTransformer::wgs84_to_utm(&original, zone, north);
        let converted = CoordinateTransformer::utm_to_wgs84(easting, northing, zone, north);

        assert!((original.lat - converted.lat).abs() < 1e-4);
        assert!((original.lon - converted.lon).abs() < 1e-4);
    }

    #[test]
    fn test_web_mercator_tiles() {
        let coord = GeoCoord::new(0.0, 0.0); // Equador, meridiano de Greenwich
        let (x, y) = CoordinateTransformer::wgs84_to_web_mercator_tile(&coord, 1);

        assert_eq!(x, 1);
        assert_eq!(y, 1);
    }
}
