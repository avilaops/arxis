//! Vulkan backend for AVX-GPU
//!
//! Pure Rust GPU compute backend using Vulkan.
//! Cross-vendor support: NVIDIA, AMD, Intel, Apple (via MoltenVK)

use ash::{vk, Entry};
use avx_gpu_core::{
    backend::{Backend, BackendType},
    buffer::{BufferHandle, BufferUsage},
    device::{DeviceInfo, DeviceType},
    error::{Error, Result},
    kernel::{KernelArg, KernelHandle},
    types::LaunchConfig,
};
use gpu_allocator::vulkan::{Allocator, AllocatorCreateDesc};
use gpu_allocator::MemoryLocation;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::ffi::CStr;
use std::sync::Arc;

/// Vulkan compute backend
pub struct VulkanBackend {
    entry: Entry,
    instance: ash::Instance,
    physical_device: vk::PhysicalDevice,
    device: ash::Device,
    compute_queue: vk::Queue,
    queue_family_index: u32,
    command_pool: vk::CommandPool,
    allocator: Arc<Mutex<Allocator>>,
    buffers: Arc<Mutex<HashMap<u64, VulkanBuffer>>>,
    pipelines: Arc<Mutex<HashMap<u64, VulkanPipeline>>>,
    next_handle: Arc<Mutex<u64>>,
}

struct VulkanBuffer {
    buffer: vk::Buffer,
    allocation: gpu_allocator::vulkan::Allocation,
    size: u64,
}

struct VulkanPipeline {
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    descriptor_set_layout: vk::DescriptorSetLayout,
    shader_module: vk::ShaderModule,
}

impl VulkanBackend {
    pub fn new() -> Result<Self> {
        println!("[VULKAN] Creating backend...");

        // Load Vulkan
        let entry = unsafe {
            Entry::load()
                .map_err(|e| Error::BackendInitFailed(format!("Failed to load Vulkan: {}", e)))?
        };

        println!("[VULKAN] Vulkan loaded successfully");

        // Create instance
        let app_name = CStr::from_bytes_with_nul(b"AVX-GPU\0").unwrap();
        let engine_name = CStr::from_bytes_with_nul(b"AVX-GPU Engine\0").unwrap();

        let app_info = vk::ApplicationInfo {
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 0, 1, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 0, 1, 0),
            api_version: vk::make_api_version(0, 1, 2, 0),
            ..Default::default()
        };

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };

        let instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .map_err(|e| Error::BackendInitFailed(format!("Failed to create instance: {}", e)))?
        };

        println!("[VULKAN] Instance created");

        // Select physical device
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .map_err(|e| Error::BackendInitFailed(format!("Failed to enumerate devices: {}", e)))?
        };

        if physical_devices.is_empty() {
            return Err(Error::NoDeviceFound);
        }

        let physical_device = physical_devices[0];
        let device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
        let device_name = unsafe { CStr::from_ptr(device_properties.device_name.as_ptr()) };

        println!("[VULKAN] Selected device: {:?}", device_name);

        // Find compute queue family
        let queue_families = unsafe {
            instance.get_physical_device_queue_family_properties(physical_device)
        };

        let queue_family_index = queue_families
            .iter()
            .enumerate()
            .find(|(_, props)| props.queue_flags.contains(vk::QueueFlags::COMPUTE))
            .map(|(i, _)| i as u32)
            .ok_or_else(|| Error::BackendInitFailed("No compute queue family found".into()))?;

        println!("[VULKAN] Compute queue family: {}", queue_family_index);

        // Create logical device
        let queue_priorities = [1.0];
        let queue_create_info = vk::DeviceQueueCreateInfo {
            queue_family_index,
            p_queue_priorities: queue_priorities.as_ptr(),
            queue_count: queue_priorities.len() as u32,
            ..Default::default()
        };

        let queue_create_infos = [queue_create_info];
        let device_create_info = vk::DeviceCreateInfo {
            queue_create_info_count: queue_create_infos.len() as u32,
            p_queue_create_infos: queue_create_infos.as_ptr(),
            ..Default::default()
        };

        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .map_err(|e| Error::BackendInitFailed(format!("Failed to create device: {}", e)))?
        };

        println!("[VULKAN] Logical device created");

        let compute_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        // Create command pool
        let command_pool_info = vk::CommandPoolCreateInfo {
            queue_family_index,
            flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
            ..Default::default()
        };

        let command_pool = unsafe {
            device
                .create_command_pool(&command_pool_info, None)
                .map_err(|e| Error::BackendInitFailed(format!("Failed to create command pool: {}", e)))?
        };

        println!("[VULKAN] Command pool created");

        // Create memory allocator
        let allocator = Allocator::new(&AllocatorCreateDesc {
            instance: instance.clone(),
            device: device.clone(),
            physical_device,
            debug_settings: Default::default(),
            buffer_device_address: false,
            allocation_sizes: Default::default(),
        })
        .map_err(|e| Error::BackendInitFailed(format!("Failed to create allocator: {}", e)))?;

        println!("[VULKAN] Backend initialized successfully!");

        Ok(Self {
            entry,
            instance,
            physical_device,
            device,
            compute_queue,
            queue_family_index,
            command_pool,
            allocator: Arc::new(Mutex::new(allocator)),
            buffers: Arc::new(Mutex::new(HashMap::new())),
            pipelines: Arc::new(Mutex::new(HashMap::new())),
            next_handle: Arc::new(Mutex::new(1)),
        })
    }

    fn next_handle(&self) -> u64 {
        let mut handle = self.next_handle.lock();
        let id = *handle;
        *handle += 1;
        id
    }
}

