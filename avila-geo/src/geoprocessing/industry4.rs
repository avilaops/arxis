//! Indústria 4.0 - IoT Espacial e Digital Twins
//!
//! Este módulo implementa funcionalidades para:
//! - Digital Twins de ativos geográficos
//! - Monitoramento IoT em tempo real
//! - Análise preditiva espacial
//! - Otimização de rotas e logística
//! - Manutenção preditiva baseada em localização

use crate::coords::GeoCoord;
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Timestamp Unix em milissegundos
pub type Timestamp = u64;

/// Identificador único de dispositivo IoT
pub type DeviceId = String;

/// Identificador único de ativo (Digital Twin)
pub type AssetId = String;

/// Estado de saúde de um ativo
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HealthStatus {
    Healthy,      // Funcionamento normal
    Warning,      // Necessita atenção
    Critical,     // Falha iminente
    Offline,      // Desconectado
    Maintenance,  // Em manutenção
}

/// Tipo de sensor IoT
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum SensorType {
    Temperature,
    Humidity,
    Pressure,
    Vibration,
    Speed,
    FuelLevel,
    BatteryLevel,
    GPS,
    Accelerometer,
    Custom(String),
}

/// Leitura de sensor IoT
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SensorReading {
    pub sensor_type: SensorType,
    pub value: f64,
    pub unit: String,
    pub timestamp: Timestamp,
    pub location: Option<GeoCoord>,
}

impl SensorReading {
    pub fn new(sensor_type: SensorType, value: f64, unit: String) -> Self {
        Self {
            sensor_type,
            value,
            unit,
            timestamp: current_timestamp(),
            location: None,
        }
    }

    pub fn with_location(mut self, location: GeoCoord) -> Self {
        self.location = Some(location);
        self
    }
}

/// Dispositivo IoT com múltiplos sensores
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IoTDevice {
    pub id: DeviceId,
    pub name: String,
    pub device_type: String,
    pub location: GeoCoord,
    pub last_seen: Timestamp,
    pub health: HealthStatus,
    pub readings: Vec<SensorReading>,
}

impl IoTDevice {
    pub fn new(id: DeviceId, name: String, device_type: String, location: GeoCoord) -> Self {
        Self {
            id,
            name,
            device_type,
            location,
            last_seen: current_timestamp(),
            health: HealthStatus::Healthy,
            readings: Vec::new(),
        }
    }

    /// Adiciona uma nova leitura de sensor
    pub fn add_reading(&mut self, reading: SensorReading) {
        self.last_seen = reading.timestamp;
        self.readings.push(reading);

        // Manter apenas últimas 1000 leituras para economizar memória
        if self.readings.len() > 1000 {
            self.readings.drain(0..self.readings.len() - 1000);
        }
    }

    /// Obtém a última leitura de um tipo de sensor
    pub fn last_reading(&self, sensor_type: &SensorType) -> Option<&SensorReading> {
        self.readings
            .iter()
            .rev()
            .find(|r| &r.sensor_type == sensor_type)
    }

    /// Verifica se o dispositivo está online (última leitura < 5 minutos)
    pub fn is_online(&self) -> bool {
        let now = current_timestamp();
        let threshold = 5 * 60 * 1000; // 5 minutos em ms
        now - self.last_seen < threshold
    }

    /// Atualiza localização do dispositivo
    pub fn update_location(&mut self, location: GeoCoord) {
        self.location = location;
        self.last_seen = current_timestamp();
    }
}

/// Digital Twin de um ativo industrial ou veículo
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DigitalTwin {
    pub id: AssetId,
    pub name: String,
    pub asset_type: String,
    pub location: GeoCoord,
    pub health: HealthStatus,
    pub devices: Vec<DeviceId>,
    pub properties: HashMap<String, String>,
    pub maintenance_history: Vec<MaintenanceEvent>,
    pub predicted_failure_time: Option<Timestamp>,
}

impl DigitalTwin {
    pub fn new(id: AssetId, name: String, asset_type: String, location: GeoCoord) -> Self {
        Self {
            id,
            name,
            asset_type,
            location,
            health: HealthStatus::Healthy,
            devices: Vec::new(),
            properties: HashMap::new(),
            maintenance_history: Vec::new(),
            predicted_failure_time: None,
        }
    }

    /// Adiciona um dispositivo IoT ao ativo
    pub fn attach_device(&mut self, device_id: DeviceId) {
        if !self.devices.contains(&device_id) {
            self.devices.push(device_id);
        }
    }

