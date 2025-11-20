// Biblioteca Arxis - Matemática Avançada
// Quaternions para Rotações 3D e 4D com suporte para grupo SO(4)
// Tensores generalizados (ordem 0-4) para relatividade, ML e processamento de imagens
// Geometria 4D com politopos regulares e projeções

pub mod dual_quaternion;
pub mod geometry4d;
pub mod quaternion3d;
pub mod relativity;
pub mod tensor;
pub mod tensor4d;

pub use dual_quaternion::{DualQuat, SO4Rotation};
pub use geometry4d::{
    Cell24, Matrix4x4, Point4D, Projection4Dto3D, RigidBody4D, Simplex4D, Tesseract, Vector4D,
};
pub use quaternion3d::Quat3D;
pub use relativity::{LorentzTransform, MinkowskiMetric, RiemannTensor, StressEnergyTensor};
pub use tensor::{Matrix, Scalar, Tensor, Vector};
pub use tensor4d::{Tensor3D, Tensor4D};
