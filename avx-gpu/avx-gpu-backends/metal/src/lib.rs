//! Metal backend for AVX-GPU (Apple Silicon)

#![cfg(feature = "metal")]

use avx_gpu_core::{
    backend::{Backend, BackendType},
    buffer::{BufferHandle, BufferUsage},
    device::{DeviceInfo, DeviceType},
    error::{Error, Result},
    kernel::{KernelArg, KernelHandle},
    types::LaunchConfig,
};

pub struct MetalBackend {
    // TODO: Implement with metal-rs
}

impl MetalBackend {
    pub fn new() -> Result<Self> {
        Err(Error::UnsupportedFeature(
            "Metal backend not yet implemented".into(),
        ))
    }
}

impl Backend for MetalBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Metal
    }

    fn device_info(&self) -> Result<DeviceInfo> {
        unimplemented!("Metal backend coming soon")
    }

    fn allocate_buffer(&mut self, _size: usize, _usage: BufferUsage) -> Result<BufferHandle> {
        unimplemented!()
    }

    fn free_buffer(&mut self, _handle: BufferHandle) -> Result<()> {
        unimplemented!()
    }

    fn write_buffer(&mut self, _handle: BufferHandle, _data: &[u8]) -> Result<()> {
        unimplemented!()
    }

    fn read_buffer(&mut self, _handle: BufferHandle, _data: &mut [u8]) -> Result<()> {
        unimplemented!()
    }

    fn copy_buffer(&mut self, _src: BufferHandle, _dst: BufferHandle, _size: usize) -> Result<()> {
        unimplemented!()
    }

    fn compile_kernel(&mut self, _source: &str, _entry_point: &str) -> Result<KernelHandle> {
        unimplemented!()
    }

    fn free_kernel(&mut self, _handle: KernelHandle) -> Result<()> {
        unimplemented!()
    }

    fn launch_kernel(
        &mut self,
        _kernel: KernelHandle,
        _config: LaunchConfig,
        _args: &[&dyn KernelArg],
    ) -> Result<()> {
        unimplemented!()
    }

    fn synchronize(&mut self) -> Result<()> {
        unimplemented!()
    }
}
