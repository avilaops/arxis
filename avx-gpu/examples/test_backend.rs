//! Simple example to test wgpu backend

use avx_gpu_backend_wgpu::WgpuBackend;
use avx_gpu_core::prelude::*;

fn main() {
    println!("🚀 Testing AVX-GPU wgpu backend\n");

    // Create device
    println!("Creating wgpu backend...");
    match WgpuBackend::new() {
        Ok(backend) => {
            println!("✓ Backend created successfully");

            match Device::from_backend(Box::new(backend)) {
                Ok(device) => {
                    let info = device.info();
                    println!("✓ Device: {} ({:?})", info.name, info.device_type);
                    println!("  Backend: {:?}\n", info.backend);

                    // Test buffer allocation
                    println!("Testing buffer operations...");
                    let data = vec![1.0f32, 2.0, 3.0, 4.0];
                    match device.buffer_from_slice(&data) {
                        Ok(buffer) => {
                            println!("✓ Created buffer with {} elements", buffer.len());

                            // Test buffer read
                            match buffer.read() {
                                Ok(readback) => {
                                    println!("✓ Read buffer: {:?}", readback);

                                    if data == readback {
                                        println!("\n✅ All tests passed!");
                                    } else {
                                        eprintln!("❌ Buffer data mismatch!");
                                    }
                                }
                                Err(e) => eprintln!("❌ Failed to read buffer: {}", e),
                            }
                        }
                        Err(e) => eprintln!("❌ Failed to create buffer: {}", e),
                    }
                }
                Err(e) => eprintln!("❌ Failed to create device: {}", e),
            }
        }
        Err(e) => eprintln!("❌ Failed to create backend: {}", e),
    }
}
