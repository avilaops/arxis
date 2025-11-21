//! Astronomy and astrophysics functions

use crate::core::{DataFrame, Series};
use crate::error::Result;

/// Physical constants
const SPEED_OF_LIGHT: f64 = 299792.458; // km/s
const H0: f64 = 70.0; // Hubble constant in km/s/Mpc
const PI: f64 = std::f64::consts::PI;

/// Calculate luminosity distance from redshift (standalone function)
/// Uses simplified Hubble law: D_L = (c * z) / H0
pub fn luminosity_distance(z: f64) -> Result<f64> {
    Ok((SPEED_OF_LIGHT * z) / H0) // in Mpc
}

/// Calculate angular separation between two points on the sky (standalone function)
/// Uses haversine formula
///
/// # Arguments
/// * `ra1`, `dec1` - Right ascension and declination of first object (degrees)
/// * `ra2`, `dec2` - Right ascension and declination of second object (degrees)
///
/// # Returns
/// Angular separation in degrees
pub fn angular_separation(ra1: f64, dec1: f64, ra2: f64, dec2: f64) -> Result<f64> {
    let ra1_rad = ra1.to_radians();
    let dec1_rad = dec1.to_radians();
    let ra2_rad = ra2.to_radians();
    let dec2_rad = dec2.to_radians();

    let delta_ra = ra2_rad - ra1_rad;
    let delta_dec = dec2_rad - dec1_rad;

    let a = (delta_dec / 2.0).sin().powi(2)
        + dec1_rad.cos() * dec2_rad.cos() * (delta_ra / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();

    Ok(c.to_degrees())
}

/// Calculate absolute magnitude from apparent magnitude and distance (standalone function)
///
/// # Arguments
/// * `apparent_mag` - Apparent magnitude
/// * `distance_mpc` - Luminosity distance in Mpc
///
/// # Returns
/// Absolute magnitude
pub fn absolute_magnitude(apparent_mag: f64, distance_mpc: f64) -> Result<f64> {
    let distance_pc = distance_mpc * 1e6; // Convert Mpc to pc
    let distance_modulus = 5.0 * distance_pc.log10() - 5.0;
    Ok(apparent_mag - distance_modulus)
}

impl DataFrame {
    /// Apply redshift correction to wavelength
    ///
    /// # Arguments
    /// * `wavelength_col` - Column with observed wavelengths
    /// * `z` - Redshift value
    ///
    /// # Returns
    /// DataFrame with rest-frame wavelength
    pub fn redshift_correction(&self, wavelength_col: &str, z: f64) -> Result<Self> {
        let wavelengths = self.column(wavelength_col)?;
        let n = wavelengths.len();

        let rest_wavelengths: Vec<f64> = (0..n)
            .map(|i| {
                let observed = wavelengths.get_f64(i).unwrap_or(0.0);
                observed / (1.0 + z)
            })
            .collect();

        let mut result = self.clone();
        result = result.with_column(Series::new("rest_wavelength", rest_wavelengths))?;
        Ok(result)
    }

    /// Calculate luminosity distance from redshift
    ///
    /// Uses simplified Hubble law: D_L = (c * z) / H0
    pub fn luminosity_distance(&self, redshift_col: &str) -> Result<Self> {
        let redshifts = self.column(redshift_col)?;
        let n = redshifts.len();

        let distances: Vec<f64> = (0..n)
            .map(|i| {
                let z = redshifts.get_f64(i).unwrap_or(0.0);
                (SPEED_OF_LIGHT * z) / H0 // in Mpc
            })
            .collect();

        let mut result = self.clone();
        result = result.with_column(Series::new("luminosity_distance_mpc", distances))?;
        Ok(result)
    }

    /// Calculate angular separation between two celestial coordinates
    ///
    /// Uses the haversine formula
    ///
    /// # Arguments
    /// * `ra1`, `dec1` - Right ascension and declination of first object (degrees)
    /// * `ra2`, `dec2` - Right ascension and declination of second object (degrees)
    ///
    /// # Returns
    /// Angular separation in degrees
    pub fn angular_separation(&self, ra1: &str, dec1: &str, ra2: &str, dec2: &str) -> Result<Self> {
        let ra1_series = self.column(ra1)?;
        let dec1_series = self.column(dec1)?;
        let ra2_series = self.column(ra2)?;
        let dec2_series = self.column(dec2)?;
        let n = ra1_series.len();

        let separations: Vec<f64> = (0..n)
            .map(|i| {
                let ra1_deg = ra1_series.get_f64(i).unwrap_or(0.0);
                let dec1_deg = dec1_series.get_f64(i).unwrap_or(0.0);
                let ra2_deg = ra2_series.get_f64(i).unwrap_or(0.0);
                let dec2_deg = dec2_series.get_f64(i).unwrap_or(0.0);

                // Convert to radians
                let ra1_rad = ra1_deg.to_radians();
                let dec1_rad = dec1_deg.to_radians();
                let ra2_rad = ra2_deg.to_radians();
                let dec2_rad = dec2_deg.to_radians();

                // Haversine formula
                let delta_ra = ra2_rad - ra1_rad;
                let delta_dec = dec2_rad - dec1_rad;

                let a = (delta_dec / 2.0).sin().powi(2)
                    + dec1_rad.cos() * dec2_rad.cos() * (delta_ra / 2.0).sin().powi(2);

                let c = 2.0 * a.sqrt().asin();

                c.to_degrees()
            })
            .collect();

        let mut result = self.clone();
        result = result.with_column(Series::new("angular_separation_deg", separations))?;
        Ok(result)
    }

    /// Convert galactic coordinates (l, b) to equatorial (RA, Dec)
    pub fn galactic_to_equatorial(&self, l_col: &str, b_col: &str) -> Result<Self> {
        // TODO: Implement coordinate transformation
        // Requires rotation matrices and proper epoch handling
        Err(crate::error::AvilaError::not_implemented(
            "galactic_to_equatorial",
        ))
    }

    /// Calculate absolute magnitude from apparent magnitude and distance
    pub fn absolute_magnitude(&self, apparent_mag: &str, distance_pc: &str) -> Result<Self> {
        let app_mag = self.column(apparent_mag)?;
        let dist = self.column(distance_pc)?;
        let n = app_mag.len();

        let abs_mag: Vec<f64> = (0..n)
            .map(|i| {
                let m = app_mag.get_f64(i).unwrap_or(0.0);
                let d = dist.get_f64(i).unwrap_or(1.0);

                // M = m - 5 * (log10(d) - 1)
                m - 5.0 * (d.log10() - 1.0)
            })
            .collect();

        let mut result = self.clone();
        result = result.with_column(Series::new("absolute_magnitude", abs_mag))?;
        Ok(result)
    }

    /// Calculate color index (e.g., B-V)
    pub fn color_index(&self, mag1: &str, mag2: &str, name: &str) -> Result<Self> {
        let m1 = self.column(mag1)?;
        let m2 = self.column(mag2)?;
        let n = m1.len();

        let color: Vec<f64> = (0..n)
            .map(|i| {
                let mag1_val = m1.get_f64(i).unwrap_or(0.0);
                let mag2_val = m2.get_f64(i).unwrap_or(0.0);
                mag1_val - mag2_val
            })
            .collect();

        let mut result = self.clone();
        result = result.with_column(Series::new(name, color))?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_redshift_correction() {
        let df = DataFrame::new(vec![
            Series::new("wavelength", vec![600.0, 700.0, 800.0]), // nm
        ])
        .unwrap();

        let z = 0.1; // 10% redshift
        let corrected = df.redshift_correction("wavelength", z).unwrap();

        let rest = corrected.column("rest_wavelength").unwrap();
        let expected = 600.0 / 1.1;

        assert_abs_diff_eq!(rest.get_f64(0).unwrap(), expected, epsilon = 0.01);
    }

    #[test]
    fn test_angular_separation() {
        // Two objects at same position
        let df = DataFrame::new(vec![
            Series::new("ra1", vec![180.0]),
            Series::new("dec1", vec![45.0]),
            Series::new("ra2", vec![180.0]),
            Series::new("dec2", vec![45.0]),
        ])
        .unwrap();

        let result = df.angular_separation("ra1", "dec1", "ra2", "dec2").unwrap();
        let sep = result.column("angular_separation_deg").unwrap();

        // Separation should be ~0
        assert!(sep.get_f64(0).unwrap() < 0.001);
    }
}
