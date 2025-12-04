# avila-tesselation

**IFC Geometry Tesselation - 100% Rust**

Converts IFC parametric solids to triangle meshes.

## Features

- **ExtrudedAreaSolid**: Profile extrusion with caps and sides
- **BRep**: Boundary representation with face triangulation
- **Primitives**: Box, Cylinder, Sphere tesselation
- **Configurable**: Adjustable tolerance and segment counts

## Example

```rust
use avila_tesselation::{Tesselator, IfcGeometry, Profile, ProfileType};
use avila_vec3d::Vec3;

// Create tesselator
let tesselator = Tesselator::new(0.01); // 1cm tolerance

// Define rectangular profile
let profile = Profile {
    profile_type: ProfileType::Rectangle { width: 2.0, height: 1.0 },
    position: Vec3::ZERO,
};

// Extrude to 3D solid
let geometry = IfcGeometry::ExtrudedAreaSolid {
    profile,
    extrusion_direction: Vec3::Z,
    extrusion_depth: 3.0,
};

// Tesselate to mesh
let mesh = tesselator.tesselate(&geometry)?;
println!("Triangles: {}", mesh.indices.len() / 3);
```

## Supported IFC Types

- `IfcExtrudedAreaSolid`
- `IfcBoundingBox`
- `IfcCylinder`
- `IfcSphere`
- `IfcFacetedBrep`
- `IfcTriangulatedFaceSet`

## License

MIT OR Apache-2.0
