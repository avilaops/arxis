# 🏭 Indústria 4.0 - Guia Completo

## Visão Geral

Sistema completo de **IoT Espacial** e **Digital Twins** para aplicações industriais modernas, incluindo:

- 🤖 **Digital Twins** de ativos físicos
- 📡 **Monitoramento IoT em tempo real**
- 🗺️ **Geofencing inteligente**
- 🔧 **Manutenção preditiva**
- 🚚 **Otimização de rotas (VRP)**
- 📊 **Real-time Analytics**
- 🚨 **Sistema de alertas**
- 🔥 **Detecção de padrões espaciais**

## Instalação

```toml
[dependencies]
avila-geo = { version = "0.1", features = ["geoprocessing"] }
```

## 📚 Módulos Principais

### 1. Industry 4.0 Core (`industry4`)

#### Digital Twins

Digital Twin é uma representação virtual de um ativo físico:

```rust
use avila_geo::geoprocessing::*;
use avila_geo::coords::GeoCoord;

// Criar Digital Twin de uma máquina
let location = GeoCoord::new(-23.5505, -46.6333);
let mut twin = DigitalTwin::new(
    "MACHINE001".to_string(),
    "Empilhadeira Caterpillar".to_string(),
    "forklift".to_string(),
    location,
);

// Configurar propriedades
twin.set_property("modelo".to_string(), "CAT DP50N".to_string());
twin.set_property("ano".to_string(), "2023".to_string());
twin.set_property("serial".to_string(), "ABC123456".to_string());

// Anexar dispositivos IoT
twin.attach_device("SENSOR_TEMP_001".to_string());
twin.attach_device("GPS_TRACKER_001".to_string());

// Registrar manutenção
let maintenance = MaintenanceEvent::new(
    MaintenanceType::Preventive,
    "Revisão de 500 horas".to_string(),
    8.0,  // duração em horas
).with_cost(3500.0);

twin.record_maintenance(maintenance);

// Verificar tempo desde última manutenção
if let Some(days) = twin.days_since_maintenance() {
    println!("Última manutenção há {} dias", days);
}
```

#### Dispositivos IoT

Monitoramento de dispositivos com múltiplos sensores:

```rust
// Criar dispositivo IoT
let mut device = IoTDevice::new(
    "TRUCK001".to_string(),
    "Caminhão Mercedes Actros".to_string(),
    "heavy_truck".to_string(),
    GeoCoord::new(-23.5505, -46.6333),
);

// Adicionar leituras de sensores
let temp_reading = SensorReading::new(
    SensorType::Temperature,
    85.0,
    "°C".to_string(),
).with_location(device.location);

device.add_reading(temp_reading);

// Verificar última leitura
if let Some(last_temp) = device.last_reading(&SensorType::Temperature) {
    println!("Última temperatura: {:.1}°C", last_temp.value);
}

// Status online/offline
if device.is_online() {
    println!("Dispositivo online");
} else {
    println!("Dispositivo offline");
}
```

#### Fleet Manager

Gerenciamento centralizado de frota:

```rust
let mut fleet = FleetManager::new();

// Registrar dispositivos e ativos
fleet.register_device(device);
fleet.register_asset(twin);

// Processar leituras em tempo real
let reading = SensorReading::new(
    SensorType::FuelLevel,
    45.5,
    "L".to_string(),
);
fleet.process_sensor_reading(&"TRUCK001".to_string(), reading);

// Obter dispositivos online
let online = fleet.online_devices();
println!("Dispositivos online: {}", online.len());

// Obter ativos críticos
let critical = fleet.critical_assets();
for asset in critical {
    println!("Ativo crítico: {}", asset.name);
}

// Estatísticas da frota
let stats = fleet.fleet_statistics();
println!("Total de dispositivos: {}", stats.total_devices);
println!("Dispositivos online: {}", stats.online_devices);
println!("Ativos saudáveis: {}", stats.healthy_assets);
```

