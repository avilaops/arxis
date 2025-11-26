//! Engine principal de geoprocessamento
//!
//! Este módulo integra todos os componentes de geoprocessamento em uma
//! interface unificada para análise espacial completa.

use crate::coords::GeoCoord;
use crate::geoprocessing::analysis::{Grid, SpatialWeights};
use crate::geoprocessing::network::SpatialNetwork;
use crate::geoprocessing::operations::{buffer_polygon, point_in_polygon, polygon_area};
use crate::geoprocessing::spatial::{BoundingBox, FeatureGeometry, QuadTree, SpatialFeature};
use crate::geoprocessing::terrain::DigitalElevationModel;
use std::collections::HashMap;

/// Operações de overlay entre camadas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayOp {
    Union,
    Intersection,
    Difference,
    SymmetricDifference,
}

/// Camada vetorial
#[derive(Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub features: Vec<SpatialFeature>,
    pub crs: String,
}

impl Layer {
    /// Cria uma nova camada
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            features: Vec::new(),
            crs: "WGS84".to_string(),
        }
    }

    /// Adiciona uma feature à camada
    pub fn add_feature(&mut self, feature: SpatialFeature) {
        self.features.push(feature);
    }

    /// Calcula a bounding box da camada
    pub fn bounding_box(&self) -> Option<BoundingBox> {
        if self.features.is_empty() {
            return None;
        }

        let mut bbox = self.features[0].bounding_box();
        for feature in &self.features[1..] {
            bbox = bbox.union(&feature.bounding_box());
        }

        Some(bbox)
    }

    /// Filtra features por propriedade
    pub fn filter(&self, key: &str, value: &str) -> Layer {
        let mut filtered = Layer::new(format!("{}_filtered", self.name));
        filtered.crs = self.crs.clone();

        for feature in &self.features {
            if let Some(prop_value) = feature.properties.get(key) {
                if prop_value == value {
                    filtered.add_feature(feature.clone());
                }
            }
        }

        filtered
    }

    /// Conta features na camada
    pub fn count(&self) -> usize {
        self.features.len()
    }
}

/// Camada raster
#[derive(Debug, Clone)]
pub struct Raster {
    pub name: String,
    pub data: Vec<Vec<f64>>,
    pub bounds: BoundingBox,
    pub resolution: f64,
}

impl Raster {
    /// Cria um novo raster
    pub fn new(name: impl Into<String>, bounds: BoundingBox, resolution: f64) -> Self {
        let cols = ((bounds.max_x - bounds.min_x) / resolution).ceil() as usize;
        let rows = ((bounds.max_y - bounds.min_y) / resolution).ceil() as usize;

        Self {
            name: name.into(),
            data: vec![vec![0.0; cols]; rows],
            bounds,
            resolution,
        }
    }

    /// Define o valor de uma célula
    pub fn set_value(&mut self, row: usize, col: usize, value: f64) {
        if row < self.data.len() && col < self.data[0].len() {
            self.data[row][col] = value;
        }
    }

    /// Obtém o valor de uma célula
    pub fn get_value(&self, row: usize, col: usize) -> Option<f64> {
        self.data.get(row)?.get(col).copied()
    }
}

/// Engine principal de geoprocessamento
pub struct GeoprocessingEngine {
    layers: HashMap<String, Layer>,
    rasters: HashMap<String, Raster>,
    networks: HashMap<String, SpatialNetwork>,
    dems: HashMap<String, DigitalElevationModel>,
    spatial_indices: HashMap<String, QuadTree<usize>>,
}