    /// Define propriedade customizada
    pub fn set_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    /// Registra evento de manutenção
    pub fn record_maintenance(&mut self, event: MaintenanceEvent) {
        self.maintenance_history.push(event);

        // Após manutenção, reseta previsão de falha
        if event.event_type == MaintenanceType::Corrective
            || event.event_type == MaintenanceType::Preventive {
            self.predicted_failure_time = None;
            self.health = HealthStatus::Healthy;
        }
    }

    /// Calcula tempo desde última manutenção (em dias)
    pub fn days_since_maintenance(&self) -> Option<f64> {
        self.maintenance_history
            .last()
            .map(|event| {
                let now = current_timestamp();
                let diff_ms = now - event.timestamp;
                diff_ms as f64 / (1000.0 * 60.0 * 60.0 * 24.0)
            })
    }
}

/// Tipo de manutenção
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MaintenanceType {
    Preventive,   // Manutenção preventiva
    Corrective,   // Manutenção corretiva
    Predictive,   // Manutenção preditiva
    Emergency,    // Emergência
}

/// Evento de manutenção
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MaintenanceEvent {
    pub event_type: MaintenanceType,
    pub timestamp: Timestamp,
    pub description: String,
    pub cost: Option<f64>,
    pub duration_hours: f64,
}

impl MaintenanceEvent {
    pub fn new(event_type: MaintenanceType, description: String, duration_hours: f64) -> Self {
        Self {
            event_type,
            timestamp: current_timestamp(),
            description,
            cost: None,
            duration_hours,
        }
    }

    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost = Some(cost);
        self
    }
}

/// Sistema de gerenciamento de frota/ativos (Fleet Management)
#[derive(Debug)]
pub struct FleetManager {
    pub devices: HashMap<DeviceId, IoTDevice>,
    pub assets: HashMap<AssetId, DigitalTwin>,
    pub geofences: Vec<Geofence>,
}

impl FleetManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
            assets: HashMap::new(),
            geofences: Vec::new(),
        }
    }

    /// Registra um novo dispositivo IoT
    pub fn register_device(&mut self, device: IoTDevice) {
        self.devices.insert(device.id.clone(), device);
    }

    /// Registra um novo ativo (Digital Twin)
    pub fn register_asset(&mut self, asset: DigitalTwin) {
        self.assets.insert(asset.id.clone(), asset);
    }

    /// Processa leitura de sensor em tempo real
    pub fn process_sensor_reading(&mut self, device_id: &DeviceId, reading: SensorReading) {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.add_reading(reading);

            // Atualizar localização se for leitura GPS
            if let Some(location) = device.readings.last().and_then(|r| r.location) {
                device.update_location(location);
            }
        }
    }

    /// Obtém todos os dispositivos online
    pub fn online_devices(&self) -> Vec<&IoTDevice> {
        self.devices
            .values()
            .filter(|d| d.is_online())
            .collect()
    }

    /// Obtém dispositivos offline
    pub fn offline_devices(&self) -> Vec<&IoTDevice> {
        self.devices
            .values()
            .filter(|d| !d.is_online())
            .collect()
    }

    /// Obtém ativos com saúde crítica
    pub fn critical_assets(&self) -> Vec<&DigitalTwin> {
        self.assets
            .values()
            .filter(|a| a.health == HealthStatus::Critical)
            .collect()
    }

    /// Adiciona uma geofence (cerca virtual)
    pub fn add_geofence(&mut self, geofence: Geofence) {
        self.geofences.push(geofence);
    }

    /// Verifica violações de geofence
    pub fn check_geofence_violations(&self) -> Vec<GeofenceViolation> {
        let mut violations = Vec::new();

        for device in self.devices.values() {
            for geofence in &self.geofences {
                if !geofence.contains(&device.location) && geofence.rule == GeofenceRule::MustStayInside {
                    violations.push(GeofenceViolation {
                        device_id: device.id.clone(),
                        geofence_name: geofence.name.clone(),
                        location: device.location,
                        timestamp: current_timestamp(),
                    });
                } else if geofence.contains(&device.location) && geofence.rule == GeofenceRule::MustStayOutside {
                    violations.push(GeofenceViolation {
                        device_id: device.id.clone(),
                        geofence_name: geofence.name.clone(),
                        location: device.location,
                        timestamp: current_timestamp(),
                    });
                }
            }
        }

        violations
    }

    /// Análise preditiva: identifica ativos que precisam de manutenção
    pub fn predictive_maintenance_analysis(&mut self) -> Vec<MaintenanceRecommendation> {
        let mut recommendations = Vec::new();

        for asset in self.assets.values() {
            // Verificar tempo desde última manutenção
            if let Some(days) = asset.days_since_maintenance() {
                if days > 90.0 && asset.health != HealthStatus::Maintenance {
                    recommendations.push(MaintenanceRecommendation {
                        asset_id: asset.id.clone(),
                        asset_name: asset.name.clone(),
                        priority: if days > 120.0 { Priority::High } else { Priority::Medium },
                        reason: format!("{:.0} dias desde última manutenção", days),
                        estimated_cost: Some(5000.0),
                    });
                }
            }

            // Verificar dispositivos críticos
            let critical_devices = asset.devices.iter()
                .filter_map(|device_id| self.devices.get(device_id))
                .filter(|d| d.health == HealthStatus::Critical)
                .count();

            if critical_devices > 0 {
                recommendations.push(MaintenanceRecommendation {
                    asset_id: asset.id.clone(),
                    asset_name: asset.name.clone(),
                    priority: Priority::Critical,
                    reason: format!("{} dispositivo(s) crítico(s)", critical_devices),
                    estimated_cost: Some(10000.0),
                });
            }
        }

        recommendations
    }

    /// Estatísticas da frota
    pub fn fleet_statistics(&self) -> FleetStatistics {
        let total_devices = self.devices.len();
        let online = self.online_devices().len();
        let offline = self.offline_devices().len();

        let total_assets = self.assets.len();
        let healthy = self.assets.values().filter(|a| a.health == HealthStatus::Healthy).count();
        let critical = self.assets.values().filter(|a| a.health == HealthStatus::Critical).count();
        let maintenance = self.assets.values().filter(|a| a.health == HealthStatus::Maintenance).count();

        FleetStatistics {
            total_devices,
            online_devices: online,
            offline_devices: offline,
            total_assets,
            healthy_assets: healthy,
            critical_assets: critical,
            maintenance_assets: maintenance,
            geofences_count: self.geofences.len(),
        }
    }
}

