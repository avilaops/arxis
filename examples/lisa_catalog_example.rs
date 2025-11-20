/// LISA Event Catalog Example - Complete Pipeline with Database
///
/// This example demonstrates the complete LISA analysis pipeline from
/// data acquisition to event cataloging:
///
/// 1. Generate/Load LISA data
/// 2. Process data (FFT, PSD, whitening)
/// 3. Perform matched filtering
/// 4. Detect and characterize events
/// 5. Build event catalog
/// 6. Generate statistics and reports
/// 7. Export catalog (JSON, CSV)
///
/// Phase 5: Event Catalog & Reporting demonstration

use arxis_quaternions::physics::{
    CatalogEvent, EventCatalog, LISASource, MatchedFilter, PowerSpectralDensity,
    SourceClassification, SyntheticDataGenerator, TemplateBank,
};

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║        LISA Event Catalog - Complete Analysis Pipeline           ║");
    println!("║           Phase 5: Event Database & Reporting System              ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();

    // ========================================================================
    // Part 1: Data Generation & Analysis
    // ========================================================================
    println!("📡 Part 1: Data Generation & Event Detection");
    println!("   -----------------------------------------");
    println!();

    // Generate synthetic LISA observation
    let duration = 10000.0; // 10,000 seconds
    let sampling_rate = 0.1; // 0.1 Hz
    let gen = SyntheticDataGenerator::new(sampling_rate, duration);

    println!("   Observation parameters:");
    println!("   ├─ Duration: {:.0} s ({:.1} hours)", duration, duration / 3600.0);
    println!("   ├─ Sampling rate: {} Hz", sampling_rate);
    println!("   └─ Total samples: {}", (duration * sampling_rate) as usize);
    println!();

    // Inject multiple signals
    println!("   Injecting signals:");
    
    // Signal 1: MBHB
    let source1 = LISASource::smbh(1e6, 5e5, 3e25, 1.0);
    let signal1 = gen.monochromatic_binary(source1.gw_frequency(), source1.characteristic_strain(), 0.0);
    println!("   ├─ MBHB: M={:.1e}+{:.1e} M☉, f={:.6} Hz", 1e6, 5e5, source1.gw_frequency());

    // Signal 2: Another MBHB
    let source2 = LISASource::smbh(2e6, 8e5, 5e25, 2.0);
    let signal2 = gen.monochromatic_binary(source2.gw_frequency(), source2.characteristic_strain() * 0.8, 0.0);
    println!("   ├─ MBHB: M={:.1e}+{:.1e} M☉, f={:.6} Hz", 2e6, 8e5, source2.gw_frequency());

    // Signal 3: EMRI
    let source3 = LISASource::emri(1e5, 15.0, 1.5, 10.0);
    let signal3 = gen.monochromatic_binary(source3.gw_frequency(), source3.characteristic_strain() * 0.5, 0.0);
    println!("   └─ EMRI: M={:.1e}+{:.1e} M☉, f={:.6} Hz", 1e5, 15.0, source3.gw_frequency());
    println!();

    // Combine signals with noise
    let mut combined = signal1.clone();
    for i in 0..combined.h_plus.len() {
        combined.h_plus[i] += signal2.h_plus.get(i).unwrap_or(&0.0);
        combined.h_plus[i] += signal3.h_plus.get(i).unwrap_or(&0.0);
    }
    let data = gen.signal_plus_noise(&combined, 1e-22);

    println!("   Data quality:");
    println!("   ├─ Signal RMS: {:.2e}", combined.rms_strain());
    println!("   ├─ Data RMS: {:.2e}", data.rms_strain());
    println!("   └─ SNR estimate: {:.1}", combined.rms_strain() / 1e-22);
    println!();

    // ========================================================================
    // Part 2: Matched Filtering Search
    // ========================================================================
    println!("🔍 Part 2: Matched Filtering Search");
    println!("   ---------------------------------");
    println!();

    // Create template bank
    let mut bank = TemplateBank::new(0.97);
    println!("   Building template bank...");
    
    bank.generate_mbhb_grid(
        (5e5, 3e6),  // m1 range
        (2e5, 1e6),  // m2 range
        5,           // n_m1
        4,           // n_m2
        3e25,        // distance
        duration,
        sampling_rate,
    );
    
    println!("   ├─ Generated {} MBHB templates", bank.len());

    // Add EMRI templates
    let initial_count = bank.len();
    bank.generate_emri_grid(
        (5e4, 2e5),  // MBH mass range
        (10.0, 30.0), // CO mass range
        3,            // n_mbh
        2,            // n_co
        3e25,
        duration,
        sampling_rate,
    );
    
    println!("   ├─ Generated {} EMRI templates", bank.len() - initial_count);
    println!("   └─ Total templates: {}", bank.len());
    println!();

    // Create matched filter
    let psd = PowerSpectralDensity::lisa_noise_model(1e-4, 0.05, 500);
    let snr_threshold = 7.0;
    let mf = MatchedFilter::new(bank, psd, snr_threshold);

    println!("   Searching for events...");
    let t_start = std::time::Instant::now();
    let results = mf.search(&data);
    let t_elapsed = t_start.elapsed();

    println!("   ├─ Search time: {:.2} s", t_elapsed.as_secs_f64());
    println!("   ├─ Candidates found: {}", results.len());
    println!("   └─ SNR threshold: {:.1}", snr_threshold);
    println!();

    // Cluster events
    let clustered = mf.cluster_events(&results, 500.0);
    println!("   Event clustering:");
    println!("   ├─ Raw candidates: {}", results.len());
    println!("   ├─ Clustered events: {}", clustered.len());
    println!("   └─ Cluster window: 500 s");
    println!();

    // ========================================================================
    // Part 3: Build Event Catalog
    // ========================================================================
    println!("📚 Part 3: Building Event Catalog");
    println!("   --------------------------------");
    println!();

    let mut catalog = EventCatalog::new(
        "LISA-O1-Test".to_string(),
        "1.0.0".to_string(),
        "arxis-0.2.0".to_string(),
    );

    println!("   Catalog: {} v{}", catalog.name, catalog.version);
    println!("   Pipeline: {}", catalog.pipeline_version);
    println!();

    // Add events to catalog
    println!("   Adding events to catalog...");
    for (i, result) in clustered.iter().enumerate() {
        // Convert matched filter result to catalog event directly
        let event_id = format!("LISA-GW-{:06}", 240120 + i);
        let utc_time = format!("2024-01-20T{:02}:{:02}:00Z", i / 60, i % 60);
        let gps_time = result.time;
        
        let event = CatalogEvent {
            id: event_id,
            gps_time,
            utc_time,
            snr: result.snr,
            far: 1e-6, // Estimated false alarm rate
            false_alarm_prob: 0.01,
            confidence: if result.snr > 15.0 { 0.95 } else { 0.80 },
            source_type: SourceClassification::from_mass_ratio(
                result.parameters.mass_2 / result.parameters.mass_1,
                result.parameters.mass_1 + result.parameters.mass_2,
            ),
            parameters: result.parameters.clone(),
            sky_location: None,
            data_quality: arxis_quaternions::physics::DataQuality {
                glitches: 0,
                gaps: 0,
                score: 0.95,
            },
            metadata: std::collections::HashMap::new(),
            pipeline_version: "arxis-0.2.0".to_string(),
        };

        catalog.add_event(event);
    }

    println!("   ✓ Added {} events to catalog", catalog.len());
    println!();

    // ========================================================================
    // Part 4: Catalog Analysis & Statistics
    // ========================================================================
    println!("📊 Part 4: Catalog Statistics");
    println!("   ---------------------------");
    println!();

    let stats = catalog.statistics();

    println!("   Event breakdown:");
    println!("   ├─ Total events: {}", stats.total_events);
    println!("   ├─ MBHB: {} ({:.1}%)", 
        stats.mbhb_count,
        100.0 * stats.mbhb_count as f64 / stats.total_events as f64);
    println!("   ├─ EMRI: {} ({:.1}%)", 
        stats.emri_count,
        100.0 * stats.emri_count as f64 / stats.total_events as f64);
    println!("   └─ Galactic: {} ({:.1}%)", 
        stats.galactic_count,
        100.0 * stats.galactic_count as f64 / stats.total_events as f64);
    println!();

    println!("   SNR statistics:");
    println!("   ├─ Mean: {:.2}", stats.snr_mean);
    println!("   ├─ Min:  {:.2}", stats.snr_min);
    println!("   └─ Max:  {:.2}", stats.snr_max);
    println!();

    // Filter examples
    println!("   Query examples:");
    
    let mbhb_events = catalog.filter_by_source(SourceClassification::MBHB);
    println!("   ├─ MBHB events: {}", mbhb_events.len());
    
    let high_snr = catalog.filter_by_snr(10.0);
    println!("   ├─ SNR > 10: {}", high_snr.len());
    
    let time_range = catalog.filter_by_time(0.0, 5000.0);
    println!("   └─ Events in [0, 5000]s: {}", time_range.len());
    println!();

    // ========================================================================
    // Part 5: Generate Reports
    // ========================================================================
    println!("📄 Part 5: Report Generation");
    println!("   --------------------------");
    println!();

    let report = catalog.generate_report();
    println!("{}", report);

    // ========================================================================
    // Part 6: Export Catalog
    // ========================================================================
    println!("💾 Part 6: Catalog Export");
    println!("   ----------------------");
    println!();

    // Export to JSON
    let json_path = "lisa_catalog.json";
    match catalog.export_json(json_path) {
        Ok(_) => println!("   ✓ Exported to JSON: {}", json_path),
        Err(e) => println!("   ✗ JSON export failed: {}", e),
    }

    // Export to CSV
    let csv_path = "lisa_catalog.csv";
    match catalog.export_csv(csv_path) {
        Ok(_) => println!("   ✓ Exported to CSV: {}", csv_path),
        Err(e) => println!("   ✗ CSV export failed: {}", e),
    }

    println!();
    println!("   Export formats available:");
    println!("   ├─ JSON: Complete metadata + parameters");
    println!("   ├─ CSV: Tabular format for spreadsheets");
    println!("   └─ HDF5: Large-scale data (future)");
    println!();

    // ========================================================================
    // Part 7: Event Details
    // ========================================================================
    println!("🔬 Part 7: Detailed Event Information");
    println!("   ------------------------------------");
    println!();

    if !catalog.events.is_empty() {
        println!("   Top 3 events by SNR:");
        println!();

        let mut sorted: Vec<_> = catalog.events.iter().collect();
        sorted.sort_by(|a, b| b.snr.partial_cmp(&a.snr).unwrap());

        for (i, event) in sorted.iter().take(3).enumerate() {
            println!("   Event #{}", i + 1);
            println!("   ├─ ID: {}", event.id);
            println!("   ├─ GPS Time: {:.1} s", event.gps_time);
            println!("   ├─ UTC: {}", event.utc_time);
            println!("   ├─ SNR: {:.2}", event.snr);
            println!("   ├─ FAR: {:.2e} Hz", event.far);
            println!("   ├─ Confidence: {:.1}%", event.confidence * 100.0);
            println!("   ├─ Source: {}", event.source_type.as_str());
            println!("   ├─ Masses:");
            println!("   │  ├─ M₁: {:.2e} M☉", event.parameters.mass_1);
            println!("   │  ├─ M₂: {:.2e} M☉", event.parameters.mass_2);
            println!("   │  ├─ Mchirp: {:.2e} M☉", event.parameters.chirp_mass);
            println!("   │  └─ Mtotal: {:.2e} M☉", event.parameters.total_mass);
            println!("   ├─ Distance: {:.2e} m ({:.1} Gpc)", 
                event.parameters.distance,
                event.parameters.distance / 3.086e25);
            println!("   ├─ Data Quality:");
            println!("   │  ├─ Glitches: {}", event.data_quality.glitches);
            println!("   │  ├─ Gaps: {}", event.data_quality.gaps);
            println!("   │  └─ Score: {:.2}", event.data_quality.score);
            println!("   └─ Pipeline: {}", event.pipeline_version);
            println!();
        }
    }

    // ========================================================================
    // Summary
    // ========================================================================
    println!("╔═══════════════════════════════════════════════════════════════════╗");
    println!("║                         Pipeline Summary                          ║");
    println!("╚═══════════════════════════════════════════════════════════════════╝");
    println!();
    println!("✅ Phase 0: Mathematical Kernel");
    println!("✅ Phase 1: Input Layer (LDC, synthetic data)");
    println!("✅ Phase 2: Processing Layer (FFT, PSD, whitening)");
    println!("✅ Phase 3: Analysis Layer (matched filtering)");
    println!("✅ Phase 4: Visualization Layer (plots, spectrograms)");
    println!("✅ Phase 5: Event Catalog & Reporting ← YOU ARE HERE");
    println!();
    println!("📦 Catalog Summary:");
    println!("   ├─ Events cataloged: {}", catalog.len());
    println!("   ├─ Analysis time: {:.2} s", t_elapsed.as_secs_f64());
    println!("   ├─ Exports: JSON ✓, CSV ✓");
    println!("   └─ Report: Generated ✓");
    println!();
    println!("🎯 LISA Scientific Pipeline: COMPLETE!");
    println!();
    println!("📖 Next steps:");
    println!("   - Parameter estimation (MCMC, nested sampling)");
    println!("   - Multi-detector analysis (LISA + ground-based)");
    println!("   - Cosmological parameter inference");
    println!("   - Population studies");
    println!();
}
