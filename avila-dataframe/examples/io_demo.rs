//! Example demonstrating all I/O operations
//!
//! This example shows how to:
//! - Read/write Parquet files (columnar format)
//! - Read/write CSV files (with streaming)
//! - Work with HDF5 (scientific data)
//! - Connect to AvilaDB (native cloud database)

use avila_dataframe::io::{
    AvilaDbConfig, AvilaDbQuery, CsvReadOptions, CsvWriteOptions, ParquetCompression,
    ParquetWriteOptions,
};
use avila_dataframe::prelude::*;

fn main() -> Result<()> {
    println!("🚀 AvilaDB DataFrame - I/O Operations Demo");
    println!("==========================================\n");

    // Create sample dataset: Gravitational wave events
    let events_df = DataFrame::new(vec![
        Series::new("event_id", vec![1.0, 2.0, 3.0, 4.0, 5.0]),
        Series::new("mass1", vec![30.0, 35.0, 40.0, 45.0, 50.0]),
        Series::new("mass2", vec![25.0, 30.0, 35.0, 40.0, 45.0]),
        Series::new("snr", vec![15.3, 18.5, 12.7, 20.1, 16.8]),
        Series::new("distance_mpc", vec![440.0, 540.0, 620.0, 380.0, 510.0]),
    ])?;

    println!("📊 Original Dataset:");
    println!("{}", events_df);

    // ========== 1. PARQUET I/O ==========
    println!("\n{}", "=".repeat(60));
    println!("1️⃣  PARQUET - High-Performance Columnar Storage");
    println!("{}", "=".repeat(60));

    // Write Parquet
    let parquet_path = "temp_events.parquet";
    println!("\n✍️  Writing to Parquet with Zstd compression...");

    let parquet_options = ParquetWriteOptions {
        compression: ParquetCompression::Zstd,
        row_group_size: Some(1000),
        statistics: true,
    };

    events_df.write_parquet_with_options(parquet_path, parquet_options)?;
    println!("✅ Written to: {}", parquet_path);

    // Get metadata
    let metadata = DataFrame::parquet_metadata(parquet_path)?;
    println!("\n📋 Parquet Metadata:");
    println!("   Rows: {}", metadata.num_rows);
    println!("   Columns: {}", metadata.num_columns);
    println!("   Row Groups: {}", metadata.num_row_groups);
    if let Some(created_by) = metadata.created_by {
        println!("   Created by: {}", created_by);
    }

    // Read back
    println!("\n📖 Reading from Parquet...");
    let events_from_parquet = DataFrame::read_parquet(parquet_path)?;
    println!(
        "✅ Loaded {} rows, {} columns",
        events_from_parquet.height(),
        events_from_parquet.width()
    );

    // Read specific columns only (column pruning)
    println!("\n📖 Reading specific columns only...");
    let subset = DataFrame::read_parquet_columns(parquet_path, &["event_id", "snr"])?;
    println!("✅ Loaded columns: {}", subset.column_names().join(", "));

    // ========== 2. CSV I/O ==========
    println!("\n{}", "=".repeat(60));
    println!("2️⃣  CSV - Universal Text Format");
    println!("{}", "=".repeat(60));

    // Write CSV
    let csv_path = "temp_events.csv";
    println!("\n✍️  Writing to CSV...");
    events_df.write_csv(csv_path)?;
    println!("✅ Written to: {}", csv_path);

    // Write with custom delimiter
    let tsv_path = "temp_events.tsv";
    let csv_options = CsvWriteOptions {
        delimiter: b'\t', // Tab-separated
        header: true,
        ..Default::default()
    };
    events_df.write_csv_with_options(tsv_path, csv_options)?;
    println!("✅ Written TSV to: {}", tsv_path);

    // Read CSV
    println!("\n📖 Reading from CSV...");
    let events_from_csv = DataFrame::read_csv(csv_path)?;
    println!(
        "✅ Loaded {} rows, {} columns",
        events_from_csv.height(),
        events_from_csv.width()
    );

    // ========== 3. CSV STREAMING ==========
    println!("\n{}", "=".repeat(60));
    println!("3️⃣  CSV STREAMING - For Large Files");
    println!("{}", "=".repeat(60));

    // Create larger dataset for streaming demo
    println!("\n📝 Creating large CSV file for streaming...");
    let mut large_data = Vec::new();
    for i in 0..1000 {
        large_data.push(i as f64);
    }
    let large_df = DataFrame::new(vec![
        Series::new("index", large_data.clone()),
        Series::new("value", large_data.iter().map(|x| x * 2.0).collect()),
        Series::new("squared", large_data.iter().map(|x| x * x).collect()),
    ])?;

    let large_csv_path = "temp_large.csv";
    large_df.write_csv(large_csv_path)?;
    println!("✅ Created file with {} rows", large_df.height());

    // Stream read in chunks
    println!("\n🌊 Streaming CSV in chunks of 100 rows...");
    let mut reader = DataFrame::read_csv_chunked(large_csv_path, 100)?;

    let mut total_rows = 0;
    let mut chunk_count = 0;

    while let Some(chunk) = reader.next_chunk()? {
        chunk_count += 1;
        total_rows += chunk.height();
        if chunk_count <= 3 {
            println!("   Chunk {}: {} rows", chunk_count, chunk.height());
        }
    }

    println!(
        "✅ Processed {} chunks, {} total rows",
        chunk_count, total_rows
    );

    // Process with callback
    println!("\n🔄 Processing chunks with callback (filter)...");
    let mut reader2 = DataFrame::read_csv_chunked(large_csv_path, 100)?;
    let filtered_chunks =
        reader2.process_chunks(|chunk| chunk.filter(col("value").gt(lit(500.0))))?;

    let filtered_total: usize = filtered_chunks.iter().map(|df| df.height()).sum();
    println!("✅ Filtered result: {} rows (value > 500)", filtered_total);

    // ========== 4. HDF5 (if enabled) ==========
    println!("\n{}", "=".repeat(60));
    println!("4️⃣  HDF5 - Hierarchical Scientific Data");
    println!("{}", "=".repeat(60));

    #[cfg(feature = "io-hdf5")]
    {
        let hdf5_path = "temp_events.h5";
        println!("\n✍️  Writing to HDF5...");
        events_df.write_hdf5(hdf5_path, "gw_events")?;
        println!("✅ Written to: {} (dataset: gw_events)", hdf5_path);

        println!("\n📖 Reading from HDF5...");
        let events_from_hdf5 = DataFrame::read_hdf5(hdf5_path, "gw_events")?;
        println!(
            "✅ Loaded {} rows, {} columns",
            events_from_hdf5.height(),
            events_from_hdf5.width()
        );

        println!("\n📋 Listing HDF5 datasets...");
        let datasets = DataFrame::list_hdf5_datasets(hdf5_path)?;
        println!("   Datasets: {}", datasets.join(", "));

        // Cleanup
        let _ = std::fs::remove_file(hdf5_path);
    }

    #[cfg(not(feature = "io-hdf5"))]
    {
        println!("\n⚠️  HDF5 support not enabled.");
        println!("   Enable with: cargo run --example io_demo --features io-hdf5");
    }

    // ========== 5. AVILADB INTEGRATION ==========
    println!("\n{}", "=".repeat(60));
    println!("5️⃣  AVILADB - Native Cloud Database Integration");
    println!("{}", "=".repeat(60));

    // Configure AvilaDB connection
    let aviladb_config = AvilaDbConfig::new("my-account", "astrophysics", "gw_events")
        .with_endpoint("https://avila.cloud")
        .with_auth_key("your-auth-key-here");

    println!("\n🔌 AvilaDB Configuration:");
    println!("   Account: {}", aviladb_config.account);
    println!("   Database: {}", aviladb_config.database);
    println!("   Collection: {}", aviladb_config.collection);
    println!("   Connection: {}", aviladb_config.connection_string());

    // Write to AvilaDB
    println!("\n✍️  Writing to AvilaDB...");
    println!("   (Simulated - HTTP client pending)");
    let write_result = events_df.write_aviladb(&aviladb_config);
    match write_result {
        Ok(_) => println!("✅ Would write {} documents", events_df.height()),
        Err(e) => println!("ℹ️  {}", e),
    }

    // Query AvilaDB
    println!("\n🔍 Querying AvilaDB with SQL...");
    let query = AvilaDbQuery::new("SELECT * FROM gw_events WHERE snr > @min_snr")
        .param("min_snr", 15.0)
        .limit(100);

    println!("   Query: {}", query.query);
    println!("   Parameters: {} defined", query.parameters.len());

    let query_result = DataFrame::read_aviladb(&aviladb_config, &query);
    match query_result {
        Err(e) if e.to_string().contains("pending") => {
            println!("ℹ️  Query prepared (HTTP client pending implementation)");
        }
        Err(e) => println!("❌ Error: {}", e),
        Ok(df) => println!("✅ Retrieved {} rows", df.height()),
    }

    // Batch writer
    println!("\n📦 Batch Writer (for bulk inserts):");
    use avila_dataframe::io::AvilaDbBatchWriter;
    let mut batch_writer = AvilaDbBatchWriter::new(aviladb_config.clone(), 1000);
    batch_writer.write(&events_df)?;
    println!("   Added {} documents to batch", events_df.height());
    batch_writer.flush()?;
    println!("✅ Batch flushed");

    // ========== 6. FORMAT COMPARISON ==========
    println!("\n{}", "=".repeat(60));
    println!("6️⃣  FORMAT COMPARISON");
    println!("{}", "=".repeat(60));

    // Get file sizes
    let parquet_size = std::fs::metadata(parquet_path)?.len();
    let csv_size = std::fs::metadata(csv_path)?.len();

    println!("\n📊 File Sizes:");
    println!("   Parquet (compressed): {} bytes", parquet_size);
    println!("   CSV (text): {} bytes", csv_size);
    println!(
        "   Compression ratio: {:.1}x",
        csv_size as f64 / parquet_size as f64
    );

    println!("\n⚡ Performance Characteristics:");
    println!();
    println!("   PARQUET:");
    println!("   ✅ Fast columnar reads");
    println!("   ✅ Excellent compression (40-60% smaller)");
    println!("   ✅ Supports column pruning");
    println!("   ✅ Built-in statistics");
    println!("   ✅ Industry standard for big data");
    println!();
    println!("   CSV:");
    println!("   ✅ Human-readable");
    println!("   ✅ Universal compatibility");
    println!("   ✅ Streaming support");
    println!("   ❌ Slower for large files");
    println!("   ❌ No compression by default");
    println!();
    println!("   HDF5:");
    println!("   ✅ Hierarchical organization");
    println!("   ✅ Standard in scientific computing");
    println!("   ✅ Supports complex data structures");
    println!("   ✅ Built-in metadata");
    println!();
    println!("   AvilaDB:");
    println!("   ✅ Cloud-native (no files!)");
    println!("   ✅ Real-time queries with SQL");
    println!("   ✅ Automatic scaling");
    println!("   ✅ 40-60% cheaper than AWS/Azure");
    println!("   ✅ Built in Brazil 🇧🇷");

    // ========== CLEANUP ==========
    println!("\n{}", "=".repeat(60));
    println!("🧹 Cleaning up temporary files...");
    println!("{}", "=".repeat(60));

    let files_to_remove = vec![parquet_path, csv_path, tsv_path, large_csv_path];
    for file in files_to_remove {
        if let Err(e) = std::fs::remove_file(file) {
            println!("   ⚠️  Could not remove {}: {}", file, e);
        } else {
            println!("   ✅ Removed {}", file);
        }
    }

    // ========== SUMMARY ==========
    println!("\n{}", "=".repeat(60));
    println!("✅ I/O OPERATIONS COMPLETE!");
    println!("{}", "=".repeat(60));
    println!();
    println!("📚 What you learned:");
    println!("   ✅ Parquet: High-performance columnar format");
    println!("   ✅ CSV: Universal text format with streaming");
    println!("   ✅ HDF5: Scientific hierarchical data");
    println!("   ✅ AvilaDB: Cloud-native database integration");
    println!();
    println!("💡 Real-world applications:");
    println!("   • Parquet: Data lakes, analytics pipelines");
    println!("   • CSV: Data exchange, spreadsheets");
    println!("   • HDF5: LIGO/LISA, climate models, genomics");
    println!("   • AvilaDB: Real-time apps, microservices");
    println!();
    println!("🚀 Next steps:");
    println!("   • Try with your own datasets");
    println!("   • Benchmark different formats");
    println!("   • Integrate with AvilaDB cloud");
    println!("   • Build data pipelines");
    println!();
    println!("🔥 Destruindo concorrência com I/O nativo! 🇧🇷");

    Ok(())
}
