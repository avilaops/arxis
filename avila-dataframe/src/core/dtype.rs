//! Scientific data types unique to avila-dataframe
#![allow(missing_docs)]

use num_complex::Complex;

/// Extended data types for scientific computing
#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum DType {
    // Basic Arrow types
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Bool,
    String,
    Binary,

    // Complex numbers
    Complex64,  // Complex<f32>
    Complex128, // Complex<f64>

    // Advanced scientific types (DIFFERENTIAL!)
    Quaternion,               // For 3D/4D rotations, orientation tracking
    Tensor4D(Shape4D),        // Space-time data (x, y, z, t)
    SpinorWeyl,               // Particle physics (2-component spinors)
    Geodesic,                 // Coordinates on curved manifolds (GR)
    TimeSeries(String),       // Time series with timezone
    Categorical(Vec<String>), // Categorical with dictionary
    Nested(Box<Schema>),      // Nested DataFrames
    Graph(GraphType),         // Graph structures as column type
}

/// Shape for 4D tensors
#[derive(Debug, Clone, PartialEq)]
pub struct Shape4D {
    /// Dimension 1 (e.g., x)
    pub dim1: usize,
    /// Dimension 2 (e.g., y)
    pub dim2: usize,
    /// Dimension 3 (e.g., z)
    pub dim3: usize,
    /// Dimension 4 (e.g., time)
    pub dim4: usize,
}

/// Graph types for graph-as-column
#[derive(Debug, Clone, PartialEq)]
pub enum GraphType {
    /// Directed graph
    Directed,
    /// Undirected graph
    Undirected,
    /// Weighted graph
    Weighted,
    /// Bipartite graph
    Bipartite,
}

/// Schema definition
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    /// Column names and types
    pub fields: Vec<(String, DType)>,
}

impl Schema {
    /// Create a new schema
    pub fn new(fields: Vec<(String, DType)>) -> Self {
        Self { fields }
    }

    /// Get field by name
    pub fn get_field(&self, name: &str) -> Option<&DType> {
        self.fields.iter().find(|(n, _)| n == name).map(|(_, t)| t)
    }

    /// Get all field names
    pub fn field_names(&self) -> Vec<&str> {
        self.fields.iter().map(|(n, _)| n.as_str()).collect()
    }
}

/// Quaternion data for rotations (DIFFERENTIAL - ninguém tem isso!)
#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    /// Real part
    pub w: f64,
    /// i component
    pub x: f64,
    /// j component
    pub y: f64,
    /// k component
    pub z: f64,
}

impl Quaternion {
    /// Create a new quaternion
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }

    /// Create from axis-angle representation
    pub fn from_axis_angle(axis: [f64; 3], angle: f64) -> Self {
        let half_angle = angle / 2.0;
        let s = half_angle.sin();
        Self {
            w: half_angle.cos(),
            x: axis[0] * s,
            y: axis[1] * s,
            z: axis[2] * s,
        }
    }

    /// Normalize the quaternion
    pub fn normalize(self) -> Self {
        let norm = (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            w: self.w / norm,
            x: self.x / norm,
            y: self.y / norm,
            z: self.z / norm,
        }
    }

    /// Convert to rotation matrix
    pub fn to_rotation_matrix(&self) -> [[f64; 3]; 3] {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        [
            [
                1.0 - 2.0 * (y * y + z * z),
                2.0 * (x * y - w * z),
                2.0 * (x * z + w * y),
            ],
            [
                2.0 * (x * y + w * z),
                1.0 - 2.0 * (x * x + z * z),
                2.0 * (y * z - w * x),
            ],
            [
                2.0 * (x * z - w * y),
                2.0 * (y * z + w * x),
                1.0 - 2.0 * (x * x + y * y),
            ],
        ]
    }
}

/// Weyl spinor for particle physics (DIFFERENTIAL!)
#[derive(Debug, Clone, Copy)]
pub struct SpinorWeyl {
    /// Component 1
    pub a: Complex<f64>,
    /// Component 2
    pub b: Complex<f64>,
}

impl SpinorWeyl {
    /// Create a new Weyl spinor
    pub fn new(a: Complex<f64>, b: Complex<f64>) -> Self {
        Self { a, b }
    }

    /// Lorentz boost
    pub fn boost(&self, beta: f64) -> Self {
        let gamma = 1.0 / (1.0 - beta * beta).sqrt();
        let factor = ((gamma - 1.0) / 2.0).sqrt();
        Self {
            a: self.a * Complex::new(gamma, 0.0) + self.b * Complex::new(factor, 0.0),
            b: self.b * Complex::new(gamma, 0.0) + self.a * Complex::new(factor, 0.0),
        }
    }
}

/// Geodesic coordinates for curved space-time (DIFFERENTIAL!)
#[derive(Debug, Clone, Copy)]
pub struct GeodesicCoord {
    /// Time coordinate
    pub t: f64,
    /// Radial coordinate
    pub r: f64,
    /// Theta angle
    pub theta: f64,
    /// Phi angle
    pub phi: f64,
}

impl GeodesicCoord {
    /// Create new geodesic coordinate
    pub fn new(t: f64, r: f64, theta: f64, phi: f64) -> Self {
        Self { t, r, theta, phi }
    }

    /// Convert to Cartesian (flat space approximation)
    pub fn to_cartesian(&self) -> [f64; 4] {
        [
            self.t,
            self.r * self.theta.sin() * self.phi.cos(),
            self.r * self.theta.sin() * self.phi.sin(),
            self.r * self.theta.cos(),
        ]
    }

    /// Schwarzschild metric component g_tt
    pub fn schwarzschild_gtt(&self, mass: f64) -> f64 {
        let rs = 2.0 * mass; // Schwarzschild radius (G=c=1 units)
        -(1.0 - rs / self.r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_quaternion_normalize() {
        let q = Quaternion::new(1.0, 2.0, 3.0, 4.0).normalize();
        let norm = (q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z).sqrt();
        assert_abs_diff_eq!(norm, 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_geodesic_schwarzschild() {
        let coord = GeodesicCoord::new(0.0, 10.0, 0.0, 0.0);
        let gtt = coord.schwarzschild_gtt(1.0);
        assert!(gtt < 0.0); // Timelike
        assert!(gtt > -1.0); // Outside event horizon
    }
}
