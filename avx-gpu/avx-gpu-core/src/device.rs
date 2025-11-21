//! GPU device management and detection

use crate::backend::{Backend, BackendType};
use crate::buffer::{Buffer, BufferUsage};
use crate::error::{Error, Result};
use crate::kernel::Kernel;
use parking_lot::RwLock;
use std::sync::Arc;

/// GPU device types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    /// NVIDIA GPU (CUDA)
    Nvidia,
    /// AMD GPU (ROCm/HIP)
    Amd,
    /// Apple Silicon (Metal)
    Apple,
    /// Intel GPU
    Intel,
    /// Generic GPU (Vulkan/OpenCL)
    Generic,
    /// CPU fallback
    Cpu,
}

/// Information about a GPU device
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub name: String,
    pub device_type: DeviceType,
    pub backend: BackendType,
    pub compute_units: u32,
    pub memory_bytes: u64,
    pub max_threads_per_block: u32,
    pub max_shared_memory: u32,
}

/// GPU compute device
pub struct Device {
    info: DeviceInfo,
    backend: Arc<RwLock<Box<dyn Backend>>>,
}

impl Device {
    /// Create device from backend
    pub fn from_backend(backend: Box<dyn Backend>) -> Result<Self> {
        let info = backend.device_info()?;
        Ok(Self {
            info,
            backend: Arc::new(RwLock::new(backend)),
        })
    }

    /// Auto-detect best available GPU device
    pub fn auto() -> Result<Self> {
        Err(Error::UnsupportedFeature(
            "Device::auto() must be called from a crate that includes a backend implementation".into()
        ))
    }

    /// List all available devices
    pub fn list_devices() -> Result<Vec<Self>> {
        let mut devices = Vec::new();

        // Always try wgpu backend (most portable)
        // Other backends can be added based on runtime detection
        devices.push(Self::auto()?);

        Ok(devices)
    }

    /// Get device information
    pub fn info(&self) -> &DeviceInfo {
        &self.info
    }

    /// Allocate GPU buffer
    pub fn buffer<T: bytemuck::Pod>(&self, count: usize) -> Result<Buffer<T>> {
        let size = count * std::mem::size_of::<T>();
        let handle = self
            .backend
            .write()
            .allocate_buffer(size, BufferUsage::default())?;
        Ok(Buffer::new(handle, count, Arc::clone(&self.backend)))
    }

    /// Create GPU buffer from slice
    pub fn buffer_from_slice<T: bytemuck::Pod>(&self, data: &[T]) -> Result<Buffer<T>> {
        let mut buffer = self.buffer(data.len())?;
        buffer.write(data)?;
        Ok(buffer)
    }

    /// Compile kernel from source
    pub fn compile_kernel(&self, source: &str, entry_point: &str) -> Result<Kernel> {
        let handle = self.backend.write().compile_kernel(source, entry_point)?;
        Ok(Kernel::new(handle, Arc::clone(&self.backend)))
    }

    /// Execute kernel (simple API)
    #[allow(dead_code)]
    pub fn execute_kernel(&self, kernel: &Kernel, args: &[&dyn crate::kernel::KernelArg]) -> Result<()> {
        self.backend.write().execute_kernel(kernel.handle(), args)
    }

    /// Synchronize device (wait for all operations to complete)
    pub fn synchronize(&self) -> Result<()> {
        self.backend.write().synchronize()
    }
}

impl std::fmt::Debug for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Device")
            .field("info", &self.info)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_types() {
        assert_ne!(DeviceType::Nvidia, DeviceType::Amd);
        assert_eq!(DeviceType::Nvidia, DeviceType::Nvidia);
    }
}
