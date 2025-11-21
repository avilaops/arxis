//! Simple example to test wgpu backend

use avx_gpu_backend_wgpu::WgpuBackend;
use avx_gpu_core::prelude::*;

fn main() -> Result<()> {
    println!("🚀 Testing AVX-GPU wgpu backend\n");

    // Create device
    println!("Creating wgpu backend...");
    let backend = WgpuBackend::new()?;
    let device = Device::from_backend(Box::new(backend))?;
    
    let info = device.info();
    println!("✓ Device: {} ({:?})", info.name, info.device_type);
    println!("  Backend: {:?}\n", info.backend);

    // Test buffer allocation
    println!("Testing buffer operations...");
    let data = vec![1.0f32, 2.0, 3.0, 4.0];
    let mut buffer = device.buffer_from_slice(&data)?;
    println!("✓ Created buffer with {} elements", buffer.len());

    // Test buffer read
    let readback = buffer.read()?;
    println!("✓ Read buffer: {:?}", readback);

    assert_eq!(data, readback, "Buffer data mismatch!");
    
    println!("\n✅ All tests passed!");
    Ok(())
}
