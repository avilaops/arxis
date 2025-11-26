//! Análise espacial e interpolação
//!
//! Este módulo implementa:
//! - Cálculos de distância (Haversine, Euclidiana)
//! - Interpolação espacial (IDW - Inverse Distance Weighting)
//! - Kernel Density Estimation (KDE)
//! - Análise estatística espacial (Moran's I, Getis-Ord Gi*)
//! - Análise de clusters espaciais

use crate::coords::GeoCoord;
use crate::geoprocessing::spatial::{BoundingBox, SpatialFeature};
use std::collections::HashMap;

/// Constante do raio da Terra em metros
pub const EARTH_RADIUS_METERS: f64 = 6371000.0;

/// Calcula a distância Haversine entre dois pontos geográficos (em metros)
pub fn haversine_distance(p1: &GeoCoord, p2: &GeoCoord) -> f64 {
    let lat1 = p1.lat.to_radians();
    let lat2 = p2.lat.to_radians();
    let dlat = (p2.lat - p1.lat).to_radians();
    let dlon = (p2.lon - p1.lon).to_radians();

    let a = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_METERS * c
}

/// Calcula a distância euclidiana entre dois pontos
pub fn euclidean_distance(p1: &GeoCoord, p2: &GeoCoord) -> f64 {
    let dx = p2.lon - p1.lon;
    let dy = p2.lat - p1.lat;
    (dx * dx + dy * dy).sqrt()
}

/// Interpolação espacial usando Inverse Distance Weighting (IDW)
///
/// # Argumentos
/// * `points` - Pontos com valores conhecidos
/// * `target` - Ponto onde deseja-se estimar o valor
/// * `power` - Expoente da distância (geralmente 2.0)
///
/// # Retorna
/// Valor interpolado no ponto alvo
pub fn idw_interpolation(points: &[(GeoCoord, f64)], target: &GeoCoord, power: f64) -> f64 {
    let mut weighted_sum = 0.0;
    let mut weight_sum = 0.0;

    for (point, value) in points {
        let distance = haversine_distance(point, target);

        if distance < 1e-10 {
            return *value; // Ponto coincidente
        }

        let weight = 1.0 / distance.powf(power);
        weighted_sum += weight * value;
        weight_sum += weight;
    }

    if weight_sum > 0.0 {
        weighted_sum / weight_sum
    } else {
        0.0
    }
}

/// Grid regular para análise raster
#[derive(Debug, Clone)]
pub struct Grid {
    pub bounds: BoundingBox,
    pub rows: usize,
    pub cols: usize,
    pub cell_width: f64,
    pub cell_height: f64,
}

impl Grid {
    /// Cria um novo grid
    pub fn new(bounds: BoundingBox, rows: usize, cols: usize) -> Self {
        let cell_width = (bounds.max_x - bounds.min_x) / cols as f64;
        let cell_height = (bounds.max_y - bounds.min_y) / rows as f64;

        Self {
            bounds,
            rows,
            cols,
            cell_width,
            cell_height,
        }
    }

    /// Retorna o centro de uma célula
    pub fn cell_center(&self, row: usize, col: usize) -> GeoCoord {
        let lon = self.bounds.min_x + (col as f64 + 0.5) * self.cell_width;
        let lat = self.bounds.min_y + (row as f64 + 0.5) * self.cell_height;
        GeoCoord { lat, lon }
    }

    /// Retorna a célula que contém um ponto
    pub fn cell_at(&self, coord: &GeoCoord) -> Option<(usize, usize)> {
        if !self.bounds.contains_point(coord.lon, coord.lat) {
            return None;
        }

        let col = ((coord.lon - self.bounds.min_x) / self.cell_width) as usize;
        let row = ((coord.lat - self.bounds.min_y) / self.cell_height) as usize;

        if row < self.rows && col < self.cols {
            Some((row, col))
        } else {
            None
        }
    }
}

/// Kernel Density Estimation (KDE) para análise de densidade de pontos
///
/// # Argumentos
/// * `points` - Pontos para análise de densidade
/// * `grid` - Grid onde calcular a densidade
/// * `bandwidth` - Largura de banda do kernel (em graus)
///
/// # Retorna
/// Matriz de densidade para cada célula do grid
pub fn kernel_density(points: &[GeoCoord], grid: &Grid, bandwidth: f64) -> Vec<Vec<f64>> {
    let mut density = vec![vec![0.0; grid.cols]; grid.rows];

    let pi_sqrt = (2.0 * std::f64::consts::PI).sqrt();

    for row in 0..grid.rows {
        for col in 0..grid.cols {
            let cell_center = grid.cell_center(row, col);

            for point in points {
                let dist = euclidean_distance(point, &cell_center);

                // Gaussian kernel
                let kernel_value = (-0.5 * (dist / bandwidth).powi(2)).exp() / (bandwidth * pi_sqrt);

                density[row][col] += kernel_value;
            }
        }
    }

    density
}

