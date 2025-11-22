//! Integration tests for AvilaDF

use avila_dataframe::prelude::*;

#[test]
fn test_dataframe_creation_and_operations() {
    let df = DataFrame::new(vec![
        Series::new("a", vec![1.0, 2.0, 3.0]),
        Series::new("b", vec![4.0, 5.0, 6.0]),
    ])
    .expect("Failed to create DataFrame");

    assert_eq!(df.height(), 3);
    assert_eq!(df.width(), 2);
}

#[test]
fn test_series_statistics() {
    let series = Series::new("test", vec![1.0, 2.0, 3.0, 4.0, 5.0]);

    assert_eq!(series.len(), 5);
    assert_eq!(series.mean(), 3.0);
    assert_eq!(series.sum(), 15.0);
    assert!(series.std() > 1.4 && series.std() < 1.5);
}

#[test]
fn test_scientific_types() {
    use avila_dataframe::core::dtype::{GeodesicCoord, Quaternion};

    // Test quaternion normalization
    let mut q = Quaternion::from_axis_angle([0.0, 0.0, 1.0], std::f64::consts::PI / 4.0);
    q = q.normalize();

    let mag_sq = q.w * q.w + q.x * q.x + q.y * q.y + q.z * q.z;
    assert!(
        (mag_sq - 1.0).abs() < 1e-10,
        "Quaternion should be normalized"
    );

    // Test geodesic coordinates
    let coord = GeodesicCoord {
        t: 0.0,
        r: 10.0,
        theta: 0.0,
        phi: 0.0,
    };

    let g_tt = coord.schwarzschild_gtt(1.0);
    assert!(g_tt < 0.0, "g_tt should be negative (timelike)");
    assert!(g_tt > -1.0, "g_tt should be > -1 for r > 2M");
}

#[cfg(feature = "scientific")]
#[test]
fn test_astronomy_functions() {
    use avila_dataframe::scientific::astronomy::{
        absolute_magnitude, angular_separation, luminosity_distance,
    };

    // Test luminosity distance (Hubble law approximation)
    let z = 0.1; // redshift
    let d = luminosity_distance(z);
    assert!(
        d > 400.0 && d < 450.0,
        "Distance should be ~427 Mpc for z=0.1"
    );

    // Test angular separation
    let sep = angular_separation(0.0, 0.0, 1.0, 1.0);
    assert!(
        sep > 1.4 && sep < 1.5,
        "Separation should be ~1.414 degrees"
    );

    // Test absolute magnitude
    let m_abs = absolute_magnitude(-26.0, 100.0); // apparent mag, distance in Mpc
    assert!(
        m_abs < -25.0,
        "Absolute magnitude should be brighter for nearby objects"
    );
}

#[test]
fn test_expression_system() {
    use avila_dataframe::ops::expressions::{col, lit, Expr};

    let expr = col("mass1") + col("mass2");

    match expr {
        Expr::BinaryOp { op, .. } => {
            assert_eq!(op, "+");
        }
        _ => panic!("Expected BinaryOp"),
    }
}

#[cfg(feature = "scientific")]
#[test]
fn test_ai_standardization() {
    let df = DataFrame::new(vec![
        Series::new("feature1", vec![1.0, 2.0, 3.0, 4.0, 5.0]),
        Series::new("feature2", vec![10.0, 20.0, 30.0, 40.0, 50.0]),
    ])
    .expect("Failed to create DataFrame");

    let standardized = df
        .standardize(&["feature1", "feature2"])
        .expect("Standardization failed");

    // Check that standardized features have mean ≈ 0 and std ≈ 1
    let feature1 = standardized
        .column("feature1_std")
        .expect("Column not found");
    let mean = feature1.mean();
    let std = feature1.std();

    assert!(mean.abs() < 1e-10, "Standardized mean should be ~0");
    assert!((std - 1.0).abs() < 1e-10, "Standardized std should be ~1");
}
