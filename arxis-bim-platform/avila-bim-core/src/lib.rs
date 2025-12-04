//! # avila-bim-core
//!
//! Primitivos BIM fundamentais para a plataforma ARXIS.
//! Define tipos base para modelos, elementos, geometria e metadados.
//!
//! ## Estruturas Principais
//! - **BimModel**: Contêiner raiz para um modelo IFC
//! - **BimElement**: Entidade BIM (parede, laje, coluna, etc.)
//! - **Geometry**: Representação geométrica (meshes, BRep)
//! - **Properties**: Metadados semânticos (material, dimensões, etc.)
//! - **Hierarchy**: Árvore espacial (projeto > pavimento > ambiente > elemento)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use nalgebra as na;

pub type Result<T> = std::result::Result<T, BimError>;

// ============================================================================
// ERROS
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum BimError {
    #[error("Invalid GUID: {0}")]
    InvalidGuid(String),

    #[error("Element not found: {0}")]
    ElementNotFound(String),

    #[error("Invalid geometry: {0}")]
    InvalidGeometry(String),

    #[error("Property error: {0}")]
    PropertyError(String),

    #[error("Hierarchy error: {0}")]
    HierarchyError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

// ============================================================================
// GUID (IFC global unique identifier - Base64 de 22 chars)
// ============================================================================

/// IFC GUID (22 caracteres Base64)
/// Exemplo: "2O_RrAJHv7xv2dl5cNZYOF"
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IfcGuid(pub String);

impl IfcGuid {
    /// Criar novo GUID a partir de string
    pub fn new(guid: impl Into<String>) -> Result<Self> {
        let guid = guid.into();
        if guid.len() != 22 {
            return Err(BimError::InvalidGuid(format!("GUID deve ter 22 caracteres, encontrado: {}", guid.len())));
        }
        Ok(Self(guid))
    }

    /// Gerar novo GUID aleatório
    pub fn generate() -> Self {
        use uuid::Uuid;
        let uuid = Uuid::new_v4();
        let bytes = uuid.as_bytes();

        // Converter para Base64 IFC (22 chars)
        const CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$";
        let mut result = String::with_capacity(22);

        // Algoritmo IFC GUID (conversão Base64 específica)
        let mut n = u128::from_be_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15],
        ]);

        for _ in 0..22 {
            result.push(CHARS[(n % 64) as usize] as char);
            n /= 64;
        }

        Self(result)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// MODELO BIM
// ============================================================================

/// Modelo BIM completo (arquivo IFC)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BimModel {
    pub id: Uuid,
    pub name: String,
    pub version: u32,
    pub schema: IfcSchema,
    pub metadata: ModelMetadata,
    pub elements: HashMap<IfcGuid, BimElement>,
    pub hierarchy: Hierarchy,
    pub created_at: DateTime<Utc>,
}

impl BimModel {
    pub fn new(name: impl Into<String>, schema: IfcSchema) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            version: 1,
            schema,
            metadata: ModelMetadata::default(),
            elements: HashMap::new(),
            hierarchy: Hierarchy::new(),
            created_at: Utc::now(),
        }
    }

    /// Adicionar elemento ao modelo
    pub fn add_element(&mut self, element: BimElement) {
        self.elements.insert(element.guid.clone(), element);
    }

    /// Buscar elemento por GUID
    pub fn get_element(&self, guid: &IfcGuid) -> Option<&BimElement> {
        self.elements.get(guid)
    }

    /// Filtrar elementos por tipo
    pub fn filter_by_type(&self, element_type: &str) -> Vec<&BimElement> {
        self.elements.values()
            .filter(|e| e.element_type == element_type)
            .collect()
    }
}

/// Schema IFC (IFC2x3, IFC4, IFC4x3)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IfcSchema {
    Ifc2x3,
    Ifc4,
    Ifc4x3,
}

