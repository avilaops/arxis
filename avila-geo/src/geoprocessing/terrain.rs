//! Análise de terreno com Digital Elevation Model (DEM)
//!
//! Este módulo implementa:
//! - Digital Elevation Model (DEM)
//! - Cálculo de inclinação (slope)
//! - Cálculo de orientação (aspect)
//! - Análise de visibilidade (viewshed)
//! - Sombreamento (hillshade)
//! - Perfis de elevação

use crate::coords::GeoCoord;
use crate::geoprocessing::spatial::BoundingBox;

/// Digital Elevation Model (Modelo Digital de Elevação)
#[derive(Debug, Clone)]
pub struct DigitalElevationModel {
    pub data: Vec<Vec<f64>>,
    pub bounds: BoundingBox,
    pub resolution: f64, // Resolução em graus
    pub rows: usize,
    pub cols: usize,
}

impl DigitalElevationModel {
    /// Cria um novo DEM
    pub fn new(bounds: BoundingBox, resolution: f64) -> Self {
        let cols = ((bounds.max_x - bounds.min_x) / resolution).ceil() as usize;
        let rows = ((bounds.max_y - bounds.min_y) / resolution).ceil() as usize;

        let data = vec![vec![0.0; cols]; rows];

        Self {
            data,
            bounds,
            resolution,
            rows,
            cols,
        }
    }

    /// Cria um DEM a partir de dados existentes
    pub fn from_data(data: Vec<Vec<f64>>, bounds: BoundingBox) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        let resolution = (bounds.max_x - bounds.min_x) / cols as f64;

