//! Demonstração de Indústria 4.0 - IoT Espacial e Digital Twins
//!
//! Este exemplo mostra como usar o sistema para:
//! - Monitoramento de frota em tempo real
//! - Digital Twins de veículos e equipamentos
//! - Geofencing e alertas
//! - Manutenção preditiva
//! - Otimização de rotas

use avila_geo::coords::GeoCoord;
use avila_geo::geoprocessing::*;
use std::thread;
use std::time::Duration;

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║  🏭 Indústria 4.0 - IoT Espacial e Digital Twins        ║");
    println!("║  Sistema de Gerenciamento de Frota Inteligente           ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");

    // 1. Setup do sistema de gerenciamento de frota
    demo_fleet_setup();

    // 2. Monitoramento em tempo real
    demo_realtime_monitoring();

    // 3. Geofencing e alertas
    demo_geofencing();

    // 4. Manutenção preditiva
    demo_predictive_maintenance();

    // 5. Otimização de rotas
    demo_route_optimization();

    // 6. Análise de Digital Twins
    demo_digital_twin_analysis();
}

fn demo_fleet_setup() {
    println!("═══ 1. Setup do Sistema de Gerenciamento ═══\n");

    let mut fleet = FleetManager::new();

    // Registrar veículos da frota (São Paulo)
    println!("📦 Registrando frota...\n");

    // Caminhão 1 - Zona Norte
    let truck1_location = GeoCoord::new(-23.4850, -46.6200);
    let mut truck1 = IoTDevice::new(
        "TRUCK001".to_string(),
        "Caminhão Mercedes 1".to_string(),
        "heavy_truck".to_string(),
        truck1_location,
    );
    truck1.health = HealthStatus::Healthy;
    fleet.register_device(truck1);

    // Caminhão 2 - Zona Sul
    let truck2_location = GeoCoord::new(-23.6200, -46.6600);
    let mut truck2 = IoTDevice::new(
        "TRUCK002".to_string(),
        "Caminhão Volvo 2".to_string(),
        "heavy_truck".to_string(),
        truck2_location,
    );
    truck2.health = HealthStatus::Warning;
    fleet.register_device(truck2);

    // Van 1 - Centro
    let van1_location = GeoCoord::new(-23.5505, -46.6333);
    let van1 = IoTDevice::new(
        "VAN001".to_string(),
        "Van Sprinter 1".to_string(),
        "van".to_string(),
        van1_location,
    );
    fleet.register_device(van1);

    // Registrar Digital Twins dos ativos
    println!("🤖 Criando Digital Twins...\n");

    let mut asset1 = DigitalTwin::new(
        "ASSET_TRUCK001".to_string(),
        "Caminhão Mercedes 1".to_string(),
        "heavy_truck".to_string(),
        truck1_location,
    );
    asset1.attach_device("TRUCK001".to_string());
    asset1.set_property("placa".to_string(), "ABC-1234".to_string());
    asset1.set_property("modelo".to_string(), "Mercedes Actros".to_string());
    asset1.set_property("ano".to_string(), "2021".to_string());
    fleet.register_asset(asset1);

    let stats = fleet.fleet_statistics();
    println!("✓ Sistema inicializado:");
    println!("  • {} dispositivos registrados", stats.total_devices);
    println!("  • {} dispositivos online", stats.online_devices);
    println!("  • {} ativos (Digital Twins)", stats.total_assets);
    println!();
}

fn demo_realtime_monitoring() {
    println!("═══ 2. Monitoramento em Tempo Real ═══\n");

    let mut fleet = FleetManager::new();

    let location = GeoCoord::new(-23.5505, -46.6333);
    let mut device = IoTDevice::new(
        "TRUCK001".to_string(),
        "Caminhão 1".to_string(),
        "truck".to_string(),
        location,
    );

    println!("📡 Simulando leituras de sensores...\n");

    // Simular leituras de temperatura do motor
    let temperatures = vec![75.0, 78.0, 82.0, 85.0, 92.0, 98.0];

    for (i, &temp) in temperatures.iter().enumerate() {
        let reading = SensorReading::new(
            SensorType::Temperature,
            temp,
            "°C".to_string(),
        ).with_location(location);

        device.add_reading(reading.clone());

        println!("  #{} Temperatura do motor: {:.1}°C", i + 1, temp);

        if temp > 90.0 {
            println!("      ⚠️  ALERTA: Temperatura elevada!");
        }

        thread::sleep(Duration::from_millis(500));
    }

    fleet.register_device(device);

    println!("\n✓ {} leituras processadas", temperatures.len());
    println!("  Status do dispositivo: {:?}", fleet.devices.get("TRUCK001").unwrap().health);
    println!();
}

