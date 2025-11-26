use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::iot::{ProductionTelemetry, MachineStatus};

/// Gêmeo Digital - Representação virtual de máquinas físicas
pub struct DigitalTwin {
    pub twin_id: String,
    pub physical_asset_id: String,
    pub state: TwinState,
    pub historical_states: Vec<TwinSnapshot>,
    pub simulation_model: SimulationModel,
}

impl DigitalTwin {
    pub fn new(twin_id: String, physical_asset_id: String) -> Self {
        Self {
            twin_id,
            physical_asset_id,
            state: TwinState::default(),
            historical_states: Vec::new(),
            simulation_model: SimulationModel::default(),
        }
    }

    /// Atualizar estado do twin com dados de telemetria
    pub fn update_from_telemetry(&mut self, telemetry: &ProductionTelemetry) {
        self.state.temperature_c = telemetry.temperature_c;
        self.state.vibration_mm_s = telemetry.vibration_mm_s;
        self.state.pressure_bar = telemetry.pressure_bar;
        self.state.speed_rpm = telemetry.speed_rpm;
        self.state.power_consumption_kw = telemetry.power_consumption_kw;
        self.state.last_updated = Utc::now();

        // Salvar snapshot
        self.save_snapshot();
    }

    /// Simular comportamento futuro
    pub fn simulate_future(&self, hours_ahead: u32) -> Vec<TwinState> {
        self.simulation_model.simulate(&self.state, hours_ahead)
    }

    /// Detectar anomalias
    pub fn detect_anomalies(&self) -> Vec<Anomaly> {
        let mut anomalies = Vec::new();

        if self.state.temperature_c > 75.0 {
            anomalies.push(Anomaly {
                anomaly_type: AnomalyType::HighTemperature,
                severity: 0.8,
                description: format!("Temperature at {:.1}°C", self.state.temperature_c),
            });
        }

        if self.state.vibration_mm_s > 12.0 {
            anomalies.push(Anomaly {
                anomaly_type: AnomalyType::HighVibration,
                severity: 0.9,
                description: format!("Vibration at {:.1} mm/s", self.state.vibration_mm_s),
            });
        }

        anomalies
    }

    fn save_snapshot(&mut self) {
        self.historical_states.push(TwinSnapshot {
            state: self.state.clone(),
            timestamp: Utc::now(),
        });

        // Manter apenas últimos 1000 snapshots
        if self.historical_states.len() > 1000 {
            self.historical_states.drain(0..100);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinState {
    pub temperature_c: f64,
    pub vibration_mm_s: f64,
    pub pressure_bar: f64,
    pub speed_rpm: f64,
    pub power_consumption_kw: f64,
    pub machine_status: MachineStatus,
    pub last_updated: DateTime<Utc>,
}

impl Default for TwinState {
    fn default() -> Self {
        Self {
            temperature_c: 50.0,
            vibration_mm_s: 5.0,
            pressure_bar: 10.0,
            speed_rpm: 1500.0,
            power_consumption_kw: 100.0,
            machine_status: MachineStatus::Idle,
            last_updated: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwinSnapshot {
    pub state: TwinState,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct SimulationModel {
    degradation_rate: f64,
}

impl Default for SimulationModel {
    fn default() -> Self {
        Self {
            degradation_rate: 0.01,
        }
    }
}

impl SimulationModel {
    pub fn simulate(&self, current_state: &TwinState, hours_ahead: u32) -> Vec<TwinState> {
        let mut states = Vec::new();
        let mut state = current_state.clone();

        for _ in 0..hours_ahead {
            state.vibration_mm_s += self.degradation_rate;
            state.temperature_c += self.degradation_rate * 0.5;
            states.push(state.clone());
        }

        states
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Anomaly {
    pub anomaly_type: AnomalyType,
    pub severity: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnomalyType {
    HighTemperature,
    HighVibration,
    LowPressure,
    UnexpectedStop,
}
