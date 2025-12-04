//! # avila-ifc
//!
//! Parser IFC (Industry Foundation Classes) **100% Rust nativo**.
//! Suporta IFC2x3, IFC4, IFC4x3 via formato STEP (ISO 10303-21).
//!
//! ## Arquitetura
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │       IFC File (.ifc / P21)         │
//! └────────────┬────────────────────────┘
//!              │
//!              ▼
//! ┌─────────────────────────────────────┐
//! │   STEP Lexer (Tokens)               │  ← Tokeniza DATA; #123 = IFCWALL(...)
//! └────────────┬────────────────────────┘
//!              │
//!              ▼
//! ┌─────────────────────────────────────┐
//! │   Entity Parser                     │  ← Parse entidades (#ID, Type, Attrs)
//! └────────────┬────────────────────────┘
//!              │
//!              ▼
//! ┌─────────────────────────────────────┐
//! │   Entity Graph (HashMap<ID, Ent>)   │  ← Resolve referências (#123, #456)
//! └────────────┬────────────────────────┘
//!              │
//!              ▼
//! ┌─────────────────────────────────────┐
//! │   BimModel Converter                │  ← Converte para avila-bim-core
//! └─────────────────────────────────────┘
//! ```
//!
//! ## Exemplo de Uso
//!
//! ```no_run
//! use avila_ifc::IfcParser;
//!
//! let ifc_content = std::fs::read_to_string("model.ifc")?;
//! let parser = IfcParser::new(&ifc_content)?;
//! let bim_model = parser.parse()?;
//!
//! println!("Modelo: {}", bim_model.name);
//! println!("Elementos: {}", bim_model.elements.len());
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```

use std::collections::HashMap;
use avila_bim_core::*;
use regex::Regex;
use lazy_static::lazy_static;

pub type Result<T> = std::result::Result<T, IfcError>;

// ============================================================================
// ERROS
// ============================================================================

#[derive(Debug, thiserror::Error)]
pub enum IfcError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Invalid STEP format: {0}")]
    InvalidStepFormat(String),

    #[error("Entity not found: {0}")]
    EntityNotFound(u64),

    #[error("Unsupported schema: {0}")]
    UnsupportedSchema(String),

    #[error("Geometry error: {0}")]
    GeometryError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("BIM core error: {0}")]
    BimCoreError(#[from] BimError),
}

// ============================================================================
// PARSER IFC
// ============================================================================

/// Parser IFC principal
pub struct IfcParser {
    raw_content: String,
    header: IfcHeader,
    entities: HashMap<u64, StepEntity>,
}

impl IfcParser {
    /// Criar parser a partir de conteúdo IFC
    pub fn new(content: &str) -> Result<Self> {
        let header = parse_header(content)?;
        let entities = parse_data_section(content)?;

        Ok(Self {
            raw_content: content.to_string(),
            header,
            entities,
        })
    }

    /// Parsear e converter para BimModel
    pub fn parse(&self) -> Result<BimModel> {
        let mut model = BimModel::new(
            self.header.project_name.clone().unwrap_or_else(|| "Unnamed Project".into()),
            self.header.schema,
        );

        // Preencher metadata
        model.metadata.author = self.header.author.clone();
        model.metadata.organization = self.header.organization.clone();
        model.metadata.application = self.header.application.clone();
        model.metadata.timestamp = self.header.timestamp;

        // Processar entidades
        self.process_spatial_structure(&mut model)?;
        self.process_elements(&mut model)?;
        self.process_geometry(&mut model)?;

        Ok(model)
    }

    /// Processar estrutura espacial (IfcProject → IfcSite → IfcBuilding → IfcBuildingStorey)
    fn process_spatial_structure(&self, model: &mut BimModel) -> Result<()> {
        // TODO: Implementar construção da hierarquia
        // 1. Encontrar IfcProject (root)
        // 2. Recursivamente adicionar IfcSite, IfcBuilding, IfcBuildingStorey
        // 3. Usar IfcRelAggregates para montar árvore

        Ok(())
    }

    /// Processar elementos construtivos
    fn process_elements(&self, model: &mut BimModel) -> Result<()> {
        // Tipos de elementos BIM comuns
        const ELEMENT_TYPES: &[&str] = &[
            "IFCWALL", "IFCWALLSTANDARDCASE",
            "IFCSLAB", "IFCCOLUMN", "IFCBEAM",
            "IFCDOOR", "IFCWINDOW",
            "IFCROOF", "IFCSTAIR",
            "IFCFURNISHINGELEMENT",
        ];

        for (entity_id, entity) in &self.entities {
            if ELEMENT_TYPES.contains(&entity.entity_type.to_uppercase().as_str()) {
                let bim_element = self.convert_entity_to_element(entity)?;
                model.add_element(bim_element);
            }
        }

        Ok(())
    }

    /// Processar geometria dos elementos
    fn process_geometry(&self, model: &mut BimModel) -> Result<()> {
        // TODO: Processar IfcProductRepresentation → IfcShapeRepresentation → IfcGeometricRepresentationItem
        // Converter BRep (IfcFacetedBrep, IfcSweptSolid) para Mesh triangulada

        Ok(())
    }

    /// Converter entidade STEP para BimElement
    fn convert_entity_to_element(&self, entity: &StepEntity) -> Result<BimElement> {
        let mut element = BimElement::new(&entity.entity_type);

        // Parsear atributos básicos
        if let Some(guid) = entity.get_attribute(0) {
            if let StepValue::String(guid_str) = guid {
                element.guid = IfcGuid::new(guid_str.clone())?;
            }
        }

        // Nome
        if let Some(name) = entity.get_attribute(2) {
            if let StepValue::String(name_str) = name {
                element.name = Some(name_str.clone());
            }
        }

        // Descrição
        if let Some(desc) = entity.get_attribute(3) {
            if let StepValue::String(desc_str) = desc {
                element.description = Some(desc_str.clone());
            }
        }

        Ok(element)
    }
}

// ============================================================================
// HEADER IFC
// ============================================================================

#[derive(Debug, Clone)]
pub struct IfcHeader {
    pub schema: IfcSchema,
    pub project_name: Option<String>,
    pub author: Option<String>,
    pub organization: Option<String>,
    pub application: Option<String>,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

/// Parsear seção HEADER do arquivo IFC
fn parse_header(content: &str) -> Result<IfcHeader> {
    // Exemplo de HEADER:
    // ISO-10303-21;
    // HEADER;
    // FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');
    // FILE_NAME('model.ifc','2024-12-04T10:00:00',('Author'),('Organization'),'PreProc','Application','');
    // FILE_SCHEMA(('IFC4'));
    // ENDSEC;

    lazy_static! {
        static ref HEADER_RE: Regex = Regex::new(r"HEADER;(.*?)ENDSEC;").unwrap();
        static ref FILE_SCHEMA_RE: Regex = Regex::new(r"FILE_SCHEMA\(\('([^']+)'\)\);").unwrap();
        static ref FILE_NAME_RE: Regex = Regex::new(r"FILE_NAME\('([^']*)',").unwrap();
    }

    let header_match = HEADER_RE.find(content)
        .ok_or_else(|| IfcError::InvalidStepFormat("HEADER section not found".into()))?;

    let header_text = &content[header_match.start()..header_match.end()];

    // Parsear schema
    let schema_str = FILE_SCHEMA_RE.captures(header_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str())
        .unwrap_or("IFC4");

    let schema = match schema_str {
        "IFC2X3" | "IFC2X3_TC1" => IfcSchema::Ifc2x3,
        "IFC4" => IfcSchema::Ifc4,
        "IFC4X3" | "IFC4X3_RC4" => IfcSchema::Ifc4x3,
        _ => return Err(IfcError::UnsupportedSchema(schema_str.into())),
    };

    // Parsear nome do projeto (opcional)
    let project_name = FILE_NAME_RE.captures(header_text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    Ok(IfcHeader {
        schema,
        project_name,
        author: None,
        organization: None,
        application: None,
        timestamp: None,
    })
}

// ============================================================================
// STEP ENTITY
// ============================================================================

/// Entidade STEP (linha DATA)
/// Exemplo: #123 = IFCWALL('2O_RrAJHv7xv2dl5cNZYOF', #456, 'Wall-01', $, $, #789, #890, 'LoadBearing');
#[derive(Debug, Clone)]
pub struct StepEntity {
    pub id: u64,
    pub entity_type: String,
    pub attributes: Vec<StepValue>,
}

impl StepEntity {
    pub fn get_attribute(&self, index: usize) -> Option<&StepValue> {
        self.attributes.get(index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StepValue {
    String(String),
    Integer(i64),
    Float(f64),
    Ref(u64),         // Referência (#123)
    Null,             // $
    List(Vec<StepValue>),
}

/// Parsear seção DATA
fn parse_data_section(content: &str) -> Result<HashMap<u64, StepEntity>> {
    lazy_static! {
        static ref DATA_RE: Regex = Regex::new(r"DATA;(.*?)ENDSEC;").unwrap();
        static ref ENTITY_RE: Regex = Regex::new(r"#(\d+)\s*=\s*([A-Z0-9_]+)\((.*?)\);").unwrap();
    }

    let data_match = DATA_RE.find(content)
        .ok_or_else(|| IfcError::InvalidStepFormat("DATA section not found".into()))?;

    let data_text = &content[data_match.start()..data_match.end()];

    let mut entities = HashMap::new();

    for captures in ENTITY_RE.captures_iter(data_text) {
        let id: u64 = captures.get(1).unwrap().as_str().parse()
            .map_err(|_| IfcError::ParseError("Invalid entity ID".into()))?;

        let entity_type = captures.get(2).unwrap().as_str().to_string();
        let attrs_str = captures.get(3).unwrap().as_str();

        let attributes = parse_attributes(attrs_str)?;

        entities.insert(id, StepEntity {
            id,
            entity_type,
            attributes,
        });
    }

    Ok(entities)
}

/// Parsear atributos de uma entidade
fn parse_attributes(attrs_str: &str) -> Result<Vec<StepValue>> {
    let mut attributes = Vec::new();
    let mut buffer = String::new();
    let mut in_string = false;
    let mut paren_depth = 0;

    for ch in attrs_str.chars() {
        match ch {
            '\'' if !in_string => {
                in_string = true;
                buffer.push(ch);
            }
            '\'' if in_string => {
                in_string = false;
                buffer.push(ch);
            }
            '(' if !in_string => {
                paren_depth += 1;
                buffer.push(ch);
            }
            ')' if !in_string => {
                paren_depth -= 1;
                buffer.push(ch);
            }
            ',' if !in_string && paren_depth == 0 => {
                attributes.push(parse_single_value(&buffer)?);
                buffer.clear();
            }
            _ => buffer.push(ch),
        }
    }

    // Último atributo
    if !buffer.is_empty() {
        attributes.push(parse_single_value(&buffer)?);
    }

    Ok(attributes)
}

/// Parsear valor individual
fn parse_single_value(value_str: &str) -> Result<StepValue> {
    let trimmed = value_str.trim();

    // Null
    if trimmed == "$" {
        return Ok(StepValue::Null);
    }

    // Referência
    if trimmed.starts_with('#') {
        let id: u64 = trimmed[1..].parse()
            .map_err(|_| IfcError::ParseError(format!("Invalid reference: {}", trimmed)))?;
        return Ok(StepValue::Ref(id));
    }

    // String
    if trimmed.starts_with('\'') && trimmed.ends_with('\'') {
        return Ok(StepValue::String(trimmed[1..trimmed.len()-1].to_string()));
    }

    // Lista
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        let inner = &trimmed[1..trimmed.len()-1];
        let items = parse_attributes(inner)?;
        return Ok(StepValue::List(items));
    }

    // Float
    if trimmed.contains('.') {
        if let Ok(f) = trimmed.parse::<f64>() {
            return Ok(StepValue::Float(f));
        }
    }

    // Integer
    if let Ok(i) = trimmed.parse::<i64>() {
        return Ok(StepValue::Integer(i));
    }

    // Fallback: tratar como string sem aspas (enums IFC)
    Ok(StepValue::String(trimmed.to_string()))
}

// ============================================================================
// GEOMETRY CONVERSION (BRep → Mesh)
// ============================================================================

/// Converter geometria BRep para mesh triangulada
pub fn brep_to_mesh(_brep: &BRep) -> Result<Mesh> {
    // TODO: Implementar triangulação de superfícies BRep
    // 1. Iterar faces
    // 2. Triangular polígonos (Earcut, Delaunay)
    // 3. Calcular normais
    // 4. Gerar índices

    Err(IfcError::GeometryError("BRep triangulation not yet implemented".into()))
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_value() {
        assert_eq!(parse_single_value("$").unwrap(), StepValue::Null);
        assert_eq!(parse_single_value("#123").unwrap(), StepValue::Ref(123));
        assert_eq!(parse_single_value("'Hello'").unwrap(), StepValue::String("Hello".into()));
        assert_eq!(parse_single_value("42").unwrap(), StepValue::Integer(42));
        assert_eq!(parse_single_value("3.14").unwrap(), StepValue::Float(3.14));
    }

    #[test]
    fn test_parse_attributes() {
        let attrs = parse_attributes("'GUID',#456,'Name',$,123").unwrap();
        assert_eq!(attrs.len(), 5);
        assert_eq!(attrs[0], StepValue::String("GUID".into()));
        assert_eq!(attrs[1], StepValue::Ref(456));
        assert_eq!(attrs[2], StepValue::String("Name".into()));
        assert_eq!(attrs[3], StepValue::Null);
        assert_eq!(attrs[4], StepValue::Integer(123));
    }

    #[test]
    fn test_parse_header() {
        let ifc_content = r#"
ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');
FILE_NAME('test.ifc','2024-12-04T10:00:00',('Author'),('Org'),'App','Soft','');
FILE_SCHEMA(('IFC4'));
ENDSEC;
DATA;
ENDSEC;
END-ISO-10303-21;
        "#;

        let header = parse_header(ifc_content).unwrap();
        assert_eq!(header.schema, IfcSchema::Ifc4);
        assert_eq!(header.project_name, Some("test.ifc".into()));
    }
}