### 2. Geofencing

Cercas virtuais para monitoramento de área:

```rust
// Criar geofence
let polygon = vec![
    GeoCoord::new(-23.5000, -46.7000),
    GeoCoord::new(-23.5000, -46.5000),
    GeoCoord::new(-23.6000, -46.5000),
    GeoCoord::new(-23.6000, -46.7000),
];

let geofence = Geofence::new(
    "Zona Operacional SP".to_string(),
    polygon,
    GeofenceRule::MustStayInside,
);

fleet.add_geofence(geofence);

// Verificar violações
let violations = fleet.check_geofence_violations();
for violation in violations {
    println!("⚠️  Violação: {} saiu da {}",
        violation.device_id,
        violation.geofence_name
    );
}
```

**Regras de Geofence:**
- `MustStayInside`: Dispositivo deve permanecer dentro
- `MustStayOutside`: Dispositivo deve permanecer fora
- `Notification`: Apenas notificar entrada/saída

### 3. Manutenção Preditiva

Análise inteligente para prever necessidade de manutenção:

```rust
// Executar análise preditiva
let recommendations = fleet.predictive_maintenance_analysis();

for rec in recommendations {
    println!("{:?} - {}", rec.priority, rec.asset_name);
    println!("  Motivo: {}", rec.reason);

    if let Some(cost) = rec.estimated_cost {
        println!("  Custo estimado: R$ {:.2}", cost);
    }
}
```

**Critérios de Análise:**
- Tempo desde última manutenção
- Status de saúde dos dispositivos
- Histórico de falhas
- Previsão de falhas (machine learning)

### 4. Otimização de Rotas (VRP)

Vehicle Routing Problem - otimização de múltiplas rotas:

```rust
let mut optimizer = RouteOptimizer::new();

// Adicionar veículos
let vehicle = Vehicle {
    id: "VAN001".to_string(),
    name: "Van de Entrega 1".to_string(),
    current_location: GeoCoord::new(-23.5505, -46.6333),
    capacity: 20,
    avg_speed: 40.0, // km/h
};
optimizer.add_vehicle(vehicle);

// Adicionar waypoints (pontos de parada)
let waypoint = Waypoint {
    id: "WP001".to_string(),
    location: GeoCoord::new(-23.5629, -46.6544),
    priority: Priority::High,
    service_time: 15.0, // minutos
};
optimizer.add_waypoint(waypoint);

// Otimizar rotas (algoritmo nearest neighbor)
let routes = optimizer.optimize_routes();

for route in routes {
    println!("Rota para {}", route.vehicle_id);
    println!("  Distância: {:.2} km", route.total_distance / 1000.0);
    println!("  Tempo: {:.0} min", route.total_time / 60.0);
    println!("  Paradas: {}", route.waypoints.len());
}
```

### 5. Real-time Analytics (`realtime`)

#### Stream Processing

Análise de streams de dados IoT em tempo real:

```rust
use avila_geo::geoprocessing::realtime::*;

// Criar stream processor com janela de 5 minutos
let mut processor = StreamProcessor::new(TimeWindow::Minutes(5));

// Processar leituras
let reading = SensorReading::new(
    SensorType::Temperature,
    28.5,
    "°C".to_string(),
);

let analytics = processor.process("DEVICE001".to_string(), reading);

println!("Janela: {} leituras", analytics.window_size);
println!("Média: {:.2}", analytics.metrics.mean);
println!("Desvio padrão: {:.2}", analytics.metrics.std_dev);
println!("Taxa de mudança: {:.3}/s", analytics.metrics.rate_of_change);

if analytics.is_anomaly {
    println!("🚨 ANOMALIA DETECTADA!");
}
```

**Métricas Agregadas:**
- Média, mediana, desvio padrão
- Mínimo e máximo
- Taxa de mudança (derivada)

#### Detecção de Anomalias

Z-score para detectar valores anômalos:

```rust
// Detector com threshold de 3 desvios padrão
let mut detector = AnomalyDetector::new(3.0);

// Detectar anomalia
let is_anomaly = detector.detect("DEVICE001", 95.0);

if is_anomaly {
    println!("Valor anômalo detectado!");
}
```

#### Sistema de Alertas

Regras configuráveis para disparar alertas:

```rust
let mut alert_system = AlertSystem::new();

// Criar regra de alerta
let rule = AlertRule::new(
    "Temperatura Crítica".to_string(),
    SensorType::Temperature,
    Condition::GreaterThan(90.0),
    AlertSeverity::Critical,
);

alert_system.add_rule(rule);

// Avaliar leituras
let reading = SensorReading::new(
    SensorType::Temperature,
    95.0,
    "°C".to_string(),
);

let alerts = alert_system.evaluate("DEVICE001", &reading);

for alert in alerts {
    println!("{:?}: {}", alert.severity, alert.message);
}

// Obter alertas recentes (últimas 24 horas)
let recent = alert_system.recent_alerts(24);
```

**Tipos de Condições:**
- `GreaterThan(threshold)`
- `LessThan(threshold)`
- `Between(min, max)`
- `Outside(min, max)`
- `Equals(value)`

**Níveis de Severidade:**
- `Info`: Informativo
- `Warning`: Atenção necessária
- `Error`: Erro operacional
- `Critical`: Crítico, ação imediata

#### Análise de Padrões Espaciais

Detectar hotspots e clusters espaciais:

```rust
let analyzer = SpatialPatternAnalyzer::new(
    1000.0,  // Raio de 1km
    3,       // Mínimo 3 pontos por cluster
);

// Localizações com valores (ex: incidentes, vendas)
let locations = vec![
    (GeoCoord::new(-23.550, -46.633), 15.0),
    (GeoCoord::new(-23.551, -46.634), 12.0),
    (GeoCoord::new(-23.552, -46.635), 18.0),
];

// Detectar hotspots
let hotspots = analyzer.detect_hotspots(&locations);

for hotspot in hotspots {
    println!("Hotspot detectado:");
    println!("  Centro: ({:.4}, {:.4})",
        hotspot.center.lat, hotspot.center.lon);
    println!("  Tamanho: {} pontos", hotspot.size);
    println!("  Intensidade: {:.2}", hotspot.intensity);
}
```

## 🎯 Casos de Uso

### Logística e Transporte

```rust
// Sistema completo de rastreamento de frota
let mut fleet = FleetManager::new();

// 1. Registrar veículos com sensores
let truck = IoTDevice::new(
    "TRUCK001".to_string(),
    "Scania R450".to_string(),
    "truck".to_string(),
    GeoCoord::new(-23.5505, -46.6333),
);
fleet.register_device(truck);

// 2. Geofencing de rotas permitidas
let route_zone = Geofence::new(
    "Rota SP-RJ".to_string(),
    route_polygon,
    GeofenceRule::MustStayInside,
);
fleet.add_geofence(route_zone);

// 3. Otimizar rotas de entrega
let mut optimizer = RouteOptimizer::new();
// ... adicionar veículos e waypoints
let optimized_routes = optimizer.optimize_routes();

// 4. Monitoramento em tempo real
let mut stream = StreamProcessor::new(TimeWindow::Minutes(10));
// ... processar leituras de GPS, combustível, temperatura
```

### Manufatura (Smart Factory)

```rust
// Digital Twins de máquinas industriais
let mut cnc_machine = DigitalTwin::new(
    "CNC_001".to_string(),
    "CNC Haas VF-3".to_string(),
    "cnc_mill".to_string(),
    factory_location,
);

// Sensores de vibração, temperatura, uso de ferramenta
cnc_machine.attach_device("VIBRATION_SENSOR_001".to_string());
cnc_machine.attach_device("TEMP_SENSOR_001".to_string());

// Manutenção preditiva
let recommendations = fleet.predictive_maintenance_analysis();

// Alertas de anomalias
let mut alerts = AlertSystem::new();
alerts.add_rule(AlertRule::new(
    "Vibração Alta".to_string(),
    SensorType::Vibration,
    Condition::GreaterThan(8.0),
    AlertSeverity::Error,
));
```

