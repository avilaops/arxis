//! Demonstração de Real-time Analytics
//!
//! Este exemplo mostra:
//! - Stream processing de dados IoT
//! - Detecção de anomalias em tempo real
//! - Sistema de alertas
//! - Análise de padrões espaciais

use avila_geo::coords::GeoCoord;
use avila_geo::geoprocessing::*;
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  📊 Real-time Analytics - Stream Processing IoT          ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // 1. Stream Processing
    demo_stream_processing();

    // 2. Detecção de Anomalias
    demo_anomaly_detection();

    // 3. Sistema de Alertas
    demo_alert_system();

    // 4. Análise de Padrões Espaciais
    demo_spatial_patterns();
}

fn demo_stream_processing() {
    println!("═══ 1. Stream Processing ═══\n");

    let mut processor = StreamProcessor::new(TimeWindow::Minutes(5));

    println!("📡 Processando stream de temperatura em tempo real...\n");
    println!("   Janela temporal: 5 minutos");
    println!("   Dispositivo: SENSOR_TEMP_001\n");

    // Simular stream de dados de temperatura
    let temperatures = vec![
        22.5, 23.0, 22.8, 23.5, 24.0, 24.5, 25.0, 25.8,
        26.5, 27.0, 28.5, 29.0, 30.5, 31.0, 32.5, 33.0,
    ];

    for (i, &temp) in temperatures.iter().enumerate() {
        let reading = SensorReading::new(
            SensorType::Temperature,
            temp,
            "°C".to_string(),
        );

        let analytics = processor.process("SENSOR_TEMP_001".to_string(), reading);

        if i % 4 == 0 {
            println!("   [t+{}s] Leitura #{}", i * 2, i + 1);
            println!("      Temperatura: {:.1}°C", temp);
            println!("      Janela: {} leituras", analytics.window_size);
            println!("      Média: {:.2}°C", analytics.metrics.mean);
            println!("      Desvio Padrão: {:.2}°C", analytics.metrics.std_dev);
            println!("      Taxa de mudança: {:.3}°C/s", analytics.metrics.rate_of_change);

            if analytics.is_anomaly {
                println!("      🚨 ANOMALIA DETECTADA!");
            }
            println!();
        }

        thread::sleep(Duration::from_millis(100));
    }

    println!("✓ Stream processado: {} leituras", temperatures.len());
    println!();
}

fn demo_anomaly_detection() {
    println!("═══ 2. Detecção de Anomalias (Z-Score) ═══\n");

    let mut detector = AnomalyDetector::new(3.0); // 3 desvios padrão

    println!("🔍 Treinando detector com dados normais...\n");

    // Dados normais (temperatura estável)
    let normal_data: Vec<f64> = (0..30)
        .map(|i| 25.0 + (i as f64 * 0.1).sin() * 2.0) // Variação senoidal
        .collect();

    for (i, &value) in normal_data.iter().enumerate() {
        let is_anomaly = detector.detect("SENSOR_001", value);

        if i % 10 == 0 {
            println!("   Leitura #{}: {:.2}°C [{}]",
                i + 1,
                value,
                if is_anomaly { "ANOMALIA" } else { "Normal" }
            );
        }
    }

    println!("\n📊 Injetando valores anômalos...\n");

    // Valores anômalos
    let anomalies = vec![
        ("Spike de temperatura", 50.0),
        ("Queda abrupta", 5.0),
        ("Valor extremo", 100.0),
    ];

    for (description, value) in &anomalies {
        let is_anomaly = detector.detect("SENSOR_001", *value);

        println!("   {} - {:.1}°C", description, value);
        println!("      Resultado: {}",
            if is_anomaly { "🚨 ANOMALIA DETECTADA" } else { "✓ Normal" }
        );
        println!();
    }

    println!("✓ Detector de anomalias funcionando corretamente");
    println!();
}

