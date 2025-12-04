//! # avila-metadata-extractor
//!
//! **Extrator de Metadados BIM - Semantic Layer**
//!
//! Extrai metadados semânticos de modelos IFC e os exporta em JSON estruturado
//! para uso em aplicações web/mobile/AR.
//!
//! ## Output JSON Schema
//! ```json
//! {
//!   "elements": [{
//!     "guid": "2O_RrAJHv7xv2dl5cNZYOF",
//!     "ifcType": "IfcWall",
//!     "meshNode": 17,
//!     "name": "Parede 01",
//!     "properties": { ... },
//!     "quantities": { ... }
//!   }],
//!   "structure": {
//!     "project": { ... },
//!     "buildings": [ ... ],
//!     "storeys": [ ... ]
//!   }
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, MetadataError>;

// ============================================================================
// ERROS
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("Extraction error: {0}")]
    ExtractionError(String),

    #[error("Invalid element: {0}")]
    InvalidElement(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

// ============================================================================
// ESTRUTURAS DE METADADOS
// ============================================================================

/// Metadados BIM completos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BimMetadata {
    /// Elementos BIM (paredes, lajes, colunas, etc.)
    pub elements: Vec<ElementMetadata>,

    /// Estrutura espacial (projeto → edifício → pavimento)
    pub structure: SpatialStructure,

    /// Estatísticas do modelo
    pub statistics: ModelStatistics,
}

/// Metadados de um elemento BIM
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ElementMetadata {
    /// GUID do IFC (22 caracteres Base64)
    pub guid: String,

    /// Tipo IFC (IfcWall, IfcSlab, etc.)
    pub ifc_type: String,

    /// Índice do node no glTF (para linking)
    pub mesh_node: Option<u32>,

    /// Nome do elemento
    pub name: String,

    /// Descrição (opcional)
    pub description: Option<String>,

    /// Propriedades (Property Sets)
    pub properties: HashMap<String, HashMap<String, PropertyValue>>,

    /// Quantidades (áreas, volumes, comprimentos)
    pub quantities: HashMap<String, f64>,

    /// Material
    pub material: Option<String>,

    /// Bounding box [minX, minY, minZ, maxX, maxY, maxZ]
    pub bounding_box: Option<[f32; 6]>,

    /// Tags/classificações
    pub tags: Vec<String>,
}

/// Valor de propriedade (pode ser string, número, booleano)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
}

