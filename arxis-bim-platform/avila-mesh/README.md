# avila-mesh

**3D Mesh Library - 100% Rust**

High-performance 3D mesh structures with PBR materials and primitive generators.

## Features

- **Vertex structures**: Position, normal, UV, tangent, color
- **Mesh operations**: Merge, transform, recalculate normals, to_buffers()
- **PBR Materials**: Physically-based rendering with IFC material mapping
- **Primitives**: Cube, sphere, plane generators
- **Scene management**: Multi-mesh scenes with material library

## Example

```rust
use avila_mesh::{Scene, Mesh, PbrMaterial, primitives};
use avila_vec3d::Vec3;

// Create scene
let mut scene = Scene::new();

// Add cube
let mut cube = primitives::cube(2.0);
cube.material_id = Some("concrete".into());
scene.add_mesh(cube);

// Add material
let material = PbrMaterial::from_ifc_material("concrete", "Concrete");
scene.add_material(material);

// Convert to GPU buffers
let buffers = scene.meshes[0].to_buffers();
println!("Vertices: {}", buffers.vertex_count());
```

## IFC Integration

Seamlessly converts IFC materials to PBR:

```rust
let material = PbrMaterial::from_ifc_material("IfcWall", "Concrete Wall");
// Automatically sets base_color, metallic, roughness
```

## License

MIT OR Apache-2.0