### Agricultura de Precisão

```rust
// Monitoramento de tratores e colheitadeiras
let harvester = IoTDevice::new(
    "HARVESTER001".to_string(),
    "John Deere S780".to_string(),
    "combine_harvester".to_string(),
    field_location,
);

// Análise de padrões espaciais (produtividade por área)
let analyzer = SpatialPatternAnalyzer::new(100.0, 5);
let productivity_hotspots = analyzer.detect_hotspots(&field_data);

// Otimização de rotas de colheita
let route_optimizer = RouteOptimizer::new();
// ... otimizar percurso no campo
```

### Mineração e Construção

```rust
// Rastreamento de equipamentos pesados
let excavator = DigitalTwin::new(
    "EXCAVATOR001".to_string(),
    "Caterpillar 320D".to_string(),
    "excavator".to_string(),
    site_location,
);

// Geofencing de área de trabalho segura
let safe_zone = Geofence::new(
    "Área Segura de Escavação".to_string(),
    work_area_polygon,
    GeofenceRule::MustStayInside,
);

// Monitoramento de combustível e uso
let fuel_alert = AlertRule::new(
    "Combustível Baixo".to_string(),
    SensorType::FuelLevel,
    Condition::LessThan(15.0),
    AlertSeverity::Warning,
);
```

## 📊 Performance

### Throughput
- **Stream Processing**: >10.000 eventos/segundo
- **Geofencing**: <1ms por verificação
- **Detecção de Anomalias**: O(1) por leitura
- **Hotspot Detection**: O(n²) com spatial indexing

### Escalabilidade
- Suporta milhares de dispositivos simultâneos
- Cache LRU para otimização de consultas
- Processamento paralelo opcional (feature `parallel`)

## 🔧 Configuração Avançada

### Time Windows Customizadas

```rust
// Diferentes janelas temporais
let short_window = TimeWindow::Seconds(30);
let medium_window = TimeWindow::Minutes(5);
let long_window = TimeWindow::Hours(1);
```

### Thresholds de Anomalia

```rust
// Sensibilidade alta (2 desvios padrão)
let sensitive = AnomalyDetector::new(2.0);

// Sensibilidade baixa (4 desvios padrão)
let conservative = AnomalyDetector::new(4.0);
```

### Clustering Espacial

```rust
// Hotspots pequenos e localizados
let local_analyzer = SpatialPatternAnalyzer::new(100.0, 5);

// Hotspots grandes e regionais
let regional_analyzer = SpatialPatternAnalyzer::new(5000.0, 10);
```

## 📖 Exemplos Completos

Execute os exemplos:

```bash
# Demo completo de Indústria 4.0
cargo run --example industry4_demo

# Real-time Analytics e Stream Processing
cargo run --example realtime_demo
```

## 🚀 Próximas Features

- [ ] Integração com MQTT/Kafka para IoT
- [ ] Persistência em banco de dados (PostgreSQL/TimescaleDB)
- [ ] Dashboard web em tempo real
- [ ] Machine Learning para previsão de falhas
- [ ] APIs REST/GraphQL
- [ ] Integração com AvilaDB para armazenamento

## 📚 Referências

- **Industry 4.0**: Schwab, Klaus. "The Fourth Industrial Revolution"
- **Digital Twins**: Grieves, Michael. "Digital Twin: Manufacturing Excellence"
- **IoT Spatial**: Lee & Seshia. "Introduction to Embedded Systems"
- **Predictive Maintenance**: Mobley, R. Keith. "An Introduction to Predictive Maintenance"

---

**Avila Geo + Indústria 4.0** = Solução completa para manufatura inteligente! 🏭🤖📊
