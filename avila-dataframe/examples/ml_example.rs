use avila_dataframe::ai::{CorrelationMethod, VectorEncoder};
use avila_dataframe::prelude::*;

fn main() -> Result<()> {
    println!("=== AvilaDF AI/ML Example ===\n");

    // Create sample dataset: exoplanet detection
    let star_names = vec![
        "Kepler-442",
        "Kepler-452",
        "Kepler-186",
        "TRAPPIST-1",
        "Proxima Centauri",
        "LHS 1140",
        "K2-18",
        "TOI-700",
    ];

    let masses = vec![0.61, 1.04, 0.54, 0.09, 0.12, 0.15, 0.45, 0.42];
    let radii = vec![0.60, 1.11, 0.53, 0.12, 0.14, 0.19, 0.41, 0.40];
    let temperatures = vec![
        4402.0, 5757.0, 3788.0, 2559.0, 3042.0, 3131.0, 3457.0, 3480.0,
    ];
    let distances = vec![1206.0, 1402.0, 582.0, 39.0, 4.2, 40.7, 124.0, 101.0];

    // Planet properties
    let planet_radii = vec![1.34, 1.63, 1.17, 1.13, 1.07, 1.43, 2.61, 1.19];
    let orbital_periods = vec![112.3, 384.8, 129.9, 12.4, 11.2, 24.7, 32.9, 37.4];
    let habitable = vec![true, true, true, true, true, true, true, true];

    println!(
        "Creating exoplanet dataset with {} stars...\n",
        star_names.len()
    );

    let mut df = DataFrame::new(vec![
        Series::new("star_name", star_names),
        Series::new("star_mass_solar", masses),
        Series::new("star_radius_solar", radii),
        Series::new("star_temp_k", temperatures),
        Series::new("distance_ly", distances),
        Series::new("planet_radius_earth", planet_radii),
        Series::new("orbital_period_days", orbital_periods),
        Series::new(
            "potentially_habitable",
            habitable
                .iter()
                .map(|&b| if b { 1.0 } else { 0.0 })
                .collect(),
        ),
    ])?;

    println!("Original Dataset:");
    println!("{}\n", df);

    // Feature Engineering
    println!("🔧 Feature Engineering...\n");

    // 1. Calculate stellar luminosity (L ∝ R² T⁴)
    let luminosities: Vec<f64> = (0..df.len())
        .map(|i| {
            let r = df.column("star_radius_solar").unwrap().get_f64(i).unwrap();
            let t = df.column("star_temp_k").unwrap().get_f64(i).unwrap();
            r * r * (t / 5778.0).powi(4)
        })
        .collect();

    df = df.with_column(Series::new("stellar_luminosity", luminosities))?;

    // 2. Calculate equilibrium temperature
    let eq_temps: Vec<f64> = (0..df.len())
        .map(|i| {
            let l = df.column("stellar_luminosity").unwrap().get_f64(i).unwrap();
            let p = df
                .column("orbital_period_days")
                .unwrap()
                .get_f64(i)
                .unwrap();
            let a = (p / 365.25).powf(2.0 / 3.0); // Orbital distance in AU
            278.0 * l.sqrt() / a.sqrt() // Simplified equilibrium temp
        })
        .collect();

    df = df.with_column(Series::new("eq_temp_k", eq_temps))?;

    // 3. Earth Similarity Index (simplified)
    let esi_scores: Vec<f64> = (0..df.len())
        .map(|i| {
            let pr = df
                .column("planet_radius_earth")
                .unwrap()
                .get_f64(i)
                .unwrap();
            let et = df.column("eq_temp_k").unwrap().get_f64(i).unwrap();

            let r_term = 1.0 - ((pr - 1.0) / (pr + 1.0)).abs();
            let t_term = 1.0 - ((et - 288.0) / (et + 288.0)).abs();

            r_term * t_term
        })
        .collect();

    df = df.with_column(Series::new("esi_score", esi_scores))?;

    println!("Enhanced Dataset with Features:");
    println!("{}\n", df);

    // Standardization
    println!("📊 Standardizing numerical features...\n");

    df = df.standardize(&[
        "star_mass_solar",
        "star_temp_k",
        "stellar_luminosity",
        "planet_radius_earth",
        "eq_temp_k",
    ])?;

    // Train/Test Split
    println!("🎯 Splitting into train/test sets...\n");
    let (train, test, _validate) = df.train_test_validate_split(0.6, 0.2, None)?;

    println!("Training set: {} rows", train.len());
    println!("Test set: {} rows", test.len());

    // Statistics
    let stats = df.describe()?;
    println!("\n📈 Dataset Statistics:");
    println!("{}\n", stats);

    // Rank by ESI score
    println!("🏆 Top candidates by Earth Similarity Index:\n");
    println!("(Note: Sorting not yet implemented, showing top 3 manually)");

    let mut esi_with_names: Vec<(f64, String)> = (0..df.len())
        .map(|i| {
            let esi = df.column("esi_score").unwrap().get_f64(i).unwrap();
            let name = df.column("star_name").unwrap().get_string(i).unwrap();
            (esi, name)
        })
        .collect();

    esi_with_names.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    for (rank, (esi, name)) in esi_with_names.iter().take(3).enumerate() {
        println!("{}. {} - ESI: {:.3}", rank + 1, name, esi);
    }

    println!("\n✅ Machine Learning pipeline complete!");
    println!("🤖 AvilaDF: Built for data science");
    println!("🌍 Ready for AvilaDB vector storage and RAG queries");

    Ok(())
}
