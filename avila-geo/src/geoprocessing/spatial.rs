//! Estruturas de dados espaciais para consultas eficientes
//!
//! Este módulo implementa:
//! - BoundingBox: Caixa delimitadora para geometrias
//! - QuadTree: Árvore quaternária para particionamento espacial
//! - Índices espaciais usando R-Tree (via rstar)

use crate::coords::GeoCoord;
use std::collections::HashMap;

/// Caixa delimitadora (Bounding Box) para representar limites espaciais
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl BoundingBox {
    /// Cria uma nova bounding box
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    /// Cria uma bounding box a partir de coordenadas geográficas
    pub fn from_coords(coords: &[GeoCoord]) -> Self {
        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for coord in coords {
            min_x = min_x.min(coord.lon);
            min_y = min_y.min(coord.lat);
            max_x = max_x.max(coord.lon);
            max_y = max_y.max(coord.lat);
        }

        Self::new(min_x, min_y, max_x, max_y)
    }

    /// Verifica se esta bounding box intersecta com outra
    pub fn intersects(&self, other: &BoundingBox) -> bool {
        !(self.max_x < other.min_x
            || self.min_x > other.max_x
            || self.max_y < other.min_y
            || self.min_y > other.max_y)
    }

    /// Verifica se esta bounding box contém um ponto
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    /// Verifica se esta bounding box contém outra completamente
    pub fn contains(&self, other: &BoundingBox) -> bool {
        self.min_x <= other.min_x
            && self.max_x >= other.max_x
            && self.min_y <= other.min_y
            && self.max_y >= other.max_y
    }

    /// Calcula a área da bounding box
    pub fn area(&self) -> f64 {
        (self.max_x - self.min_x) * (self.max_y - self.min_y)
    }

    /// Calcula o centro da bounding box
    pub fn center(&self) -> (f64, f64) {
        (
            (self.min_x + self.max_x) / 2.0,
            (self.min_y + self.max_y) / 2.0,
        )
    }

    /// Expande a bounding box para incluir um ponto
    pub fn expand_to_include(&mut self, x: f64, y: f64) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    /// Expande a bounding box por uma margem
    pub fn buffer(&self, distance: f64) -> Self {
        Self::new(
            self.min_x - distance,
            self.min_y - distance,
            self.max_x + distance,
            self.max_y + distance,
        )
    }

    /// Calcula a união com outra bounding box
    pub fn union(&self, other: &BoundingBox) -> Self {
        Self::new(
            self.min_x.min(other.min_x),
            self.min_y.min(other.min_y),
            self.max_x.max(other.max_x),
            self.max_y.max(other.max_y),
        )
    }

    /// Calcula a interseção com outra bounding box
    pub fn intersection(&self, other: &BoundingBox) -> Option<Self> {
        if !self.intersects(other) {
            return None;
        }

        Some(Self::new(
            self.min_x.max(other.min_x),
            self.min_y.max(other.min_y),
            self.max_x.min(other.max_x),
            self.max_y.min(other.max_y),
        ))
    }
}

/// QuadTree para particionamento espacial eficiente
pub struct QuadTree<T> {
    bounds: BoundingBox,
    capacity: usize,
    points: Vec<(GeoCoord, T)>,
    divided: bool,
    northwest: Option<Box<QuadTree<T>>>,
    northeast: Option<Box<QuadTree<T>>>,
    southwest: Option<Box<QuadTree<T>>>,
    southeast: Option<Box<QuadTree<T>>>,
}

impl<T: Clone> QuadTree<T> {
    /// Cria uma nova QuadTree
    pub fn new(bounds: BoundingBox, capacity: usize) -> Self {
        Self {
            bounds,
            capacity,
            points: Vec::new(),
            divided: false,
            northwest: None,
            northeast: None,
            southwest: None,
            southeast: None,
        }
    }

    /// Insere um ponto com dados associados na QuadTree
    pub fn insert(&mut self, coord: GeoCoord, data: T) -> bool {
        if !self.bounds.contains_point(coord.lon, coord.lat) {
            return false;
        }

        if self.points.len() < self.capacity && !self.divided {
            self.points.push((coord, data));
            return true;
        }

        if !self.divided {
            self.subdivide();
        }

        self.northwest.as_mut().unwrap().insert(coord, data.clone())
            || self.northeast.as_mut().unwrap().insert(coord, data.clone())
            || self.southwest.as_mut().unwrap().insert(coord, data.clone())
            || self.southeast.as_mut().unwrap().insert(coord, data)
    }

    /// Subdivide a QuadTree em quatro quadrantes
    fn subdivide(&mut self) {
        let (center_x, center_y) = self.bounds.center();

        let nw = BoundingBox::new(self.bounds.min_x, center_y, center_x, self.bounds.max_y);
        let ne = BoundingBox::new(center_x, center_y, self.bounds.max_x, self.bounds.max_y);
        let sw = BoundingBox::new(self.bounds.min_x, self.bounds.min_y, center_x, center_y);
        let se = BoundingBox::new(center_x, self.bounds.min_y, self.bounds.max_x, center_y);

        self.northwest = Some(Box::new(QuadTree::new(nw, self.capacity)));
        self.northeast = Some(Box::new(QuadTree::new(ne, self.capacity)));
        self.southwest = Some(Box::new(QuadTree::new(sw, self.capacity)));
        self.southeast = Some(Box::new(QuadTree::new(se, self.capacity)));

        self.divided = true;

        // Redistribuir pontos existentes
        let points = std::mem::take(&mut self.points);
        for (coord, data) in points {
            self.northwest.as_mut().unwrap().insert(coord, data.clone())
                || self.northeast.as_mut().unwrap().insert(coord, data.clone())
                || self.southwest.as_mut().unwrap().insert(coord, data.clone())
                || self.southeast.as_mut().unwrap().insert(coord, data);
        }
    }

