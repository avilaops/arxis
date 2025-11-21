use avila_dataframe::prelude::*;
use avila_dataframe::scientific::astronomy::{
    absolute_magnitude, angular_separation, luminosity_distance,
};

fn main() -> Result<()> {
    println!("=== AvilaDF Astronomy Example ===\n");

    // Observed quasar data
    let quasar_names: Vec<String> = vec!["3C 273", "PKS 2155-304", "Mrk 421", "3C 279"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let redshifts = vec![0.158, 0.116, 0.031, 0.536];
    let apparent_mags = vec![12.9, 13.0, 13.5, 15.3];

    println!("Processing {} quasars...\n", quasar_names.len());

    // Calculate cosmological distances
    let mut lum_distances = Vec::new();
    let mut angular_sizes = Vec::new();
    let mut abs_mags = Vec::new();

    for (i, &z) in redshifts.iter().enumerate() {
        let d_l = luminosity_distance(z)?;
        lum_distances.push(d_l);

        let m_abs = absolute_magnitude(apparent_mags[i], d_l)?;
        abs_mags.push(m_abs);

        // Angular size (assuming 1 kpc physical size)
        let d_a = d_l / (1.0_f64 + z).powi(2); // Angular diameter distance
        let angular_size_rad = 1e-3 / d_a; // 1 kpc = 1e-3 Mpc
        let angular_size_arcsec = angular_size_rad * 206265.0; // rad to arcsec
        angular_sizes.push(angular_size_arcsec);

        println!(
            "{}: z={:.3}, D_L={:.1} Mpc, M={:.2}",
            quasar_names[i], z, d_l, m_abs
        );
    }

    println!();

    // Calculate angular separations between quasars
    // (Using example sky coordinates in degrees)
    let ra_deg = vec![187.28, 329.72, 166.11, 194.05];
    let dec_deg = vec![2.05, -30.38, 38.21, -5.79];

    println!("Angular separations:");
    for i in 0..quasar_names.len() {
        for j in (i + 1)..quasar_names.len() {
            let sep = angular_separation(ra_deg[i], dec_deg[i], ra_deg[j], dec_deg[j])?;

            println!("  {} <-> {}: {:.2}°", quasar_names[i], quasar_names[j], sep);
        }
    }

    // Create comprehensive DataFrame (using indices instead of names for now)
    let indices: Vec<f64> = (0..quasar_names.len()).map(|i| i as f64).collect();

    let df = DataFrame::new(vec![
        Series::new("id", indices),
        Series::new("redshift", redshifts),
        Series::new("apparent_mag", apparent_mags),
        Series::new("absolute_mag", abs_mags),
        Series::new("luminosity_distance_mpc", lum_distances),
        Series::new("angular_size_arcsec", angular_sizes),
        Series::new("ra_deg", ra_deg),
        Series::new("dec_deg", dec_deg),
    ])?;

    println!("\nQuasar Catalog:");
    println!("{}", df);

    // Calculate statistics
    let stats = df.describe()?;
    println!("\nStatistics:");
    println!("{}", stats);

    // Filter high-redshift objects (note: filter not yet implemented)
    println!("\nHigh-redshift quasars (z > 0.2):");
    println!("(Filtering will be implemented in next phase)");

    println!("\n✅ Astronomy analysis complete!");
    println!("🔭 AvilaDF: Built for astrophysics research");
    println!("📡 Using Hubble constant H₀ = 70 km/s/Mpc");

    Ok(())
}
