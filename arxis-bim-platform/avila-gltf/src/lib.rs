//! # avila-gltf
//!
//! Exporter glTF 2.0 / GLB **100% Rust nativo - DO ZERO**.

use avila_mesh::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::Write;

pub type Result<T> = std::result::Result<T, GltfError>;

#[derive(Debug, thiserror::Error)]
pub enum GltfError {
    #[error("Export error: {0}")]
    ExportError(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub struct GltfExporter;

#[derive(Debug, Clone)]
pub struct ExportOptions {
    pub asset_name: String,
    pub include_normals: bool,
    pub include_uvs: bool,
}

impl Default for ExportOptions {
    fn default() -> Self {
        Self {
            asset_name: "Avila BIM".into(),
            include_normals: true,
            include_uvs: true,
        }
    }
}

impl GltfExporter {
    pub fn new() -> Self {
        Self
    }

    /// Exporta cena completa para GLB (binário glTF 2.0)
    pub fn export_glb(&self, scene: &Scene, opts: &ExportOptions) -> Result<Vec<u8>> {
        let (json, bin) = self.export_parts(scene, opts)?;

        let mut glb = Vec::new();

        // Header (12 bytes)
        glb.write_all(&0x46546C67u32.to_le_bytes())?; // magic "glTF"
        glb.write_all(&2u32.to_le_bytes())?; // version 2

        let json_bytes = json.as_bytes();
        let json_padding = (4 - (json_bytes.len() % 4)) % 4;
        let bin_padding = (4 - (bin.len() % 4)) % 4;

        let total = 12 + 8 + json_bytes.len() + json_padding + 8 + bin.len() + bin_padding;
        glb.write_all(&(total as u32).to_le_bytes())?;

        // JSON chunk
        glb.write_all(&((json_bytes.len() + json_padding) as u32).to_le_bytes())?;
        glb.write_all(&0x4E4F534Au32.to_le_bytes())?; // "JSON"
        glb.write_all(json_bytes)?;
        glb.write_all(&vec![0x20u8; json_padding])?;

        // BIN chunk
        if !bin.is_empty() {
            glb.write_all(&((bin.len() + bin_padding) as u32).to_le_bytes())?;
            glb.write_all(&0x004E4942u32.to_le_bytes())?; // "BIN"
            glb.write_all(&bin)?;
            glb.write_all(&vec![0u8; bin_padding])?;
        }

        Ok(glb)
    }

    fn export_parts(&self, scene: &Scene, opts: &ExportOptions) -> Result<(String, Vec<u8>)> {
        let mut gltf = GltfRoot {
            asset: GltfAsset {
                version: "2.0".into(),
                generator: Some("avila-gltf".into()),
            },
            scene: Some(0),
            scenes: vec![GltfScene {
                nodes: (0..scene.meshes.len()).map(|i| i as u32).collect(),
            }],
            nodes: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            buffers: Vec::new(),
            buffer_views: Vec::new(),
            accessors: Vec::new(),
        };

        let mut bin_data = Vec::new();
        let mut material_map = HashMap::new();

        // Materiais
        for (mat_id, material) in &scene.materials {
            let idx = gltf.materials.len() as u32;
            material_map.insert(mat_id.clone(), idx);
            gltf.materials.push(material_to_gltf(material));
        }

        // Meshes
        for mesh in &scene.meshes {
            let material_idx = mesh.material_id.as_ref()
                .and_then(|id| material_map.get(id).copied());

            let gltf_mesh = self.mesh_to_gltf(
                mesh,
                &mut bin_data,
                &mut gltf.buffer_views,
                &mut gltf.accessors,
                material_idx,
                opts
            )?;

            let mesh_idx = gltf.meshes.len() as u32;
            gltf.meshes.push(gltf_mesh);

            gltf.nodes.push(GltfNode {
                mesh: Some(mesh_idx),
                matrix: None,
            });
        }

        if !bin_data.is_empty() {
            gltf.buffers.push(GltfBuffer {
                byte_length: bin_data.len() as u32,
                uri: None,
            });
        }

        let json = serde_json::to_string_pretty(&gltf)?;
        Ok((json, bin_data))
    }

    fn mesh_to_gltf(
        &self,
        mesh: &Mesh,
        bin_data: &mut Vec<u8>,
        buffer_views: &mut Vec<GltfBufferView>,
        accessors: &mut Vec<GltfAccessor>,
        material_idx: Option<u32>,
        opts: &ExportOptions,
    ) -> Result<GltfMesh> {
        let mut attributes = HashMap::new();

        // Converter para buffers
        let buffers = mesh.to_buffers();

        // POSITION
        let pos_accessor = self.add_buffer(
            bin_data,
            buffer_views,
            accessors,
            &buffers.positions,
            5126, // FLOAT
            "VEC3",
            buffers.positions.len() / 3,
            true,
        )?;
        attributes.insert("POSITION".into(), pos_accessor);

        // NORMAL
        if opts.include_normals && !buffers.normals.is_empty() {
            let norm_accessor = self.add_buffer(
                bin_data,
                buffer_views,
                accessors,
                &buffers.normals,
                5126,
                "VEC3",
                buffers.normals.len() / 3,
                false,
            )?;
            attributes.insert("NORMAL".into(), norm_accessor);
        }

        // TEXCOORD_0
        if opts.include_uvs && !buffers.uvs.is_empty() {
            let uv_accessor = self.add_buffer(
                bin_data,
                buffer_views,
                accessors,
                &buffers.uvs,
                5126,
                "VEC2",
                buffers.uvs.len() / 2,
                false,
            )?;
            attributes.insert("TEXCOORD_0".into(), uv_accessor);
        }

        // INDICES
        let indices_accessor = self.add_indices_buffer(
            &buffers.indices,
            bin_data,
            buffer_views,
            accessors,
        )?;

        Ok(GltfMesh {
            primitives: vec![GltfPrimitive {
                attributes,
                indices: Some(indices_accessor),
                material: material_idx,
                mode: 4, // TRIANGLES
            }],
        })
    }

    fn add_buffer(
        &self,
        bin_data: &mut Vec<u8>,
        buffer_views: &mut Vec<GltfBufferView>,
        accessors: &mut Vec<GltfAccessor>,
        data: &[f32],
        component_type: u32,
        accessor_type: &str,
        count: usize,
        calc_bounds: bool,
    ) -> Result<u32> {
        let byte_offset = bin_data.len() as u32;

        for &value in data {
            bin_data.write_all(&value.to_le_bytes())?;
        }

        let byte_length = (bin_data.len() - byte_offset as usize) as u32;

        // Padding
        let padding = (4 - (bin_data.len() % 4)) % 4;
        bin_data.extend_from_slice(&vec![0u8; padding]);

        let buffer_view_idx = buffer_views.len() as u32;
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset,
            byte_length,
            target: Some(34962), // ARRAY_BUFFER
        });

        let (min, max) = if calc_bounds && accessor_type == "VEC3" {
            calc_bounds_vec3(data)
        } else {
            (None, None)
        };

        let accessor_idx = accessors.len() as u32;
        accessors.push(GltfAccessor {
            buffer_view: Some(buffer_view_idx),
            byte_offset: 0,
            component_type,
            count,
            accessor_type: accessor_type.into(),
            min,
            max,
        });

        Ok(accessor_idx)
    }

