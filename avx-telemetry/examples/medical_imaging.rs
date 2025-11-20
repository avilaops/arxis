/// Exemplo de aplicação de telemetria em Medical Imaging
///
/// Este exemplo demonstra como usar avx-telemetry para:
/// 1. Monitorar latências de processamento de imagens médicas
/// 2. Detectar anomalias em pipelines de análise de CT/MRI
/// 3. Prever carga de trabalho futura
/// 4. Garantir qualidade de dados NASA-grade
///
/// # Cenários
/// - Cancer Detection (CT Tumor Segmentation)
/// - Bone Density Analysis (DEXA scans)
/// - Pathology Slide Analysis (Whole Slide Imaging)
///
/// # Execução
/// ```bash
/// cargo run --example medical_imaging
/// ```
use avx_telemetry::{AvxContext, AvxMetrics, TimeSeries};
use std::time::Instant;

#[derive(Debug)]
struct MedicalScanMetrics {
    scan_type: String,
    processing_time_ms: f64,
    image_size_mb: f64,
    accuracy: f64,
    patient_id: String,
}

/// Simula processamento de scan médico
fn process_medical_scan(scan_type: &str, image_size_mb: f64) -> MedicalScanMetrics {
    let start = Instant::now();

    // Simular processamento (tempo proporcional ao tamanho da imagem)
    let base_time_ms = 100.0;
    let processing_time = base_time_ms * (1.0 + image_size_mb / 100.0);

    std::thread::sleep(std::time::Duration::from_millis(processing_time as u64));

    let elapsed = start.elapsed().as_secs_f64() * 1000.0;

    // Simular accuracy baseado no tipo de scan
    let accuracy = match scan_type {
        "CT-Tumor-Segmentation" => 0.983,
        "DEXA-Bone-Density" => 0.971,
        "Pathology-WSI" => 0.968,
        _ => 0.950,
    };

    MedicalScanMetrics {
        scan_type: scan_type.to_string(),
        processing_time_ms: elapsed,
        image_size_mb,
        accuracy,
        patient_id: format!("PT{:06}", rand::random::<u32>() % 1000000),
    }
}

