//! Real-time Analytics para IoT Espacial
//!
//! Este módulo implementa análise em tempo real de streams de dados IoT:
//! - Agregações em janela temporal (time windows)
//! - Detecção de anomalias em tempo real
//! - Análise de padrões espaciais
//! - Alertas e triggers baseados em regras

use crate::coords::GeoCoord;
use crate::geoprocessing::industry4::{SensorReading, SensorType, Timestamp, current_timestamp};
use std::collections::{HashMap, VecDeque};

/// Janela temporal para agregação de dados
#[derive(Debug, Clone, Copy)]
pub enum TimeWindow {
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
}

impl TimeWindow {
    /// Converte para milissegundos
    pub fn as_millis(&self) -> u64 {
        match self {
            TimeWindow::Seconds(s) => s * 1000,
            TimeWindow::Minutes(m) => m * 60 * 1000,
            TimeWindow::Hours(h) => h * 60 * 60 * 1000,
        }
    }
}

/// Stream processor para análise em tempo real
pub struct StreamProcessor {
    buffers: HashMap<String, VecDeque<SensorReading>>,
    window: TimeWindow,
    anomaly_detector: AnomalyDetector,
}

impl StreamProcessor {
    pub fn new(window: TimeWindow) -> Self {
        Self {
            buffers: HashMap::new(),
            window,
            anomaly_detector: AnomalyDetector::new(3.0), // 3 desvios padrão
        }
    }

    /// Processa uma nova leitura de sensor
    pub fn process(&mut self, device_id: String, reading: SensorReading) -> StreamAnalytics {
        // Obter ou criar buffer para este dispositivo
        let buffer = self.buffers.entry(device_id.clone()).or_insert_with(VecDeque::new);

        // Adicionar leitura ao buffer
        buffer.push_back(reading.clone());

        // Remover leituras antigas (fora da janela temporal)
        let now = current_timestamp();
        let window_start = now.saturating_sub(self.window.as_millis());
        buffer.retain(|r| r.timestamp >= window_start);

        // Calcular métricas agregadas
        let analytics = self.calculate_analytics(&device_id, buffer);

        // Detectar anomalias
        let is_anomaly = self.anomaly_detector.detect(&device_id, reading.value);

        StreamAnalytics {
            device_id,
            window_size: buffer.len(),
            metrics: analytics,
            is_anomaly,
            timestamp: now,
        }
    }

    /// Calcula métricas agregadas sobre os dados na janela
    fn calculate_analytics(&self, device_id: &str, buffer: &VecDeque<SensorReading>) -> AggregateMetrics {
        if buffer.is_empty() {
            return AggregateMetrics::default();
        }

        let values: Vec<f64> = buffer.iter().map(|r| r.value).collect();
        let sum: f64 = values.iter().sum();
        let count = values.len() as f64;
        let mean = sum / count;

        let variance = values.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / count;
        let std_dev = variance.sqrt();

        let mut sorted = values.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = sorted[sorted.len() / 2];

        // Taxa de mudança (derivada)
        let rate_of_change = if buffer.len() >= 2 {
            let latest = buffer.back().unwrap();
            let previous = buffer.get(buffer.len() - 2).unwrap();
            let dt = (latest.timestamp - previous.timestamp) as f64 / 1000.0; // segundos
            if dt > 0.0 {
                (latest.value - previous.value) / dt
            } else {
                0.0
            }
        } else {
            0.0
        };

        AggregateMetrics {
            mean,
            median,
            std_dev,
            min: *sorted.first().unwrap(),
            max: *sorted.last().unwrap(),
            rate_of_change,
        }
    }

    /// Limpa todos os buffers
    pub fn clear(&mut self) {
        self.buffers.clear();
        self.anomaly_detector.clear();
    }
}

/// Métricas agregadas de um stream
#[derive(Debug, Clone, Default)]
pub struct AggregateMetrics {
    pub mean: f64,
    pub median: f64,
    pub std_dev: f64,
    pub min: f64,
    pub max: f64,
    pub rate_of_change: f64, // Taxa de mudança por segundo
}

/// Resultado da análise de stream
#[derive(Debug, Clone)]
pub struct StreamAnalytics {
    pub device_id: String,
    pub window_size: usize,
    pub metrics: AggregateMetrics,
    pub is_anomaly: bool,
    pub timestamp: Timestamp,
}

/// Detector de anomalias usando Z-score
pub struct AnomalyDetector {
    history: HashMap<String, VecDeque<f64>>,
    threshold: f64, // Número de desvios padrão
    max_history: usize,
}

