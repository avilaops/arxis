use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::iot::{ProductionTelemetry, OperatingLimits};

/// Motor de manutenção preditiva usando ML
pub struct PredictiveMaintenanceEngine {
    models: HashMap<String, MaintenanceModel>,
    alerts: Vec<MaintenanceAlert>,
    threshold_critical: f64,
    threshold_warning: f64,
}

impl PredictiveMaintenanceEngine {
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            alerts: Vec::new(),
            threshold_critical: 0.8,
            threshold_warning: 0.6,
        }
    }

    /// Treinar modelo para um dispositivo
    pub fn train_model(&mut self, device_id: String, historical_data: &[ProductionTelemetry]) {
        let model = MaintenanceModel::train(device_id.clone(), historical_data);
        self.models.insert(device_id, model);
    }

    /// Prever falha baseado em telemetria atual
    pub fn predict_failure(&mut self, telemetry: &ProductionTelemetry) -> Option<MaintenanceAlert> {
        let model = self.models.get(&telemetry.device_id)?;
        let failure_probability = model.predict_failure_probability(telemetry);

        if failure_probability >= self.threshold_critical {
            let alert = MaintenanceAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                device_id: telemetry.device_id.clone(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Critical,
                failure_probability,
                estimated_time_to_failure: Duration::hours(24),
                recommended_action: "Parar máquina imediatamente e realizar manutenção".to_string(),
                predicted_failure_type: FailureType::BearingFailure,
                root_causes: vec![
                    "Vibração excessiva detectada".to_string(),
                    "Temperatura acima do normal".to_string(),
                ],
            };
            self.alerts.push(alert.clone());
            Some(alert)
        } else if failure_probability >= self.threshold_warning {
            let alert = MaintenanceAlert {
                alert_id: uuid::Uuid::new_v4().to_string(),
                device_id: telemetry.device_id.clone(),
                timestamp: Utc::now(),
                severity: AlertSeverity::Warning,
                failure_probability,
                estimated_time_to_failure: Duration::days(7),
                recommended_action: "Agendar manutenção preventiva na próxima parada programada".to_string(),
                predicted_failure_type: FailureType::WearAndTear,
                root_causes: vec!["Desgaste gradual detectado".to_string()],
            };
            self.alerts.push(alert.clone());
            Some(alert)
        } else {
            None
        }
    }

    /// Calcular RUL (Remaining Useful Life)
    pub fn calculate_rul(&self, device_id: &str, telemetry: &ProductionTelemetry) -> Option<Duration> {
        let model = self.models.get(device_id)?;
        Some(model.estimate_rul(telemetry))
    }

    /// Obter alertas ativos
    pub fn get_active_alerts(&self) -> Vec<MaintenanceAlert> {
        self.alerts
            .iter()
            .filter(|alert| {
                let age = Utc::now().signed_duration_since(alert.timestamp);
                age < Duration::days(1)
            })
            .cloned()
            .collect()
    }

    /// Obter alertas críticos
    pub fn get_critical_alerts(&self) -> Vec<MaintenanceAlert> {
        self.alerts
            .iter()
            .filter(|alert| alert.severity == AlertSeverity::Critical)
            .cloned()
            .collect()
    }
}