impl Default for FleetManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Geofence (cerca virtual geográfica)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Geofence {
    pub name: String,
    pub polygon: Vec<GeoCoord>,
    pub rule: GeofenceRule,
}

impl Geofence {
    pub fn new(name: String, polygon: Vec<GeoCoord>, rule: GeofenceRule) -> Self {
        Self { name, polygon, rule }
    }

    /// Verifica se um ponto está dentro do polígono (ray casting)
    pub fn contains(&self, point: &GeoCoord) -> bool {
        use crate::geoprocessing::operations::point_in_polygon;
        point_in_polygon(point, &self.polygon)
    }
}

/// Regra de geofence
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GeofenceRule {
    MustStayInside,   // Dispositivo deve permanecer dentro
    MustStayOutside,  // Dispositivo deve permanecer fora
    Notification,     // Apenas notificar quando entrar/sair
}

/// Violação de geofence
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GeofenceViolation {
    pub device_id: DeviceId,
    pub geofence_name: String,
    pub location: GeoCoord,
    pub timestamp: Timestamp,
}

/// Recomendação de manutenção
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MaintenanceRecommendation {
    pub asset_id: AssetId,
    pub asset_name: String,
    pub priority: Priority,
    pub reason: String,
    pub estimated_cost: Option<f64>,
}

/// Prioridade de manutenção
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

/// Estatísticas da frota
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FleetStatistics {
    pub total_devices: usize,
    pub online_devices: usize,
    pub offline_devices: usize,
    pub total_assets: usize,
    pub healthy_assets: usize,
    pub critical_assets: usize,
    pub maintenance_assets: usize,
    pub geofences_count: usize,
}

/// Retorna o timestamp atual em milissegundos
pub fn current_timestamp() -> Timestamp {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis() as u64
}

/// Otimização de rotas para múltiplos veículos (Vehicle Routing Problem)
pub struct RouteOptimizer {
    pub vehicles: Vec<Vehicle>,
    pub waypoints: Vec<Waypoint>,
}

impl RouteOptimizer {
    pub fn new() -> Self {
        Self {
            vehicles: Vec::new(),
            waypoints: Vec::new(),
        }
    }

