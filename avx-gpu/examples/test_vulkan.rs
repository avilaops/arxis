//! Test Vulkan backend

use avx_gpu_backend_vulkan::VulkanBackend;
use avx_gpu_core::prelude::*;

fn main() {
    println!("🚀 Testing AVX-GPU Vulkan Backend\n");

    println!("Creating Vulkan backend...");
    match VulkanBackend::new() {
        Ok(backend) => {
            println!("✓ Backend created successfully!");
            
            match Device::from_backend(Box::new(backend)) {
                Ok(device) => {
                    let info = device.info();
                    println!("\n📊 Device Info:");
                    println!("  Name: {}", info.name);
                    println!("  Type: {:?}", info.device_type);
                    println!("  Backend: {:?}", info.backend);
                    println!("  Max threads/block: {}", info.max_threads_per_block);
                    println!("  Max shared memory: {} bytes", info.max_shared_memory);

                    // Test buffer allocation
                    println!("\n🔧 Testing buffer allocation...");
                    match device.buffer::<f32>(1024) {
                        Ok(buffer) => {
                            println!("✓ Allocated buffer: {} elements ({} bytes)", 
                                buffer.len(), buffer.size_bytes());
                            println!("\n✅ All basic tests passed!");
                        }
                        Err(e) => eprintln!("❌ Failed to allocate buffer: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Failed to create device: {}", e),
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to create Vulkan backend: {}", e);
            eprintln!("\n💡 Troubleshooting:");
            eprintln!("  1. Install Vulkan SDK: https://vulkan.lunarg.com/");
            eprintln!("  2. Update GPU drivers");
            eprintln!("  3. Check if your GPU supports Vulkan");
        }
    }
}
