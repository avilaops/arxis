use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Dispositivo IoT Industrial
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTDevice {
    pub device_id: String,
    pub device_type: DeviceType,
    pub location: Location,
    pub status: DeviceStatus,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Sensor,
    Actuator,
    Machine,
    Robot,
    Conveyor,
    PLC,
    Gateway,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub facility: String,
    pub area: String,
    pub line: String,
    pub station: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceStatus {
    Online,
    Offline,
    Maintenance,
    Error,
    Warning,
}

/// Dados de sensor em tempo real
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorData {
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub readings: HashMap<String, SensorReading>,
    pub machine_status: MachineStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub value: f64,
    pub unit: String,
    pub quality: ReadingQuality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReadingQuality {
    Good,
    Uncertain,
    Bad,
}

/// Status de máquina
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MachineStatus {
    Idle,
    Running,
    Stopped,
    Maintenance,
    Error,
    Setup,
    Changeover,
}

/// Evento de máquina
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineEvent {
    pub event_id: String,
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: MachineEventType,
    pub severity: EventSeverity,
    pub description: String,
    pub parameters: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MachineEventType {
    StartProduction,
    StopProduction,
    ErrorOccurred,
    MaintenanceRequired,
    QualityIssue,
    MaterialShortage,
    ToolChange,
    Alarm,
    ParameterChange,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Telemetria de produção
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionTelemetry {
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub production_count: u64,
    pub reject_count: u64,
    pub cycle_time_ms: u64,
    pub temperature_c: f64,
    pub vibration_mm_s: f64,
    pub power_consumption_kw: f64,
    pub pressure_bar: f64,
    pub speed_rpm: f64,
}

impl ProductionTelemetry {
    /// Verificar se os parâmetros estão dentro dos limites
    pub fn is_within_limits(&self, limits: &OperatingLimits) -> bool {
        self.temperature_c >= limits.temp_min && self.temperature_c <= limits.temp_max
            && self.vibration_mm_s <= limits.vibration_max
            && self.pressure_bar >= limits.pressure_min && self.pressure_bar <= limits.pressure_max
            && self.speed_rpm >= limits.speed_min && self.speed_rpm <= limits.speed_max
    }

    /// Calcular score de saúde (0.0 a 1.0)
    pub fn health_score(&self, limits: &OperatingLimits) -> f64 {
        let mut score = 1.0;

        // Penalizar por temperatura
        let temp_deviation = (self.temperature_c - limits.temp_optimal).abs()
            / (limits.temp_max - limits.temp_min);
        score -= temp_deviation * 0.3;

        // Penalizar por vibração
        let vibration_ratio = self.vibration_mm_s / limits.vibration_max;
        score -= vibration_ratio * 0.3;

        // Penalizar por pressão
        let pressure_deviation = (self.pressure_bar - limits.pressure_optimal).abs()
            / (limits.pressure_max - limits.pressure_min);
        score -= pressure_deviation * 0.2;

        // Penalizar por velocidade
        let speed_deviation = (self.speed_rpm - limits.speed_optimal).abs()
            / (limits.speed_max - limits.speed_min);
        score -= speed_deviation * 0.2;

        score.max(0.0).min(1.0)
    }
}

/// Limites operacionais
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatingLimits {
    pub temp_min: f64,
    pub temp_max: f64,
    pub temp_optimal: f64,
    pub vibration_max: f64,
    pub pressure_min: f64,
    pub pressure_max: f64,
    pub pressure_optimal: f64,
    pub speed_min: f64,
    pub speed_max: f64,
    pub speed_optimal: f64,
}

impl Default for OperatingLimits {
    fn default() -> Self {
        Self {
            temp_min: 20.0,
            temp_max: 80.0,
            temp_optimal: 50.0,
            vibration_max: 10.0,
            pressure_min: 5.0,
            pressure_max: 15.0,
            pressure_optimal: 10.0,
            speed_min: 100.0,
            speed_max: 3000.0,
            speed_optimal: 1500.0,
        }
    }
}

/// Sistema de ingestão de dados IoT
pub struct IoTDataIngester {
    devices: HashMap<String, IoTDevice>,
    buffer: Vec<SensorData>,
    buffer_size: usize,
}

impl IoTDataIngester {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            devices: HashMap::new(),
            buffer: Vec::with_capacity(buffer_size),
            buffer_size,
        }
    }

    /// Registrar dispositivo
    pub fn register_device(&mut self, device: IoTDevice) {
        self.devices.insert(device.device_id.clone(), device);
    }

    /// Ingerir dados de sensor
    pub fn ingest(&mut self, data: SensorData) {
        self.buffer.push(data);

        // Flush se buffer estiver cheio
        if self.buffer.len() >= self.buffer_size {
            self.flush();
        }
    }

    /// Flush buffer para storage
    pub fn flush(&mut self) {
        // Em produção, enviar para AvilaDB
        tracing::info!("Flushing {} sensor readings to storage", self.buffer.len());
        self.buffer.clear();
    }

    /// Obter status de todos os dispositivos
    pub fn get_devices_status(&self) -> Vec<(String, DeviceStatus)> {
        self.devices
            .iter()
            .map(|(id, device)| (id.clone(), device.status.clone()))
            .collect()
    }

    /// Obter dispositivos offline
    pub fn get_offline_devices(&self) -> Vec<String> {
        self.devices
            .iter()
            .filter(|(_, device)| device.status == DeviceStatus::Offline)
            .map(|(id, _)| id.clone())
            .collect()
    }
}

/// Simulador de dados de sensores
pub struct SensorSimulator {
    device_id: String,
    limits: OperatingLimits,
}

impl SensorSimulator {
    pub fn new(device_id: String) -> Self {
        Self {
            device_id,
            limits: OperatingLimits::default(),
        }
    }

    /// Gerar telemetria simulada
    pub fn generate_telemetry(&self) -> ProductionTelemetry {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        ProductionTelemetry {
            device_id: self.device_id.clone(),
            timestamp: Utc::now(),
            production_count: rng.gen_range(1000..2000),
            reject_count: rng.gen_range(0..50),
            cycle_time_ms: rng.gen_range(800..1200),
            temperature_c: rng.gen_range(45.0..55.0),
            vibration_mm_s: rng.gen_range(2.0..8.0),
            power_consumption_kw: rng.gen_range(50.0..150.0),
            pressure_bar: rng.gen_range(8.0..12.0),
            speed_rpm: rng.gen_range(1200.0..1800.0),
        }
    }

    /// Gerar telemetria com anomalia
    pub fn generate_anomaly(&self, anomaly_type: AnomalyType) -> ProductionTelemetry {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut telemetry = self.generate_telemetry();

        match anomaly_type {
            AnomalyType::HighTemperature => {
                telemetry.temperature_c = rng.gen_range(75.0..85.0);
            }
            AnomalyType::HighVibration => {
                telemetry.vibration_mm_s = rng.gen_range(12.0..20.0);
            }
            AnomalyType::LowPressure => {
                telemetry.pressure_bar = rng.gen_range(3.0..5.0);
            }
            AnomalyType::HighSpeed => {
                telemetry.speed_rpm = rng.gen_range(3200.0..3500.0);
            }
        }

        telemetry
    }
}

#[derive(Debug, Clone)]
pub enum AnomalyType {
    HighTemperature,
    HighVibration,
    LowPressure,
    HighSpeed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_telemetry_within_limits() {
        let telemetry = ProductionTelemetry {
            device_id: "machine-1".to_string(),
            timestamp: Utc::now(),
            production_count: 1000,
            reject_count: 10,
            cycle_time_ms: 1000,
            temperature_c: 50.0,
            vibration_mm_s: 5.0,
            power_consumption_kw: 100.0,
            pressure_bar: 10.0,
            speed_rpm: 1500.0,
        };

        let limits = OperatingLimits::default();
        assert!(telemetry.is_within_limits(&limits));
    }

    #[test]
    fn test_health_score() {
        let telemetry = ProductionTelemetry {
            device_id: "machine-1".to_string(),
            timestamp: Utc::now(),
            production_count: 1000,
            reject_count: 10,
            cycle_time_ms: 1000,
            temperature_c: 50.0,
            vibration_mm_s: 5.0,
            power_consumption_kw: 100.0,
            pressure_bar: 10.0,
            speed_rpm: 1500.0,
        };

        let limits = OperatingLimits::default();
        let score = telemetry.health_score(&limits);
        assert!(score > 0.8); // Score alto quando tudo está ótimo
    }

    #[test]
    fn test_device_registration() {
        let mut ingester = IoTDataIngester::new(100);

        let device = IoTDevice {
            device_id: "machine-1".to_string(),
            device_type: DeviceType::Machine,
            location: Location {
                facility: "Factory A".to_string(),
                area: "Assembly".to_string(),
                line: "Line 1".to_string(),
                station: "Station 3".to_string(),
            },
            status: DeviceStatus::Online,
            metadata: HashMap::new(),
        };

        ingester.register_device(device);
        assert_eq!(ingester.devices.len(), 1);
    }
}