    /// Adiciona um veículo
    pub fn add_vehicle(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    /// Adiciona um ponto de parada
    pub fn add_waypoint(&mut self, waypoint: Waypoint) {
        self.waypoints.push(waypoint);
    }

    /// Otimiza rotas usando algoritmo guloso (nearest neighbor)
    pub fn optimize_routes(&self) -> Vec<Route> {
        let mut routes = Vec::new();
        let mut unvisited: Vec<_> = (0..self.waypoints.len()).collect();

        for vehicle in &self.vehicles {
            let mut route = Route {
                vehicle_id: vehicle.id.clone(),
                waypoints: Vec::new(),
                total_distance: 0.0,
                total_time: 0.0,
            };

            let mut current_location = vehicle.current_location;

            while !unvisited.is_empty() {
                // Encontrar waypoint mais próximo
                let (idx, nearest_idx) = unvisited.iter()
                    .enumerate()
                    .min_by(|(_, &a), (_, &b)| {
                        let dist_a = haversine_distance(&current_location, &self.waypoints[a].location);
                        let dist_b = haversine_distance(&current_location, &self.waypoints[b].location);
                        dist_a.partial_cmp(&dist_b).unwrap()
                    })
                    .unwrap();

                let waypoint_idx = *nearest_idx;
                let distance = haversine_distance(&current_location, &self.waypoints[waypoint_idx].location);

                route.waypoints.push(waypoint_idx);
                route.total_distance += distance;
                route.total_time += distance / (vehicle.avg_speed * 1000.0 / 3600.0); // segundos

                current_location = self.waypoints[waypoint_idx].location;
                unvisited.remove(idx);

                // Respeitar capacidade do veículo
                if route.waypoints.len() >= vehicle.capacity {
                    break;
                }
            }

            routes.push(route);

            if unvisited.is_empty() {
                break;
            }
        }

        routes
    }
}

impl Default for RouteOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Veículo para otimização de rotas
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vehicle {
    pub id: String,
    pub name: String,
    pub current_location: GeoCoord,
    pub capacity: usize,
    pub avg_speed: f64, // km/h
}

/// Ponto de parada (waypoint)
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Waypoint {
    pub id: String,
    pub location: GeoCoord,
    pub priority: Priority,
    pub service_time: f64, // minutos
}

/// Rota otimizada
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Route {
    pub vehicle_id: String,
    pub waypoints: Vec<usize>,
    pub total_distance: f64,
    pub total_time: f64,
}

/// Calcula distância haversine entre dois pontos
fn haversine_distance(a: &GeoCoord, b: &GeoCoord) -> f64 {
    use crate::geoprocessing::analysis::haversine_distance as hav;
    hav(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iot_device() {
        let location = GeoCoord::new(-23.5505, -46.6333);
        let mut device = IoTDevice::new(
            "device001".to_string(),
            "Sensor Temperatura".to_string(),
            "temperature".to_string(),
            location,
        );

        let reading = SensorReading::new(
            SensorType::Temperature,
            25.5,
            "°C".to_string(),
        );

        device.add_reading(reading);
        assert_eq!(device.readings.len(), 1);
        assert!(device.is_online());
    }

    #[test]
    fn test_digital_twin() {
        let location = GeoCoord::new(-23.5505, -46.6333);
        let mut twin = DigitalTwin::new(
            "asset001".to_string(),
            "Máquina CNC".to_string(),
            "cnc".to_string(),
            location,
        );

        twin.attach_device("device001".to_string());
        twin.set_property("modelo".to_string(), "XYZ-1000".to_string());

        assert_eq!(twin.devices.len(), 1);
        assert_eq!(twin.properties.get("modelo").unwrap(), "XYZ-1000");
    }

    #[test]
    fn test_fleet_manager() {
        let mut fleet = FleetManager::new();

        let device = IoTDevice::new(
            "truck001".to_string(),
            "Caminhão 1".to_string(),
            "vehicle".to_string(),
            GeoCoord::new(-23.5505, -46.6333),
        );

        fleet.register_device(device);
        assert_eq!(fleet.devices.len(), 1);
        assert_eq!(fleet.online_devices().len(), 1);
    }

    #[test]
    fn test_geofence() {
        let polygon = vec![
            GeoCoord::new(-23.5, -46.7),
            GeoCoord::new(-23.5, -46.5),
            GeoCoord::new(-23.6, -46.5),
            GeoCoord::new(-23.6, -46.7),
        ];

        let geofence = Geofence::new(
            "Zona SP".to_string(),
            polygon,
            GeofenceRule::MustStayInside,
        );

        let inside = GeoCoord::new(-23.55, -46.6);
        let outside = GeoCoord::new(-23.7, -46.8);

        assert!(geofence.contains(&inside));
        assert!(!geofence.contains(&outside));
    }
}
