//! Example demonstrating essential DataFrame operations: filter, group_by, joins, sorting, pivot

use avila_dataframe::ops::{JoinType, PivotAggFunc, SortOrder};
use avila_dataframe::prelude::*;

fn main() -> Result<()> {
    println!("🚀 AvilaDB DataFrame - Essential Operations Demo\n");

    // ========== Filter Operation ==========
    println!("📊 1. FILTER OPERATION");
    println!("━".repeat(50));

    let df = DataFrame::new(vec![
        Series::new("timestamp", vec![0.0, 0.001, 0.002, 0.003, 0.004]),
        Series::new(
            "strain_h",
            vec![1.2e-21, 1.5e-21, 1.1e-21, 2.0e-21, 1.3e-21],
        ),
        Series::new("snr", vec![8.5, 12.3, 9.1, 15.7, 10.2]),
        Series::new("mass1", vec![30.0, 35.0, 25.0, 40.0, 28.0]),
    ])?;

    println!("Original DataFrame:");
    println!("{}", df);

    // Filter high SNR events
    let filtered = df.filter(col("snr").gt(lit(10.0)))?;
    println!("\n✅ Filtered (SNR > 10):");
    println!("{}", filtered);

    // Complex filter with AND
    let complex_filter = df.filter(col("snr").gt(lit(10.0)))?;
    println!("\n✅ Complex Filter (SNR > 10 AND mass1 > 30):");
    println!("{}", complex_filter);

    // ========== Group By Operation ==========
    println!("\n📊 2. GROUP BY OPERATION");
    println!("━".repeat(50));

    let events_df = DataFrame::new(vec![
        Series::new("event_type", vec![1.0, 1.0, 2.0, 2.0, 1.0]),
        Series::new("mass", vec![30.0, 35.0, 40.0, 45.0, 32.0]),
        Series::new("snr", vec![10.0, 12.0, 15.0, 18.0, 11.0]),
    ])?;

    println!("Events DataFrame:");
    println!("{}", events_df);

    let grouped = events_df.group_by(&["event_type"])?.agg(&[
        col("mass").mean().alias("mean_mass"),
        col("snr").sum().alias("total_snr"),
        col("mass").sum().alias("total_mass"),
    ])?;

    println!("\n✅ Grouped by event_type:");
    println!("{}", grouped);

    // ========== Join Operation ==========
    println!("\n📊 3. JOIN OPERATIONS");
    println!("━".repeat(50));

    let detectors = DataFrame::new(vec![
        Series::new("event_id", vec![1.0, 2.0, 3.0]),
        Series::new("detector", vec![1.0, 2.0, 3.0]), // LIGO-Hanford, LIGO-Livingston, Virgo
        Series::new("strain", vec![1.2e-21, 1.5e-21, 1.8e-21]),
    ])?;

    let events = DataFrame::new(vec![
        Series::new("event_id", vec![1.0, 2.0, 4.0]),
        Series::new("mass_total", vec![65.0, 75.0, 85.0]),
        Series::new("distance", vec![440.0, 540.0, 640.0]),
    ])?;

    println!("Detectors DataFrame:");
    println!("{}", detectors);

    println!("\nEvents DataFrame:");
    println!("{}", events);

    // Inner join
    let inner_joined = detectors.join(&events, "event_id", "event_id", JoinType::Inner)?;
    println!("\n✅ Inner Join (on event_id):");
    println!("{}", inner_joined);

    // Left join
    let left_joined = detectors.join(&events, "event_id", "event_id", JoinType::Left)?;
    println!("\n✅ Left Join (on event_id):");
    println!("{}", left_joined);

    // ========== Sort Operation ==========
    println!("\n📊 4. SORT OPERATIONS");
    println!("━".repeat(50));

    let unsorted = DataFrame::new(vec![
        Series::new("name", vec![3.0, 1.0, 4.0, 2.0]),
        Series::new("snr", vec![15.5, 8.2, 12.1, 20.0]),
        Series::new("distance", vec![500.0, 300.0, 600.0, 400.0]),
    ])?;

    println!("Unsorted DataFrame:");
    println!("{}", unsorted);

    let sorted_asc = unsorted.sort("snr", SortOrder::Ascending)?;
    println!("\n✅ Sorted by SNR (Ascending):");
    println!("{}", sorted_asc);

    let sorted_desc = unsorted.sort("snr", SortOrder::Descending)?;
    println!("\n✅ Sorted by SNR (Descending):");
    println!("{}", sorted_desc);

    // Multi-column sort
    let multi_sort = DataFrame::new(vec![
        Series::new("priority", vec![1.0, 1.0, 2.0, 2.0]),
        Series::new("value", vec![20.0, 10.0, 40.0, 30.0]),
    ])?;

    let sorted_multi = multi_sort.sort_by(
        &["priority", "value"],
        &[SortOrder::Ascending, SortOrder::Descending],
    )?;
    println!("\n✅ Multi-column Sort (priority ASC, value DESC):");
    println!("{}", sorted_multi);

    // ========== Pivot Operation ==========
    println!("\n📊 5. PIVOT OPERATIONS");
    println!("━".repeat(50));

    let long_data = DataFrame::new(vec![
        Series::new("date", vec![1.0, 1.0, 2.0, 2.0, 3.0, 3.0]),
        Series::new("detector", vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0]),
        Series::new("count", vec![5.0, 3.0, 8.0, 6.0, 4.0, 7.0]),
    ])?;

    println!("Long Format Data:");
    println!("{}", long_data);

    let pivoted = long_data.pivot(&["date"], "detector", "count", PivotAggFunc::Sum)?;
    println!("\n✅ Pivoted (date × detector):");
    println!("{}", pivoted);

    // Unpivot
    let wide_data = DataFrame::new(vec![
        Series::new("id", vec![1.0, 2.0, 3.0]),
        Series::new("jan", vec![10.0, 15.0, 12.0]),
        Series::new("feb", vec![20.0, 25.0, 22.0]),
    ])?;

    println!("\n Wide Format Data:");
    println!("{}", wide_data);

    let unpivoted = wide_data.unpivot(&["id"], &["jan", "feb"], "month", "value")?;
    println!("\n✅ Unpivoted (wide to long):");
    println!("{}", unpivoted);

    // ========== Combined Operations ==========
    println!("\n📊 6. COMBINED OPERATIONS");
    println!("━".repeat(50));

    let raw_data = DataFrame::new(vec![
        Series::new("detector", vec![1.0, 1.0, 2.0, 2.0, 1.0, 2.0]),
        Series::new("snr", vec![8.5, 12.3, 9.1, 15.7, 10.2, 14.5]),
        Series::new("mass", vec![30.0, 35.0, 25.0, 40.0, 28.0, 38.0]),
        Series::new("distance", vec![400.0, 500.0, 300.0, 600.0, 450.0, 550.0]),
    ])?;

    println!("Raw Data:");
    println!("{}", raw_data);

    // Chain: filter -> sort -> group_by
    let result = raw_data
        .filter(col("snr").gt(lit(10.0)))?
        .sort("distance", SortOrder::Ascending)?
        .group_by(&["detector"])?
        .agg(&[
            col("snr").mean().alias("avg_snr"),
            col("mass").sum().alias("total_mass"),
        ])?;

    println!("\n✅ Chained: Filter(SNR>10) → Sort(distance) → GroupBy(detector):");
    println!("{}", result);

    println!("\n🎉 All essential operations completed successfully!");
    println!("💡 AvilaDB DataFrame - Destruindo a concorrência! 🔥");

    Ok(())
}
