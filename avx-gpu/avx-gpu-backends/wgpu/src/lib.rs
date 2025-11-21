//! WebGPU/wgpu backend for AVX-GPU
//!
//! This is the most portable backend, supporting:
//! - Windows (DX12, Vulkan)
//! - Linux (Vulkan)
//! - macOS (Metal)
//! - Web (WebGPU)

use avx_gpu_core::{
    backend::{Backend, BackendType},
    buffer::{BufferHandle, BufferUsage},
    device::{DeviceInfo, DeviceType},
    error::{Error, Result},
    kernel::{KernelArg, KernelHandle},
    types::LaunchConfig,
};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub struct WgpuBackend {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    adapter: Arc<wgpu::Adapter>,
    buffers: Arc<Mutex<HashMap<u64, Arc<wgpu::Buffer>>>>,
    pipelines: Arc<Mutex<HashMap<u64, Arc<wgpu::ComputePipeline>>>>,
    next_handle: Arc<Mutex<u64>>,
}

impl WgpuBackend {
    pub fn new() -> Result<Self> {
        println!("[DEBUG] Entering WgpuBackend::new()");
        let result = Self::new_async().map_err(|e| {
            println!("[DEBUG] new_async failed: {}", e);
            Error::BackendInitFailed(e.to_string())
        });
        println!("[DEBUG] Exiting WgpuBackend::new()");
        result
    }

    fn new_async() -> anyhow::Result<Self> {
        println!("[DEBUG] Entering new_async");
        pollster::block_on(async {
            println!("[DEBUG] Creating instance");
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                ..Default::default()
            });

            println!("[DEBUG] Requesting adapter");
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    force_fallback_adapter: false,
                    compatible_surface: None,
                })
                .await
                .ok_or_else(|| anyhow::anyhow!("No suitable GPU adapter found"))?;

            println!("[DEBUG] Adapter found: {:?}", adapter.get_info());

            println!("[DEBUG] Requesting device");
            let (device, queue) = adapter
                .request_device(
                    &wgpu::DeviceDescriptor {
                        label: Some("AVX-GPU Device"),
                        required_features: wgpu::Features::empty(),
                        required_limits: wgpu::Limits::default(),
                        memory_hints: wgpu::MemoryHints::Performance,
                    },
                    None,
                )
                .await?;

            println!("[DEBUG] Device created successfully");

            Ok(Self {
                device: Arc::new(device),
                queue: Arc::new(queue),
                adapter: Arc::new(adapter),
                buffers: Arc::new(Mutex::new(HashMap::new())),
                pipelines: Arc::new(Mutex::new(HashMap::new())),
                next_handle: Arc::new(Mutex::new(1)),
            })
        })
    }

    fn next_handle(&self) -> u64 {
        let mut handle = self.next_handle.lock();
        let id = *handle;
        *handle += 1;
        id
    }

    fn get_buffer(&self, handle: BufferHandle) -> Result<Arc<wgpu::Buffer>> {
        self.buffers
            .lock()
            .get(&handle.0)
            .cloned()
            .ok_or_else(|| Error::BackendError(format!("Invalid buffer handle: {:?}", handle)))
    }

    fn get_pipeline(&self, handle: KernelHandle) -> Result<Arc<wgpu::ComputePipeline>> {
        self.pipelines
            .lock()
            .get(&handle.0)
            .cloned()
            .ok_or_else(|| Error::BackendError(format!("Invalid kernel handle: {:?}", handle)))
    }
}