    /// Consulta pontos dentro de uma área
    pub fn query(&self, range: &BoundingBox, found: &mut Vec<(GeoCoord, T)>) {
        if !self.bounds.intersects(range) {
            return;
        }

        for (coord, data) in &self.points {
            if range.contains_point(coord.lon, coord.lat) {
                found.push((*coord, data.clone()));
            }
        }

        if self.divided {
            self.northwest.as_ref().unwrap().query(range, found);
            self.northeast.as_ref().unwrap().query(range, found);
            self.southwest.as_ref().unwrap().query(range, found);
            self.southeast.as_ref().unwrap().query(range, found);
        }
    }

    /// Consulta pontos dentro de um raio
    pub fn query_radius(
        &self,
        center: &GeoCoord,
        radius: f64,
        found: &mut Vec<(GeoCoord, T)>,
    ) {
        // Criar bounding box aproximada do raio
        let range = BoundingBox::new(
            center.lon - radius,
            center.lat - radius,
            center.lon + radius,
            center.lat + radius,
        );

        let mut candidates = Vec::new();
        self.query(&range, &mut candidates);

        // Filtrar por distância real
        for (coord, data) in candidates {
            let dist = crate::calc::haversine_distance(center, &coord);
            if dist <= radius {
                found.push((coord, data));
            }
        }
    }

    /// Retorna o número total de pontos na QuadTree
    pub fn size(&self) -> usize {
        let mut count = self.points.len();
        if self.divided {
            count += self.northwest.as_ref().unwrap().size();
            count += self.northeast.as_ref().unwrap().size();
            count += self.southwest.as_ref().unwrap().size();
            count += self.southeast.as_ref().unwrap().size();
        }
        count
    }
}

/// Feature espacial genérica
#[derive(Debug, Clone)]
pub struct SpatialFeature {
    pub id: usize,
    pub geometry: FeatureGeometry,
    pub properties: HashMap<String, String>,
}

/// Tipos de geometria para features
#[derive(Debug, Clone)]
pub enum FeatureGeometry {
    Point(GeoCoord),
    LineString(Vec<GeoCoord>),
    Polygon(Vec<GeoCoord>),
    MultiPoint(Vec<GeoCoord>),
    MultiLineString(Vec<Vec<GeoCoord>>),
    MultiPolygon(Vec<Vec<GeoCoord>>),
}

impl SpatialFeature {
    /// Cria uma nova feature de ponto
    pub fn point(id: usize, coord: GeoCoord) -> Self {
        Self {
            id,
            geometry: FeatureGeometry::Point(coord),
            properties: HashMap::new(),
        }
    }

    /// Cria uma nova feature de linha
    pub fn line_string(id: usize, coords: Vec<GeoCoord>) -> Self {
        Self {
            id,
            geometry: FeatureGeometry::LineString(coords),
            properties: HashMap::new(),
        }
    }

    /// Cria uma nova feature de polígono
    pub fn polygon(id: usize, coords: Vec<GeoCoord>) -> Self {
        Self {
            id,
            geometry: FeatureGeometry::Polygon(coords),
            properties: HashMap::new(),
        }
    }

    /// Adiciona uma propriedade à feature
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Calcula a bounding box da feature
    pub fn bounding_box(&self) -> BoundingBox {
        match &self.geometry {
            FeatureGeometry::Point(coord) => {
                BoundingBox::new(coord.lon, coord.lat, coord.lon, coord.lat)
            }
            FeatureGeometry::LineString(coords) | FeatureGeometry::Polygon(coords) => {
                BoundingBox::from_coords(coords)
            }
            FeatureGeometry::MultiPoint(coords) => BoundingBox::from_coords(coords),
            FeatureGeometry::MultiLineString(lines) => {
                let all_coords: Vec<GeoCoord> = lines.iter().flatten().copied().collect();
                BoundingBox::from_coords(&all_coords)
            }
            FeatureGeometry::MultiPolygon(polygons) => {
                let all_coords: Vec<GeoCoord> = polygons.iter().flatten().copied().collect();
                BoundingBox::from_coords(&all_coords)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounding_box_intersects() {
        let bbox1 = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let bbox2 = BoundingBox::new(5.0, 5.0, 15.0, 15.0);
        let bbox3 = BoundingBox::new(20.0, 20.0, 30.0, 30.0);

        assert!(bbox1.intersects(&bbox2));
        assert!(!bbox1.intersects(&bbox3));
    }

    #[test]
    fn test_bounding_box_contains_point() {
        let bbox = BoundingBox::new(0.0, 0.0, 10.0, 10.0);

        assert!(bbox.contains_point(5.0, 5.0));
        assert!(!bbox.contains_point(15.0, 15.0));
    }

    #[test]
    fn test_quadtree_insert_and_query() {
        let bounds = BoundingBox::new(-180.0, -90.0, 180.0, 90.0);
        let mut qtree = QuadTree::new(bounds, 4);

        qtree.insert(GeoCoord::new(0.0, 0.0), "Point A");
        qtree.insert(GeoCoord::new(10.0, 10.0), "Point B");
        qtree.insert(GeoCoord::new(-10.0, -10.0), "Point C");

        let query_range = BoundingBox::new(-5.0, -5.0, 5.0, 5.0);
        let mut results = Vec::new();
        qtree.query(&query_range, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].1, "Point A");
    }
}