    fn add_indices_buffer(
        &self,
        indices: &[u32],
        bin_data: &mut Vec<u8>,
        buffer_views: &mut Vec<GltfBufferView>,
        accessors: &mut Vec<GltfAccessor>,
    ) -> Result<u32> {
        let byte_offset = bin_data.len() as u32;
        let max_index = *indices.iter().max().unwrap_or(&0);
        let use_u16 = max_index < 65536;

        if use_u16 {
            for &idx in indices {
                bin_data.write_all(&(idx as u16).to_le_bytes())?;
            }
        } else {
            for &idx in indices {
                bin_data.write_all(&idx.to_le_bytes())?;
            }
        }

        let byte_length = (bin_data.len() - byte_offset as usize) as u32;

        // Padding
        let padding = (4 - (bin_data.len() % 4)) % 4;
        bin_data.extend_from_slice(&vec![0u8; padding]);

        let buffer_view_idx = buffer_views.len() as u32;
        buffer_views.push(GltfBufferView {
            buffer: 0,
            byte_offset,
            byte_length,
            target: Some(34963), // ELEMENT_ARRAY_BUFFER
        });

        let accessor_idx = accessors.len() as u32;
        accessors.push(GltfAccessor {
            buffer_view: Some(buffer_view_idx),
            byte_offset: 0,
            component_type: if use_u16 { 5123 } else { 5125 },
            count: indices.len(),
            accessor_type: "SCALAR".into(),
            min: None,
            max: None,
        });

        Ok(accessor_idx)
    }
}

