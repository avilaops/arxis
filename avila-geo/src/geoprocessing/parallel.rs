//! Processamento paralelo para operações de geoprocessamento
//!
//! Este módulo fornece versões paralelas de operações pesadas usando Rayon

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::coords::GeoCoord;
use crate::geoprocessing::analysis::{Grid, haversine_distance, idw_interpolation};
use crate::geoprocessing::spatial::{BoundingBox, SpatialFeature};
use crate::geoprocessing::operations::point_in_polygon;

/// Kernel Density paralelo - até 10x mais rápido em multi-core
#[cfg(feature = "parallel")]
pub fn kernel_density_parallel(
    points: &[GeoCoord],
    grid: &Grid,
    bandwidth: f64,
) -> Vec<Vec<f64>> {
    let rows = grid.rows;
    let cols = grid.cols;
    let pi_sqrt = (2.0 * std::f64::consts::PI).sqrt();

    // Processar linhas em paralelo
    (0..rows)
        .into_par_iter()
        .map(|row| {
            let mut row_data = vec![0.0; cols];

            for col in 0..cols {
                let cell_center = grid.cell_center(row, col);

                for point in points {
                    let dist = crate::geoprocessing::analysis::euclidean_distance(point, &cell_center);
                    let kernel_value = (-0.5 * (dist / bandwidth).powi(2)).exp() / (bandwidth * pi_sqrt);
                    row_data[col] += kernel_value;
                }
            }

            row_data
        })
        .collect()
}

/// Interpolação IDW paralela para múltiplos alvos
#[cfg(feature = "parallel")]
pub fn idw_interpolation_batch_parallel(
    stations: &[(GeoCoord, f64)],
    targets: &[GeoCoord],
    power: f64,
) -> Vec<f64> {
    targets
        .par_iter()
        .map(|target| idw_interpolation(stations, target, power))
        .collect()
}

/// Query espacial paralela - processa múltiplas áreas simultaneamente
#[cfg(feature = "parallel")]
pub fn multi_query_parallel(
    features: &[SpatialFeature],
    query_bounds: &[BoundingBox],
) -> Vec<Vec<usize>> {
    query_bounds
        .par_iter()
        .map(|bounds| {
            features
                .iter()
                .enumerate()
                .filter(|(_, f)| f.bounding_box().intersects(bounds))
                .map(|(idx, _)| idx)
                .collect()
        })
        .collect()
}

/// Calcula distâncias entre todos os pares de pontos em paralelo
#[cfg(feature = "parallel")]
pub fn distance_matrix_parallel(points: &[GeoCoord]) -> Vec<Vec<f64>> {
    (0..points.len())
        .into_par_iter()
        .map(|i| {
            (0..points.len())
                .map(|j| {
                    if i == j {
                        0.0
                    } else {
                        haversine_distance(&points[i], &points[j])
                    }
                })
                .collect()
        })
        .collect()
}

/// Point-in-polygon para múltiplos pontos em paralelo
#[cfg(feature = "parallel")]
pub fn batch_point_in_polygon_parallel(
    points: &[GeoCoord],
    polygon: &[GeoCoord],
) -> Vec<bool> {
    points
        .par_iter()
        .map(|point| point_in_polygon(point, polygon))
        .collect()
}

/// Rasterização paralela - converte features para raster
#[cfg(feature = "parallel")]
pub fn rasterize_parallel(
    features: &[SpatialFeature],
    grid: &Grid,
) -> Vec<Vec<f64>> {
    use crate::geoprocessing::spatial::FeatureGeometry;

    (0..grid.rows)
        .into_par_iter()
        .map(|row| {
            let mut row_data = vec![0.0; grid.cols];

            for col in 0..grid.cols {
                let center = grid.cell_center(row, col);

                for feature in features {
                    let value = match &feature.geometry {
                        FeatureGeometry::Point(coord) => {
                            let dist = haversine_distance(&center, coord);
                            if dist < grid.resolution * 111000.0 {
                                1.0
                            } else {
                                0.0
                            }
                        }
                        FeatureGeometry::Polygon(coords) => {
                            if point_in_polygon(&center, coords) {
                                1.0
                            } else {
                                0.0
                            }
                        }
                        _ => 0.0,
                    };

                    row_data[col] = row_data[col].max(value);
                }
            }

            row_data
        })
        .collect()
}

/// Cálculo paralelo de slope para DEM
#[cfg(feature = "parallel")]
pub fn slope_map_parallel(
    data: &[Vec<f64>],
    resolution: f64,
) -> Vec<Vec<f64>> {
    let rows = data.len();
    let cols = if rows > 0 { data[0].len() } else { 0 };

    (1..rows - 1)
        .into_par_iter()
        .map(|row| {
            let mut row_slopes = vec![0.0; cols];

            for col in 1..cols - 1 {
                let dz_dx = (data[row][col + 1] - data[row][col - 1]) / (2.0 * resolution);
                let dz_dy = (data[row + 1][col] - data[row - 1][col]) / (2.0 * resolution);
                let slope_rad = (dz_dx.powi(2) + dz_dy.powi(2)).sqrt().atan();
                row_slopes[col] = slope_rad.to_degrees();
            }

            row_slopes
        })
        .collect()
}

/// Busca de vizinhos mais próximos (k-NN) paralela
#[cfg(feature = "parallel")]
pub fn k_nearest_neighbors_parallel(
    points: &[GeoCoord],
    query_points: &[GeoCoord],
    k: usize,
) -> Vec<Vec<(usize, f64)>> {
    query_points
        .par_iter()
        .map(|query| {
            let mut distances: Vec<(usize, f64)> = points
                .iter()
                .enumerate()
                .map(|(idx, point)| (idx, haversine_distance(query, point)))
                .collect();

            distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            distances.into_iter().take(k).collect()
        })
        .collect()
}

/// Agrupa features por grid cell em paralelo
#[cfg(feature = "parallel")]
pub fn grid_aggregation_parallel(
    features: &[SpatialFeature],
    grid: &Grid,
) -> Vec<Vec<Vec<usize>>> {
    (0..grid.rows)
        .into_par_iter()
        .map(|row| {
            let mut row_cells = vec![Vec::new(); grid.cols];

            for (idx, feature) in features.iter().enumerate() {
                let bbox = feature.bounding_box();
                let (cx, cy) = bbox.center();
                let center = GeoCoord { lat: cy, lon: cx };

                if let Some((cell_row, cell_col)) = grid.cell_at(&center) {
                    if cell_row == row {
                        row_cells[cell_col].push(idx);
                    }
                }
            }

            row_cells
        })
        .collect()
}

#[cfg(test)]
#[cfg(feature = "parallel")]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_distance_matrix() {
        let points = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(1.0, 1.0),
            GeoCoord::new(2.0, 2.0),
        ];

        let matrix = distance_matrix_parallel(&points);

        assert_eq!(matrix.len(), 3);
        assert_eq!(matrix[0][0], 0.0);
        assert!(matrix[0][1] > 0.0);
    }

    #[test]
    fn test_batch_point_in_polygon_parallel() {
        let polygon = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(0.0, 10.0),
            GeoCoord::new(10.0, 10.0),
            GeoCoord::new(10.0, 0.0),
        ];

        let points = vec![
            GeoCoord::new(5.0, 5.0),
            GeoCoord::new(15.0, 15.0),
        ];

        let results = batch_point_in_polygon_parallel(&points, &polygon);

        assert_eq!(results, vec![true, false]);
    }
}
