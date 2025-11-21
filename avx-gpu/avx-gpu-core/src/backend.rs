//! Backend abstraction layer

use crate::buffer::{BufferHandle, BufferUsage};
use crate::device::DeviceInfo;
use crate::error::Result;
use crate::kernel::{KernelArg, KernelHandle};
use crate::types::LaunchConfig;

/// Backend type identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BackendType {
    Wgpu,
    Cuda,
    Metal,
    Rocm,
    Vulkan,
}

/// GPU backend trait
pub trait Backend: Send + Sync {
    /// Get backend type
    fn backend_type(&self) -> BackendType;

    /// Get device information
    fn device_info(&self) -> Result<DeviceInfo>;

    /// Allocate GPU buffer
    fn allocate_buffer(&mut self, size: usize, usage: BufferUsage) -> Result<BufferHandle>;

    /// Free GPU buffer
    fn free_buffer(&mut self, handle: BufferHandle) -> Result<()>;

    /// Write data to buffer
    fn write_buffer(&mut self, handle: BufferHandle, data: &[u8]) -> Result<()>;

    /// Read data from buffer
    fn read_buffer(&mut self, handle: BufferHandle, data: &mut [u8]) -> Result<()>;

    /// Copy between buffers
    fn copy_buffer(&mut self, src: BufferHandle, dst: BufferHandle, size: usize) -> Result<()>;

    /// Compile kernel from source
    fn compile_kernel(&mut self, source: &str, entry_point: &str) -> Result<KernelHandle>;

    /// Free compiled kernel
    fn free_kernel(&mut self, handle: KernelHandle) -> Result<()>;

    /// Launch kernel with configuration
    fn launch_kernel(
        &mut self,
        kernel: KernelHandle,
        config: LaunchConfig,
        args: &[&dyn KernelArg],
    ) -> Result<()>;

    /// Execute kernel (simple API without explicit config)
    fn execute_kernel(&mut self, kernel: KernelHandle, args: &[&dyn KernelArg]) -> Result<()> {
        let config = LaunchConfig::linear(1024, 256); // Default config
        self.launch_kernel(kernel, config, args)
    }

    /// Synchronize device
    fn synchronize(&mut self) -> Result<()>;
}