        Self {
            data,
            bounds,
            resolution,
            rows,
            cols,
        }
    }

    /// Define a elevação em uma célula
    pub fn set_elevation(&mut self, row: usize, col: usize, elevation: f64) {
        if row < self.rows && col < self.cols {
            self.data[row][col] = elevation;
        }
    }

    /// Obtém a elevação em uma célula
    pub fn get_elevation(&self, row: usize, col: usize) -> Option<f64> {
        self.data.get(row)?.get(col).copied()
    }

    /// Obtém a elevação em uma coordenada geográfica (interpolada)
    pub fn elevation_at(&self, coord: &GeoCoord) -> Option<f64> {
        if !self.bounds.contains_point(coord.lon, coord.lat) {
            return None;
        }

        let col_f = (coord.lon - self.bounds.min_x) / self.resolution;
        let row_f = (coord.lat - self.bounds.min_y) / self.resolution;

        let col = col_f.floor() as usize;
        let row = row_f.floor() as usize;

        // Interpolação bilinear
        if col + 1 < self.cols && row + 1 < self.rows {
            let dx = col_f - col as f64;
            let dy = row_f - row as f64;

            let z00 = self.data[row][col];
            let z10 = self.data[row][col + 1];
            let z01 = self.data[row + 1][col];
            let z11 = self.data[row + 1][col + 1];

            let z0 = z00 * (1.0 - dx) + z10 * dx;
            let z1 = z01 * (1.0 - dx) + z11 * dx;
            let z = z0 * (1.0 - dy) + z1 * dy;

            Some(z)
        } else {
            self.get_elevation(row, col)
        }
    }

    /// Calcula a inclinação (slope) em graus para uma célula
    pub fn slope(&self, row: usize, col: usize) -> Option<f64> {
        if row == 0 || row >= self.rows - 1 || col == 0 || col >= self.cols - 1 {
            return None;
        }

        // Usar matriz 3x3 para calcular gradiente
        let dz_dx = (self.data[row][col + 1] - self.data[row][col - 1]) / (2.0 * self.resolution);
        let dz_dy = (self.data[row + 1][col] - self.data[row - 1][col]) / (2.0 * self.resolution);

        let slope_rad = (dz_dx.powi(2) + dz_dy.powi(2)).sqrt().atan();
        Some(slope_rad.to_degrees())
    }

    /// Calcula a orientação (aspect) em graus para uma célula
    /// Retorna ângulo de 0-360 graus onde 0 é Norte
    pub fn aspect(&self, row: usize, col: usize) -> Option<f64> {
        if row == 0 || row >= self.rows - 1 || col == 0 || col >= self.cols - 1 {
            return None;
        }

        let dz_dx = (self.data[row][col + 1] - self.data[row][col - 1]) / (2.0 * self.resolution);
        let dz_dy = (self.data[row + 1][col] - self.data[row - 1][col]) / (2.0 * self.resolution);

        if dz_dx.abs() < 1e-10 && dz_dy.abs() < 1e-10 {
            return Some(-1.0); // Área plana
        }

        let mut aspect = dz_dy.atan2(-dz_dx).to_degrees();

        // Converter para 0-360 com 0 = Norte
        if aspect < 0.0 {
            aspect = 90.0 - aspect;
        } else if aspect > 90.0 {
            aspect = 360.0 - aspect + 90.0;
        } else {
            aspect = 90.0 - aspect;
        }

        Some(aspect)
    }

    /// Calcula matriz de inclinação para todo o DEM
    pub fn slope_map(&self) -> Vec<Vec<f64>> {
        let mut slope_map = vec![vec![0.0; self.cols]; self.rows];

        for row in 1..self.rows - 1 {
            for col in 1..self.cols - 1 {
                if let Some(slope) = self.slope(row, col) {
                    slope_map[row][col] = slope;
                }
            }
        }

        slope_map
    }

    /// Calcula matriz de orientação para todo o DEM
    pub fn aspect_map(&self) -> Vec<Vec<f64>> {
        let mut aspect_map = vec![vec![0.0; self.cols]; self.rows];

        for row in 1..self.rows - 1 {
            for col in 1..self.cols - 1 {
                if let Some(aspect) = self.aspect(row, col) {
                    aspect_map[row][col] = aspect;
                }
            }
        }

        aspect_map
    }

    /// Calcula sombreamento (hillshade) para iluminação do terreno
    ///
    /// # Argumentos
    /// * `azimuth` - Ângulo azimutal da luz (0-360, 0 = Norte)
    /// * `altitude` - Ângulo de altitude da luz (0-90 graus)
    pub fn hillshade(&self, azimuth: f64, altitude: f64) -> Vec<Vec<f64>> {
        let mut hillshade = vec![vec![0.0; self.cols]; self.rows];

        let azimuth_rad = (360.0 - azimuth + 90.0).to_radians();
        let altitude_rad = altitude.to_radians();

        for row in 1..self.rows - 1 {
            for col in 1..self.cols - 1 {
                let dz_dx = (self.data[row][col + 1] - self.data[row][col - 1]) / (2.0 * self.resolution);
                let dz_dy = (self.data[row + 1][col] - self.data[row - 1][col]) / (2.0 * self.resolution);

                let slope_rad = (dz_dx.powi(2) + dz_dy.powi(2)).sqrt().atan();
                let aspect_rad = dz_dy.atan2(-dz_dx);

                let hillshade_value = ((altitude_rad.cos() * slope_rad.cos())
                    + (altitude_rad.sin() * slope_rad.sin() * (azimuth_rad - aspect_rad).cos()))
                    .max(0.0)
                    * 255.0;

                hillshade[row][col] = hillshade_value;
            }
        }

        hillshade
    }

    /// Análise de visibilidade (viewshed) a partir de um ponto observador
    ///
    /// # Argumentos
    /// * `observer_row` - Linha do observador
    /// * `observer_col` - Coluna do observador
    /// * `observer_height` - Altura do observador acima do terreno
    pub fn viewshed(&self, observer_row: usize, observer_col: usize, observer_height: f64) -> Vec<Vec<bool>> {
        let mut visible = vec![vec![false; self.cols]; self.rows];

        if observer_row >= self.rows || observer_col >= self.cols {
            return visible;
        }

        let observer_elev = self.data[observer_row][observer_col] + observer_height;
        visible[observer_row][observer_col] = true;

        // Verificar visibilidade para cada célula usando ray tracing
        for row in 0..self.rows {
            for col in 0..self.cols {
                if row == observer_row && col == observer_col {
                    continue;
                }

                visible[row][col] = self.is_visible(
                    observer_row,
                    observer_col,
                    observer_elev,
                    row,
                    col,
                );
            }
        }

        visible
    }

    /// Verifica se um ponto é visível do observador
    fn is_visible(
        &self,
        obs_row: usize,
        obs_col: usize,
        obs_elev: f64,
        target_row: usize,
        target_col: usize,
    ) -> bool {
        let target_elev = self.data[target_row][target_col];

        // Linha de visão do observador ao alvo
        let dx = target_col as f64 - obs_col as f64;
        let dy = target_row as f64 - obs_row as f64;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < 1.0 {
            return true;
        }

        let steps = distance.ceil() as usize;
        let step_x = dx / steps as f64;
        let step_y = dy / steps as f64;

        // Ângulo da linha de visão
        let sight_angle = (target_elev - obs_elev).atan2(distance * self.resolution);

        // Verificar se há obstruções ao longo da linha
        for i in 1..steps {
            let x = obs_col as f64 + step_x * i as f64;
            let y = obs_row as f64 + step_y * i as f64;

            let col = x.round() as usize;
            let row = y.round() as usize;

            if row >= self.rows || col >= self.cols {
                continue;
            }

            let current_dist = (i as f64 / steps as f64) * distance;
            let terrain_elev = self.data[row][col];
            let terrain_angle = (terrain_elev - obs_elev).atan2(current_dist * self.resolution);

            if terrain_angle > sight_angle {
                return false; // Obstruído
            }
        }

        true
    }

    /// Extrai perfil de elevação ao longo de uma linha
    pub fn elevation_profile(&self, start: &GeoCoord, end: &GeoCoord, num_samples: usize) -> Vec<(f64, f64)> {
        let mut profile = Vec::with_capacity(num_samples);

        let dx = end.lon - start.lon;
        let dy = end.lat - start.lat;
        let total_distance = (dx * dx + dy * dy).sqrt();

        for i in 0..num_samples {
            let t = i as f64 / (num_samples - 1) as f64;
            let coord = GeoCoord {
                lon: start.lon + t * dx,
                lat: start.lat + t * dy,
            };

            let distance = t * total_distance;
            let elevation = self.elevation_at(&coord).unwrap_or(0.0);

            profile.push((distance, elevation));
        }

        profile
    }

    /// Calcula contornos (isolinhas) para uma elevação específica
    pub fn contour_lines(&self, elevation: f64) -> Vec<Vec<GeoCoord>> {
        let mut contours = Vec::new();

        // Algoritmo de marching squares simplificado
        for row in 0..self.rows - 1 {
            for col in 0..self.cols - 1 {
                let z00 = self.data[row][col];
                let z10 = self.data[row][col + 1];
                let z01 = self.data[row + 1][col];
                let z11 = self.data[row + 1][col + 1];

                // Verificar se o contorno passa por esta célula
                let crosses = (z00 < elevation) != (z10 < elevation)
                    || (z10 < elevation) != (z11 < elevation)
                    || (z11 < elevation) != (z01 < elevation)
                    || (z01 < elevation) != (z00 < elevation);

                if crosses {
                    // Criar segmento de contorno (simplificado)
                    let lat = self.bounds.min_y + (row as f64 + 0.5) * self.resolution;
                    let lon = self.bounds.min_x + (col as f64 + 0.5) * self.resolution;
                    contours.push(vec![GeoCoord { lat, lon }]);
                }
            }
        }

        contours
    }

    /// Calcula curvatura do terreno
    pub fn curvature(&self, row: usize, col: usize) -> Option<f64> {
        if row < 2 || row >= self.rows - 2 || col < 2 || col >= self.cols - 2 {
            return None;
        }

        // Segunda derivada
        let d2z_dx2 = (self.data[row][col + 1] - 2.0 * self.data[row][col] + self.data[row][col - 1])
            / (self.resolution * self.resolution);
        let d2z_dy2 = (self.data[row + 1][col] - 2.0 * self.data[row][col] + self.data[row - 1][col])
            / (self.resolution * self.resolution);

        Some(d2z_dx2 + d2z_dy2)
    }

    /// Identifica picos (elevações locais máximas)
    pub fn find_peaks(&self, min_prominence: f64) -> Vec<(usize, usize, f64)> {
        let mut peaks = Vec::new();

        for row in 1..self.rows - 1 {
            for col in 1..self.cols - 1 {
                let center = self.data[row][col];
                let mut is_peak = true;

                // Verificar vizinhos 3x3
                for dr in -1..=1 {
                    for dc in -1..=1 {
                        if dr == 0 && dc == 0 {
                            continue;
                        }

                        let neighbor = self.data[(row as i32 + dr) as usize][(col as i32 + dc) as usize];
                        if neighbor >= center {
                            is_peak = false;
                            break;
                        }
                    }
                    if !is_peak {
                        break;
                    }
                }

                if is_peak {
                    let min_neighbor = (row.saturating_sub(1)..=(row + 1).min(self.rows - 1))
                        .flat_map(|r| {
                            (col.saturating_sub(1)..=(col + 1).min(self.cols - 1))
                                .map(move |c| self.data[r][c])
                        })
                        .filter(|&z| z < center)
                        .min_by(|a, b| a.partial_cmp(b).unwrap())
                        .unwrap_or(center);

                    let prominence = center - min_neighbor;
                    if prominence >= min_prominence {
                        peaks.push((row, col, center));
                    }
                }
            }
        }

        peaks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dem_creation() {
        let bounds = BoundingBox::new(-50.0, -30.0, -40.0, -20.0);
        let dem = DigitalElevationModel::new(bounds, 0.01);

        assert!(dem.rows > 0);
        assert!(dem.cols > 0);
    }

    #[test]
    fn test_slope_calculation() {
        let bounds = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let mut dem = DigitalElevationModel::new(bounds, 1.0);

        // Criar inclinação artificial
        for row in 0..dem.rows {
            for col in 0..dem.cols {
                dem.set_elevation(row, col, row as f64 * 10.0);
            }
        }

        let slope = dem.slope(5, 5);
        assert!(slope.is_some());
        assert!(slope.unwrap() > 0.0);
    }

    #[test]
    fn test_elevation_profile() {
        let bounds = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let mut dem = DigitalElevationModel::new(bounds, 0.1);

        for row in 0..dem.rows {
            for col in 0..dem.cols {
                dem.set_elevation(row, col, (row + col) as f64);
            }
        }

        let start = GeoCoord::new(0.0, 0.0);
        let end = GeoCoord::new(5.0, 5.0);
        let profile = dem.elevation_profile(&start, &end, 10);

        assert_eq!(profile.len(), 10);
    }
}
