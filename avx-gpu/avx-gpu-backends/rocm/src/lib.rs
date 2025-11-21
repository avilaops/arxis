//! ROCm/HIP backend for AVX-GPU (AMD GPUs)

use avx_gpu_core::{
    backend::{Backend, BackendType},
    buffer::{BufferHandle, BufferUsage},
    device::{DeviceInfo, DeviceType},
    error::{Error, Result},
    kernel::{KernelArg, KernelHandle},
    types::LaunchConfig,
};

pub struct RocmBackend {
    // TODO: Implement with HIP bindings
}

impl RocmBackend {
    pub fn new() -> Result<Self> {
        Err(Error::UnsupportedFeature(
            "ROCm backend not yet implemented".into(),
        ))
    }
}

impl Backend for RocmBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Rocm
    }

    fn device_info(&self) -> Result<DeviceInfo> {
        unimplemented!("ROCm backend coming soon")
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
