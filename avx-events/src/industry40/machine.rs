//! Machine and sensor abstractions for Industry 4.0

use crate::Event;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Machine identifier following ISA-95 hierarchy
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MachineId {
    pub enterprise: String,
    pub site: String,
    pub area: String,
    pub line: String,
    pub machine: String,
}

impl MachineId {
    pub fn new(
        enterprise: impl Into<String>,
        site: impl Into<String>,
        area: impl Into<String>,
        line: impl Into<String>,
        machine: impl Into<String>,
    ) -> Self {
        Self {
            enterprise: enterprise.into(),
            site: site.into(),
            area: area.into(),
            line: line.into(),
            machine: machine.into(),
        }
    }

    /// Convert to topic path (e.g., "factory.area1.line2.robot3")
    pub fn to_topic(&self) -> String {
        format!(
            "{}.{}.{}.{}.{}",
            self.enterprise, self.site, self.area, self.line, self.machine
        )
    }

    /// Convert to OPC UA node path
    pub fn to_opcua_path(&self) -> String {
        format!(
            "ns=2;s={}/{}/{}/{}/{}",
            self.enterprise, self.site, self.area, self.line, self.machine
        )
    }
}

impl std::fmt::Display for MachineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_topic())
    }
}

/// Machine status following ISA-88 state model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MachineStatus {
    /// Machine is stopped
    Stopped,
    /// Machine is starting up
    Starting,
    /// Machine is running and producing
    Running,
    /// Machine is temporarily paused
    Held,
    /// Machine is stopping
    Stopping,
    /// Machine has aborted due to error
    Aborted,
    /// Machine is in idle state
    Idle,
}

/// Machine event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineEvent {
    pub machine_id: MachineId,
    pub event_type: MachineEventType,
    pub timestamp: DateTime<Utc>,
    pub value: Option<f64>,
    pub unit: Option<String>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MachineEventType {
    /// Machine status changed
    StatusChanged { from: MachineStatus, to: MachineStatus },
    /// Sensor reading
    SensorReading { sensor_id: String, value: f64, unit: String },
    /// Production count
    ProductionCount { count: u64 },
    /// Quality check result
    QualityCheck { passed: bool, defect_type: Option<String> },
    /// Alarm triggered
    Alarm { code: String, severity: AlarmSeverity, message: String },
    /// Maintenance required
    MaintenanceRequired { reason: String, urgency: MaintenanceUrgency },
    /// Custom event
    Custom { name: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AlarmSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MaintenanceUrgency {
    Low,
    Medium,
    High,
    Critical,
}

impl Event for MachineEvent {
    fn event_type(&self) -> &'static str {
        "industry40.machine.event"
    }

    fn aggregate_id(&self) -> String {
        self.machine_id.to_string()
    }
}

/// OEE (Overall Equipment Effectiveness) metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OeeMetrics {
    pub machine_id: MachineId,
    pub timestamp: DateTime<Utc>,
    pub availability: f64,  // 0.0 to 1.0
    pub performance: f64,   // 0.0 to 1.0
    pub quality: f64,       // 0.0 to 1.0
    pub oee: f64,          // availability * performance * quality
}

impl OeeMetrics {
    pub fn new(machine_id: MachineId, availability: f64, performance: f64, quality: f64) -> Self {
        Self {
            machine_id,
            timestamp: Utc::now(),
            availability,
            performance,
            quality,
            oee: availability * performance * quality,
        }
    }

    pub fn meets_world_class(&self) -> bool {
        self.oee >= 0.85 // World-class OEE is typically 85%+
    }
}

impl Event for OeeMetrics {
    fn event_type(&self) -> &'static str {
        "industry40.metrics.oee"
    }

    fn aggregate_id(&self) -> String {
        self.machine_id.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_machine_id_topic() {
        let id = MachineId::new("factory", "site1", "assembly", "line1", "robot3");
        assert_eq!(id.to_topic(), "factory.site1.assembly.line1.robot3");
    }

    #[test]
    fn test_oee_calculation() {
        let id = MachineId::new("factory", "site1", "assembly", "line1", "robot1");
        let oee = OeeMetrics::new(id, 0.95, 0.92, 0.98);

        assert!((oee.oee - 0.8554).abs() < 0.001);
        assert!(oee.meets_world_class());
    }
}