impl Default for GltfExporter {
    fn default() -> Self {
        Self::new()
    }
}

fn calc_bounds_vec3(vertices: &[f32]) -> (Option<Vec<f32>>, Option<Vec<f32>>) {
    if vertices.is_empty() {
        return (None, None);
    }

    let mut min = [f32::INFINITY; 3];
    let mut max = [f32::NEG_INFINITY; 3];

    for chunk in vertices.chunks(3) {
        for i in 0..3 {
            min[i] = min[i].min(chunk[i]);
            max[i] = max[i].max(chunk[i]);
        }
    }

    (Some(min.to_vec()), Some(max.to_vec()))
}

fn material_to_gltf(mat: &PbrMaterial) -> GltfMaterial {
    GltfMaterial {
        name: Some(mat.name.clone()),
        pbr_metallic_roughness: PbrMetallicRoughness {
            base_color_factor: mat.base_color_factor,
            metallic_factor: mat.metallic_factor,
            roughness_factor: mat.roughness_factor,
        },
        double_sided: Some(mat.double_sided),
    }
}

// ============================================================================
// ESTRUTURAS glTF 2.0
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct GltfRoot {
    asset: GltfAsset,
    #[serde(skip_serializing_if = "Option::is_none")]
    scene: Option<u32>,
    scenes: Vec<GltfScene>,
    nodes: Vec<GltfNode>,
    meshes: Vec<GltfMesh>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    materials: Vec<GltfMaterial>,
    buffers: Vec<GltfBuffer>,
    #[serde(rename = "bufferViews")]
    buffer_views: Vec<GltfBufferView>,
    accessors: Vec<GltfAccessor>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfAsset {
    version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    generator: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfScene {
    nodes: Vec<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfNode {
    #[serde(skip_serializing_if = "Option::is_none")]
    mesh: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    matrix: Option<[f32; 16]>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfMesh {
    primitives: Vec<GltfPrimitive>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfPrimitive {
    attributes: HashMap<String, u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    indices: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<u32>,
    mode: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfMaterial {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(rename = "pbrMetallicRoughness")]
    pbr_metallic_roughness: PbrMetallicRoughness,
    #[serde(skip_serializing_if = "Option::is_none")]
    double_sided: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PbrMetallicRoughness {
    #[serde(rename = "baseColorFactor")]
    base_color_factor: [f32; 4],
    #[serde(rename = "metallicFactor")]
    metallic_factor: f32,
    #[serde(rename = "roughnessFactor")]
    roughness_factor: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfBuffer {
    #[serde(rename = "byteLength")]
    byte_length: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfBufferView {
    buffer: u32,
    #[serde(rename = "byteOffset")]
    byte_offset: u32,
    #[serde(rename = "byteLength")]
    byte_length: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    target: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GltfAccessor {
    #[serde(rename = "bufferView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    buffer_view: Option<u32>,
    #[serde(rename = "byteOffset")]
    byte_offset: u32,
    #[serde(rename = "componentType")]
    component_type: u32,
    count: usize,
    #[serde(rename = "type")]
    accessor_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    min: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<Vec<f32>>,
}

// ============================================================================
// TESTES
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use avila_mesh::primitives;

    #[test]
    fn test_export_cube() {
        let mut scene = Scene::new();
        let cube = primitives::cube(2.0);
        scene.add_mesh(cube);

        let exporter = GltfExporter::new();
        let glb = exporter.export_glb(&scene, &ExportOptions::default()).unwrap();

        assert_eq!(&glb[0..4], b"glTF");
        assert!(glb.len() > 100);
    }

    #[test]
    fn test_export_with_material() {
        let mut scene = Scene::new();
        let mut cube = primitives::cube(1.0);
        cube.material_id = Some("concrete".into());
        scene.add_mesh(cube);

        let material = PbrMaterial::from_ifc_material("concrete", "Concreto");
        scene.add_material(material);

        let exporter = GltfExporter::new();
        let glb = exporter.export_glb(&scene, &ExportOptions::default()).unwrap();

        assert!(glb.len() > 100);
    }
}