impl GeoprocessingEngine {
    /// Cria uma nova engine de geoprocessamento
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
            rasters: HashMap::new(),
            networks: HashMap::new(),
            dems: HashMap::new(),
            spatial_indices: HashMap::new(),
        }
    }

    /// Adiciona uma camada vetorial
    pub fn add_layer(&mut self, layer: Layer) {
        let name = layer.name.clone();
        self.layers.insert(name.clone(), layer);
        self.build_spatial_index(&name);
    }

    /// Adiciona uma camada raster
    pub fn add_raster(&mut self, raster: Raster) {
        self.rasters.insert(raster.name.clone(), raster);
    }

    /// Adiciona uma rede espacial
    pub fn add_network(&mut self, name: impl Into<String>, network: SpatialNetwork) {
        self.networks.insert(name.into(), network);
    }

    /// Adiciona um DEM
    pub fn add_dem(&mut self, name: impl Into<String>, dem: DigitalElevationModel) {
        self.dems.insert(name.into(), dem);
    }

    /// Constrói índice espacial para uma camada
    fn build_spatial_index(&mut self, layer_name: &str) {
        if let Some(layer) = self.layers.get(layer_name) {
            if let Some(bbox) = layer.bounding_box() {
                let mut qtree = QuadTree::new(bbox, 10);

                for (idx, feature) in layer.features.iter().enumerate() {
                    match &feature.geometry {
                        FeatureGeometry::Point(coord) => {
                            qtree.insert(*coord, idx);
                        }
                        FeatureGeometry::LineString(coords) | FeatureGeometry::Polygon(coords) => {
                            // Inserir ponto central
                            let bbox = BoundingBox::from_coords(coords);
                            let (cx, cy) = bbox.center();
                            qtree.insert(GeoCoord { lat: cy, lon: cx }, idx);
                        }
                        _ => {}
                    }
                }

                self.spatial_indices.insert(layer_name.to_string(), qtree);
            }
        }
    }

    /// Query espacial - encontra features dentro de uma área
    pub fn query_spatial(&self, layer_name: &str, bounds: &BoundingBox) -> Vec<&SpatialFeature> {
        let mut results = Vec::new();

        if let Some(qtree) = self.spatial_indices.get(layer_name) {
            if let Some(layer) = self.layers.get(layer_name) {
                let mut indices = Vec::new();
                qtree.query(bounds, &mut indices);

                for (_, idx) in indices {
                    if let Some(feature) = layer.features.get(idx) {
                        if feature.bounding_box().intersects(bounds) {
                            results.push(feature);
                        }
                    }
                }
            }
        }

        results
    }

    /// Análise de proximidade - encontra features dentro de um raio
    pub fn proximity_analysis(&self, layer_name: &str, point: &GeoCoord, radius: f64) -> Vec<&SpatialFeature> {
        let mut results = Vec::new();

        if let Some(qtree) = self.spatial_indices.get(layer_name) {
            if let Some(layer) = self.layers.get(layer_name) {
                let mut candidates = Vec::new();
                qtree.query_radius(point, radius, &mut candidates);

                for (_, idx) in candidates {
                    if let Some(feature) = layer.features.get(idx) {
                        results.push(feature);
                    }
                }
            }
        }

        results
    }

    /// Overlay operation entre duas camadas
    pub fn overlay(&self, layer1_name: &str, layer2_name: &str, op: OverlayOp) -> Option<Layer> {
        let layer1 = self.layers.get(layer1_name)?;
        let layer2 = self.layers.get(layer2_name)?;

        let result_name = format!("{}_{:?}_{}", layer1_name, op, layer2_name);
        let mut result = Layer::new(result_name);

        match op {
            OverlayOp::Union => {
                // Adicionar todas as features de ambas as camadas
                result.features.extend(layer1.features.clone());
                result.features.extend(layer2.features.clone());
            }
            OverlayOp::Intersection => {
                // Encontrar features que se intersectam
                for f1 in &layer1.features {
                    for f2 in &layer2.features {
                        if f1.bounding_box().intersects(&f2.bounding_box()) {
                            result.features.push(f1.clone());
                            break;
                        }
                    }
                }
            }
            OverlayOp::Difference => {
                // Features de layer1 que não intersectam com layer2
                for f1 in &layer1.features {
                    let mut intersects = false;
                    for f2 in &layer2.features {
                        if f1.bounding_box().intersects(&f2.bounding_box()) {
                            intersects = true;
                            break;
                        }
                    }
                    if !intersects {
                        result.features.push(f1.clone());
                    }
                }
            }
            OverlayOp::SymmetricDifference => {
                // Features que não se intersectam em ambas as camadas
                for f1 in &layer1.features {
                    let mut intersects = false;
                    for f2 in &layer2.features {
                        if f1.bounding_box().intersects(&f2.bounding_box()) {
                            intersects = true;
                            break;
                        }
                    }
                    if !intersects {
                        result.features.push(f1.clone());
                    }
                }

                for f2 in &layer2.features {
                    let mut intersects = false;
                    for f1 in &layer1.features {
                        if f2.bounding_box().intersects(&f1.bounding_box()) {
                            intersects = true;
                            break;
                        }
                    }
                    if !intersects {
                        result.features.push(f2.clone());
                    }
                }
            }
        }

        Some(result)
    }

    /// Rasteriza features vetoriais
    pub fn rasterize(&self, layer_name: &str, resolution: f64) -> Option<Raster> {
        let layer = self.layers.get(layer_name)?;
        let bounds = layer.bounding_box()?;

        let mut raster = Raster::new(format!("{}_rasterized", layer_name), bounds, resolution);

        let grid = Grid::new(bounds, raster.data.len(), raster.data[0].len());

        for feature in &layer.features {
            match &feature.geometry {
                FeatureGeometry::Point(coord) => {
                    if let Some((row, col)) = grid.cell_at(coord) {
                        raster.set_value(row, col, 1.0);
                    }
                }
                FeatureGeometry::Polygon(coords) => {
                    // Rasterizar polígono
                    for row in 0..raster.data.len() {
                        for col in 0..raster.data[0].len() {
                            let center = grid.cell_center(row, col);
                            if point_in_polygon(&center, coords) {
                                raster.set_value(row, col, 1.0);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Some(raster)
    }

    /// Vetoriza raster (cria polígonos a partir de células)
    pub fn vectorize(&self, raster_name: &str, threshold: f64) -> Option<Layer> {
        let raster = self.rasters.get(raster_name)?;
        let mut layer = Layer::new(format!("{}_vectorized", raster_name));

        let mut feature_id = 0;
        let grid = Grid::new(raster.bounds, raster.data.len(), raster.data[0].len());

        for row in 0..raster.data.len() {
            for col in 0..raster.data[0].len() {
                if raster.data[row][col] >= threshold {
                    let center = grid.cell_center(row, col);
                    layer.add_feature(SpatialFeature::point(feature_id, center));
                    feature_id += 1;
                }
            }
        }

        Some(layer)
    }

    /// Cria zona de buffer ao redor de features
    pub fn buffer_layer(&self, layer_name: &str, distance: f64) -> Option<Layer> {
        let layer = self.layers.get(layer_name)?;
        let mut buffered = Layer::new(format!("{}_buffered", layer_name));

        for feature in &layer.features {
            match &feature.geometry {
                FeatureGeometry::Point(coord) => {
                    // Criar polígono circular ao redor do ponto
                    let num_segments = 32;
                    let mut circle = Vec::new();

                    for i in 0..num_segments {
                        let angle = 2.0 * std::f64::consts::PI * i as f64 / num_segments as f64;
                        let lat = coord.lat + distance * angle.cos();
                        let lon = coord.lon + distance * angle.sin();
                        circle.push(GeoCoord { lat, lon });
                    }

                    buffered.add_feature(SpatialFeature::polygon(feature.id, circle));
                }
                FeatureGeometry::Polygon(coords) => {
                    let buffered_coords = buffer_polygon(coords, distance);
                    buffered.add_feature(SpatialFeature::polygon(feature.id, buffered_coords));
                }
                _ => {}
            }
        }

        Some(buffered)
    }

    /// Calcula estatísticas de uma camada
    pub fn layer_statistics(&self, layer_name: &str) -> Option<LayerStatistics> {
        let layer = self.layers.get(layer_name)?;

        let mut total_area = 0.0;
        let mut total_length = 0.0;
        let mut point_count = 0;
        let mut line_count = 0;
        let mut polygon_count = 0;

        for feature in &layer.features {
            match &feature.geometry {
                FeatureGeometry::Point(_) => point_count += 1,
                FeatureGeometry::LineString(coords) => {
                    line_count += 1;
                    for i in 0..coords.len() - 1 {
                        total_length += crate::geoprocessing::analysis::haversine_distance(
                            &coords[i],
                            &coords[i + 1],
                        );
                    }
                }
                FeatureGeometry::Polygon(coords) => {
                    polygon_count += 1;
                    total_area += polygon_area(coords);
                }
                _ => {}
            }
        }

        Some(LayerStatistics {
            feature_count: layer.features.len(),
            point_count,
            line_count,
            polygon_count,
            total_area,
            total_length,
            bounds: layer.bounding_box(),
        })
    }

    /// Obtém camada por nome
    pub fn get_layer(&self, name: &str) -> Option<&Layer> {
        self.layers.get(name)
    }

    /// Obtém raster por nome
    pub fn get_raster(&self, name: &str) -> Option<&Raster> {
        self.rasters.get(name)
    }

    /// Obtém rede por nome
    pub fn get_network(&self, name: &str) -> Option<&SpatialNetwork> {
        self.networks.get(name)
    }

    /// Obtém DEM por nome
    pub fn get_dem(&self, name: &str) -> Option<&DigitalElevationModel> {
        self.dems.get(name)
    }

    /// Lista todas as camadas
    pub fn list_layers(&self) -> Vec<&str> {
        self.layers.keys().map(|s| s.as_str()).collect()
    }

    /// Lista todos os rasters
    pub fn list_rasters(&self) -> Vec<&str> {
        self.rasters.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for GeoprocessingEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Estatísticas de uma camada
#[derive(Debug, Clone)]
pub struct LayerStatistics {
    pub feature_count: usize,
    pub point_count: usize,
    pub line_count: usize,
    pub polygon_count: usize,
    pub total_area: f64,
    pub total_length: f64,
    pub bounds: Option<BoundingBox>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let engine = GeoprocessingEngine::new();
        assert_eq!(engine.list_layers().len(), 0);
    }

    #[test]
    fn test_add_layer() {
        let mut engine = GeoprocessingEngine::new();
        let mut layer = Layer::new("test");

        layer.add_feature(SpatialFeature::point(0, GeoCoord::new(0.0, 0.0)));

        engine.add_layer(layer);
        assert_eq!(engine.list_layers().len(), 1);
    }

    #[test]
    fn test_spatial_query() {
        let mut engine = GeoprocessingEngine::new();
        let mut layer = Layer::new("test");

        layer.add_feature(SpatialFeature::point(0, GeoCoord::new(5.0, 5.0)));
        layer.add_feature(SpatialFeature::point(1, GeoCoord::new(15.0, 15.0)));

        engine.add_layer(layer);

        let query_bounds = BoundingBox::new(0.0, 0.0, 10.0, 10.0);
        let results = engine.query_spatial("test", &query_bounds);

        assert_eq!(results.len(), 1);
    }
}