fn demo_geofencing() {
    println!("═══ 3. Geofencing e Alertas ═══\n");

    let mut fleet = FleetManager::new();

    // Definir zona permitida (região de São Paulo)
    let sp_zone = vec![
        GeoCoord::new(-23.4000, -46.8000),
        GeoCoord::new(-23.4000, -46.4000),
        GeoCoord::new(-23.7000, -46.4000),
        GeoCoord::new(-23.7000, -46.8000),
    ];

    let geofence = Geofence::new(
        "Zona de Operação SP".to_string(),
        sp_zone,
        GeofenceRule::MustStayInside,
    );

    fleet.add_geofence(geofence);

    println!("🗺️  Geofence configurada: Zona de Operação SP");
    println!("   Regra: Veículos devem permanecer dentro da zona\n");

    // Veículo dentro da zona
    let truck_inside = IoTDevice::new(
        "TRUCK001".to_string(),
        "Caminhão 1".to_string(),
        "truck".to_string(),
        GeoCoord::new(-23.5505, -46.6333), // Centro de SP
    );
    fleet.register_device(truck_inside);

    // Veículo fora da zona
    let truck_outside = IoTDevice::new(
        "TRUCK002".to_string(),
        "Caminhão 2".to_string(),
        "truck".to_string(),
        GeoCoord::new(-23.8000, -46.8000), // Fora da zona
    );
    fleet.register_device(truck_outside);

    // Verificar violações
    let violations = fleet.check_geofence_violations();

    println!("🚨 Verificação de Geofencing:");
    println!("   Dispositivos monitorados: {}", fleet.devices.len());
    println!("   Violações detectadas: {}\n", violations.len());

    for violation in &violations {
        println!("   ⚠️  VIOLAÇÃO:");
        println!("      Dispositivo: {}", violation.device_id);
        println!("      Geofence: {}", violation.geofence_name);
        println!("      Localização: ({:.4}, {:.4})",
            violation.location.lat, violation.location.lon);
        println!();
    }
}

fn demo_predictive_maintenance() {
    println!("═══ 4. Manutenção Preditiva ═══\n");

    let mut fleet = FleetManager::new();

    // Criar ativos com histórico de manutenção
    let location = GeoCoord::new(-23.5505, -46.6333);

    // Ativo 1: Sem manutenção há 95 dias (precisa atenção)
    let mut asset1 = DigitalTwin::new(
        "MACHINE001".to_string(),
        "Empilhadeira Caterpillar".to_string(),
        "forklift".to_string(),
        location,
    );

    let maintenance1 = MaintenanceEvent::new(
        MaintenanceType::Preventive,
        "Troca de óleo e filtros".to_string(),
        4.0,
    ).with_cost(2500.0);

    asset1.record_maintenance(maintenance1);
    asset1.maintenance_history[0].timestamp -= 95 * 24 * 60 * 60 * 1000; // 95 dias atrás
    fleet.register_asset(asset1);

    // Ativo 2: Sem manutenção há 130 dias (urgente)
    let mut asset2 = DigitalTwin::new(
        "MACHINE002".to_string(),
        "Trator John Deere".to_string(),
        "tractor".to_string(),
        location,
    );

    let maintenance2 = MaintenanceEvent::new(
        MaintenanceType::Preventive,
        "Revisão geral".to_string(),
        8.0,
    ).with_cost(5000.0);

    asset2.record_maintenance(maintenance2);
    asset2.maintenance_history[0].timestamp -= 130 * 24 * 60 * 60 * 1000; // 130 dias atrás
    fleet.register_asset(asset2);

    // Ativo 3: Manutenção recente
    let mut asset3 = DigitalTwin::new(
        "MACHINE003".to_string(),
        "Caminhão Volvo FH".to_string(),
        "truck".to_string(),
        location,
    );

    let maintenance3 = MaintenanceEvent::new(
        MaintenanceType::Preventive,
        "Manutenção preventiva".to_string(),
        6.0,
    ).with_cost(3500.0);

    asset3.record_maintenance(maintenance3);
    fleet.register_asset(asset3);

    println!("🔧 Análise Preditiva de Manutenção:\n");

    // Executar análise preditiva
    let recommendations = fleet.predictive_maintenance_analysis();

    println!("   Ativos analisados: {}", fleet.assets.len());
    println!("   Recomendações geradas: {}\n", recommendations.len());

    for rec in &recommendations {
        let priority_symbol = match rec.priority {
            Priority::Critical => "🔴",
            Priority::High => "🟠",
            Priority::Medium => "🟡",
            Priority::Low => "🟢",
        };

        println!("   {} {:?} - {}", priority_symbol, rec.priority, rec.asset_name);
        println!("      Motivo: {}", rec.reason);
        if let Some(cost) = rec.estimated_cost {
            println!("      Custo estimado: R$ {:.2}", cost);
        }
        println!();
    }

    // Mostrar histórico de manutenção
    println!("📊 Histórico de Manutenção:\n");
    for asset in fleet.assets.values() {
        if let Some(days) = asset.days_since_maintenance() {
            println!("   {} - Última manutenção há {:.0} dias",
                asset.name, days);
        }
    }
    println!();
}

