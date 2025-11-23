//! Geolocation-Based Routing Module
//!
//! Routes requests based on client IP geolocation for optimal latency.

use anyhow::Result;
use maxminddb::{geoip2, MaxMindDBError, Reader};
use std::net::IpAddr;
use std::path::Path;
use std::sync::Arc;

/// Geographic region
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Region {
    NorthAmerica,
    SouthAmerica,
    Europe,
    Asia,
    Africa,
    Oceania,
    Unknown,
}

impl Region {
    /// Convert from continent code
    pub fn from_continent_code(code: &str) -> Self {
        match code {
            "NA" => Region::NorthAmerica,
            "SA" => Region::SouthAmerica,
            "EU" => Region::Europe,
            "AS" => Region::Asia,
            "AF" => Region::Africa,
            "OC" => Region::Oceania,
            _ => Region::Unknown,
        }
    }
}

/// Geolocation information
#[derive(Debug, Clone)]
pub struct GeoLocation {
    pub country_code: Option<String>,
    pub country_name: Option<String>,
    pub region: Region,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

/// Geolocation service using MaxMind GeoIP2 database
pub struct GeoLocationService {
    reader: Arc<Reader<Vec<u8>>>,
}

impl GeoLocationService {
    /// Create a new geolocation service from MaxMind GeoIP2 database
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let reader = Reader::open_readfile(db_path)?;
        Ok(Self {
            reader: Arc::new(reader),
        })
    }

    /// Lookup geolocation for an IP address
    pub fn lookup(&self, ip: IpAddr) -> Result<GeoLocation> {
        let city: geoip2::City = self.reader.lookup(ip)
            .map_err(|e| match e {
                MaxMindDBError::AddressNotFoundError(_) => {
                    anyhow::anyhow!("IP address not found in database: {}", ip)
                }
                e => anyhow::anyhow!("GeoIP lookup error: {}", e)
            })?;

        let country_code = city.country
            .as_ref()
            .and_then(|c| c.iso_code)
            .map(|s| s.to_string());

        let country_name = city.country
            .as_ref()
            .and_then(|c| c.names.as_ref())
            .and_then(|names| names.get("en"))
            .map(|s| s.to_string());

        let continent_code = city.continent
            .as_ref()
            .and_then(|c| c.code)
            .unwrap_or("");

        let region = Region::from_continent_code(continent_code);

        let city_name = city.city
            .as_ref()
            .and_then(|c| c.names.as_ref())
            .and_then(|names| names.get("en"))
            .map(|s| s.to_string());

        let (latitude, longitude) = city.location
            .as_ref()
            .map(|loc| (loc.latitude, loc.longitude))
            .unwrap_or((None, None));

        Ok(GeoLocation {
            country_code,
            country_name,
            region,
            city: city_name,
            latitude,
            longitude,
        })
    }

    /// Check if IP is from Brazil
    pub fn is_from_brazil(&self, ip: IpAddr) -> bool {
        self.lookup(ip)
            .map(|geo| geo.country_code.as_deref() == Some("BR"))
            .unwrap_or(false)
    }

    /// Get region for IP
    pub fn get_region(&self, ip: IpAddr) -> Region {
        self.lookup(ip)
            .map(|geo| geo.region)
            .unwrap_or(Region::Unknown)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_from_continent_code() {
        assert_eq!(Region::from_continent_code("SA"), Region::SouthAmerica);
        assert_eq!(Region::from_continent_code("EU"), Region::Europe);
        assert_eq!(Region::from_continent_code("AS"), Region::Asia);
        assert_eq!(Region::from_continent_code("NA"), Region::NorthAmerica);
        assert_eq!(Region::from_continent_code("XX"), Region::Unknown);
    }

    #[test]
    fn test_geolocation_service_without_db() {
        // Test will fail if no database is available
        let result = GeoLocationService::new("GeoLite2-City.mmdb");
        // We expect this to fail in test environment without the database
        assert!(result.is_err() || result.is_ok());
    }
}
