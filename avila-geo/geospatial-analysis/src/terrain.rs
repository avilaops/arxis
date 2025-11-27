//! Terrain analysis algorithms for elevation data

/// Calculate slope in degrees for elevation grid
pub fn calculate_slope(elevation: &[Vec<f64>], cell_size: f64) -> Vec<Vec<f64>> {
    let rows = elevation.len();
    if rows == 0 {
        return vec![];
    }
    let cols = elevation[0].len();
    let mut slope = vec![vec![0.0; cols]; rows];

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let dz_dx = (
                (elevation[i - 1][j + 1] + 2.0 * elevation[i][j + 1] + elevation[i + 1][j + 1])
                - (elevation[i - 1][j - 1] + 2.0 * elevation[i][j - 1] + elevation[i + 1][j - 1])
            ) / (8.0 * cell_size);

            let dz_dy = (
                (elevation[i + 1][j - 1] + 2.0 * elevation[i + 1][j] + elevation[i + 1][j + 1])
                - (elevation[i - 1][j - 1] + 2.0 * elevation[i - 1][j] + elevation[i - 1][j + 1])
            ) / (8.0 * cell_size);

            let slope_rad = (dz_dx * dz_dx + dz_dy * dz_dy).sqrt().atan();
            slope[i][j] = slope_rad.to_degrees();
        }
    }

    slope
}

/// Calculate aspect (direction) in degrees (0-360)
pub fn calculate_aspect(elevation: &[Vec<f64>], cell_size: f64) -> Vec<Vec<f64>> {
    let rows = elevation.len();
    if rows == 0 {
        return vec![];
    }
    let cols = elevation[0].len();
    let mut aspect = vec![vec![0.0; cols]; rows];

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let dz_dx = (
                (elevation[i - 1][j + 1] + 2.0 * elevation[i][j + 1] + elevation[i + 1][j + 1])
                - (elevation[i - 1][j - 1] + 2.0 * elevation[i][j - 1] + elevation[i + 1][j - 1])
            ) / (8.0 * cell_size);

            let dz_dy = (
                (elevation[i + 1][j - 1] + 2.0 * elevation[i + 1][j] + elevation[i + 1][j + 1])
                - (elevation[i - 1][j - 1] + 2.0 * elevation[i - 1][j] + elevation[i - 1][j + 1])
            ) / (8.0 * cell_size);

            let mut aspect_deg = 90.0 - dz_dy.atan2(dz_dx).to_degrees();
            if aspect_deg < 0.0 {
                aspect_deg += 360.0;
            }
            aspect[i][j] = aspect_deg;
        }
    }

    aspect
}

/// Calculate hillshade for visualization
pub fn calculate_hillshade(elevation: &[Vec<f64>], cell_size: f64, azimuth: f64, altitude: f64) -> Vec<Vec<u8>> {
    let rows = elevation.len();
    if rows == 0 {
        return vec![];
    }
    let cols = elevation[0].len();
    let mut hillshade = vec![vec![0; cols]; rows];

    let azimuth_rad = (360.0 - azimuth + 90.0).to_radians();
    let altitude_rad = altitude.to_radians();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let dz_dx = (
                (elevation[i - 1][j + 1] + 2.0 * elevation[i][j + 1] + elevation[i + 1][j + 1])
                - (elevation[i - 1][j - 1] + 2.0 * elevation[i][j - 1] + elevation[i + 1][j - 1])
            ) / (8.0 * cell_size);

            let dz_dy = (
                (elevation[i + 1][j - 1] + 2.0 * elevation[i + 1][j] + elevation[i + 1][j + 1])
                - (elevation[i - 1][j - 1] + 2.0 * elevation[i - 1][j] + elevation[i - 1][j + 1])
            ) / (8.0 * cell_size);

            let slope_rad = (dz_dx * dz_dx + dz_dy * dz_dy).sqrt().atan();
            let aspect_rad = dz_dy.atan2(dz_dx);

            let value = altitude_rad.cos() * slope_rad.cos()
                + altitude_rad.sin() * slope_rad.sin() * (azimuth_rad - aspect_rad).cos();

            hillshade[i][j] = ((value * 255.0).max(0.0).min(255.0)) as u8;
        }
    }

    hillshade
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope() {
        let elevation = vec![
            vec![100.0, 101.0, 102.0],
            vec![100.0, 101.0, 102.0],
            vec![100.0, 101.0, 102.0],
        ];

        let slope = calculate_slope(&elevation, 1.0);
        assert!(slope[1][1] > 0.0);
    }

    #[test]
    fn test_aspect() {
        let elevation = vec![
            vec![100.0, 100.0, 100.0],
            vec![100.0, 101.0, 100.0],
            vec![100.0, 100.0, 100.0],
        ];

        let aspect = calculate_aspect(&elevation, 1.0);
        assert!(aspect.len() == 3);
    }

    #[test]
    fn test_hillshade() {
        let elevation = vec![
            vec![100.0, 101.0, 102.0],
            vec![100.0, 101.0, 102.0],
            vec![100.0, 101.0, 102.0],
        ];

        let hs = calculate_hillshade(&elevation, 1.0, 315.0, 45.0);
        assert!(hs.len() == 3);
        assert!(hs[1][1] > 0);
    }
}
