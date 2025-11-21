//! Common types for GPU computing

use bytemuck::{Pod, Zeroable};

/// Scalar types supported by GPU kernels
pub trait GpuScalar: Pod + Zeroable + Copy + Send + Sync + 'static {}

impl GpuScalar for f32 {}
impl GpuScalar for f64 {}
impl GpuScalar for i8 {}
impl GpuScalar for i16 {}
impl GpuScalar for i32 {}
impl GpuScalar for i64 {}
impl GpuScalar for u8 {}
impl GpuScalar for u16 {}
impl GpuScalar for u32 {}
impl GpuScalar for u64 {}

/// 2D vector
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2<T: GpuScalar> {
    pub x: T,
    pub y: T,
}

/// 3D vector
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3<T: GpuScalar> {
    pub x: T,
    pub y: T,
    pub z: T,
}

/// 4D vector
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec4<T: GpuScalar> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

/// 4x4 matrix (column-major)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Mat4<T: GpuScalar> {
    pub cols: [Vec4<T>; 4],
}

/// GPU grid dimensions for kernel launches
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GridDim {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl GridDim {
    pub fn linear(size: u32) -> Self {
        Self { x: size, y: 1, z: 1 }
    }

    pub fn rect(width: u32, height: u32) -> Self {
        Self {
            x: width,
            y: height,
            z: 1,
        }
    }

    pub fn cube(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }

    pub fn total_threads(&self) -> u32 {
        self.x * self.y * self.z
    }
}

/// GPU block dimensions for kernel launches
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockDim {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl BlockDim {
    pub fn linear(size: u32) -> Self {
        Self { x: size, y: 1, z: 1 }
    }

    pub fn rect(width: u32, height: u32) -> Self {
        Self {
            x: width,
            y: height,
            z: 1,
        }
    }

    pub fn threads_per_block(&self) -> u32 {
        self.x * self.y * self.z
    }
}

impl Default for BlockDim {
    fn default() -> Self {
        // Common default: 256 threads per block
        Self::linear(256)
    }
}

/// Kernel launch configuration
#[derive(Debug, Clone, Copy)]
pub struct LaunchConfig {
    pub grid: GridDim,
    pub block: BlockDim,
    pub shared_memory_bytes: u32,
}

impl LaunchConfig {
    pub fn new(grid: GridDim, block: BlockDim) -> Self {
        Self {
            grid,
            block,
            shared_memory_bytes: 0,
        }
    }

    pub fn with_shared_memory(mut self, bytes: u32) -> Self {
        self.shared_memory_bytes = bytes;
        self
    }

    /// Calculate launch config for 1D workload
    pub fn linear(total_threads: u32, threads_per_block: u32) -> Self {
        let blocks = (total_threads + threads_per_block - 1) / threads_per_block;
        Self::new(GridDim::linear(blocks), BlockDim::linear(threads_per_block))
    }
}