/// Metadados do modelo
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ModelMetadata {
    pub author: Option<String>,
    pub organization: Option<String>,
    pub application: Option<String>,
    pub project_name: Option<String>,
    pub site_name: Option<String>,
    pub building_name: Option<String>,
    pub timestamp: Option<DateTime<Utc>>,
    pub units: Option<LengthUnit>,
    pub north_direction: Option<f64>, // Ângulo em graus
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LengthUnit {
    Meter,
    Millimeter,
    Centimeter,
    Foot,
    Inch,
}

// ============================================================================
// ELEMENTO BIM
// ============================================================================

/// Elemento BIM individual (IfcWall, IfcSlab, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BimElement {
    pub guid: IfcGuid,
    pub element_type: String, // "IfcWall", "IfcSlab", etc.
    pub name: Option<String>,
    pub description: Option<String>,
    pub properties: Properties,
    pub geometry: Option<Geometry>,
    pub material: Option<Material>,
    pub placement: Placement, // Transformação local → global
    pub relationships: Vec<Relationship>,
    pub metadata: HashMap<String, String>,
}

impl BimElement {
    pub fn new(element_type: impl Into<String>) -> Self {
        Self {
            guid: IfcGuid::generate(),
            element_type: element_type.into(),
            name: None,
            description: None,
            properties: Properties::default(),
            geometry: None,
            material: None,
            placement: Placement::identity(),
            relationships: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    /// Definir propriedade
    pub fn set_property(&mut self, key: impl Into<String>, value: PropertyValue) {
        self.properties.set(key, value);
    }

    /// Obter propriedade
    pub fn get_property(&self, key: &str) -> Option<&PropertyValue> {
        self.properties.get(key)
    }
}

// ============================================================================
// PROPRIEDADES
// ============================================================================

/// Propriedades semânticas de um elemento
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Properties {
    pub data: HashMap<String, PropertyValue>,
}

impl Properties {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, key: impl Into<String>, value: PropertyValue) {
        self.data.insert(key.into(), value);
    }

    pub fn get(&self, key: &str) -> Option<&PropertyValue> {
        self.data.get(key)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum PropertyValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Length(f64, LengthUnit),
    Area(f64),
    Volume(f64),
    Angle(f64),
}

// ============================================================================
// GEOMETRIA
// ============================================================================

/// Representação geométrica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geometry {
    pub id: Uuid,
    pub mesh: Option<Mesh>,
    pub brep: Option<BRep>, // Boundary Representation (futuro)
    pub bounds: BoundingBox,
}

/// Mesh triangulada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    /// Vértices: [x, y, z, x, y, z, ...]
    pub vertices: Vec<f32>,

    /// Normais: [nx, ny, nz, nx, ny, nz, ...]
    pub normals: Vec<f32>,

    /// Índices de triângulos: [i0, i1, i2, i0, i1, i2, ...]
    pub indices: Vec<u32>,

    /// UV coordinates (opcional): [u, v, u, v, ...]
    pub uvs: Option<Vec<f32>>,

    /// Cores por vértice (opcional): [r, g, b, a, ...]
    pub colors: Option<Vec<f32>>,
}

impl Mesh {
    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len() / 3
    }
}

/// BRep (futuro: superfícies NURBS, CSG, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BRep {
    pub faces: Vec<Face>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub vertices: Vec<[f64; 3]>,
    pub normal: [f64; 3],
}

/// Bounding Box (AABB)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: [f64; 3],
    pub max: [f64; 3],
}

impl BoundingBox {
    pub fn from_vertices(vertices: &[f32]) -> Self {
        let mut min = [f64::INFINITY; 3];
        let mut max = [f64::NEG_INFINITY; 3];

        for chunk in vertices.chunks(3) {
            for i in 0..3 {
                min[i] = min[i].min(chunk[i] as f64);
                max[i] = max[i].max(chunk[i] as f64);
            }
        }

        Self { min, max }
    }

    pub fn center(&self) -> [f64; 3] {
        [
            (self.min[0] + self.max[0]) / 2.0,
            (self.min[1] + self.max[1]) / 2.0,
            (self.min[2] + self.max[2]) / 2.0,
        ]
    }

    pub fn size(&self) -> [f64; 3] {
        [
            self.max[0] - self.min[0],
            self.max[1] - self.min[1],
            self.max[2] - self.min[2],
        ]
    }
}