impl AnomalyDetector {
    pub fn new(threshold: f64) -> Self {
        Self {
            history: HashMap::new(),
            threshold,
            max_history: 1000,
        }
    }

    /// Detecta se um valor é anômalo
    pub fn detect(&mut self, device_id: &str, value: f64) -> bool {
        let history = self.history
            .entry(device_id.to_string())
            .or_insert_with(VecDeque::new);

        // Adicionar valor ao histórico
        history.push_back(value);
        if history.len() > self.max_history {
            history.pop_front();
        }

        // Precisa de pelo menos 10 valores para calcular anomalia
        if history.len() < 10 {
            return false;
        }

        // Calcular média e desvio padrão
        let mean = history.iter().sum::<f64>() / history.len() as f64;
        let variance = history.iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>() / history.len() as f64;
        let std_dev = variance.sqrt();

        // Z-score
        if std_dev == 0.0 {
            return false;
        }

        let z_score = (value - mean).abs() / std_dev;
        z_score > self.threshold
    }

    /// Limpa histórico
    pub fn clear(&mut self) {
        self.history.clear();
    }
}

/// Sistema de alertas baseado em regras
pub struct AlertSystem {
    rules: Vec<AlertRule>,
    triggered_alerts: Vec<Alert>,
}

impl AlertSystem {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            triggered_alerts: Vec::new(),
        }
    }

    /// Adiciona uma regra de alerta
    pub fn add_rule(&mut self, rule: AlertRule) {
        self.rules.push(rule);
    }

    /// Avalia regras e gera alertas
    pub fn evaluate(&mut self, device_id: &str, reading: &SensorReading) -> Vec<Alert> {
        let mut new_alerts = Vec::new();

        for rule in &self.rules {
            if rule.evaluate(reading) {
                let alert = Alert {
                    rule_name: rule.name.clone(),
                    device_id: device_id.to_string(),
                    severity: rule.severity,
                    message: format!(
                        "{}: {} = {:.2} {}",
                        rule.name,
                        sensor_type_name(&reading.sensor_type),
                        reading.value,
                        reading.unit
                    ),
                    timestamp: reading.timestamp,
                };

                new_alerts.push(alert.clone());
                self.triggered_alerts.push(alert);
            }
        }

        new_alerts
    }

    /// Obtém alertas recentes (últimas N horas)
    pub fn recent_alerts(&self, hours: u64) -> Vec<&Alert> {
        let now = current_timestamp();
        let threshold = now.saturating_sub(hours * 60 * 60 * 1000);

        self.triggered_alerts
            .iter()
            .filter(|a| a.timestamp >= threshold)
            .collect()
    }

    /// Limpa alertas antigos
    pub fn clear_old_alerts(&mut self, hours: u64) {
        let now = current_timestamp();
        let threshold = now.saturating_sub(hours * 60 * 60 * 1000);

        self.triggered_alerts.retain(|a| a.timestamp >= threshold);
    }
}

impl Default for AlertSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Regra de alerta
#[derive(Debug, Clone)]
pub struct AlertRule {
    pub name: String,
    pub sensor_type: SensorType,
    pub condition: Condition,
    pub severity: AlertSeverity,
}

impl AlertRule {
    pub fn new(name: String, sensor_type: SensorType, condition: Condition, severity: AlertSeverity) -> Self {
        Self {
            name,
            sensor_type,
            condition,
            severity,
        }
    }

    /// Avalia se a regra foi violada
    fn evaluate(&self, reading: &SensorReading) -> bool {
        if reading.sensor_type != self.sensor_type {
            return false;
        }

        match self.condition {
            Condition::GreaterThan(threshold) => reading.value > threshold,
            Condition::LessThan(threshold) => reading.value < threshold,
            Condition::Between(min, max) => reading.value >= min && reading.value <= max,
            Condition::Outside(min, max) => reading.value < min || reading.value > max,
            Condition::Equals(value) => (reading.value - value).abs() < 0.001,
        }
    }
}

/// Condição de alerta
#[derive(Debug, Clone, Copy)]
pub enum Condition {
    GreaterThan(f64),
    LessThan(f64),
    Between(f64, f64),
    Outside(f64, f64),
    Equals(f64),
}

/// Severidade do alerta
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

/// Alerta disparado
#[derive(Debug, Clone)]
pub struct Alert {
    pub rule_name: String,
    pub device_id: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: Timestamp,
}

/// Análise de padrões espaciais em tempo real
pub struct SpatialPatternAnalyzer {
    pub hotspot_threshold: f64,  // Raio para clustering espacial (metros)
    pub min_cluster_size: usize,
}

