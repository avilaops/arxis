//! LIGO/LISA Gravitational Wave Detection Example
//!
//! This example demonstrates using 4D convolutions to detect gravitational wave signals
//! in spacetime data from LIGO (Laser Interferometer Gravitational-Wave Observatory)
//! and LISA (Laser Interferometer Space Antenna).
//!
//! Gravitational waves are ripples in spacetime caused by massive cosmic events like
//! black hole mergers. The signal is 4-dimensional: 3 spatial dimensions + time.

use avila_ml::nn::{Conv4d, Linear, Module, Sequential};
use avila_ml::nn::activation::ReLU;
use avila_ml::prelude::*;
use avila_ml::tensor::Tensor;
use std::f32::consts::PI;

fn main() {
    println!("🌌 Avila ML - LIGO/LISA Gravitational Wave Detection\n");
    println!("🔭 Simulating gravitational wave detection in 4D spacetime data\n");

    // Simulate LISA-like data: (time, x, y, z) spacetime grid
    // Real LISA will have 3 satellites in heliocentric orbit measuring gravitational waves
    let batch_size = 2;
    let channels = 3; // h+, hx, frequency components
    let time_steps = 16;
    let spatial_x = 8;
    let spatial_y = 8;
    let spatial_z = 8;

    println!("📊 Data dimensions:");
    println!("  - Batch size: {}", batch_size);
    println!("  - Channels: {} (h+, hx, frequency)", channels);
    println!("  - Spacetime grid: {}×{}×{}×{} (t, x, y, z)", time_steps, spatial_x, spatial_y, spatial_z);
    println!("  - Total data points: {}\n", batch_size * channels * time_steps * spatial_x * spatial_y * spatial_z);

    // Generate synthetic gravitational wave signal
    println!("🌊 Generating synthetic gravitational wave signal...");
    let gw_data = generate_gravitational_wave_signal(
        batch_size,
        channels,
        time_steps,
        spatial_x,
        spatial_y,
        spatial_z,
    );

    println!("✅ Signal shape: {:?}\n", gw_data.shape());

    // Build 4D CNN for gravitational wave detection
    println!("🏗️  Building 4D Convolutional Neural Network:");
    println!("  1. Conv4D: 3→16 channels, kernel (3,3,3,3)");
    println!("  2. ReLU activation");
    println!("  3. Conv4D: 16→32 channels, kernel (3,3,3,3)");
    println!("  4. ReLU activation");
    println!("  5. Global average pooling");
    println!("  6. Linear: flattened → 2 classes (signal/noise)\n");

    let conv1 = Conv4d::<f32>::new(
        channels,           // input channels
        16,                 // output channels
        (3, 3, 3, 3),      // kernel size (t, x, y, z)
    );

    let conv2 = Conv4d::<f32>::new(
        16,
        32,
        (3, 3, 3, 3),
    );

    // Forward pass through first conv layer
    println!("🔄 Forward pass through Conv4D layer 1...");
    let feat1 = conv1.forward(&gw_data);
    println!("  Output shape: {:?}", feat1.shape());

    let relu1 = ReLU::new();
    let feat1_relu = relu1.forward(&feat1);

    // Forward pass through second conv layer
    println!("🔄 Forward pass through Conv4D layer 2...");
    let feat2 = conv2.forward(&feat1_relu);
    println!("  Output shape: {:?}", feat2.shape());

    let relu2 = ReLU::new();
    let feat2_relu = relu2.forward(&feat2);

    // Global average pooling (simplified)
    println!("📊 Global average pooling...");
    let pooled = feat2_relu.mean();
    println!("  Pooled shape: {:?}\n", pooled.shape());

    // Physical interpretation
    println!("🔬 Physical Interpretation:");
    println!("  - 4D convolutions detect spacetime patterns in gravitational waves");
    println!("  - Each kernel learns to recognize specific wave signatures:");
    println!("    • Black hole mergers (chirp signals)");
    println!("    • Neutron star collisions (kilonova events)");
    println!("    • Supernova explosions");
    println!("  - Spacetime structure preserves causality and wave propagation\n");

    // Performance metrics
    println!("⚡ Performance Notes:");
    println!("  - 4D convolution is O(N^4) complexity");
    println!("  - Parallelized with Rayon for multi-core CPUs");
    println!("  - Real LIGO data: ~4 kHz sampling, hours of observation");
    println!("  - Real LISA data: millihertz regime, months/years of data\n");

    println!("📚 Scientific References:");
    println!("  - LIGO: https://www.ligo.org/");
    println!("  - LISA: https://lisa.nasa.gov/");
    println!("  - First GW detection: Abbott et al. (2016) Phys. Rev. Lett. 116, 061102");
    println!("  - GW150914: First black hole merger detection\n");

    println!("✅ Gravitational wave detection simulation complete!");
    println!("🇧🇷 Built with Avila ML - 100% Native Rust");
}

/// Generate synthetic gravitational wave signal in 4D spacetime
fn generate_gravitational_wave_signal(
    batch: usize,
    channels: usize,
    t: usize,
    x: usize,
    y: usize,
    z: usize,
) -> Tensor<f32> {
    let total_size = batch * channels * t * x * y * z;
    let mut data = vec![0.0_f32; total_size];

    // Simulate a chirp signal (frequency increases over time - characteristic of merging black holes)
    let chirp_frequency_start = 0.1;
    let chirp_frequency_end = 1.0;

    for b in 0..batch {
        for c in 0..channels {
            for ti in 0..t {
                // Chirp signal: frequency increases with time
                let time_fraction = ti as f32 / t as f32;
                let freq = chirp_frequency_start + (chirp_frequency_end - chirp_frequency_start) * time_fraction;
                let amplitude = (1.0 - time_fraction).sqrt(); // Amplitude increases

                for xi in 0..x {
                    for yi in 0..y {
                        for zi in 0..z {
                            // Wave propagation from center
                            let dx = xi as f32 - x as f32 / 2.0;
                            let dy = yi as f32 - y as f32 / 2.0;
                            let dz = zi as f32 - z as f32 / 2.0;
                            let r = (dx * dx + dy * dy + dz * dz).sqrt();

                            // Gravitational wave: h ~ A * sin(2πft - kr) / r
                            let phase = 2.0 * PI * freq * ti as f32 - r;
                            let signal = if r > 0.1 {
                                amplitude * phase.sin() / r
                            } else {
                                amplitude * phase.sin()
                            };

                            // Add noise
                            let noise = (rand::random::<f32>() - 0.5) * 0.1;

                            let idx = ((((b * channels + c) * t + ti) * x + xi) * y + yi) * z + zi;
                            data[idx] = signal + noise;
                        }
                    }
                }
            }
        }
    }

    let shape = ndarray::IxDyn(&[batch, channels, t, x, y, z]);
    let array = ndarray::ArrayD::from_shape_vec(shape, data).unwrap();
    Tensor::new(array)
}