impl Backend for WgpuBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Wgpu
    }

    fn device_info(&self) -> Result<DeviceInfo> {
        let info = self.adapter.get_info();

        let device_type = match info.device_type {
            wgpu::DeviceType::DiscreteGpu => {
                if info.name.to_lowercase().contains("nvidia") {
                    DeviceType::Nvidia
                } else if info.name.to_lowercase().contains("amd") {
                    DeviceType::Amd
                } else if info.name.to_lowercase().contains("intel") {
                    DeviceType::Intel
                } else {
                    DeviceType::Generic
                }
            }
            wgpu::DeviceType::IntegratedGpu => DeviceType::Generic,
            wgpu::DeviceType::VirtualGpu => DeviceType::Generic,
            wgpu::DeviceType::Cpu => DeviceType::Cpu,
            wgpu::DeviceType::Other => DeviceType::Generic,
        };

        let limits = self.device.limits();

        Ok(DeviceInfo {
            name: info.name.clone(),
            device_type,
            backend: BackendType::Wgpu,
            compute_units: 0, // wgpu doesn't expose this
            memory_bytes: 0,   // wgpu doesn't expose this directly
            max_threads_per_block: limits.max_compute_workgroup_size_x,
            max_shared_memory: limits.max_compute_workgroup_storage_size,
        })
    }

    fn allocate_buffer(&mut self, size: usize, usage: BufferUsage) -> Result<BufferHandle> {
        let mut wgpu_usage = wgpu::BufferUsages::empty();

        if usage.storage {
            wgpu_usage |= wgpu::BufferUsages::STORAGE;
        }
        if usage.uniform {
            wgpu_usage |= wgpu::BufferUsages::UNIFORM;
        }
        if usage.copy_src {
            wgpu_usage |= wgpu::BufferUsages::COPY_SRC;
        }
        if usage.copy_dst {
            wgpu_usage |= wgpu::BufferUsages::COPY_DST;
        }

        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("AVX-GPU Buffer"),
            size: size as u64,
            usage: wgpu_usage,
            mapped_at_creation: false,
        });

        let handle_id = self.next_handle();
        self.buffers.lock().insert(handle_id, Arc::new(buffer));

        Ok(BufferHandle(handle_id))
    }

    fn free_buffer(&mut self, handle: BufferHandle) -> Result<()> {
        self.buffers.lock().remove(&handle.0);
        Ok(())
    }

    fn write_buffer(&mut self, handle: BufferHandle, data: &[u8]) -> Result<()> {
        let buffer = self.get_buffer(handle)?;
        self.queue.write_buffer(&buffer, 0, data);
        Ok(())
    }

    fn read_buffer(&mut self, handle: BufferHandle, data: &mut [u8]) -> Result<()> {
        let buffer = self.get_buffer(handle)?;

        // Create staging buffer
        let staging_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Staging Buffer"),
            size: data.len() as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Copy GPU buffer to staging
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Copy Encoder"),
        });
        encoder.copy_buffer_to_buffer(&buffer, 0, &staging_buffer, 0, data.len() as u64);
        self.queue.submit(Some(encoder.finish()));

        // Map and read - simplified approach
        let buffer_slice = staging_buffer.slice(..);

        // Request mapping
        let (tx, rx) = std::sync::mpsc::channel();
        buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).ok();
        });

        // Poll device until mapping is done
        self.device.poll(wgpu::Maintain::Wait);

        // Wait for mapping result
        rx.recv()
            .map_err(|e| Error::MemoryCopyFailed(format!("Channel error: {}", e)))?
            .map_err(|e| Error::MemoryCopyFailed(format!("Mapping error: {:?}", e)))?;

        // Copy data
        let mapped = buffer_slice.get_mapped_range();
        data.copy_from_slice(&mapped);
        drop(mapped);
        staging_buffer.unmap();

        Ok(())
    }    fn copy_buffer(&mut self, src: BufferHandle, dst: BufferHandle, size: usize) -> Result<()> {
        let src_buffer = self.get_buffer(src)?;
        let dst_buffer = self.get_buffer(dst)?;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Copy Encoder"),
        });
        encoder.copy_buffer_to_buffer(&src_buffer, 0, &dst_buffer, 0, size as u64);
        self.queue.submit(Some(encoder.finish()));

        Ok(())
    }

    fn compile_kernel(&mut self, source: &str, entry_point: &str) -> Result<KernelHandle> {
        let shader = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("AVX-GPU Kernel"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("AVX-GPU Pipeline"),
            layout: None,
            module: &shader,
            entry_point,
            compilation_options: Default::default(),
            cache: None,
        });

        let handle_id = self.next_handle();
        self.pipelines.lock().insert(handle_id, Arc::new(pipeline));

        Ok(KernelHandle(handle_id))
    }

    fn free_kernel(&mut self, handle: KernelHandle) -> Result<()> {
        self.pipelines.lock().remove(&handle.0);
        Ok(())
    }

    fn launch_kernel(
        &mut self,
        kernel: KernelHandle,
        config: LaunchConfig,
        _args: &[&dyn KernelArg],
    ) -> Result<()> {
        let pipeline = self.get_pipeline(kernel)?;

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Compute Encoder"),
        });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("AVX-GPU Compute Pass"),
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&pipeline);
            compute_pass.dispatch_workgroups(config.grid.x, config.grid.y, config.grid.z);
        }

        self.queue.submit(Some(encoder.finish()));
        Ok(())
    }

    fn synchronize(&mut self) -> Result<()> {
        self.device.poll(wgpu::Maintain::Wait);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let backend = WgpuBackend::new();
        assert!(backend.is_ok(), "Should create wgpu backend");
    }

    #[test]
    fn test_buffer_allocation() {
        let mut backend = WgpuBackend::new().unwrap();
        let handle = backend.allocate_buffer(1024, BufferUsage::default());
        assert!(handle.is_ok(), "Should allocate buffer");
    }
}