/// Estrutura espacial do modelo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpatialStructure {
    /// Projeto
    pub project: ProjectInfo,

    /// Site (terreno/lote)
    pub site: Option<SiteInfo>,

    /// Edifícios
    pub buildings: Vec<BuildingInfo>,

    /// Pavimentos
    pub storeys: Vec<StoreyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteInfo {
    pub name: String,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingInfo {
    pub id: String,
    pub name: String,
    pub elevation: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreyInfo {
    pub id: String,
    pub name: String,
    pub elevation: f64,
    pub height: Option<f64>,
}

/// Estatísticas do modelo
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModelStatistics {
    /// Total de elementos
    pub total_elements: usize,

    /// Elementos por tipo
    pub elements_by_type: HashMap<String, usize>,

    /// Total de triângulos
    pub total_triangles: usize,

    /// Total de vértices
    pub total_vertices: usize,

    /// Área total (m²)
    pub total_area: Option<f64>,

    /// Volume total (m³)
    pub total_volume: Option<f64>,
}

// ============================================================================
// EXTRATOR
// ============================================================================

pub struct MetadataExtractor {
    // Configurações
}

impl MetadataExtractor {
    pub fn new() -> Self {
        Self {}
    }

    /// Extrai metadados de elementos BIM
    pub fn extract_elements(&self, elements: &[BimElement]) -> Result<Vec<ElementMetadata>> {
        elements.iter()
            .enumerate()
            .map(|(idx, elem)| self.extract_element(elem, Some(idx as u32)))
            .collect()
    }

    fn extract_element(&self, element: &BimElement, mesh_node: Option<u32>) -> Result<ElementMetadata> {
        let mut properties = HashMap::new();
        let mut quantities = HashMap::new();

        // Propriedades comuns
        let mut common_props = HashMap::new();
        if let Some(is_external) = element.is_external {
            common_props.insert("IsExternal".to_string(), PropertyValue::Boolean(is_external));
        }
        if let Some(load_bearing) = element.is_load_bearing {
            common_props.insert("LoadBearing".to_string(), PropertyValue::Boolean(load_bearing));
        }
        if !common_props.is_empty() {
            properties.insert("Pset_Common".to_string(), common_props);
        }

        // Quantidades (se disponíveis)
        if let Some(length) = element.length {
            quantities.insert("Length".to_string(), length);
        }
        if let Some(area) = element.area {
            quantities.insert("Area".to_string(), area);
        }
        if let Some(volume) = element.volume {
            quantities.insert("Volume".to_string(), volume);
        }

        // Bounding box (se disponível)
        let bounding_box = element.bounding_box.map(|bb| {
            [bb.min_x, bb.min_y, bb.min_z, bb.max_x, bb.max_y, bb.max_z]
        });

        Ok(ElementMetadata {
            guid: element.guid.clone(),
            ifc_type: element.ifc_type.clone(),
            mesh_node,
            name: element.name.clone(),
            description: element.description.clone(),
            properties,
            quantities,
            material: element.material.clone(),
            bounding_box,
            tags: element.tags.clone(),
        })
    }

    /// Extrai estrutura espacial
    pub fn extract_spatial_structure(&self, project: &ProjectData) -> SpatialStructure {
        SpatialStructure {
            project: ProjectInfo {
                name: project.name.clone(),
                description: project.description.clone(),
                author: project.author.clone(),
                organization: project.organization.clone(),
            },
            site: project.site.as_ref().map(|s| SiteInfo {
                name: s.name.clone(),
                address: s.address.clone(),
                latitude: s.latitude,
                longitude: s.longitude,
            }),
            buildings: project.buildings.iter().map(|b| BuildingInfo {
                id: b.id.clone(),
                name: b.name.clone(),
                elevation: b.elevation,
            }).collect(),
            storeys: project.storeys.iter().map(|s| StoreyInfo {
                id: s.id.clone(),
                name: s.name.clone(),
                elevation: s.elevation,
                height: s.height,
            }).collect(),
        }
    }

    /// Calcula estatísticas
    pub fn calculate_statistics(&self, elements: &[ElementMetadata], scene_stats: &SceneStats) -> ModelStatistics {
        let mut elements_by_type = HashMap::new();
        let mut total_area = 0.0;
        let mut total_volume = 0.0;

        for elem in elements {
            *elements_by_type.entry(elem.ifc_type.clone()).or_insert(0) += 1;

            if let Some(area) = elem.quantities.get("Area") {
                total_area += area;
            }
            if let Some(volume) = elem.quantities.get("Volume") {
                total_volume += volume;
            }
        }

        ModelStatistics {
            total_elements: elements.len(),
            elements_by_type,
            total_triangles: scene_stats.triangle_count,
            total_vertices: scene_stats.vertex_count,
            total_area: if total_area > 0.0 { Some(total_area) } else { None },
            total_volume: if total_volume > 0.0 { Some(total_volume) } else { None },
        }
    }

    /// Exporta metadados para JSON
    pub fn export_json(&self, metadata: &BimMetadata) -> Result<String> {
        Ok(serde_json::to_string_pretty(metadata)?)
    }
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ESTRUTURAS AUXILIARES (interface com IFC parser)
// ============================================================================

/// Elemento BIM (interface com parser IFC)
#[derive(Debug, Clone)]
pub struct BimElement {
    pub guid: String,
    pub ifc_type: String,
    pub name: String,
    pub description: Option<String>,
    pub material: Option<String>,
    pub is_external: Option<bool>,
    pub is_load_bearing: Option<bool>,
    pub length: Option<f64>,
    pub area: Option<f64>,
    pub volume: Option<f64>,
    pub bounding_box: Option<BoundingBox>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

/// Dados do projeto (interface com parser IFC)
#[derive(Debug, Clone)]
pub struct ProjectData {
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub organization: Option<String>,
    pub site: Option<SiteData>,
    pub buildings: Vec<BuildingData>,
    pub storeys: Vec<StoreyData>,
}

#[derive(Debug, Clone)]
pub struct SiteData {
    pub name: String,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct BuildingData {
    pub id: String,
    pub name: String,
    pub elevation: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct StoreyData {
    pub id: String,
    pub name: String,
    pub elevation: f64,
    pub height: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct SceneStats {
    pub triangle_count: usize,
    pub vertex_count: usize,
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_element() {
        let element = BimElement {
            guid: "2O_RrAJHv7xv2dl5cNZYOF".to_string(),
            ifc_type: "IfcWall".to_string(),
            name: "Parede 01".to_string(),
            description: Some("Parede externa".to_string()),
            material: Some("Concreto".to_string()),
            is_external: Some(true),
            is_load_bearing: Some(true),
            length: Some(5.2),
            area: Some(15.6),
            volume: Some(3.12),
            bounding_box: None,
            tags: vec!["estrutural".to_string()],
        };

        let extractor = MetadataExtractor::new();
        let metadata = extractor.extract_element(&element, Some(0)).unwrap();

        assert_eq!(metadata.guid, "2O_RrAJHv7xv2dl5cNZYOF");
        assert_eq!(metadata.ifc_type, "IfcWall");
        assert_eq!(metadata.name, "Parede 01");
        assert!(metadata.quantities.contains_key("Length"));
        assert!(metadata.properties.contains_key("Pset_Common"));
    }

    #[test]
    fn test_export_json() {
        let metadata = BimMetadata {
            elements: vec![],
            structure: SpatialStructure {
                project: ProjectInfo {
                    name: "Teste".to_string(),
                    description: None,
                    author: None,
                    organization: None,
                },
                site: None,
                buildings: vec![],
                storeys: vec![],
            },
            statistics: ModelStatistics {
                total_elements: 0,
                elements_by_type: HashMap::new(),
                total_triangles: 0,
                total_vertices: 0,
                total_area: None,
                total_volume: None,
            },
        };

        let extractor = MetadataExtractor::new();
        let json = extractor.export_json(&metadata).unwrap();

        assert!(json.contains("elements"));
        assert!(json.contains("structure"));
        assert!(json.contains("statistics"));
    }
}