fn main() {
    // Inicializar contexto AVX
    let context = AvxContext {
        stack: "Avx".to_string(),
        layer: "medical".to_string(),
        env: "production".to_string(),
        cluster: "AVL-BR-Medical".to_string(),
        mesh: "healthcare".to_string(),
    };

    avx_telemetry::init_tracing(&context);

    println!("🏥 AVX Medical Imaging Telemetry Example");
    println!("==========================================\n");

    // Cenário 1: Cancer Detection Pipeline
    println!("📊 Scenario 1: CT Tumor Segmentation");
    println!("-------------------------------------");

    let mut ct_latencies = Vec::new();

    for i in 1..=10 {
        let scan = process_medical_scan("CT-Tumor-Segmentation", 50.0 + (i as f64 * 5.0));
        ct_latencies.push(scan.processing_time_ms);

        println!(
            "  Scan #{}: Patient {}, {} MB, {:.2}ms, Accuracy: {:.1}%",
            i,
            scan.patient_id,
            scan.image_size_mb,
            scan.processing_time_ms,
            scan.accuracy * 100.0
        );
    }

    // Detectar anomalias
    let metrics = AvxMetrics::new();
    match metrics.track_latencies(ct_latencies.clone()) {
        Ok(anomalies) => {
            if anomalies.is_empty() {
                println!("  ✅ No anomalies detected in CT processing");
            } else {
                println!("  ⚠️  Detected {} anomalies:", anomalies.len());
                for anomaly in anomalies {
                    println!(
                        "     - Value: {:.2}ms, Score: {:.2}",
                        anomaly.value, anomaly.score
                    );
                }
            }
        }
        Err(e) => {
            println!("  ❌ Error detecting anomalies: {:?}", e);
        }
    }

    println!();

    // Cenário 2: Bone Density Analysis
    println!("🦴 Scenario 2: DEXA Bone Density Analysis");
    println!("------------------------------------------");

    let mut dexa_latencies = Vec::new();

    for i in 1..=8 {
        let scan = process_medical_scan("DEXA-Bone-Density", 10.0 + (i as f64 * 2.0));
        dexa_latencies.push(scan.processing_time_ms);

        println!(
            "  Scan #{}: Patient {}, {} MB, {:.2}ms",
            i, scan.patient_id, scan.image_size_mb, scan.processing_time_ms
        );
    }

    // Calcular estatísticas
    let ts = TimeSeries::new(dexa_latencies.clone());
    let stats = ts.statistics();

    println!("  📈 Statistics:");
    println!("     - Mean: {:.2}ms", stats.mean);
    println!("     - Std Dev: {:.2}ms", stats.std_dev);
    println!("     - Min: {:.2}ms", stats.min);
    println!("     - Max: {:.2}ms", stats.max);

    println!();

    // Cenário 3: Pathology Whole Slide Imaging
    println!("🔬 Scenario 3: Pathology WSI Analysis");
    println!("--------------------------------------");

    let mut wsi_latencies = Vec::new();

    // Simular processamento de gigapixel slides (100K x 100K pixels)
    for i in 1..=6 {
        let scan = process_medical_scan("Pathology-WSI", 500.0 + (i as f64 * 100.0));
        wsi_latencies.push(scan.processing_time_ms);

        println!(
            "  Slide #{}: Patient {}, {} MB, {:.2}ms, Accuracy: {:.1}%",
            i,
            scan.patient_id,
            scan.image_size_mb,
            scan.processing_time_ms,
            scan.accuracy * 100.0
        );
    }

    println!();

    // Cenário 4: Quality Assessment (NASA Standards)
    println!("⭐ Scenario 4: Data Quality Assessment");
    println!("---------------------------------------");

    let metrics_qa = AvxMetrics::new(String::from("ct_segmentation"), 0.983);
    let quality = metrics_qa.assess_quality(
        0.983, // accuracy (CT tumor segmentation)
        0.98,  // completeness
        0.97,  // consistency
        150,   // timeliness_ms
        0.96,  // validity
    );

    println!("  Quality Metrics:");
    println!("     - Accuracy: {:.1}%", quality.accuracy * 100.0);
    println!("     - Completeness: {:.1}%", quality.completeness * 100.0);
    println!("     - Consistency: {:.1}%", quality.consistency * 100.0);
    println!("     - Timeliness: {}ms", quality.timeliness_ms);
    println!("     - Validity: {:.1}%", quality.validity * 100.0);
    println!("     - Overall Score: {:.3}", quality.overall_score);

    if quality.meets_nasa_standards() {
        println!("  ✅ Meets NASA standards (≥0.95)");
    } else {
        println!("  ❌ Does not meet NASA standards");
    }

    println!();

    // Cenário 5: Workload Forecasting
    println!("🔮 Scenario 5: Workload Forecasting");
    println!("------------------------------------");

    // Combinar todas as latências para forecast
    let mut all_latencies = Vec::new();
    all_latencies.extend(&ct_latencies);
    all_latencies.extend(&dexa_latencies);
    all_latencies.extend(&wsi_latencies);

    println!("  Historical data points: {}", all_latencies.len());
    println!("  Forecasting next 5 periods...");

    let metrics_forecast = AvxMetrics::new(String::from("latency_forecast"), all_latencies[0]);
    match metrics_forecast.forecast_metric(all_latencies.clone(), 5) {
        Ok(forecast) => {
            println!("  📊 Forecast:");
            for (i, value) in forecast.iter().enumerate() {
                println!("     - Period {}: {:.2}ms", i + 1, value);
            }

            // Calcular média prevista
            let avg_forecast: f64 = forecast.iter().sum::<f64>() / forecast.len() as f64;
            let ts = TimeSeries::new(all_latencies.clone());
            let current_avg = ts.statistics().mean;

            let change_pct = ((avg_forecast - current_avg) / current_avg) * 100.0;

            println!("  📈 Trend Analysis:");
            println!("     - Current avg: {:.2}ms", current_avg);
            println!("     - Forecast avg: {:.2}ms", avg_forecast);
            println!("     - Change: {:+.1}%", change_pct);

            if change_pct > 10.0 {
                println!("  ⚠️  Expected workload increase - consider scaling");
            } else if change_pct < -10.0 {
                println!("  📉 Expected workload decrease - can optimize resources");
            } else {
                println!("  ✅ Workload stable");
            }
        }
        Err(e) => {
            println!("  ❌ Forecasting error: {:?}", e);
        }
    }

    println!();

    // Summary
    println!("📋 Summary");
    println!("----------");
    println!("  Total scans processed: {}", all_latencies.len());

    let ts = TimeSeries::new(all_latencies);
    let final_stats = ts.statistics();

    println!("  Overall performance:");
    println!("     - Mean latency: {:.2}ms", final_stats.mean);
    println!("     - Std deviation: {:.2}ms", final_stats.std_dev);
    println!(
        "     - Range: {:.2}ms - {:.2}ms",
        final_stats.min, final_stats.max
    );

    // SLA check (exemplo: P99 < 1000ms)
    let p99 = calculate_percentile(&ts.values, 0.99);
    println!("     - P99 latency: {:.2}ms", p99);

    if p99 < 1000.0 {
        println!("  ✅ Meeting SLA target (P99 < 1000ms)");
    } else {
        println!("  ❌ SLA violation (P99 >= 1000ms)");
    }

    println!("\n🎉 Medical imaging telemetry analysis complete!");
}

/// Helper para calcular percentil
fn calculate_percentile(data: &[f64], p: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let index = (p * (sorted.len() - 1) as f64) as usize;
    sorted[index]
}