impl SpatialPatternAnalyzer {
    pub fn new(hotspot_threshold: f64, min_cluster_size: usize) -> Self {
        Self {
            hotspot_threshold,
            min_cluster_size,
        }
    }

    /// Detecta hotspots espaciais (concentração de eventos)
    pub fn detect_hotspots(&self, locations: &[(GeoCoord, f64)]) -> Vec<Hotspot> {
        use crate::geoprocessing::analysis::haversine_distance;

        let mut hotspots = Vec::new();
        let mut visited = vec![false; locations.len()];

        for i in 0..locations.len() {
            if visited[i] {
                continue;
            }

            let mut cluster = vec![i];
            let mut cluster_value = locations[i].1;

            for j in 0..locations.len() {
                if i == j || visited[j] {
                    continue;
                }

                let dist = haversine_distance(&locations[i].0, &locations[j].0);
                if dist <= self.hotspot_threshold {
                    cluster.push(j);
                    cluster_value += locations[j].1;
                }
            }

            if cluster.len() >= self.min_cluster_size {
                // Calcular centróide
                let lat_sum: f64 = cluster.iter().map(|&idx| locations[idx].0.lat).sum();
                let lon_sum: f64 = cluster.iter().map(|&idx| locations[idx].0.lon).sum();
                let center = GeoCoord::new(
                    lat_sum / cluster.len() as f64,
                    lon_sum / cluster.len() as f64,
                );

                hotspots.push(Hotspot {
                    center,
                    size: cluster.len(),
                    total_value: cluster_value,
                    intensity: cluster_value / cluster.len() as f64,
                });

                for &idx in &cluster {
                    visited[idx] = true;
                }
            }
        }

        hotspots.sort_by(|a, b| b.intensity.partial_cmp(&a.intensity).unwrap());
        hotspots
    }
}

/// Hotspot espacial detectado
#[derive(Debug, Clone)]
pub struct Hotspot {
    pub center: GeoCoord,
    pub size: usize,
    pub total_value: f64,
    pub intensity: f64,
}

fn sensor_type_name(sensor_type: &SensorType) -> String {
    match sensor_type {
        SensorType::Temperature => "Temperatura".to_string(),
        SensorType::Humidity => "Umidade".to_string(),
        SensorType::Pressure => "Pressão".to_string(),
        SensorType::Vibration => "Vibração".to_string(),
        SensorType::Speed => "Velocidade".to_string(),
        SensorType::FuelLevel => "Nível Combustível".to_string(),
        SensorType::BatteryLevel => "Nível Bateria".to_string(),
        SensorType::GPS => "GPS".to_string(),
        SensorType::Accelerometer => "Acelerômetro".to_string(),
        SensorType::Custom(name) => name.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_processor() {
        let mut processor = StreamProcessor::new(TimeWindow::Minutes(5));

        let reading = SensorReading::new(
            SensorType::Temperature,
            25.0,
            "°C".to_string(),
        );

        let analytics = processor.process("device001".to_string(), reading);
        assert_eq!(analytics.window_size, 1);
    }

    #[test]
    fn test_anomaly_detector() {
        let mut detector = AnomalyDetector::new(3.0);

        // Adicionar valores normais
        for i in 0..20 {
            let value = 25.0 + (i as f64 * 0.1);
            detector.detect("device001", value);
        }

        // Valor anômalo
        let is_anomaly = detector.detect("device001", 100.0);
        assert!(is_anomaly);
    }

    #[test]
    fn test_alert_system() {
        let mut system = AlertSystem::new();

        let rule = AlertRule::new(
            "Temperatura Alta".to_string(),
            SensorType::Temperature,
            Condition::GreaterThan(30.0),
            AlertSeverity::Warning,
        );

        system.add_rule(rule);

        let reading = SensorReading::new(
            SensorType::Temperature,
            35.0,
            "°C".to_string(),
        );

        let alerts = system.evaluate("device001", &reading);
        assert_eq!(alerts.len(), 1);
        assert_eq!(alerts[0].severity, AlertSeverity::Warning);
    }

    #[test]
    fn test_spatial_patterns() {
        let analyzer = SpatialPatternAnalyzer::new(1000.0, 2);

        let locations = vec![
            (GeoCoord::new(-23.550, -46.630), 10.0),
            (GeoCoord::new(-23.551, -46.631), 15.0),
            (GeoCoord::new(-23.552, -46.632), 12.0),
        ];

        let hotspots = analyzer.detect_hotspots(&locations);
        assert!(!hotspots.is_empty());
    }
}
