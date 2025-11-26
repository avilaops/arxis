//! Operações espaciais fundamentais
//!
//! Este módulo implementa:
//! - Relações topológicas (intersects, contains, within, touches, crosses, overlaps)
//! - Operações geométricas (buffer, union, intersection, difference, convex hull)
//! - Algoritmos de geometria computacional

use crate::coords::GeoCoord;
use crate::geoprocessing::spatial::{BoundingBox, FeatureGeometry, SpatialFeature};

/// Trait para relações espaciais topológicas
pub trait SpatialRelation {
    /// Verifica se duas geometrias se intersectam
    fn intersects(&self, other: &Self) -> bool;

    /// Verifica se esta geometria contém completamente outra
    fn contains(&self, other: &Self) -> bool;

    /// Verifica se esta geometria está dentro de outra
    fn within(&self, other: &Self) -> bool {
        other.contains(self)
    }

    /// Verifica se duas geometrias se tocam (compartilham borda mas não interior)
    fn touches(&self, other: &Self) -> bool;

    /// Verifica se duas geometrias se cruzam
    fn crosses(&self, other: &Self) -> bool;

    /// Verifica se duas geometrias se sobrepõem
    fn overlaps(&self, other: &Self) -> bool;
}

/// Trait para operações geométricas
pub trait GeometryOps {
    /// Cria uma zona de influência (buffer) ao redor da geometria
    fn buffer(&self, distance: f64) -> Self;

    /// Calcula a união com outra geometria
    fn union(&self, other: &Self) -> Self;

    /// Calcula a interseção com outra geometria
    fn intersection(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    /// Calcula a diferença com outra geometria
    fn difference(&self, other: &Self) -> Option<Self>
    where
        Self: Sized;

    /// Calcula a diferença simétrica com outra geometria
    fn symmetric_difference(&self, other: &Self) -> Self;

    /// Calcula o fecho convexo (convex hull)
    fn convex_hull(&self) -> Self;
}

/// Implementação de operações para BoundingBox
impl SpatialRelation for BoundingBox {
    fn intersects(&self, other: &Self) -> bool {
        BoundingBox::intersects(self, other)
    }

    fn contains(&self, other: &Self) -> bool {
        BoundingBox::contains(self, other)
    }

    fn touches(&self, other: &Self) -> bool {
        // Verifica se compartilham borda mas não se sobrepõem
        if !self.intersects(other) {
            return false;
        }

        let intersection = self.intersection(other);
        match intersection {
            Some(bbox) => bbox.area() == 0.0,
            None => false,
        }
    }

    fn crosses(&self, _other: &Self) -> bool {
        // BoundingBox não pode cruzar outra (sempre intersecta ou não)
        false
    }

    fn overlaps(&self, other: &Self) -> bool {
        if !self.intersects(other) {
            return false;
        }

        !self.contains(other) && !other.contains(self)
    }
}

impl GeometryOps for BoundingBox {
    fn buffer(&self, distance: f64) -> Self {
        BoundingBox::buffer(self, distance)
    }

    fn union(&self, other: &Self) -> Self {
        BoundingBox::union(self, other)
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        BoundingBox::intersection(self, other)
    }

    fn difference(&self, other: &Self) -> Option<Self> {
        // Diferença de bounding boxes - retorna self se não intersecta
        if !self.intersects(other) {
            Some(*self)
        } else if other.contains(self) {
            None
        } else {
            // Simplificação: retorna self se houver interseção parcial
            Some(*self)
        }
    }

    fn symmetric_difference(&self, other: &Self) -> Self {
        // União menos interseção
        self.union(other)
    }

