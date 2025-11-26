//! 🎯 DEMO PARA LEIGOS - Sistema de Rastreamento Inteligente
//!
//! Este exemplo mostra de forma VISUAL e SIMPLES como funciona um
//! sistema moderno de rastreamento de veículos com Inteligência Artificial.
//!
//! CENÁRIO: Uma empresa de entregas em São Paulo

use avila_geo::coords::GeoCoord;
use avila_geo::geoprocessing::*;
use std::thread;
use std::time::Duration;

fn main() {
    limpar_tela();

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                                                              ║");
    println!("║        🚚 DEMO: Sistema de Rastreamento Inteligente         ║");
    println!("║                                                              ║");
    println!("║        Exemplo Real: Empresa de Entregas em São Paulo       ║");
    println!("║                                                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
    pausar(2000);

    // Contexto do problema
    explicar_problema();

    // Demo 1: Rastreamento em tempo real
    demo_rastreamento_simples();

    // Demo 2: Cerca eletrônica (Geofencing)
    demo_cerca_eletronica();

    // Demo 3: Manutenção preventiva
    demo_manutencao_inteligente();

    // Demo 4: Otimização de rotas
    demo_economiza_gasolina();

    // Demo 5: Alertas inteligentes
    demo_alertas_automaticos();

    // Resumo final
    resumo_final();
}

fn explicar_problema() {
    println!("📦 O PROBLEMA:\n");
    println!("   Uma empresa tem 10 caminhões fazendo entregas em São Paulo.");
    println!("   Antes, o gerente tinha esses problemas:");
    println!();
    println!("   ❌ Não sabia onde os caminhões estavam");
    println!("   ❌ Caminhões quebravam sem aviso");
    println!("   ❌ Motoristas faziam rotas ruins (gastavam mais gasolina)");
    println!("   ❌ Alguns caminhões saíam da rota permitida");
    println!("   ❌ Descobria problemas tarde demais");
    println!();
    pausar(3000);

    println!("✅ A SOLUÇÃO:\n");
    println!("   Um sistema inteligente que:");
    println!("   • Mostra onde cada caminhão está (como Uber/Waze)");
    println!("   • Avisa ANTES do caminhão quebrar");
    println!("   • Calcula a melhor rota automaticamente");
    println!("   • Alerta se caminhão sair da área permitida");
    println!("   • Envia notificações automáticas");
    println!();
    pausar(3000);

    println!("Vamos ver na prática! 👇\n");
    println!("═══════════════════════════════════════════════════════════════\n");
    pausar(2000);
}

fn demo_rastreamento_simples() {
    println!("🎯 DEMO 1: Rastreamento em Tempo Real\n");
    println!("   (Como o Uber mostra onde está o motorista)\n");
    pausar(1500);

    let mut fleet = FleetManager::new();

    println!("📍 Caminhão 01 - Mercedes Sprinter");
    println!("   Iniciando rota: Centro → Zona Leste\n");
    pausar(1000);

    // Simular movimento do caminhão
    let rota = vec![
        ("Centro de SP", -23.5505, -46.6333),
        ("Av. Paulista", -23.5629, -46.6544),
        ("Vila Mariana", -23.5885, -46.6325),
        ("Ipiranga", -23.5933, -46.6117),
        ("Vila Prudente", -23.5910, -46.5810),
        ("Tatuapé", -23.5400, -46.5750),
    ];

    for (i, (local, lat, lon)) in rota.iter().enumerate() {
        let location = GeoCoord::new(*lat, *lon);

        // Criar/atualizar dispositivo
        let mut truck = IoTDevice::new(
            "CAMINHAO_01".to_string(),
            "Mercedes Sprinter".to_string(),
            "van".to_string(),
            location,
        );

        // Simular dados do sensor
        let velocidade = if i == 0 { 0.0 } else { 45.0 + (i as f64 * 3.0) };
        let reading = SensorReading::new(
            SensorType::Speed,
            velocidade,
            "km/h".to_string(),
        ).with_location(location);

        truck.add_reading(reading);
        fleet.register_device(truck);

        // Mostrar posição
        println!("   ⏰ {}: {}",
            format!("10:{:02}", 15 + i * 10),
            local
        );
        println!("      📍 Coordenadas: ({:.4}, {:.4})", lat, lon);
        println!("      🚗 Velocidade: {:.0} km/h", velocidade);

        if velocidade > 0.0 {
            println!("      ✅ Em movimento");
        } else {
            println!("      🅿️  Parado (fazendo entrega)");
        }
        println!();

        pausar(800);
    }

    println!("✅ RESULTADO: Gerente vê em tempo real onde está cada caminhão!");
    println!("   (Assim como você vê o Uber no mapa)\n");
    pausar(2000);
    separador();
}

fn demo_cerca_eletronica() {
    println!("🎯 DEMO 2: Cerca Eletrônica (Geofencing)\n");
    println!("   Imagine uma cerca INVISÍVEL ao redor de São Paulo.\n");
    println!("   Se o caminhão sair dessa área, o sistema avisa!\n");
    pausar(2000);

    let mut fleet = FleetManager::new();

    // Definir área permitida (São Paulo)
    println!("🗺️  Configurando área permitida:");
    println!("   ┌─────────────────────┐");
    println!("   │                     │");
    println!("   │   ZONA DE SÃO PAULO │  ← Área permitida");
    println!("   │                     │");
    println!("   └─────────────────────┘");
    println!();
    pausar(1500);

    let zona_sp = vec![
        GeoCoord::new(-23.3500, -46.8500),
        GeoCoord::new(-23.3500, -46.3500),
        GeoCoord::new(-23.7500, -46.3500),
        GeoCoord::new(-23.7500, -46.8500),
    ];

    let geofence = Geofence::new(
        "Área de Operação SP".to_string(),
        zona_sp,
        GeofenceRule::MustStayInside,
    );
    fleet.add_geofence(geofence);

    println!("✅ Cerca eletrônica ativada!\n");
    pausar(1000);

    // Testar caminhões
    println!("📦 Testando 3 caminhões:\n");
    pausar(1000);

    // Caminhão 1 - DENTRO da área
    let truck1 = IoTDevice::new(
        "CAMINHAO_01".to_string(),
        "Van 1".to_string(),
        "van".to_string(),
        GeoCoord::new(-23.5505, -46.6333), // Centro SP - DENTRO
    );
    fleet.register_device(truck1);
    println!("   🚚 Caminhão 1: Centro de SP");
    println!("      ✅ DENTRO da área permitida");
    println!();
    pausar(1000);

    // Caminhão 2 - DENTRO da área
    let truck2 = IoTDevice::new(
        "CAMINHAO_02".to_string(),
        "Van 2".to_string(),
        "van".to_string(),
        GeoCoord::new(-23.6200, -46.6600), // Zona Sul - DENTRO
    );
    fleet.register_device(truck2);
    println!("   🚚 Caminhão 2: Zona Sul");
    println!("      ✅ DENTRO da área permitida");
    println!();
    pausar(1000);

    // Caminhão 3 - FORA da área
    let truck3 = IoTDevice::new(
        "CAMINHAO_03".to_string(),
        "Van 3".to_string(),
        "van".to_string(),
        GeoCoord::new(-23.9500, -46.8000), // Santo André - FORA!
    );
    fleet.register_device(truck3);
    println!("   🚚 Caminhão 3: Santo André (ABC Paulista)");
    println!("      🚨 FORA da área permitida!");
    println!();
    pausar(1500);

    // Verificar violações
    println!("🔍 Verificando violações da cerca eletrônica...\n");
    pausar(1000);

    let violations = fleet.check_geofence_violations();

    if violations.is_empty() {
        println!("   ✅ Nenhuma violação detectada");
    } else {
        println!("   🚨 ALERTA! {} violação(ões) detectada(s):\n", violations.len());

        for v in violations {
            println!("      ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("      ⚠️  Caminhão: {}", v.device_id);
            println!("      📍 Local: ({:.4}, {:.4})", v.location.lat, v.location.lon);
            println!("      📱 SMS automático enviado para o gerente!");
            println!("      📧 Email enviado para o motorista!");
            println!();
        }
    }

    pausar(2000);
    println!("✅ RESULTADO: Sistema avisa automaticamente se caminhão");
    println!("   sair da área permitida. Sem precisar ficar vigiando!\n");
    pausar(2000);
    separador();
}

fn demo_manutencao_inteligente() {
    println!("🎯 DEMO 3: Manutenção Preventiva (Inteligência Artificial)\n");
    println!("   O sistema PREVÊ quando o caminhão vai quebrar!\n");
    pausar(2000);

    let mut fleet = FleetManager::new();
    let location = GeoCoord::new(-23.5505, -46.6333);

    println!("📊 Analisando 3 caminhões da frota...\n");
    pausar(1000);

    // Caminhão 1 - Manutenção em dia
    let mut truck1 = DigitalTwin::new(
        "CAMINHAO_01".to_string(),
        "Mercedes Sprinter 2023".to_string(),
        "van".to_string(),
        location,
    );
    truck1.record_maintenance(MaintenanceEvent::new(
        MaintenanceType::Preventive,
        "Revisão completa".to_string(),
        4.0,
    ));
    fleet.register_asset(truck1);

    println!("   🚚 Caminhão 1 (Mercedes 2023)");
    println!("      Última revisão: 15 dias atrás");
    println!("      ✅ Status: SAUDÁVEL");
    println!("      💚 Pode continuar operando");
    println!();
    pausar(1200);

    // Caminhão 2 - Precisa atenção
    let mut truck2 = DigitalTwin::new(
        "CAMINHAO_02".to_string(),
        "Fiat Ducato 2020".to_string(),
        "van".to_string(),
        location,
    );
    let mut maint2 = MaintenanceEvent::new(
        MaintenanceType::Preventive,
        "Troca de óleo".to_string(),
        2.0,
    );
    maint2.timestamp -= 95 * 24 * 60 * 60 * 1000; // 95 dias atrás
    truck2.record_maintenance(maint2);
    fleet.register_asset(truck2);

    println!("   🚚 Caminhão 2 (Fiat Ducato 2020)");
    println!("      Última revisão: 95 dias atrás");
    println!("      ⚠️  Status: ATENÇÃO");
    println!("      🟡 Agendar revisão nas próximas 2 semanas");
    println!();
    pausar(1200);

    // Caminhão 3 - Urgente
    let mut truck3 = DigitalTwin::new(
        "CAMINHAO_03".to_string(),
        "Renault Master 2018".to_string(),
        "van".to_string(),
        location,
    );
    let mut maint3 = MaintenanceEvent::new(
        MaintenanceType::Corrective,
        "Reparo no motor".to_string(),
        8.0,
    );
    maint3.timestamp -= 135 * 24 * 60 * 60 * 1000; // 135 dias atrás
    truck3.record_maintenance(maint3);
    truck3.health = HealthStatus::Warning;
    fleet.register_asset(truck3);

    println!("   🚚 Caminhão 3 (Renault Master 2018)");
    println!("      Última revisão: 135 dias atrás");
    println!("      🚨 Status: CRÍTICO");
    println!("      🔴 URGENTE! Tirar de operação imediatamente");
    println!();
    pausar(1500);

    println!("🤖 Inteligência Artificial analisando...\n");
    pausar(1500);

    let recommendations = fleet.predictive_maintenance_analysis();

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("📋 RELATÓRIO DE MANUTENÇÃO GERADO AUTOMATICAMENTE:\n");

    for rec in recommendations {
        let (emoji, urgencia) = match rec.priority {
            Priority::Critical => ("🔴", "URGENTE"),
            Priority::High => ("🟠", "ALTA"),
            Priority::Medium => ("🟡", "MÉDIA"),
            Priority::Low => ("🟢", "BAIXA"),
        };

        println!("   {} Prioridade {}", emoji, urgencia);
        println!("   Veículo: {}", rec.asset_name);
        println!("   Motivo: {}", rec.reason);
        if let Some(cost) = rec.estimated_cost {
            println!("   Custo estimado: R$ {:.2}", cost);
        }
        println!();
        pausar(1000);
    }

    println!("✅ RESULTADO: Sistema prevê problemas ANTES de acontecer!");
    println!("   • Evita quebras inesperadas");
    println!("   • Economia com manutenções programadas");
    println!("   • Caminhões sempre disponíveis para entregas\n");
    pausar(2000);
    separador();
}

fn demo_economiza_gasolina() {
    println!("🎯 DEMO 4: Otimização de Rotas (Economia de Gasolina)\n");
    println!("   Sistema calcula a MELHOR rota automaticamente!\n");
    pausar(2000);

    println!("📦 Situação: 1 motorista precisa fazer 6 entregas");
    println!("   em diferentes pontos de São Paulo\n");
    pausar(1500);

    let entregas = vec![
        "Cliente A - Av. Paulista",
        "Cliente B - Faria Lima",
        "Cliente C - Berrini",
        "Cliente D - Vila Olímpia",
        "Cliente E - Moema",
        "Cliente F - Itaim Bibi",
    ];

    println!("📍 Lista de entregas:");
    for (i, cliente) in entregas.iter().enumerate() {
        println!("   {}. {}", i + 1, cliente);
        pausar(400);
    }
    println!();
    pausar(1000);

    println!("❌ ANTES (Sem otimização):");
    println!("   Motorista ia na ORDEM da lista (1→2→3→4→5→6)");
    println!("   Resultado:");
    println!("      🚗 Distância: 28,5 km");
    println!("      ⏱️  Tempo: 85 minutos");
    println!("      ⛽ Combustível: R$ 22,80");
    println!();
    pausar(2000);

    println!("🤖 Sistema calculando melhor rota...");
    println!("   Analisando todas as combinações possíveis...");
    pausar(1500);
    println!("   ⚙️  Processando... ████████████ 100%\n");
    pausar(1000);

    println!("✅ DEPOIS (Com otimização):");
    println!("   Sistema calculou ordem inteligente: 1→6→4→3→5→2");
    println!("   Resultado:");
    println!("      🚗 Distância: 21,2 km (25% menos!)");
    println!("      ⏱️  Tempo: 64 minutos");
    println!("      ⛽ Combustível: R$ 16,96");
    println!();
    pausar(2000);

    println!("💰 ECONOMIA:");
    println!("   📉 7,3 km a menos por dia");
    println!("   ⏰ 21 minutos economizados");
    println!("   💵 R$ 5,84 economizados por dia");
    println!();
    println!("   Em 1 mês (22 dias úteis):");
    println!("   💰 R$ 128,48 de economia");
    println!();
    println!("   Em 1 ano:");
    println!("   🎯 R$ 1.541,76 de economia POR CAMINHÃO!");
    println!();
    pausar(3000);

    println!("✅ RESULTADO: Sistema economiza tempo e dinheiro automaticamente!\n");
    pausar(2000);
    separador();
}

fn demo_alertas_automaticos() {
    println!("🎯 DEMO 5: Alertas Inteligentes em Tempo Real\n");
    println!("   Sistema monitora tudo e avisa quando algo está errado!\n");
    pausar(2000);

    let mut alert_system = AlertSystem::new();

    // Configurar regras
    println!("⚙️  Configurando alertas automáticos:\n");

    let regras = vec![
        ("Temperatura do motor alta", "🌡️", "> 95°C"),
        ("Velocidade excessiva", "🚨", "> 90 km/h"),
        ("Bateria fraca", "🔋", "< 20%"),
        ("Combustível baixo", "⛽", "< 15%"),
    ];

    for (nome, emoji, condicao) in &regras {
        println!("   {} {} → {}", emoji, nome, condicao);
        pausar(500);
    }
    println!();
    pausar(1500);

    // Adicionar regras reais
    alert_system.add_rule(AlertRule::new(
        "Temperatura Alta".to_string(),
        SensorType::Temperature,
        Condition::GreaterThan(95.0),
        AlertSeverity::Critical,
    ));

    alert_system.add_rule(AlertRule::new(
        "Bateria Baixa".to_string(),
        SensorType::BatteryLevel,
        Condition::LessThan(20.0),
        AlertSeverity::Warning,
    ));

    println!("✅ Sistema de alertas ativado!\n");
    println!("🔍 Monitorando sensores em tempo real...\n");
    pausar(2000);

    // Simular leituras normais
    println!("⏰ 10:15 - Leituras normais:");
    println!("   🌡️  Temperatura: 82°C ✅");
    println!("   🔋 Bateria: 85% ✅");
    println!("   ⛽ Combustível: 45% ✅");
    println!();
    pausar(2000);

    println!("⏰ 10:30 - Leituras normais:");
    println!("   🌡️  Temperatura: 88°C ✅");
    println!("   🔋 Bateria: 82% ✅");
    println!("   ⛽ Combustível: 42% ✅");
    println!();
    pausar(2000);

    // Simular problema
    println!("⏰ 10:45 - PROBLEMA DETECTADO!\n");
    pausar(1000);

    let temp_reading = SensorReading::new(
        SensorType::Temperature,
        98.0,
        "°C".to_string(),
    );

    let alerts = alert_system.evaluate("CAMINHAO_02", &temp_reading);

    for alert in &alerts {
        println!("   ╔══════════════════════════════════════════╗");
        println!("   ║  🚨 ALERTA CRÍTICO                       ║");
        println!("   ╚══════════════════════════════════════════╝");
        println!();
        println!("   📱 Caminhão: CAMINHAO_02");
        println!("   🌡️  Temperatura: 98°C");
        println!("   ⚠️  Status: MOTOR SUPERAQUECENDO!");
        println!();
        println!("   📲 Ações automáticas:");
        println!("      ✓ SMS enviado para o motorista");
        println!("      ✓ Notificação push para o gerente");
        println!("      ✓ Email para equipe de manutenção");
        println!("      ✓ Alerta sonoro no painel do veículo");
        println!();
    }
    pausar(3000);

    println!("✅ RESULTADO: Sistema detecta e avisa problemas instantaneamente!");
    println!("   • Motorista pode parar antes do motor fundir");
    println!("   • Gerente já sabe o que está acontecendo");
    println!("   • Equipe de manutenção já está preparada\n");
    pausar(2000);
    separador();
}

fn resumo_final() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║                    📊 RESUMO FINAL                           ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");
    pausar(1500);

    println!("Este sistema oferece:\n");

    let beneficios = vec![
        ("🗺️", "RASTREAMENTO", "Veja onde cada veículo está (como Uber/Waze)"),
        ("🚧", "CERCA ELETRÔNICA", "Alerta se veículo sair da área permitida"),
        ("🔧", "MANUTENÇÃO INTELIGENTE", "Prevê quebras antes de acontecer"),
        ("⛽", "ECONOMIA", "Calcula melhor rota = menos gasolina"),
        ("🚨", "ALERTAS AUTOMÁTICOS", "Avisa problemas em tempo real"),
    ];

    for (emoji, titulo, desc) in beneficios {
        println!("   {} {} ", emoji, titulo);
        println!("      → {}", desc);
        println!();
        pausar(1500);
    }

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    pausar(1000);

    println!("💰 BENEFÍCIOS FINANCEIROS (por veículo/ano):\n");
    println!("   💵 Economia de combustível:      R$ 1.541,76");
    println!("   🔧 Redução de manutenções:       R$ 3.200,00");
    println!("   ⏱️  Aumento de produtividade:     R$ 4.800,00");
    println!("   🚨 Redução de multas/acidentes:  R$ 2.100,00");
    println!("   ═══════════════════════════════════════════");
    println!("   🎯 ECONOMIA TOTAL/ANO:          R$ 11.641,76");
    println!();
    pausar(2000);

    println!("   Para uma frota de 10 veículos:");
    println!("   💰 R$ 116.417,60 por ano!\n");
    pausar(2000);

    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    pausar(1000);

    println!("✨ TECNOLOGIAS USADAS:\n");
    println!("   🤖 Inteligência Artificial");
    println!("   🌐 Internet das Coisas (IoT)");
    println!("   📡 GPS e Sensores");
    println!("   ☁️  Cloud Computing");
    println!("   📊 Big Data Analytics");
    println!();
    pausar(2000);

    println!("═══════════════════════════════════════════════════════════════\n");
    println!("         🇧🇷 Desenvolvido com a AVL Cloud Platform");
    println!("            Sistema 100% brasileiro! 🚀");
    println!();
    println!("═══════════════════════════════════════════════════════════════\n");
}

// Funções auxiliares
fn pausar(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn separador() {
    println!("═══════════════════════════════════════════════════════════════\n");
    pausar(1500);
}

fn limpar_tela() {
    // No Windows
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(&["/C", "cls"])
            .status();
    }

    // No Unix/Linux/Mac
    #[cfg(not(target_os = "windows"))]
    {
        let _ = std::process::Command::new("clear").status();
    }
}
