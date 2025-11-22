//! Example demonstrating basic usage of avila-dataframe

use avila_dataframe::prelude::*;

fn main() -> Result<()> {
    println!("🚀 AvilaDB DataFrame - Revolutionary DataFrame Library\n");

    // Create a DataFrame for LIGO/LISA gravitational wave data
    let df = DataFrame::new(vec![
        Series::new("timestamp", vec![0.0, 0.001, 0.002, 0.003, 0.004]),
        Series::new(
            "strain_h",
            vec![1.2e-21, 1.5e-21, 1.1e-21, 1.8e-21, 1.3e-21],
        ),
        Series::new("snr", vec![8.5, 12.3, 9.1, 15.7, 10.2]),
        Series::new("mass1", vec![30.0, 35.0, 25.0, 40.0, 28.0]),
        Series::new("mass2", vec![25.0, 30.0, 20.0, 35.0, 23.0]),
    ])?;

    println!("📊 Original DataFrame:");
    println!("{}", df);

    // Calculate total mass
    let df = df.with_column(Series::new(
        "total_mass",
        vec![55.0, 65.0, 45.0, 75.0, 51.0],
    ))?;

    println!("\n✨ With calculated total_mass column:");
    println!("{}", df);

    // Select specific columns
    let selected = df.select(&["timestamp", "snr", "total_mass"])?;
    println!("\n🎯 Selected columns:");
    println!("{}", selected);

    // Calculate statistics
    let snr_col = df.column("snr")?;
    println!("\n📈 SNR Statistics:");
    println!("  Mean: {:.2}", snr_col.mean()?);
    println!("  Std:  {:.2}", snr_col.std()?);
    println!("  Sum:  {:.2}", snr_col.sum()?);

    println!("\n✅ Example completed successfully!");
    println!("🔥 This is just the beginning - FFT, wavelets, and GPU acceleration coming next!");

    Ok(())
}
