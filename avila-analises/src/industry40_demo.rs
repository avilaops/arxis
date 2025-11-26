mod industry40;
mod models;

use industry40::*;
use chrono::Utc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║   INDÚSTRIA 4.0 - DEMO COMPLETA                     ║");
    println!("║   Smart Factory Analytics & Predictive Maintenance  ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    // ========== 1. INGESTÃO DE DADOS IoT ==========
    println!("📡 1. INGESTÃO DE DADOS IoT\n");

    let mut ingester = iot::IoTDataIngester::new(100);

    // Registrar dispositivos
    let machine = iot::IoTDevice {
        device_id: "CNC-001".to_string(),
        device_type: iot::DeviceType::Machine,
        location: iot::Location {
            facility: "Factory São Paulo".to_string(),
            area: "Production Line A".to_string(),
            line: "Line 1".to_string(),
            station: "Station 3".to_string(),
        },
        status: iot::DeviceStatus::Online,
        metadata: std::collections::HashMap::new(),
    };

    ingester.register_device(machine);

    let simulator = iot::SensorSimulator::new("CNC-001".to_string());
    let telemetry = simulator.generate_telemetry();

    println!("✓ Dispositivo CNC-001 registrado");
    println!("  Temperatura: {:.1}°C", telemetry.temperature_c);
    println!("  Vibração: {:.1} mm/s", telemetry.vibration_mm_s);
    println!("  Pressão: {:.1} bar", telemetry.pressure_bar);
    println!("  Velocidade: {:.0} RPM", telemetry.speed_rpm);
    println!("  Potência: {:.1} kW", telemetry.power_consumption_kw);

    let limits = iot::OperatingLimits::default();
    let health_score = telemetry.health_score(&limits);
    println!("  Health Score: {:.1}%\n", health_score * 100.0);

    // ========== 2. MANUTENÇÃO PREDITIVA ==========
    println!("🔧 2. MANUTENÇÃO PREDITIVA\n");

    let mut pm_engine = predictive_maintenance::PredictiveMaintenanceEngine::new();

    // Simular histórico de telemetria
    let mut historical_data = Vec::new();
    for _ in 0..100 {
        historical_data.push(simulator.generate_telemetry());
    }

    pm_engine.train_model("CNC-001".to_string(), &historical_data);
    println!("✓ Modelo treinado com 100 amostras históricas");

    // Testar predição com telemetria normal
    let normal_telemetry = simulator.generate_telemetry();
    let alert = pm_engine.predict_failure(&normal_telemetry);

    match alert {
        Some(a) => {
            println!("⚠️  Alerta de Manutenção!");
            println!("  Probabilidade de Falha: {:.1}%", a.failure_probability * 100.0);
            println!("  Severidade: {:?}", a.severity);
            println!("  Ação Recomendada: {}", a.recommended_action);
        }
        None => println!("✓ Máquina operando normalmente"),
    }

    // Testar com anomalia
    let anomaly_telemetry = simulator.generate_anomaly(iot::AnomalyType::HighTemperature);
    println!("\n🔥 Simulando anomalia - Alta Temperatura: {:.1}°C", anomaly_telemetry.temperature_c);

    let alert = pm_engine.predict_failure(&anomaly_telemetry);
    if let Some(a) = alert {
        println!("🚨 ALERTA CRÍTICO!");
        println!("  Probabilidade de Falha: {:.1}%", a.failure_probability * 100.0);
        println!("  Tipo de Falha Prevista: {:?}", a.predicted_failure_type);
        println!("  Causas: {}", a.root_causes.join(", "));
    }

    // ========== 3. CÁLCULO DE OEE ==========
    println!("\n📊 3. OEE (OVERALL EQUIPMENT EFFECTIVENESS)\n");

    let calculator = oee::OEECalculator::new(1000, 8.0); // 1s cycle, 8h shift

    let production_data = oee::ProductionData {
        device_id: "CNC-001".to_string(),
        period_start: Utc::now(),
        period_end: Utc::now(),
        actual_production_time_hours: 7.2,
        downtime_hours: 0.8,
        breakdown_time_hours: 0.5,
        setup_time_hours: 0.3,
        total_count: 24000,
        good_count: 23280,
        reject_count: 720,
        actual_avg_cycle_time_ms: 1080,
        minor_stops_count: 15,
    };

    let oee_metrics = calculator.calculate_oee(&production_data);

    println!("📈 Métricas OEE:");
    println!("  OEE Total: {:.1}%", oee_metrics.oee_percent());
    println!("  ├─ Disponibilidade: {:.1}%", oee_metrics.availability_percent());
    println!("  ├─ Performance: {:.1}%", oee_metrics.performance_percent());
    println!("  └─ Qualidade: {:.1}%", oee_metrics.quality_percent());
    println!("\n  Produção:");
    println!("    Alvo: {} peças", oee_metrics.target_count);
    println!("    Produzido: {} peças", oee_metrics.total_count);
    println!("    Boas: {} peças", oee_metrics.good_count);
    println!("    Defeitos: {} peças ({:.1}%)", oee_metrics.reject_count, oee_metrics.defect_rate());

    let classification = calculator.classify_oee(oee_metrics.oee);
    println!("\n  Classificação: {:?}", classification);

    let losses = calculator.analyze_losses(&production_data);
    println!("\n  Análise de Perdas (Six Big Losses):");
    println!("    Breakdown: {:.1}%", losses.breakdown_loss_percent);
    println!("    Setup/Changeover: {:.1}%", losses.setup_changeover_loss_percent);
    println!("    Small Stops: {:.1}%", losses.small_stops_loss_percent);
    println!("    Speed Loss: {:.1}%", losses.speed_loss_percent);
    println!("    Defects: {:.1}%", losses.defect_loss_percent);

    let (biggest_loss, loss_value) = losses.biggest_loss();
    println!("\n  ⚠️  Maior Perda: {} ({:.1}%)", biggest_loss, loss_value);

    // ========== 4. GÊMEO DIGITAL ==========
    println!("\n🔷 4. GÊMEO DIGITAL (DIGITAL TWIN)\n");

    let mut twin = digital_twin::DigitalTwin::new(
        "twin-CNC-001".to_string(),
        "CNC-001".to_string(),
    );

    twin.update_from_telemetry(&telemetry);
    println!("✓ Gêmeo digital atualizado");
    println!("  Estado atual:");
    println!("    Temperatura: {:.1}°C", twin.state.temperature_c);
    println!("    Vibração: {:.1} mm/s", twin.state.vibration_mm_s);
    println!("    Status: {:?}", twin.state.machine_status);

    // Simular comportamento futuro
    let future_states = twin.simulate_future(24);
    println!("\n  Simulação 24h futuro:");
    println!("    Vibração prevista: {:.1} mm/s (atual) → {:.1} mm/s (24h)",
        twin.state.vibration_mm_s,
        future_states.last().unwrap().vibration_mm_s);

    let anomalies = twin.detect_anomalies();
    if !anomalies.is_empty() {
        println!("\n  ⚠️  Anomalias detectadas:");
        for anomaly in anomalies {
            println!("    - {:?}: {} (severidade: {:.1})",
                anomaly.anomaly_type,
                anomaly.description,
                anomaly.severity);
        }
    }

    // ========== 5. OTIMIZAÇÃO DE PRODUÇÃO ==========
    println!("\n🎯 5. OTIMIZAÇÃO DE PRODUÇÃO\n");

    let constraints = production_optimizer::ProductionConstraints {
        max_machines: 10,
        max_shifts: 3,
        max_overtime_hours: 4.0,
        min_quality_rate: 0.95,
    };

    let orders = vec![
        production_optimizer::ProductionOrder {
            order_id: "ORD-001".to_string(),
            product_id: "PROD-A".to_string(),
            quantity: 5000,
            priority: 1,
            due_date: Utc::now(),
        },
        production_optimizer::ProductionOrder {
            order_id: "ORD-002".to_string(),
            product_id: "PROD-B".to_string(),
            quantity: 3000,
            priority: 2,
            due_date: Utc::now(),
        },
    ];

    let optimizer = production_optimizer::ProductionOptimizer::new(constraints);
    let result = optimizer.optimize_schedule(orders);

    println!("✓ Cronograma otimizado:");
    println!("  Throughput esperado: {:.0} peças/dia", result.expected_throughput);
    println!("  Custo esperado: R$ {:.2}", result.expected_cost);
    println!("  Qualidade esperada: {:.1}%", result.expected_quality * 100.0);
    println!("  Melhoria: +{:.1}%", result.improvement_percent);

    // ========== 6. CONTROLE DE QUALIDADE ==========
    println!("\n✅ 6. CONTROLE DE QUALIDADE\n");

    let inspector = quality_control::QualityInspector::new();

    let product = quality_control::Product {
        id: "PROD-12345".to_string(),
        dimensions: quality_control::Dimensions {
            length: 100.5,
            width: 50.0,
            height: 25.0,
        },
    };

    let inspection = inspector.inspect(&product);

    println!("Inspeção do produto {}:", product.id);
    println!("  Status: {}", if inspection.passed { "✓ APROVADO" } else { "✗ REJEITADO" });
    println!("  Quality Score: {:.1}%", inspection.quality_score * 100.0);

    if !inspection.defects.is_empty() {
        println!("  Defeitos encontrados:");
        for defect in &inspection.defects {
            println!("    - {:?} em {} (severidade: {:.1})",
                defect.defect_type,
                defect.location,
                defect.severity);
        }
    }

    // ========== 7. GESTÃO DE ENERGIA ==========
    println!("\n⚡ 7. GESTÃO DE ENERGIA\n");

    let mut energy_monitor = energy_management::EnergyMonitor::new();
    energy_monitor.record_consumption("CNC-001".to_string(), 125.5);

    println!("Consumo atual: {:.1} kW", energy_monitor.get_total_consumption());

    let energy_optimizer = energy_management::EnergyOptimizer::new();
    let optimization = energy_optimizer.suggest_optimization(125.5);

    println!("💡 Otimização de Energia:");
    println!("  Economia potencial: {:.1} kW ({:.1}%)",
        optimization.potential_savings_kw,
        (optimization.potential_savings_kw / optimization.current_consumption_kw) * 100.0);
    println!("  Recomendações:");
    for rec in &optimization.recommendations {
        println!("    • {}", rec);
    }

    // ========== 8. DETECÇÃO DE ANOMALIAS ==========
    println!("\n🔍 8. DETECÇÃO DE ANOMALIAS EM SÉRIES TEMPORAIS\n");

    let detector = time_series::AnomalyDetector::new(3.0);
    let values = vec![
        50.0, 51.0, 49.5, 50.5, 52.0, 85.0, // 85.0 é anomalia
        50.0, 51.5, 49.0, 50.0,
    ];

    let anomaly_indices = detector.detect(&values);

    println!("Valores analisados: {} pontos", values.len());
    println!("Anomalias detectadas: {}", anomaly_indices.len());
    for idx in anomaly_indices {
        println!("  • Índice {}: valor = {:.1}", idx, values[idx]);
    }

    // Análise de tendência
    let analyzer = time_series::TimeSeriesAnalyzer::new(3);
    let trend = analyzer.detect_trend(&values);
    println!("\nTendência: {:?}", trend);

    println!("\n╔══════════════════════════════════════════════════════╗");
    println!("║   DEMO INDÚSTRIA 4.0 CONCLUÍDA COM SUCESSO!         ║");
    println!("║   Sistema pronto para produção! 🚀                   ║");
    println!("╚══════════════════════════════════════════════════════╝\n");

    Ok(())
}
