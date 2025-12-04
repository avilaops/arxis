# avila-gltf

**glTF 2.0 / GLB Exporter - 100% Rust**

Complete glTF 2.0 specification implementation for exporting 3D scenes.

## Features

- **GLB binary format**: Single-file glTF with embedded buffers
- **Full spec compliance**: Buffers, BufferViews, Accessors, Meshes, Materials
- **Automatic optimization**: u16/u32 index selection, padding, min/max bounds
- **PBR materials**: Full PBR metallic-roughness workflow
- **Tested**: Validates with glTF validators

## Example

```rust
use avila_gltf::{GltfExporter, ExportOptions};
use avila_mesh::{Scene, primitives};

// Create scene
let mut scene = Scene::new();
scene.add_mesh(primitives::cube(2.0));

// Export to GLB
let exporter = GltfExporter::new();
let glb_bytes = exporter.export_glb(&scene, &ExportOptions::default())?;

// Save to file
std::fs::write("model.glb", glb_bytes)?;
```

## Advanced Options

```rust
let options = ExportOptions {
    asset_name: "My Model".into(),
    include_normals: true,
    include_uvs: true,
};

let glb = exporter.export_glb(&scene, &options)?;
```

## Output Format

- **GLB**: Binary glTF 2.0 with embedded buffers
- **Optimized**: Minimal file size, GPU-ready buffers
- **Compatible**: Works with Three.js, Babylon.js, Unity, Unreal

## License

MIT OR Apache-2.0
