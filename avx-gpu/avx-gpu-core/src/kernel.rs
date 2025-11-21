//! GPU kernel management

use crate::backend::Backend;
use crate::buffer::BufferHandle;
use crate::error::Result;
use crate::types::LaunchConfig;
use parking_lot::RwLock;
use std::sync::Arc;

/// Opaque handle to compiled kernel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KernelHandle(pub u64);

/// Trait for types that can be passed as kernel arguments
pub trait KernelArg: Send + Sync {
    fn as_arg(&self) -> KernelArgValue;
}

/// Kernel argument value
#[derive(Debug, Clone)]
pub enum KernelArgValue {
    Buffer(BufferHandle),
    Scalar(Vec<u8>),
}

impl KernelArg for BufferHandle {
    fn as_arg(&self) -> KernelArgValue {
        KernelArgValue::Buffer(*self)
    }
}

impl<T: bytemuck::Pod + Send + Sync> KernelArg for &crate::buffer::Buffer<T> {
    fn as_arg(&self) -> KernelArgValue {
        KernelArgValue::Buffer(self.handle())
    }
}

impl KernelArg for f32 {
    fn as_arg(&self) -> KernelArgValue {
        KernelArgValue::Scalar(bytemuck::bytes_of(self).to_vec())
    }
}

impl KernelArg for u32 {
    fn as_arg(&self) -> KernelArgValue {
        KernelArgValue::Scalar(bytemuck::bytes_of(self).to_vec())
    }
}

impl KernelArg for i32 {
    fn as_arg(&self) -> KernelArgValue {
        KernelArgValue::Scalar(bytemuck::bytes_of(self).to_vec())
    }
}

/// Compiled GPU kernel
pub struct Kernel {
    handle: KernelHandle,
    backend: Arc<RwLock<Box<dyn Backend>>>,
}

impl Kernel {
    pub(crate) fn new(handle: KernelHandle, backend: Arc<RwLock<Box<dyn Backend>>>) -> Self {
        Self { handle, backend }
    }

    /// Get kernel handle
    pub fn handle(&self) -> KernelHandle {
        self.handle
    }

    /// Launch kernel with configuration
    pub fn launch(&self, config: LaunchConfig, args: &[&dyn KernelArg]) -> Result<()> {
        self.backend
            .write()
            .launch_kernel(self.handle, config, args)
    }

    /// Launch kernel with automatic configuration
    pub fn launch_auto(&self, total_threads: u32, args: &[&dyn KernelArg]) -> Result<()> {
        let config = LaunchConfig::linear(total_threads, 256);
        self.launch(config, args)
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        let _ = self.backend.write().free_kernel(self.handle);
    }
}

impl std::fmt::Debug for Kernel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Kernel")
            .field("handle", &self.handle)
            .finish()
    }
}
