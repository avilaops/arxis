//! Sensor abstractions for Industry 4.0

use crate::Event;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Sensor types commonly found in manufacturing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorType {
    Temperature,
    Pressure,
    Vibration,
    Humidity,
    Flow,
    Level,
    Position,
    Speed,
    Torque,
    Power,
    Proximity,
    Vision,
    Custom,
}

/// Sensor reading event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub sensor_id: String,
    pub sensor_type: SensorType,
    pub value: f64,
    pub unit: String,
    pub timestamp: DateTime<Utc>,
    pub quality: SensorQuality,
    pub machine_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorQuality {
    Good,
    Uncertain,
    Bad,
}

impl Event for SensorReading {
    fn event_type(&self) -> &'static str {
        "industry40.sensor.reading"
    }

    fn aggregate_id(&self) -> String {
        self.sensor_id.clone()
    }
}

/// Sensor alarm/threshold violation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorAlarm {
    pub sensor_id: String,
    pub alarm_type: AlarmType,
    pub threshold: Threshold,
    pub current_value: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlarmType {
    HighHigh,
    High,
    Low,
    LowLow,
    RateOfChange,
    Deviation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threshold {
    pub limit: f64,
    pub unit: String,
}

impl Event for SensorAlarm {
    fn event_type(&self) -> &'static str {
        "industry40.sensor.alarm"
    }

    fn aggregate_id(&self) -> String {
        self.sensor_id.clone()
    }
}

/// Sensor health monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorHealth {
    pub sensor_id: String,
    pub status: SensorStatus,
    pub last_reading: DateTime<Utc>,
    pub battery_level: Option<f64>,
    pub signal_strength: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SensorStatus {
    Online,
    Offline,
    Degraded,
    Maintenance,
    Error,
}

impl Event for SensorHealth {
    fn event_type(&self) -> &'static str {
        "industry40.sensor.health"
    }

    fn aggregate_id(&self) -> String {
        self.sensor_id.clone()
    }
}
