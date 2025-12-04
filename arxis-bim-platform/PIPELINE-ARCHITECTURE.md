# Pipeline BIM Completo - Do Subatômico ao glTF

## Arquitetura em Camadas (Bottom-Up)

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        LAYER 0: SUBATÔMICO                              │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  avila-vec3d: Matemática 3D do Zero                            │    │
│  │  • Vec2, Vec3, Vec4 (vetores 2D/3D/4D)                         │    │
│  │  • Mat4 (matrizes 4x4 para transformações)                     │    │
│  │  • Quaternions (rotações)                                      │    │
│  │  • AABB (Axis-Aligned Bounding Boxes)                          │    │
│  │  • Ray (raios para intersecções)                               │    │
│  │  • Operações: dot, cross, normalize, transform, etc.           │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                    LAYER 1: GEOMETRIA 3D                                │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  avila-mesh: Estruturas de Mesh para Renderização             │    │
│  │  • Vertex (position, normal, UV, tangent, color)              │    │
│  │  • Mesh (vertices + indices)                                   │    │
│  │  • PbrMaterial (materiais físicos)                            │    │
│  │  • Scene (coleção de meshes + materiais)                      │    │
│  │  • Primitivas: cube, sphere, plane                            │    │
│  │  • Operações: merge, transform, recalc normals                │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│               LAYER 2: TESSELAÇÃO (IFC → Triângulos)                    │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  avila-tesselation: Conversão de Sólidos em Meshes            │    │
│  │  • ExtrudedAreaSolid → Mesh                                   │    │
│  │  • Box, Cylinder, Sphere → Mesh                               │    │
│  │  • BRep (Boundary Representation) → Mesh                      │    │
│  │  • CSG (Constructive Solid Geometry) → Mesh                   │    │
│  │  • Triangulação de faces, geração de normais                  │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                  LAYER 3: PARSER IFC (STEP/EXPRESS)                     │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  avila-ifc: Parser IFC do Zero                                │    │
│  │  • STEP Lexer (tokenização)                                   │    │
│  │  • Entity Parser (IfcWall, IfcSlab, IfcColumn, etc.)          │    │
│  │  • Relationship Graph (hierarchy, aggregates)                 │    │
│  │  • Property Sets (Pset_*)                                     │    │
│  │  • Geometry Extraction (sólidos paramétricos)                 │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                    LAYER 4: OTIMIZAÇÃO                                  │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  avila-optimizer: Performance de Geometria                    │    │
│  │  • Mesh Merging (reduz draw calls)                            │    │
│  │  • LOD Generation (Level of Detail - múltiplas resoluções)    │    │
│  │  • Draco Compression (compressão de vértices)                 │    │
│  │  • Spatial Indexing (octree, BVH)                             │    │
│  │  • Culling Optimization (frustum, occlusion)                  │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                    LAYER 5: EXPORTAÇÃO glTF                             │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  avila-gltf: Exportador glTF 2.0 / GLB                        │    │
│  │  • Buffers (binários de vértices/índices)                     │    │
│  │  • BufferViews (views em buffers)                             │    │
│  │  • Accessors (metadata de atributos)                          │    │
│  │  • Meshes & Primitives                                        │    │
│  │  • Materials (PBR: base color, metallic, roughness)           │    │
│  │  • Nodes (transform hierarchy)                                │    │
│  │  • Scenes (root composition)                                  │    │
│  │  • GLB Writer (formato binário completo)                      │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│              LAYER 6: METADADOS BIM                                     │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  avila-metadata-extractor: Exportador de Metadados BIM        │    │
│  │  • GUID → Mesh Node Mapping                                   │    │
│  │  • Properties (Pset_*, Quantities, Custom)                    │    │
│  │  • Relationships (parent/child, aggregation)                  │    │
│  │  • Spatial Structure (project → site → building → storey)     │    │
│  │  • JSON Export (para apps web/mobile)                         │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────────┐
│                 LAYER 7: WORKER DE CONVERSÃO                            │
│  ┌────────────────────────────────────────────────────────────────┐    │
│  │  avila-bim-converter: Orquestrador do Pipeline                │    │
│  │  • Lê IFC → Parse → Tesselate → Optimize → Export             │    │
│  │  • Worker Async (RabbitMQ/Kafka)                              │    │
│  │  • Progress Tracking                                          │    │
│  │  • Error Handling & Logging                                   │    │
│  │  • Output: model.glb + metadata.json                          │    │
│  └────────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────────┘
```

## Pipeline de Conversão IFC → glTF

```
┌─────────────┐
│  IFC File   │
│  (.ifc)     │
└──────┬──────┘
       │
       ▼
