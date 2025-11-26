//! Parallel processing with Rayon
//!
//! Provides parallel versions of rendering and projection operations
//! for massive performance improvements on multi-core systems.

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use crate::coords::{CartesianCoord, GeoCoord};
use crate::geometry::{GeoCollection, GeoPoint, GeoLine, GeoPolygon};
use crate::projection::Projection;
use crate::render::{Color, Framebuffer};

/// Parallel projection of multiple points
#[cfg(feature = "parallel")]
pub fn project_points_parallel(
    points: &[GeoCoord],
    projection: &dyn Projection,
    width: f64,
    height: f64,
) -> Vec<CartesianCoord> {
    points
        .par_iter()
        .map(|p| projection.project(p, width, height))
        .collect()
}

#[cfg(not(feature = "parallel"))]
pub fn project_points_parallel(
    points: &[GeoCoord],
    projection: &dyn Projection,
    width: f64,
    height: f64,
) -> Vec<CartesianCoord> {
    points
        .iter()
        .map(|p| projection.project(p, width, height))
        .collect()
}

/// Parallel rendering of multiple geometry collections
#[cfg(feature = "parallel")]
pub fn render_collections_parallel(
    collections: &[(GeoCollection, Color)],
    projection: &dyn Projection,
    width: u32,
    height: u32,
) -> Framebuffer {
    use std::sync::Mutex;

    let fb = Mutex::new(Framebuffer::new(width, height));
    let w = width as f64;
    let h = height as f64;

    collections.par_iter().for_each(|(collection, color)| {
        // Project points
        for point in &collection.points {
            let cart = projection.project(&point.coord, w, h);
            if let Ok(mut fb) = fb.lock() {
                fb.set_pixel(cart.x as u32, cart.y as u32, *color);
            }
        }

        // Project lines
        for line in &collection.lines {
            let coords: Vec<_> = line.coords
                .par_iter()
                .map(|c| projection.project(c, w, h))
                .collect();

            if let Ok(mut fb) = fb.lock() {
                for window in coords.windows(2) {
                    fb.draw_line(
                        window[0].x as i32,
                        window[0].y as i32,
                        window[1].x as i32,
                        window[1].y as i32,
                        *color,
                    );
                }
            }
        }

        // Project polygons
        for polygon in &collection.polygons {
            let coords: Vec<_> = polygon.exterior
                .par_iter()
                .map(|c| projection.project(c, w, h))
                .collect();

            if let Ok(mut fb) = fb.lock() {
                fb.fill_polygon(&coords, *color);
            }
        }
    });

    fb.into_inner().unwrap()
}

#[cfg(not(feature = "parallel"))]
pub fn render_collections_parallel(
    collections: &[(GeoCollection, Color)],
    projection: &dyn Projection,
    width: u32,
    height: u32,
) -> Framebuffer {
    let mut fb = Framebuffer::new(width, height);
    let w = width as f64;
    let h = height as f64;

    for (collection, color) in collections {
        for point in &collection.points {
            let cart = projection.project(&point.coord, w, h);
            fb.set_pixel(cart.x as u32, cart.y as u32, *color);
        }

        for line in &collection.lines {
            let coords: Vec<_> = line.coords
                .iter()
                .map(|c| projection.project(c, w, h))
                .collect();

            for window in coords.windows(2) {
                fb.draw_line(
                    window[0].x as i32,
                    window[0].y as i32,
                    window[1].x as i32,
                    window[1].y as i32,
                    *color,
                );
            }
        }

        for polygon in &collection.polygons {
            let coords: Vec<_> = polygon.exterior
                .iter()
                .map(|c| projection.project(c, w, h))
                .collect();

            fb.fill_polygon(&coords, *color);
        }
    }

    fb
}

/// Parallel calculation of distances
#[cfg(feature = "parallel")]
pub fn haversine_distances_parallel(
    from: &GeoCoord,
    to_points: &[GeoCoord],
) -> Vec<f64> {
    use crate::calc::haversine_distance;

    to_points
        .par_iter()
        .map(|p| haversine_distance(from, p))
        .collect()
}

#[cfg(not(feature = "parallel"))]
pub fn haversine_distances_parallel(
    from: &GeoCoord,
    to_points: &[GeoCoord],
) -> Vec<f64> {
    use crate::calc::haversine_distance;

    to_points
        .iter()
        .map(|p| haversine_distance(from, p))
        .collect()
}

/// Parallel simplification of multiple lines
#[cfg(feature = "parallel")]
pub fn simplify_lines_parallel(
    lines: &[Vec<GeoCoord>],
    epsilon: f64,
) -> Vec<Vec<GeoCoord>> {
    use crate::calc::douglas_peucker;

    lines
        .par_iter()
        .map(|line| douglas_peucker(line, epsilon))
        .collect()
}

#[cfg(not(feature = "parallel"))]
pub fn simplify_lines_parallel(
    lines: &[Vec<GeoCoord>],
    epsilon: f64,
) -> Vec<Vec<GeoCoord>> {
    use crate::calc::douglas_peucker;

    lines
        .iter()
        .map(|line| douglas_peucker(line, epsilon))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::projection::Equirectangular;

    #[test]
    fn test_parallel_projection() {
        let points = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(10.0, 10.0),
            GeoCoord::new(-10.0, -10.0),
        ];

        let proj = Equirectangular::new();
        let results = project_points_parallel(&points, &proj, 800.0, 600.0);

        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_parallel_distances() {
        let origin = GeoCoord::new(0.0, 0.0);
        let destinations = vec![
            GeoCoord::new(1.0, 1.0),
            GeoCoord::new(2.0, 2.0),
            GeoCoord::new(3.0, 3.0),
        ];

        let distances = haversine_distances_parallel(&origin, &destinations);

        assert_eq!(distances.len(), 3);
        // Distances should increase
        assert!(distances[0] < distances[1]);
        assert!(distances[1] < distances[2]);
    }
}