/// Pesos espaciais para análise estatística
#[derive(Debug, Clone)]
pub struct SpatialWeights {
    weights: HashMap<(usize, usize), f64>,
}

impl SpatialWeights {
    /// Cria matriz de pesos espaciais baseada em distância
    pub fn from_distance(features: &[SpatialFeature], threshold: f64) -> Self {
        let mut weights = HashMap::new();

        for i in 0..features.len() {
            for j in 0..features.len() {
                if i == j {
                    weights.insert((i, j), 0.0);
                    continue;
                }

                let dist = Self::feature_distance(&features[i], &features[j]);
                let weight = if dist <= threshold {
                    1.0 / dist.max(1e-10)
                } else {
                    0.0
                };

                weights.insert((i, j), weight);
            }
        }

        Self { weights }
    }

    /// Cria matriz de pesos espaciais baseada em k vizinhos mais próximos
    pub fn from_k_nearest(features: &[SpatialFeature], k: usize) -> Self {
        let mut weights = HashMap::new();

        for i in 0..features.len() {
            let mut distances: Vec<(usize, f64)> = features
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(j, f)| (j, Self::feature_distance(&features[i], f)))
                .collect();

            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            for (rank, (j, _)) in distances.iter().take(k).enumerate() {
                weights.insert((i, *j), 1.0 / (rank + 1) as f64);
            }
        }

        Self { weights }
    }

    fn feature_distance(f1: &SpatialFeature, f2: &SpatialFeature) -> f64 {
        let b1 = f1.bounding_box();
        let b2 = f2.bounding_box();
        let c1 = b1.center();
        let c2 = b2.center();

        euclidean_distance(&GeoCoord::new(c1.1, c1.0), &GeoCoord::new(c2.1, c2.0))
    }

    /// Obtém o peso entre duas features
    pub fn get(&self, i: usize, j: usize) -> f64 {
        *self.weights.get(&(i, j)).unwrap_or(&0.0)
    }

    /// Normaliza os pesos por linha
    pub fn row_standardize(&mut self) {
        let max_i = self
            .weights
            .keys()
            .map(|(i, _)| *i)
            .max()
            .unwrap_or(0);

        for i in 0..=max_i {
            let row_sum: f64 = self
                .weights
                .iter()
                .filter(|((row, _), _)| *row == i)
                .map(|(_, w)| w)
                .sum();

            if row_sum > 0.0 {
                for ((row, col), weight) in self.weights.iter_mut() {
                    if *row == i {
                        *weight /= row_sum;
                    }
                }
            }
        }
    }
}

/// Calcula o índice I de Moran (autocorrelação espacial global)
///
/// Valores próximos de +1 indicam clustering espacial positivo
/// Valores próximos de -1 indicam dispersão espacial
/// Valores próximos de 0 indicam aleatoriedade espacial
pub fn morans_i(features: &[SpatialFeature], values: &[f64], weights: &SpatialWeights) -> f64 {
    let n = features.len();
    if n == 0 {
        return 0.0;
    }

    let mean = values.iter().sum::<f64>() / n as f64;

    let mut numerator = 0.0;
    let mut denominator = 0.0;
    let mut w_sum = 0.0;

    for i in 0..n {
        for j in 0..n {
            let wij = weights.get(i, j);
            numerator += wij * (values[i] - mean) * (values[j] - mean);
            w_sum += wij;
        }
        denominator += (values[i] - mean).powi(2);
    }

    if denominator == 0.0 || w_sum == 0.0 {
        return 0.0;
    }

    (n as f64 / w_sum) * (numerator / denominator)
}