fn demo_alert_system() {
    println!("═══ 3. Sistema de Alertas ═══\n");

    let mut system = AlertSystem::new();

    println!("⚙️  Configurando regras de alerta...\n");

    // Regra 1: Temperatura muito alta
    let rule1 = AlertRule::new(
        "Temperatura Crítica".to_string(),
        SensorType::Temperature,
        Condition::GreaterThan(35.0),
        AlertSeverity::Critical,
    );
    system.add_rule(rule1);
    println!("   ✓ Regra 1: Temperatura > 35°C [CRÍTICO]");

    // Regra 2: Temperatura alta
    let rule2 = AlertRule::new(
        "Temperatura Elevada".to_string(),
        SensorType::Temperature,
        Condition::Between(30.0, 35.0),
        AlertSeverity::Warning,
    );
    system.add_rule(rule2);
    println!("   ✓ Regra 2: Temperatura entre 30-35°C [ALERTA]");

    // Regra 3: Vibração anormal
    let rule3 = AlertRule::new(
        "Vibração Anormal".to_string(),
        SensorType::Vibration,
        Condition::GreaterThan(5.0),
        AlertSeverity::Error,
    );
    system.add_rule(rule3);
    println!("   ✓ Regra 3: Vibração > 5.0 Hz [ERRO]");

    // Regra 4: Bateria baixa
    let rule4 = AlertRule::new(
        "Bateria Baixa".to_string(),
        SensorType::BatteryLevel,
        Condition::LessThan(20.0),
        AlertSeverity::Warning,
    );
    system.add_rule(rule4);
    println!("   ✓ Regra 4: Bateria < 20% [ALERTA]");

    println!("\n📊 Processando leituras de sensores...\n");

    // Simular leituras que disparam alertas
    let readings = vec![
        ("DEVICE_001", SensorType::Temperature, 32.5, "°C"),  // Warning
        ("DEVICE_001", SensorType::Temperature, 38.0, "°C"),  // Critical
        ("DEVICE_002", SensorType::Vibration, 7.2, "Hz"),     // Error
        ("DEVICE_003", SensorType::BatteryLevel, 15.0, "%"),  // Warning
        ("DEVICE_001", SensorType::Temperature, 25.0, "°C"),  // OK
    ];

    for (device_id, sensor_type, value, unit) in readings {
        let reading = SensorReading::new(
            sensor_type.clone(),
            value,
            unit.to_string(),
        );

        let alerts = system.evaluate(device_id, &reading);

        print!("   {} - ", device_id);
        print!("{:?}: {:.1} {} ", sensor_type, value, unit);

        if alerts.is_empty() {
            println!("[✓ OK]");
        } else {
            for alert in &alerts {
                let severity_icon = match alert.severity {
                    AlertSeverity::Info => "ℹ️",
                    AlertSeverity::Warning => "⚠️",
                    AlertSeverity::Error => "❌",
                    AlertSeverity::Critical => "🚨",
                };
                println!("[{} {:?}]", severity_icon, alert.severity);
                println!("      → {}", alert.message);
            }
        }
    }

    println!("\n📋 Resumo de Alertas:\n");

    let recent = system.recent_alerts(1); // Última hora
    println!("   Total de alertas: {}", recent.len());

    let mut by_severity: std::collections::HashMap<AlertSeverity, usize> =
        std::collections::HashMap::new();

    for alert in &recent {
        *by_severity.entry(alert.severity).or_insert(0) += 1;
    }

    for (severity, count) in &by_severity {
        println!("   {:?}: {}", severity, count);
    }

    println!();
}

fn demo_spatial_patterns() {
    println!("═══ 4. Análise de Padrões Espaciais ═══\n");

    let analyzer = SpatialPatternAnalyzer::new(
        500.0,  // 500m de raio
        3,      // Mínimo 3 pontos por cluster
    );

    println!("🗺️  Detectando hotspots de incidentes em São Paulo...\n");
    println!("   Configuração:");
    println!("      Raio de clustering: 500m");
    println!("      Mínimo de pontos: 3\n");

    // Simular incidentes em diferentes regiões
    // Cluster 1: Avenida Paulista (alta concentração)
    let paulista_incidents = vec![
        (GeoCoord::new(-23.5629, -46.6544), 10.0),  // Incidente grave
        (GeoCoord::new(-23.5631, -46.6548), 5.0),
        (GeoCoord::new(-23.5627, -46.6542), 8.0),
        (GeoCoord::new(-23.5633, -46.6546), 6.0),
        (GeoCoord::new(-23.5625, -46.6540), 7.0),
    ];

    // Cluster 2: Berrini (média concentração)
    let berrini_incidents = vec![
        (GeoCoord::new(-23.6168, -46.7023), 4.0),
        (GeoCoord::new(-23.6170, -46.7025), 3.0),
        (GeoCoord::new(-23.6165, -46.7020), 5.0),
    ];

    // Pontos isolados
    let isolated = vec![
        (GeoCoord::new(-23.5505, -46.6333), 2.0),  // Centro
        (GeoCoord::new(-23.6500, -46.7500), 1.0),  // Zona Sul
    ];

    let mut all_incidents = Vec::new();
    all_incidents.extend(paulista_incidents.clone());
    all_incidents.extend(berrini_incidents.clone());
    all_incidents.extend(isolated);

    println!("📍 Incidentes registrados: {}\n", all_incidents.len());

    // Detectar hotspots
    let hotspots = analyzer.detect_hotspots(&all_incidents);

    println!("🔥 Hotspots detectados: {}\n", hotspots.len());

    for (i, hotspot) in hotspots.iter().enumerate() {
        println!("   Hotspot #{}", i + 1);
        println!("      Centro: ({:.4}, {:.4})",
            hotspot.center.lat, hotspot.center.lon);
        println!("      Tamanho: {} incidentes", hotspot.size);
        println!("      Valor total: {:.1}", hotspot.total_value);
        println!("      Intensidade: {:.2}", hotspot.intensity);

        // Identificar região
        let region = if hotspot.center.lat > -23.57 {
            "Região Paulista"
        } else if hotspot.center.lat > -23.62 {
            "Região Berrini"
        } else {
            "Outras regiões"
        };

        println!("      Localização: {}", region);

        if hotspot.intensity > 7.0 {
            println!("      ⚠️  ALTA PRIORIDADE - Requer atenção imediata");
        } else if hotspot.intensity > 4.0 {
            println!("      ⚠️  MÉDIA PRIORIDADE - Monitoramento necessário");
        }

        println!();
    }

    println!("✓ Análise espacial concluída");
    println!("  Recomendação: Aumentar recursos na Região Paulista");
    println!();
}
