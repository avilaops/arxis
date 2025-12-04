# avila-vec3d

**3D Mathematics Library - 100% Rust, Zero Dependencies**

Foundational 3D math types for graphics, games, and BIM applications.

## Features

- **Vec2, Vec3, Vec4**: 2D/3D/4D vectors with complete operations
- **Mat4**: 4x4 transformation matrices (translation, rotation, scale, perspective)
- **Quaternion**: Rotation quaternions with slerp interpolation
- **AABB**: Axis-aligned bounding boxes with collision detection
- **Ray**: Ray casting with AABB intersection tests

## Example

```rust
use avila_vec3d::{Vec3, Mat4, Quat, Aabb};

// Vector operations
let v1 = Vec3::new(1.0, 2.0, 3.0);
let v2 = Vec3::new(4.0, 5.0, 6.0);
let dot = v1.dot(v2);
let cross = v1.cross(v2);

// Matrix transformations
let translation = Mat4::translation(Vec3::new(10.0, 0.0, 0.0));
let rotation = Mat4::rotation_y(std::f32::consts::PI / 2.0);
let transform = translation * rotation;

// Quaternions
let q = Quat::from_axis_angle(Vec3::Y, std::f32::consts::PI / 4.0);
let rotated = q.rotate_vec3(Vec3::X);

// Bounding boxes
let mut aabb = Aabb::EMPTY;
aabb.expand_point(Vec3::new(1.0, 2.0, 3.0));
aabb.expand_point(Vec3::new(4.0, 5.0, 6.0));
println!("Center: {:?}", aabb.center());
```

## Zero Dependencies

Only uses `serde` for serialization (optional feature).

## License

MIT OR Apache-2.0