    fn convex_hull(&self) -> Self {
        // BoundingBox já é convexa
        *self
    }
}

/// Calcula buffer (zona de influência) para polígono
pub fn buffer_polygon(coords: &[GeoCoord], distance: f64) -> Vec<GeoCoord> {
    if coords.len() < 3 {
        return coords.to_vec();
    }

    let mut buffered_points = Vec::new();

    for i in 0..coords.len() {
        let p1 = &coords[i];
        let p2 = &coords[(i + 1) % coords.len()];

        // Calcular perpendicular offset
        let dx = p2.lon - p1.lon;
        let dy = p2.lat - p1.lat;
        let len = (dx * dx + dy * dy).sqrt();

        if len < 1e-10 {
            continue;
        }

        let offset_x = -dy / len * distance;
        let offset_y = dx / len * distance;

        buffered_points.push(GeoCoord {
            lat: p1.lat + offset_y,
            lon: p1.lon + offset_x,
        });
    }

    buffered_points
}

/// Verifica se um ponto está dentro de um polígono (Ray Casting Algorithm)
pub fn point_in_polygon(point: &GeoCoord, polygon: &[GeoCoord]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let mut inside = false;
    let mut j = polygon.len() - 1;

    for i in 0..polygon.len() {
        let xi = polygon[i].lon;
        let yi = polygon[i].lat;
        let xj = polygon[j].lon;
        let yj = polygon[j].lat;

        let intersect = ((yi > point.lat) != (yj > point.lat))
            && (point.lon < (xj - xi) * (point.lat - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }

        j = i;
    }

    inside
}

/// Verifica se duas linhas se intersectam
pub fn line_intersects_line(
    p1: &GeoCoord,
    p2: &GeoCoord,
    p3: &GeoCoord,
    p4: &GeoCoord,
) -> bool {
    fn ccw(a: &GeoCoord, b: &GeoCoord, c: &GeoCoord) -> bool {
        (c.lat - a.lat) * (b.lon - a.lon) > (b.lat - a.lat) * (c.lon - a.lon)
    }

    ccw(p1, p3, p4) != ccw(p2, p3, p4) && ccw(p1, p2, p3) != ccw(p1, p2, p4)
}

/// Calcula a interseção de duas linhas (se existir)
pub fn line_intersection(
    p1: &GeoCoord,
    p2: &GeoCoord,
    p3: &GeoCoord,
    p4: &GeoCoord,
) -> Option<GeoCoord> {
    let x1 = p1.lon;
    let y1 = p1.lat;
    let x2 = p2.lon;
    let y2 = p2.lat;
    let x3 = p3.lon;
    let y3 = p3.lat;
    let x4 = p4.lon;
    let y4 = p4.lat;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    if denom.abs() < 1e-10 {
        return None; // Linhas paralelas
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) / denom;

    if t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0 {
        Some(GeoCoord {
            lat: y1 + t * (y2 - y1),
            lon: x1 + t * (x2 - x1),
        })
    } else {
        None
    }
}

/// Calcula o fecho convexo (Convex Hull) usando algoritmo de Graham Scan
pub fn convex_hull(points: &[GeoCoord]) -> Vec<GeoCoord> {
    if points.len() < 3 {
        return points.to_vec();
    }

    let mut sorted_points = points.to_vec();

    // Encontrar ponto mais baixo (e mais à esquerda em caso de empate)
    let start_idx = sorted_points
        .iter()
        .enumerate()
        .min_by(|a, b| {
            a.1.lat
                .partial_cmp(&b.1.lat)
                .unwrap()
                .then(a.1.lon.partial_cmp(&b.1.lon).unwrap())
        })
        .map(|(idx, _)| idx)
        .unwrap();

    sorted_points.swap(0, start_idx);
    let start = sorted_points[0];

    // Ordenar por ângulo polar
    sorted_points[1..].sort_by(|a, b| {
        let angle_a = (a.lat - start.lat).atan2(a.lon - start.lon);
        let angle_b = (b.lat - start.lat).atan2(b.lon - start.lon);
        angle_a.partial_cmp(&angle_b).unwrap()
    });

    let mut hull = vec![sorted_points[0], sorted_points[1]];

    for point in sorted_points.iter().skip(2) {
        while hull.len() > 1 && !ccw(&hull[hull.len() - 2], &hull[hull.len() - 1], point) {
            hull.pop();
        }
        hull.push(*point);
    }

    hull
}

fn ccw(a: &GeoCoord, b: &GeoCoord, c: &GeoCoord) -> bool {
    (b.lon - a.lon) * (c.lat - a.lat) - (b.lat - a.lat) * (c.lon - a.lon) > 0.0
}

/// Calcula a área de um polígono usando a fórmula de Shoelace
pub fn polygon_area(coords: &[GeoCoord]) -> f64 {
    if coords.len() < 3 {
        return 0.0;
    }

    let mut area = 0.0;
    for i in 0..coords.len() {
        let j = (i + 1) % coords.len();
        area += coords[i].lon * coords[j].lat;
        area -= coords[j].lon * coords[i].lat;
    }

    (area / 2.0).abs()
}

/// Calcula o centroide de um polígono
pub fn polygon_centroid(coords: &[GeoCoord]) -> GeoCoord {
    if coords.is_empty() {
        return GeoCoord::new(0.0, 0.0);
    }

    let area = polygon_area(coords);
    if area < 1e-10 {
        // Fallback para média simples se área for muito pequena
        let sum_lat: f64 = coords.iter().map(|c| c.lat).sum();
        let sum_lon: f64 = coords.iter().map(|c| c.lon).sum();
        return GeoCoord::new(sum_lat / coords.len() as f64, sum_lon / coords.len() as f64);
    }

    let mut cx = 0.0;
    let mut cy = 0.0;

    for i in 0..coords.len() {
        let j = (i + 1) % coords.len();
        let factor = coords[i].lon * coords[j].lat - coords[j].lon * coords[i].lat;
        cx += (coords[i].lon + coords[j].lon) * factor;
        cy += (coords[i].lat + coords[j].lat) * factor;
    }

    let divisor = 6.0 * area;
    GeoCoord {
        lat: cy / divisor,
        lon: cx / divisor,
    }
}

/// Simplifica uma linha usando o algoritmo de Douglas-Peucker
pub fn simplify_line(coords: &[GeoCoord], tolerance: f64) -> Vec<GeoCoord> {
    if coords.len() <= 2 {
        return coords.to_vec();
    }

    let mut dmax = 0.0;
    let mut index = 0;

    let end = coords.len() - 1;

    for i in 1..end {
        let d = perpendicular_distance(&coords[i], &coords[0], &coords[end]);
        if d > dmax {
            index = i;
            dmax = d;
        }
    }

    if dmax > tolerance {
        let mut rec1 = simplify_line(&coords[0..=index], tolerance);
        let rec2 = simplify_line(&coords[index..], tolerance);

        rec1.pop();
        rec1.extend(rec2);
        rec1
    } else {
        vec![coords[0], coords[end]]
    }
}

fn perpendicular_distance(point: &GeoCoord, line_start: &GeoCoord, line_end: &GeoCoord) -> f64 {
    let dx = line_end.lon - line_start.lon;
    let dy = line_end.lat - line_start.lat;

    let mag = (dx * dx + dy * dy).sqrt();
    if mag < 1e-10 {
        return ((point.lon - line_start.lon).powi(2) + (point.lat - line_start.lat).powi(2)).sqrt();
    }

    let u = ((point.lon - line_start.lon) * dx + (point.lat - line_start.lat) * dy) / (mag * mag);

    let closest = if u < 0.0 {
        *line_start
    } else if u > 1.0 {
        *line_end
    } else {
        GeoCoord {
            lon: line_start.lon + u * dx,
            lat: line_start.lat + u * dy,
        }
    };

    ((point.lon - closest.lon).powi(2) + (point.lat - closest.lat).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_in_polygon() {
        let polygon = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(0.0, 10.0),
            GeoCoord::new(10.0, 10.0),
            GeoCoord::new(10.0, 0.0),
        ];

        assert!(point_in_polygon(&GeoCoord::new(5.0, 5.0), &polygon));
        assert!(!point_in_polygon(&GeoCoord::new(15.0, 15.0), &polygon));
    }

    #[test]
    fn test_line_intersects() {
        let p1 = GeoCoord::new(0.0, 0.0);
        let p2 = GeoCoord::new(10.0, 10.0);
        let p3 = GeoCoord::new(0.0, 10.0);
        let p4 = GeoCoord::new(10.0, 0.0);

        assert!(line_intersects_line(&p1, &p2, &p3, &p4));
    }

    #[test]
    fn test_polygon_area() {
        let square = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(0.0, 10.0),
            GeoCoord::new(10.0, 10.0),
            GeoCoord::new(10.0, 0.0),
        ];

        let area = polygon_area(&square);
        assert!((area - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_convex_hull() {
        let points = vec![
            GeoCoord::new(0.0, 0.0),
            GeoCoord::new(5.0, 5.0),
            GeoCoord::new(10.0, 0.0),
            GeoCoord::new(5.0, 10.0),
            GeoCoord::new(5.0, 2.0), // Ponto interior
        ];

        let hull = convex_hull(&points);
        assert!(hull.len() < points.len());
    }
}