impl Default for PredictiveMaintenanceEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Alerta de manutenção
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceAlert {
    pub alert_id: String,
    pub device_id: String,
    pub timestamp: DateTime<Utc>,
    pub severity: AlertSeverity,
    pub failure_probability: f64,
    pub estimated_time_to_failure: Duration,
    pub recommended_action: String,
    pub predicted_failure_type: FailureType,
    pub root_causes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailureType {
    BearingFailure,
    MotorFailure,
    ElectricalFailure,
    HydraulicFailure,
    MechanicalFailure,
    SensorFailure,
    WearAndTear,
    Overheating,
    Other(String),
}

/// Modelo de manutenção preditiva
pub struct MaintenanceModel {
    device_id: String,
    baseline_metrics: BaselineMetrics,
    degradation_rate: f64,
    failure_patterns: Vec<FailurePattern>,
}

impl MaintenanceModel {
    /// Treinar modelo com dados históricos
    pub fn train(device_id: String, historical_data: &[ProductionTelemetry]) -> Self {
        let baseline = Self::calculate_baseline(historical_data);
        let degradation_rate = Self::calculate_degradation_rate(historical_data);
        let patterns = Self::identify_failure_patterns(historical_data);

        Self {
            device_id,
            baseline_metrics: baseline,
            degradation_rate,
            failure_patterns: patterns,
        }
    }

    /// Prever probabilidade de falha
    pub fn predict_failure_probability(&self, telemetry: &ProductionTelemetry) -> f64 {
        let mut probability = 0.0;

        // Análise de temperatura
        let temp_deviation = (telemetry.temperature_c - self.baseline_metrics.avg_temperature).abs()
            / self.baseline_metrics.std_temperature;
        if temp_deviation > 2.0 {
            probability += 0.3;
        } else if temp_deviation > 1.5 {
            probability += 0.15;
        }

        // Análise de vibração
        let vib_deviation = (telemetry.vibration_mm_s - self.baseline_metrics.avg_vibration).abs()
            / self.baseline_metrics.std_vibration;
        if vib_deviation > 2.0 {
            probability += 0.4;
        } else if vib_deviation > 1.5 {
            probability += 0.2;
        }

        // Análise de pressão
        let pressure_deviation = (telemetry.pressure_bar - self.baseline_metrics.avg_pressure).abs()
            / self.baseline_metrics.std_pressure;
        if pressure_deviation > 2.0 {
            probability += 0.2;
        }

        // Análise de velocidade
        let speed_deviation = (telemetry.speed_rpm - self.baseline_metrics.avg_speed).abs()
            / self.baseline_metrics.std_speed;
        if speed_deviation > 2.0 {
            probability += 0.1;
        }

        // Verificar padrões conhecidos de falha
        for pattern in &self.failure_patterns {
            if pattern.matches(telemetry) {
                probability += 0.3;
            }
        }

        probability.min(1.0)
    }

    /// Estimar vida útil remanescente (RUL)
    pub fn estimate_rul(&self, telemetry: &ProductionTelemetry) -> Duration {
        let failure_prob = self.predict_failure_probability(telemetry);

        // Modelo simplificado: RUL inversamente proporcional à probabilidade
        let days = if failure_prob > 0.8 {
            1 // Crítico - 1 dia
        } else if failure_prob > 0.6 {
            7 // Warning - 1 semana
        } else if failure_prob > 0.4 {
            30 // Atenção - 1 mês
        } else {
            90 // Normal - 3 meses
        };

        Duration::days(days)
    }

    fn calculate_baseline(data: &[ProductionTelemetry]) -> BaselineMetrics {
        if data.is_empty() {
            return BaselineMetrics::default();
        }

        let n = data.len() as f64;

        let avg_temp = data.iter().map(|t| t.temperature_c).sum::<f64>() / n;
        let avg_vib = data.iter().map(|t| t.vibration_mm_s).sum::<f64>() / n;
        let avg_pressure = data.iter().map(|t| t.pressure_bar).sum::<f64>() / n;
        let avg_speed = data.iter().map(|t| t.speed_rpm).sum::<f64>() / n;

        let std_temp = (data.iter().map(|t| (t.temperature_c - avg_temp).powi(2)).sum::<f64>() / n).sqrt();
        let std_vib = (data.iter().map(|t| (t.vibration_mm_s - avg_vib).powi(2)).sum::<f64>() / n).sqrt();
        let std_pressure = (data.iter().map(|t| (t.pressure_bar - avg_pressure).powi(2)).sum::<f64>() / n).sqrt();
        let std_speed = (data.iter().map(|t| (t.speed_rpm - avg_speed).powi(2)).sum::<f64>() / n).sqrt();

        BaselineMetrics {
            avg_temperature: avg_temp,
            std_temperature: std_temp,
            avg_vibration: avg_vib,
            std_vibration: std_vib,
            avg_pressure: avg_pressure,
            std_pressure: std_pressure,
            avg_speed: avg_speed,
            std_speed: std_speed,
        }
    }

    fn calculate_degradation_rate(data: &[ProductionTelemetry]) -> f64 {
        // Simplificado: calcular taxa de aumento de vibração ao longo do tempo
        if data.len() < 2 {
            return 0.0;
        }

        let first_vib = data.first().unwrap().vibration_mm_s;
        let last_vib = data.last().unwrap().vibration_mm_s;
        let time_diff = data.last().unwrap().timestamp
            .signed_duration_since(data.first().unwrap().timestamp)
            .num_days() as f64;

        if time_diff > 0.0 {
            (last_vib - first_vib) / time_diff
        } else {
            0.0
        }
    }

    fn identify_failure_patterns(data: &[ProductionTelemetry]) -> Vec<FailurePattern> {
        // Identificar padrões comuns de falha
        vec![
            FailurePattern {
                name: "High Temperature + High Vibration".to_string(),
                condition: Box::new(|t: &ProductionTelemetry| {
                    t.temperature_c > 70.0 && t.vibration_mm_s > 10.0
                }),
            },
            FailurePattern {
                name: "Sudden Pressure Drop".to_string(),
                condition: Box::new(|t: &ProductionTelemetry| {
                    t.pressure_bar < 6.0
                }),
            },
        ]
    }
}

/// Métricas baseline para comparação
#[derive(Debug, Clone)]
pub struct BaselineMetrics {
    pub avg_temperature: f64,
    pub std_temperature: f64,
    pub avg_vibration: f64,
    pub std_vibration: f64,
    pub avg_pressure: f64,
    pub std_pressure: f64,
    pub avg_speed: f64,
    pub std_speed: f64,
}

impl Default for BaselineMetrics {
    fn default() -> Self {
        Self {
            avg_temperature: 50.0,
            std_temperature: 5.0,
            avg_vibration: 5.0,
            std_vibration: 1.0,
            avg_pressure: 10.0,
            std_pressure: 1.0,
            avg_speed: 1500.0,
            std_speed: 100.0,
        }
    }
}

/// Padrão de falha conhecido
pub struct FailurePattern {
    pub name: String,
    pub condition: Box<dyn Fn(&ProductionTelemetry) -> bool>,
}

impl FailurePattern {
    pub fn matches(&self, telemetry: &ProductionTelemetry) -> bool {
        (self.condition)(telemetry)
    }
}

/// Agendador de manutenção
pub struct MaintenanceScheduler {
    schedule: Vec<MaintenanceTask>,
}

impl MaintenanceScheduler {
    pub fn new() -> Self {
        Self {
            schedule: Vec::new(),
        }
    }

    /// Agendar tarefa de manutenção
    pub fn schedule_task(&mut self, task: MaintenanceTask) {
        self.schedule.push(task);
        self.schedule.sort_by_key(|t| t.scheduled_date);
    }

    /// Obter tarefas pendentes
    pub fn get_pending_tasks(&self) -> Vec<&MaintenanceTask> {
        self.schedule
            .iter()
            .filter(|task| task.status == MaintenanceStatus::Pending)
            .collect()
    }

    /// Obter próximas tarefas (próximos 7 dias)
    pub fn get_upcoming_tasks(&self) -> Vec<&MaintenanceTask> {
        let now = Utc::now();
        let next_week = now + Duration::days(7);

        self.schedule
            .iter()
            .filter(|task| {
                task.scheduled_date >= now && task.scheduled_date <= next_week
            })
            .collect()
    }
}

impl Default for MaintenanceScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceTask {
    pub task_id: String,
    pub device_id: String,
    pub task_type: MaintenanceType,
    pub scheduled_date: DateTime<Utc>,
    pub estimated_duration_hours: u32,
    pub status: MaintenanceStatus,
    pub priority: TaskPriority,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaintenanceType {
    Preventive,
    Predictive,
    Corrective,
    Inspection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaintenanceStatus {
    Pending,
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_prediction() {
        let mut engine = PredictiveMaintenanceEngine::new();

        let historical_data = vec![
            ProductionTelemetry {
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
            },
        ];

        engine.train_model("machine-1".to_string(), &historical_data);

        let critical_telemetry = ProductionTelemetry {
            device_id: "machine-1".to_string(),
            timestamp: Utc::now(),
            production_count: 1000,
            reject_count: 50,
            cycle_time_ms: 1000,
            temperature_c: 85.0, // Temperatura crítica
            vibration_mm_s: 15.0, // Vibração alta
            power_consumption_kw: 100.0,
            pressure_bar: 10.0,
            speed_rpm: 1500.0,
        };

        let alert = engine.predict_failure(&critical_telemetry);
        assert!(alert.is_some());
    }
}