// ============================================================================
// MATERIAL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Material {
    pub name: String,
    pub color: Option<[f32; 4]>, // RGBA
    pub metallic: f32,
    pub roughness: f32,
    pub textures: HashMap<String, String>, // Tipo → URL
}

// ============================================================================
// PLACEMENT (transformação espacial)
// ============================================================================

/// Posição e orientação no espaço 3D
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placement {
    /// Matriz de transformação 4x4 (coluna-major)
    pub matrix: [f64; 16],
}

impl Placement {
    pub fn identity() -> Self {
        Self {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        }
    }

    pub fn from_translation(x: f64, y: f64, z: f64) -> Self {
        Self {
            matrix: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                x, y, z, 1.0,
            ],
        }
    }

    /// Extrair posição (translação)
    pub fn position(&self) -> [f64; 3] {
        [self.matrix[12], self.matrix[13], self.matrix[14]]
    }
}

// ============================================================================
// RELACIONAMENTOS
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    pub relation_type: RelationType,
    pub target_guid: IfcGuid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationType {
    ContainedIn,    // Elemento contido em agregação (ex: parede em pavimento)
    ConnectedTo,    // Conexão física (ex: parede conectada a outra)
    DependsOn,      // Dependência funcional
    AssignedTo,     // Atribuição (ex: material atribuído a elemento)
}

// ============================================================================
// HIERARQUIA ESPACIAL
// ============================================================================

/// Árvore hierárquica (IfcProject → IfcSite → IfcBuilding → IfcBuildingStorey → elementos)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hierarchy {
    pub root: Option<HierarchyNode>,
}

impl Hierarchy {
    pub fn new() -> Self {
        Self { root: None }
    }

    /// Adicionar nó à hierarquia
    pub fn add_node(&mut self, parent: Option<&IfcGuid>, node: HierarchyNode) -> Result<()> {
        if parent.is_none() {
            self.root = Some(node);
            return Ok(());
        }

        // Buscar parent e adicionar child (TODO: implementar busca recursiva)
        Err(BimError::HierarchyError("Parent not found".into()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HierarchyNode {
    pub guid: IfcGuid,
    pub name: String,
    pub node_type: String, // "IfcProject", "IfcBuildingStorey", etc.
    pub children: Vec<HierarchyNode>,
}

impl HierarchyNode {
    pub fn new(guid: IfcGuid, name: impl Into<String>, node_type: impl Into<String>) -> Self {
        Self {
            guid,
            name: name.into(),
            node_type: node_type.into(),
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: HierarchyNode) {
        self.children.push(child);
    }
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ifc_guid_generation() {
        let guid = IfcGuid::generate();
        assert_eq!(guid.as_str().len(), 22);
    }

    #[test]
    fn test_bim_model_creation() {
        let mut model = BimModel::new("Test Project", IfcSchema::Ifc4);
        assert_eq!(model.name, "Test Project");
        assert_eq!(model.schema, IfcSchema::Ifc4);

        let mut wall = BimElement::new("IfcWall");
        wall.set_property("LoadBearing", PropertyValue::Boolean(true));
        model.add_element(wall);

        assert_eq!(model.elements.len(), 1);
    }

    #[test]
    fn test_bounding_box() {
        let vertices = vec![
            0.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ];

        let bbox = BoundingBox::from_vertices(&vertices);
        assert_eq!(bbox.min, [0.0, 0.0, 0.0]);
        assert_eq!(bbox.max, [1.0, 1.0, 1.0]);
        assert_eq!(bbox.center(), [0.5, 0.5, 0.5]);
    }

    #[test]
    fn test_mesh_stats() {
        let mesh = Mesh {
            vertices: vec![0.0; 9],  // 3 vértices
            normals: vec![0.0; 9],
            indices: vec![0, 1, 2],  // 1 triângulo
            uvs: None,
            colors: None,
        };

        assert_eq!(mesh.vertex_count(), 3);
        assert_eq!(mesh.triangle_count(), 1);
    }
}