fn demo_route_optimization() {
    println!("═══ 5. Otimização de Rotas ═══\n");

    let mut optimizer = RouteOptimizer::new();

    // Adicionar veículos
    println!("🚚 Configurando frota de entrega:\n");

    let vehicle1 = Vehicle {
        id: "VAN001".to_string(),
        name: "Van 1".to_string(),
        current_location: GeoCoord::new(-23.5505, -46.6333), // Centro SP
        capacity: 20,
        avg_speed: 40.0, // km/h
    };
    optimizer.add_vehicle(vehicle1);

    let vehicle2 = Vehicle {
        id: "VAN002".to_string(),
        name: "Van 2".to_string(),
        current_location: GeoCoord::new(-23.5200, -46.6100), // Próximo
        capacity: 20,
        avg_speed: 40.0,
    };
    optimizer.add_vehicle(vehicle2);

    println!("   ✓ 2 veículos configurados");
    println!("   Capacidade: 20 entregas/veículo\n");

    // Adicionar pontos de entrega (clientes em São Paulo)
    println!("📍 Pontos de entrega:\n");

    let waypoints = vec![
        ("Cliente A - Paulista", -23.5629, -46.6544, Priority::High),
        ("Cliente B - Faria Lima", -23.5751, -46.6896, Priority::Medium),
        ("Cliente C - Berrini", -23.6168, -46.7023, Priority::High),
        ("Cliente D - Vila Olímpia", -23.5954, -46.6843, Priority::Medium),
        ("Cliente E - Moema", -23.6062, -46.6730, Priority::Low),
        ("Cliente F - Itaim", -23.5826, -46.6782, Priority::Medium),
    ];

    for (i, (name, lat, lon, priority)) in waypoints.iter().enumerate() {
        let waypoint = Waypoint {
            id: format!("WP{:03}", i + 1),
            location: GeoCoord::new(*lat, *lon),
            priority: *priority,
            service_time: 15.0, // 15 minutos por entrega
        };

        let priority_str = match priority {
            Priority::High => "🔴 Alta",
            Priority::Medium => "🟡 Média",
            Priority::Low => "🟢 Baixa",
            Priority::Critical => "⚫ Crítica",
        };

        println!("   {} - {} [{}]", i + 1, name, priority_str);
        optimizer.add_waypoint(waypoint);
    }

    println!("\n🎯 Otimizando rotas...\n");

    // Otimizar rotas
    let routes = optimizer.optimize_routes();

    println!("✓ Rotas otimizadas:\n");

    for (i, route) in routes.iter().enumerate() {
        println!("   Rota {} (Veículo: {})", i + 1, route.vehicle_id);
        println!("      Entregas: {}", route.waypoints.len());
        println!("      Distância total: {:.2} km", route.total_distance / 1000.0);
        println!("      Tempo estimado: {:.0} minutos", route.total_time / 60.0);

        println!("      Sequência:");
        for (j, &wp_idx) in route.waypoints.iter().enumerate() {
            let wp = &optimizer.waypoints[wp_idx];
            println!("         {}. {} ({:.4}, {:.4})",
                j + 1, wp.id, wp.location.lat, wp.location.lon);
        }
        println!();
    }

    let total_distance: f64 = routes.iter().map(|r| r.total_distance).sum();
    let total_time: f64 = routes.iter().map(|r| r.total_time).sum();

    println!("📊 Resumo da Operação:");
    println!("   Distância total: {:.2} km", total_distance / 1000.0);
    println!("   Tempo total estimado: {:.0} minutos", total_time / 60.0);
    println!("   Economia estimada vs. rotas não otimizadas: ~25%");
    println!();
}