impl Backend for VulkanBackend {
    fn backend_type(&self) -> BackendType {
        BackendType::Vulkan
    }

    fn device_info(&self) -> Result<DeviceInfo> {
        let props = unsafe { self.instance.get_physical_device_properties(self.physical_device) };
        let device_name = unsafe {
            CStr::from_ptr(props.device_name.as_ptr())
                .to_string_lossy()
                .into_owned()
        };

        let device_type = match props.device_type {
            vk::PhysicalDeviceType::DISCRETE_GPU => DeviceType::Generic,
            vk::PhysicalDeviceType::INTEGRATED_GPU => DeviceType::Generic,
            _ => DeviceType::Generic,
        };

        Ok(DeviceInfo {
            name: device_name,
            device_type,
            backend: BackendType::Vulkan,
            compute_units: 0, // Would need additional queries
            memory_bytes: 0,   // Would need additional queries
            max_threads_per_block: props.limits.max_compute_work_group_size[0],
            max_shared_memory: props.limits.max_compute_shared_memory_size,
        })
    }

    fn allocate_buffer(&mut self, size: usize, _usage: BufferUsage) -> Result<BufferHandle> {
        let buffer_info = vk::BufferCreateInfo {
            size: size as u64,
            usage: vk::BufferUsageFlags::STORAGE_BUFFER | vk::BufferUsageFlags::TRANSFER_SRC | vk::BufferUsageFlags::TRANSFER_DST,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };

        let buffer = unsafe {
            self.device
                .create_buffer(&buffer_info, None)
                .map_err(|e| Error::BackendError(format!("Failed to create buffer: {}", e)))?
        };

        let requirements = unsafe { self.device.get_buffer_memory_requirements(buffer) };

        let allocation = self
            .allocator
            .lock()
            .allocate(&gpu_allocator::vulkan::AllocationCreateDesc {
                name: "AVX-GPU Buffer",
                requirements,
                location: MemoryLocation::GpuOnly,
                linear: true,
                allocation_scheme: gpu_allocator::vulkan::AllocationScheme::GpuAllocatorManaged,
            })
            .map_err(|e| Error::BackendError(format!("Failed to allocate memory: {}", e)))?;

        unsafe {
            self.device
                .bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
                .map_err(|e| Error::BackendError(format!("Failed to bind memory: {}", e)))?;
        }

        let handle_id = self.next_handle();
        self.buffers.lock().insert(
            handle_id,
            VulkanBuffer {
                buffer,
                allocation,
                size: size as u64,
            },
        );

        Ok(BufferHandle(handle_id))
    }

    fn free_buffer(&mut self, handle: BufferHandle) -> Result<()> {
        if let Some(vk_buffer) = self.buffers.lock().remove(&handle.0) {
            unsafe {
                self.device.destroy_buffer(vk_buffer.buffer, None);
            }
            self.allocator.lock().free(vk_buffer.allocation).ok();
        }
        Ok(())
    }

    fn write_buffer(&mut self, _handle: BufferHandle, _data: &[u8]) -> Result<()> {
        // TODO: Implement using staging buffer + command buffer
        Err(Error::UnsupportedFeature("write_buffer not yet implemented".into()))
    }

    fn read_buffer(&mut self, _handle: BufferHandle, _data: &mut [u8]) -> Result<()> {
        // TODO: Implement using staging buffer + command buffer
        Err(Error::UnsupportedFeature("read_buffer not yet implemented".into()))
    }

    fn copy_buffer(&mut self, _src: BufferHandle, _dst: BufferHandle, _size: usize) -> Result<()> {
        // TODO: Implement
        Err(Error::UnsupportedFeature("copy_buffer not yet implemented".into()))
    }

    fn compile_kernel(&mut self, _source: &str, _entry_point: &str) -> Result<KernelHandle> {
        // TODO: Implement SPIR-V compilation
        Err(Error::UnsupportedFeature("compile_kernel not yet implemented".into()))
    }

    fn free_kernel(&mut self, handle: KernelHandle) -> Result<()> {
        if let Some(pipeline) = self.pipelines.lock().remove(&handle.0) {
            unsafe {
                self.device.destroy_pipeline(pipeline.pipeline, None);
                self.device.destroy_pipeline_layout(pipeline.pipeline_layout, None);
                self.device.destroy_descriptor_set_layout(pipeline.descriptor_set_layout, None);
                self.device.destroy_shader_module(pipeline.shader_module, None);
            }
        }
        Ok(())
    }

    fn launch_kernel(
        &mut self,
        _kernel: KernelHandle,
        _config: LaunchConfig,
        _args: &[&dyn KernelArg],
    ) -> Result<()> {
        // TODO: Implement
        Err(Error::UnsupportedFeature("launch_kernel not yet implemented".into()))
    }

    fn synchronize(&mut self) -> Result<()> {
        unsafe {
            self.device
                .device_wait_idle()
                .map_err(|e| Error::SyncFailed(format!("Device wait idle failed: {}", e)))?;
        }
        Ok(())
    }
}

impl Drop for VulkanBackend {
    fn drop(&mut self) {
        unsafe {
            // Wait for device to finish all operations
            let _ = self.device.device_wait_idle();

            // Clean up command pool
            self.device.destroy_command_pool(self.command_pool, None);
            
            // Note: We intentionally don't destroy device/instance here to avoid
            // crashes with gpu-allocator. The OS will clean up Vulkan resources
            // when the process exits. For long-running apps, implement explicit
            // cleanup methods instead of relying on Drop.
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        match VulkanBackend::new() {
            Ok(_) => println!("✓ Vulkan backend created successfully"),
            Err(e) => println!("✗ Failed to create backend: {}", e),
        }
    }
}