/// Calcula o Getis-Ord Gi* (análise de hotspots)
///
/// Valores positivos altos indicam hotspots (clusters de valores altos)
/// Valores negativos baixos indicam coldspots (clusters de valores baixos)
pub fn getis_ord_gi_star(
    features: &[SpatialFeature],
    values: &[f64],
    weights: &SpatialWeights,
    index: usize,
) -> f64 {
    let n = features.len();
    if n == 0 || index >= n {
        return 0.0;
    }

    let mean = values.iter().sum::<f64>() / n as f64;
    let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
    let std_dev = variance.sqrt();

    if std_dev == 0.0 {
        return 0.0;
    }

    let mut weighted_sum = 0.0;
    let mut weight_sum = 0.0;

    for j in 0..n {
        let wij = weights.get(index, j);
        weighted_sum += wij * values[j];
        weight_sum += wij;
    }

    if weight_sum == 0.0 {
        return 0.0;
    }

    let numerator = weighted_sum - mean * weight_sum;
    let denominator = std_dev * ((n as f64 * weight_sum.powi(2) - weight_sum.powi(2)) / (n as f64 - 1.0)).sqrt();

    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}

/// Análise de Local Indicators of Spatial Association (LISA)
pub struct LisaAnalysis {
    pub local_morans_i: Vec<f64>,
    pub z_scores: Vec<f64>,
    pub p_values: Vec<f64>,
}

impl LisaAnalysis {
    /// Calcula LISA para cada feature
    pub fn compute(features: &[SpatialFeature], values: &[f64], weights: &SpatialWeights) -> Self {
        let n = features.len();
        let mean = values.iter().sum::<f64>() / n as f64;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;

        let mut local_morans_i = Vec::with_capacity(n);
        let mut z_scores = Vec::with_capacity(n);
        let mut p_values = Vec::with_capacity(n);

        for i in 0..n {
            let zi = values[i] - mean;
            let mut spatial_lag = 0.0;

            for j in 0..n {
                let wij = weights.get(i, j);
                spatial_lag += wij * (values[j] - mean);
            }

            let local_i = zi * spatial_lag;
            local_morans_i.push(local_i);

            // Calcular z-score (simplificado)
            let z = if variance > 0.0 {
                local_i / variance
            } else {
                0.0
            };
            z_scores.push(z);

            // p-value aproximado (distribuição normal)
            let p = 2.0 * (1.0 - normal_cdf(z.abs()));
            p_values.push(p);
        }

        Self {
            local_morans_i,
            z_scores,
            p_values,
        }
    }

    /// Classifica features em categorias LISA
    pub fn classify(&self, values: &[f64], significance_level: f64) -> Vec<LisaCategory> {
        let n = values.len();
        let mean = values.iter().sum::<f64>() / n as f64;

        self.local_morans_i
            .iter()
            .zip(self.p_values.iter())
            .enumerate()
            .map(|(i, (&local_i, &p_value))| {
                if p_value > significance_level {
                    LisaCategory::NotSignificant
                } else if local_i > 0.0 {
                    if values[i] > mean {
                        LisaCategory::HighHigh
                    } else {
                        LisaCategory::LowLow
                    }
                } else {
                    if values[i] > mean {
                        LisaCategory::HighLow
                    } else {
                        LisaCategory::LowHigh
                    }
                }
            })
            .collect()
    }
}

/// Categorias de LISA
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LisaCategory {
    HighHigh,       // Hotspot
    LowLow,         // Coldspot
    HighLow,        // Outlier alto
    LowHigh,        // Outlier baixo
    NotSignificant, // Não significativo
}

/// Função de distribuição cumulativa normal (aproximação)
fn normal_cdf(x: f64) -> f64 {
    0.5 * (1.0 + erf(x / 2.0_f64.sqrt()))
}

/// Função erro (aproximação de Abramowitz e Stegun)
fn erf(x: f64) -> f64 {
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;

    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();

    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x).exp();

    sign * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_haversine_distance() {
        let p1 = GeoCoord::new(-23.5505, -46.6333); // São Paulo
        let p2 = GeoCoord::new(-22.9068, -43.1729); // Rio de Janeiro

        let dist = haversine_distance(&p1, &p2);
        assert!(dist > 350000.0 && dist < 450000.0); // ~400km
    }

    #[test]
    fn test_idw_interpolation() {
        let points = vec![
            (GeoCoord::new(0.0, 0.0), 100.0),
            (GeoCoord::new(10.0, 0.0), 200.0),
            (GeoCoord::new(0.0, 10.0), 150.0),
        ];

        let target = GeoCoord::new(5.0, 5.0);
        let value = idw_interpolation(&points, &target, 2.0);

        assert!(value > 100.0 && value < 200.0);
    }

    #[test]
    fn test_grid_cell_at() {
        let bounds = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let grid = Grid::new(bounds, 10, 10);

        let coord = GeoCoord::new(5.0, 5.0);
        let cell = grid.cell_at(&coord);

        assert!(cell.is_some());
    }
}