┌────────────────────────────────────┐
│  1. IFC Parser (avila-ifc)         │
│  ────────────────────────────────  │
│  • Tokenização STEP (ISO 10303-21) │
│  • Parse de entidades:             │
│    - IfcWall, IfcSlab, IfcColumn   │
│    - IfcDoor, IfcWindow, etc.      │
│  • Resolve referências (#123, #456)│
│  • Extrai geometria paramétrica    │
│  • Extrai propriedades (Psets)     │
│                                    │
│  Output: IfcModel (in-memory)      │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│  2. Geometry Engine (tesselation)  │
│  ────────────────────────────────  │
│  Para cada IfcElement:             │
│  • Resolve LocalPlacement (matrix) │
│  • Tesselate geometry:             │
│    - ExtrudedSolid → triangles     │
│    - BRep → triangles              │
│    - CSG → triangles               │
│  • Aplica transformações           │
│  • Calcula normais                 │
│  • Gera UVs (se necessário)        │
│                                    │
│  Output: Vec<Mesh> (triangulated)  │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│  3. Material Mapping               │
│  ────────────────────────────────  │
│  • IfcMaterial → PbrMaterial       │
│  • Mapeia cores, texturas          │
│  • Define metallic/roughness       │
│    com base no tipo de material:   │
│    - Concreto: rough, não-metálico │
│    - Aço: metálico, smooth         │
│    - Vidro: transparente           │
│                                    │
│  Output: HashMap<MaterialId, Mat>  │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│  4. Optimizer (avila-optimizer)    │
│  ────────────────────────────────  │
│  • Merge meshes por material       │
│    (reduz draw calls massivamente) │
│  • Gera LODs (3 níveis):           │
│    - LOD0: 5% (distância)          │
│    - LOD1: 25% (média)             │
│    - LOD2: 100% (perto)            │
│  • (Opcional) Draco compression    │
│  • Spatial indexing (BVH)          │
│                                    │
│  Output: Scene (optimized)         │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│  5. glTF Exporter (avila-gltf)     │
│  ────────────────────────────────  │
│  • Cria buffers binários:          │
│    - Positions (Vec3 per vertex)   │
│    - Normals (Vec3)                │
│    - UVs (Vec2)                    │
│    - Indices (u16/u32)             │
│  • Cria BufferViews                │
│  • Cria Accessors (metadata)       │
│  • Cria Meshes & Primitives        │
│  • Cria Materials (PBR)            │
│  • Cria Nodes (hierarchy)          │
│  • Monta JSON descriptor           │
│  • Escreve GLB:                    │
│    Header + JSON Chunk + BIN Chunk │
│                                    │
│  Output: model.glb (bytes)         │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│  6. Metadata Extractor             │
│  ────────────────────────────────  │
│  • Para cada elemento BIM:         │
│    - GUID                          │
│    - IfcType                       │
│    - meshNodeIndex (link to glTF)  │
│    - Properties (Psets)            │
│    - Quantities                    │
│  • Spatial structure (tree)        │
│  • Relationships                   │
│                                    │
│  Output: metadata.json             │
└────────┬───────────────────────────┘
         │
         ▼
┌────────────────────────────────────┐
│  7. Storage (MinIO/S3)             │
│  ────────────────────────────────  │
│  /models/{modelId}/                │
│    ├── original.ifc                │
│    ├── model.glb                   │
│    ├── metadata.json               │
│    ├── lod0.glb                    │
│    ├── lod1.glb                    │
│    └── lod2.glb                    │
└────────────────────────────────────┘
```

## Estrutura de Dados (Camada por Camada)

### Camada 0: Vec3D
```rust
Vec3 { x: f32, y: f32, z: f32 }
Mat4 { m: [[f32; 4]; 4] } // column-major
Quat { x, y, z, w: f32 }
Aabb { min: Vec3, max: Vec3 }
```

### Camada 1: Mesh
```rust
Vertex {
  position: Vec3,
  normal: Vec3,
  uv: Vec2,
  tangent: Option<Vec3>,
  color: Option<[f32; 4]>
}

Mesh {
  vertices: Vec<Vertex>,
  indices: Vec<u32>,
  material_id: Option<String>,
  bounds: Aabb
}

PbrMaterial {
  base_color_factor: [f32; 4],
  metallic_factor: f32,
  roughness_factor: f32,
  // ... outros atributos PBR
}
```

### Camada 2: IFC Geometry
```rust
enum IfcGeometry {
  ExtrudedAreaSolid { profile, direction, depth },
  Box { center, size },
  Cylinder { base, radius, height },
  Brep { faces },
  // ...
}
```

### Camada 3: IFC Entities
```rust
struct IfcWall {
  guid: IfcGuid,
  name: String,
  geometry: IfcGeometry,
  material: Option<String>,
  properties: HashMap<String, IfcProperty>,
  placement: Mat4
}
```

### Camada 5: glTF Structures
```json
{
  "asset": { "version": "2.0" },
  "scenes": [{ "nodes": [0, 1, 2] }],
  "nodes": [{ "mesh": 0, "matrix": [...] }],
  "meshes": [{
    "primitives": [{
      "attributes": {
        "POSITION": 0,
        "NORMAL": 1,
        "TEXCOORD_0": 2
      },
      "indices": 3,
      "material": 0
    }]
  }],
  "materials": [{
    "pbrMetallicRoughness": {
      "baseColorFactor": [0.8, 0.8, 0.8, 1.0],
      "metallicFactor": 0.0,
      "roughnessFactor": 1.0
    }
  }],
  "accessors": [...],
  "bufferViews": [...],
  "buffers": [{ "byteLength": 123456 }]
}
```

### Camada 6: Metadata JSON
```json
{
  "elements": [
    {
      "guid": "2O_RrAJHv7xv2dl5cNZYOF",
      "ifcType": "IfcWall",
      "meshNode": 17,
      "name": "Parede 01",
      "properties": {
        "Pset_WallCommon": {
          "IsExternal": true,
          "LoadBearing": true,
          "FireRating": "EI60"
        }
      },
      "quantities": {
        "Length": 5.2,
        "Height": 3.0,
        "Width": 0.2,
        "Area": 15.6,
        "Volume": 3.12
      }
    }
  ],
  "structure": {
    "project": { "name": "Edifício Exemplo" },
    "site": { "name": "Lote 15" },
    "building": { "name": "Torre A" },
    "storeys": [
      { "id": "1F", "name": "Térreo", "elevation": 0.0 },
      { "id": "2F", "name": "Pavimento 1", "elevation": 3.0 }
    ]
  }
}
```

## Uso do Pipeline

### Código de Exemplo Completo
```rust
use avila_ifc::IfcParser;
use avila_tesselation::Tesselator;
use avila_optimizer::Optimizer;
use avila_gltf::{GltfExporter, ExportOptions};
use avila_metadata_extractor::MetadataExtractor;

fn convert_ifc_to_gltf(ifc_path: &str, output_dir: &str) -> anyhow::Result<()> {
    // 1. Parse IFC
    let ifc_content = std::fs::read_to_string(ifc_path)?;
    let parser = IfcParser::new(&ifc_content)?;
    let ifc_model = parser.parse()?;

    // 2. Tesselate geometria
    let tesselator = Tesselator::new();
    let mut scene = Scene::new();

    for element in &ifc_model.elements {
        if let Some(geom) = &element.geometry {
            let mesh = tesselator.tesselate(geom)?;
            scene.add_mesh(mesh);
        }

        // Material
        if let Some(mat_name) = &element.material {
            let material = PbrMaterial::from_ifc_material(&element.guid, mat_name);
            scene.add_material(material);
        }
    }

    // 3. Otimizar
    let optimizer = Optimizer::new();
    optimizer.optimize_scene(&mut scene)?;

    // 4. Exportar glTF
    let exporter = GltfExporter::new();
    let options = ExportOptions::default();
    let glb = exporter.export_glb(&scene, &options)?;

    let glb_path = format!("{}/model.glb", output_dir);
    std::fs::write(&glb_path, glb)?;

    // 5. Exportar metadata
    let metadata_extractor = MetadataExtractor::new();
    let metadata = metadata_extractor.extract(&ifc_model, &scene)?;

    let metadata_path = format!("{}/metadata.json", output_dir);
    std::fs::write(&metadata_path, serde_json::to_string_pretty(&metadata)?)?;

    println!("Conversão completa:");
    println!("  - GLB: {}", glb_path);
    println!("  - Metadata: {}", metadata_path);
    println!("  - Triângulos: {}", scene.triangle_count());
    println!("  - Vértices: {}", scene.vertex_count());

    Ok(())
}
```

## Próximos Passos

1. **Implementar avila-optimizer** (merge, LOD, Draco)
2. **Implementar avila-metadata-extractor** (JSON BIM)
3. **Criar worker avila-bim-converter** (orquestrador assíncrono)
4. **Implementar microserviços** (file-ingest, model-metadata, etc.)
5. **Setup infra** (docker-compose, PostgreSQL, MinIO, RabbitMQ)
6. **Criar frontends** (web-viewer com Three.js, mobile AR)

## Dependências Zero Externas

Todo o código é **100% Rust nativo**, sem dependências pesadas de C++ (IfcOpenShell) ou bibliotecas externas de geometria. Apenas:
- `serde` (serialização)
- `thiserror` (erros)
- Bibliotecas Avila internas

Isso permite:
- ✅ Build rápido
- ✅ Zero dependencies conflicts
- ✅ Total controle do código
- ✅ Performance otimizada
- ✅ WASM-ready (futuro)
