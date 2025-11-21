//! Example demonstrating scientific types (Quaternions, etc.)

use avila_dataframe::core::{GeodesicCoord, Quaternion, SpinorWeyl};
use avila_dataframe::prelude::*;
use num_complex::Complex;

fn main() -> Result<()> {
    println!("🌌 AvilaDB DataFrame - Advanced Scientific Types\n");
    println!("=== DIFFERENTIAL FEATURES (Nobody else has this!) ===\n");

    // 1. Quaternions for 3D rotations
    println!("1️⃣  Quaternions (for spacecraft orientation, 3D rotations):");
    let q = Quaternion::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::PI / 4.0);
    let q_norm = q.normalize();
    println!(
        "   Quaternion (45° rotation around Z): w={:.3}, x={:.3}, y={:.3}, z={:.3}",
        q_norm.w, q_norm.x, q_norm.y, q_norm.z
    );
    let rot_matrix = q_norm.to_rotation_matrix();
    println!("   Rotation matrix:");
    for row in &rot_matrix {
        println!("     [{:.3}, {:.3}, {:.3}]", row[0], row[1], row[2]);
    }

    // 2. Weyl Spinors for particle physics
    println!("\n2️⃣  Weyl Spinors (for particle physics, neutrinos):");
    let spinor = SpinorWeyl::new(Complex::new(1.0, 0.0), Complex::new(0.0, 1.0));
    println!("   Initial spinor: a={}, b={}", spinor.a, spinor.b);
    let boosted = spinor.boost(0.5); // 50% speed of light
    println!(
        "   After Lorentz boost (β=0.5): a={}, b={}",
        boosted.a, boosted.b
    );

    // 3. Geodesic coordinates for curved space-time
    println!("\n3️⃣  Geodesic Coordinates (for General Relativity):");
    let coord = GeodesicCoord::new(0.0, 10.0, std::f64::consts::PI / 2.0, 0.0);
    println!(
        "   Schwarzschild coordinates: t={:.1}, r={:.1}, θ={:.3}, φ={:.3}",
        coord.t, coord.r, coord.theta, coord.phi
    );

    let gtt = coord.schwarzschild_gtt(1.0); // Black hole mass = 1
    println!(
        "   Metric component g_tt = {:.4} (should be < 0 for timelike)",
        gtt
    );

    let cartesian = coord.to_cartesian();
    println!(
        "   Cartesian approximation: [{:.2}, {:.2}, {:.2}, {:.2}]",
        cartesian[0], cartesian[1], cartesian[2], cartesian[3]
    );

    println!("\n✅ Scientific types demonstration complete!");
    println!("🔥 These types will be natively integrated into DataFrames!");
    println!("💡 Use case: LISA mission gravitational wave analysis");

    Ok(())
}