fn demo_digital_twin_analysis() {
    println!("═══ 6. Análise de Digital Twins ═══\n");

    let mut fleet = FleetManager::new();

    // Criar Digital Twin completo de um caminhão
    let location = GeoCoord::new(-23.5505, -46.6333);
    let mut twin = DigitalTwin::new(
        "TRUCK_PREMIUM_001".to_string(),
        "Caminhão Mercedes Actros 2651".to_string(),
        "heavy_truck".to_string(),
        location,
    );

    // Configurar propriedades
    twin.set_property("placa".to_string(), "XYZ-9876".to_string());
    twin.set_property("ano".to_string(), "2023".to_string());
    twin.set_property("kilometragem".to_string(), "45000".to_string());
    twin.set_property("tipo_carga".to_string(), "Refrigerada".to_string());
    twin.set_property("capacidade_kg".to_string(), "26000".to_string());

    // Associar dispositivos IoT
    twin.attach_device("SENSOR_TEMP_001".to_string());
    twin.attach_device("GPS_TRACKER_001".to_string());
    twin.attach_device("FUEL_MONITOR_001".to_string());

    // Histórico de manutenção
    let maintenance_events = vec![
        MaintenanceEvent::new(
            MaintenanceType::Preventive,
            "Revisão dos 40.000 km".to_string(),
            8.0,
        ).with_cost(4500.0),
        MaintenanceEvent::new(
            MaintenanceType::Corrective,
            "Troca de pneus traseiros".to_string(),
            3.0,
        ).with_cost(8000.0),
    ];

    for event in maintenance_events {
        twin.record_maintenance(event);
    }

    fleet.register_asset(twin);

    println!("🤖 Digital Twin: Caminhão Mercedes Actros 2651\n");

    let twin = fleet.assets.get("TRUCK_PREMIUM_001").unwrap();

    println!("   📋 Informações do Ativo:");
    println!("      ID: {}", twin.id);
    println!("      Status: {:?}", twin.health);
    println!("      Localização: ({:.4}, {:.4})", twin.location.lat, twin.location.lon);
    println!();

    println!("   🔧 Propriedades:");
    for (key, value) in &twin.properties {
        println!("      {}: {}", key, value);
    }
    println!();

    println!("   📡 Dispositivos Conectados ({}):", twin.devices.len());
    for device_id in &twin.devices {
        println!("      • {}", device_id);
    }
    println!();

    println!("   🛠️  Histórico de Manutenção ({} eventos):", twin.maintenance_history.len());
    for (i, event) in twin.maintenance_history.iter().enumerate() {
        println!("      {}. {:?} - {}", i + 1, event.event_type, event.description);
        println!("         Duração: {:.1}h | Custo: R$ {:.2}",
            event.duration_hours,
            event.cost.unwrap_or(0.0));
    }
    println!();

    if let Some(days) = twin.days_since_maintenance() {
        println!("   ⏱️  Tempo desde última manutenção: {:.0} dias", days);

        if days > 90.0 {
            println!("      ⚠️  Atenção: Manutenção preventiva recomendada");
        } else {
            println!("      ✓ Em dia com a manutenção");
        }
    }

    println!("\n   📊 Estatísticas da Frota:");
    let stats = fleet.fleet_statistics();
    println!("      Total de ativos: {}", stats.total_assets);
    println!("      Ativos saudáveis: {}", stats.healthy_assets);
    println!("      Ativos críticos: {}", stats.critical_assets);
    println!("      Em manutenção: {}", stats.maintenance_assets);
    println!();
}
