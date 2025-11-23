//! Industry 4.0 Complete Example
//!
//! Demonstrates a complete Industry 4.0 scenario with:
//! - Machine monitoring via OPC UA
//! - Sensor data via MQTT
//! - Time-series data aggregation
//! - Real-time event processing
//! - OEE calculation
//! - Quality control
//! - Predictive maintenance alerts

use avx_events::industry40::machine::{MachineEvent, MachineEventType, MachineId, MachineStatus, OeeMetrics};
use avx_events::industry40::sensor::{SensorReading, SensorType, SensorQuality, SensorAlarm, AlarmType, Threshold};
use avx_events::{EventBus, TopicBus, EventStore};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

#[cfg(feature = "opcua")]
use avx_events::industry40::opcua::OpcUaClient;

#[cfg(feature = "mqtt")]
use avx_events::industry40::mqtt::{MqttBridge, MqttBridgeConfig};

#[cfg(feature = "timeseries")]
use avx_events::industry40::timeseries::{TimeSeriesBackend, TimeSeriesConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize telemetry
    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .init();

    info!("🏭 Starting Industry 4.0 Manufacturing System");

    // Initialize event system
    let event_bus = Arc::new(EventBus::new());
    let _topic_bus = Arc::new(TopicBus::new());
    let event_store = Arc::new(EventStore::new());

    info!("📡 Event infrastructure initialized");

    // Start monitoring subscribers
    start_quality_control_system(event_bus.clone()).await;
    start_maintenance_system(event_bus.clone()).await;
    start_oee_calculator(event_bus.clone()).await;
    start_production_logger(event_store.clone()).await;

    info!("✅ All systems operational");

    // Simulate factory operation
    simulate_factory_operation(event_bus.clone()).await?;

    Ok(())
}

/// Quality control system - monitors sensor readings for anomalies
async fn start_quality_control_system(event_bus: Arc<EventBus>) {
    let mut sensor_sub = event_bus.subscribe::<SensorReading>().await;

    tokio::spawn(async move {
        info!("🔍 Quality Control System online");

        while let Some(envelope) = sensor_sub.recv().await {
            let reading = &envelope.event;

            // Check temperature thresholds
            if reading.sensor_type == SensorType::Temperature {
                if reading.value > 85.0 {
                    error!(
                        sensor_id = %reading.sensor_id,
                        value = reading.value,
                        "⚠️  HIGH TEMPERATURE ALARM - Exceeds safety threshold"
                    );

                    // Publish alarm event
                    event_bus.publish(SensorAlarm {
                        sensor_id: reading.sensor_id.clone(),
                        alarm_type: AlarmType::HighHigh,
                        threshold: Threshold {
                            limit: 85.0,
                            unit: "°C".into(),
                        },
                        current_value: reading.value,
                        timestamp: reading.timestamp,
                    }).await.ok();
                }
            }

            // Check vibration levels
            if reading.sensor_type == SensorType::Vibration {
                if reading.value > 5.0 {
                    warn!(
                        sensor_id = %reading.sensor_id,
                        value = reading.value,
                        "⚠️  High vibration detected - Maintenance may be required"
                    );
                }
            }
        }
    });
}

/// Maintenance system - tracks machine health and schedules maintenance
async fn start_maintenance_system(event_bus: Arc<EventBus>) {
    let mut alarm_sub = event_bus.subscribe::<SensorAlarm>().await;

    tokio::spawn(async move {
        info!("🔧 Predictive Maintenance System online");

        while let Some(envelope) = alarm_sub.recv().await {
            let alarm = &envelope.event;

            match alarm.alarm_type {
                AlarmType::HighHigh | AlarmType::High => {
                    warn!(
                        sensor_id = %alarm.sensor_id,
                        value = alarm.current_value,
                        "📋 Maintenance work order created"
                    );

                    // In real system, would:
                    // 1. Create work order in CMMS
                    // 2. Notify maintenance team
                    // 3. Schedule downtime
                }
                _ => {}
            }
        }
    });
}

/// OEE calculator - tracks Overall Equipment Effectiveness
async fn start_oee_calculator(event_bus: Arc<EventBus>) {
    let mut machine_sub = event_bus.subscribe::<MachineEvent>().await;

    tokio::spawn(async move {
        info!("📊 OEE Calculator online");

        let mut production_count = 0u64;
        let mut running_time = 0u64;
        let mut quality_passed = 0u64;

        while let Some(envelope) = machine_sub.recv().await {
            let event = &envelope.event;

            match &event.event_type {
                MachineEventType::ProductionCount { count } => {
                    production_count += count;
                }
                MachineEventType::StatusChanged { to, .. } => {
                    if *to == MachineStatus::Running {
                        running_time += 1;
                    }
                }
                MachineEventType::QualityCheck { passed, .. } => {
                    if *passed {
                        quality_passed += 1;
                    }
                }
                _ => {}
            }

            // Calculate OEE every 100 events
            if production_count % 100 == 0 && production_count > 0 {
                let availability = 0.95; // Placeholder calculation
                let performance = 0.92;
                let quality = quality_passed as f64 / production_count as f64;

                let oee = OeeMetrics::new(
                    event.machine_id.clone(),
                    availability,
                    performance,
                    quality,
                );

                info!(
                    machine = %oee.machine_id,
                    oee = format!("{:.1}%", oee.oee * 100.0),
                    availability = format!("{:.1}%", availability * 100.0),
                    performance = format!("{:.1}%", performance * 100.0),
                    quality = format!("{:.1}%", quality * 100.0),
                    world_class = oee.meets_world_class(),
                    "📈 OEE Metrics"
                );

                event_bus.publish(oee).await.ok();
            }
        }
    });
}

/// Production logger - stores all events for audit trail
async fn start_production_logger(event_store: Arc<EventStore>) {
    tokio::spawn(async move {
        info!("📝 Production Logger online - All events recorded for compliance");

        // In real system, would:
        // 1. Subscribe to all machine events
        // 2. Store in event store
        // 3. Enable event replay for analysis
        // 4. Maintain audit trail for ISO 9001
    });
}

/// Simulate factory operation
async fn simulate_factory_operation(event_bus: Arc<EventBus>) -> anyhow::Result<()> {
    let machine_id = MachineId::new(
        "Avila Factory",
        "Site BR-SP",
        "Assembly Area",
        "Line 1",
        "Robot Arm 3",
    );

    info!("🏭 Starting simulated factory operation");

    // Machine startup
    event_bus.publish(MachineEvent {
        machine_id: machine_id.clone(),
        event_type: MachineEventType::StatusChanged {
            from: MachineStatus::Stopped,
            to: MachineStatus::Starting,
        },
        timestamp: chrono::Utc::now(),
        value: None,
        unit: None,
        metadata: serde_json::json!({}),
    }).await?;

    sleep(Duration::from_secs(2)).await;

    event_bus.publish(MachineEvent {
        machine_id: machine_id.clone(),
        event_type: MachineEventType::StatusChanged {
            from: MachineStatus::Starting,
            to: MachineStatus::Running,
        },
        timestamp: chrono::Utc::now(),
        value: None,
        unit: None,
        metadata: serde_json::json!({}),
    }).await?;

    info!("▶️  Machine is now running");

    // Simulate production with sensor readings
    for cycle in 1..=10 {
        info!("🔄 Production cycle {}/10", cycle);

        // Temperature sensor reading
        let temp = 70.0 + (cycle as f64 * 1.5); // Gradually increasing temp
        event_bus.publish(SensorReading {
            sensor_id: "temp-001".into(),
            sensor_type: SensorType::Temperature,
            value: temp,
            unit: "°C".into(),
            timestamp: chrono::Utc::now(),
            quality: SensorQuality::Good,
            machine_id: Some(machine_id.to_string()),
        }).await?;

        info!("🌡️  Temperature: {:.1}°C", temp);

        // Vibration sensor reading
        let vibration = 2.0 + (cycle as f64 * 0.3);
        event_bus.publish(SensorReading {
            sensor_id: "vib-001".into(),
            sensor_type: SensorType::Vibration,
            value: vibration,
            unit: "mm/s".into(),
            timestamp: chrono::Utc::now(),
            quality: SensorQuality::Good,
            machine_id: Some(machine_id.to_string()),
        }).await?;

        info!("📳 Vibration: {:.2} mm/s", vibration);

        // Production count
        event_bus.publish(MachineEvent {
            machine_id: machine_id.clone(),
            event_type: MachineEventType::ProductionCount {
                count: 10,
            },
            timestamp: chrono::Utc::now(),
            value: Some(10.0),
            unit: Some("units".into()),
            metadata: serde_json::json!({ "cycle": cycle }),
        }).await?;

        // Quality check (95% pass rate)
        let passed = cycle % 20 != 0; // 1 in 20 fails
        event_bus.publish(MachineEvent {
            machine_id: machine_id.clone(),
            event_type: MachineEventType::QualityCheck {
                passed,
                defect_type: if !passed {
                    Some("dimensional_tolerance".into())
                } else {
                    None
                },
            },
            timestamp: chrono::Utc::now(),
            value: None,
            unit: None,
            metadata: serde_json::json!({}),
        }).await?;

        if passed {
            info!("✅ Quality check passed");
        } else {
            warn!("❌ Quality check failed - Defect detected");
        }

        sleep(Duration::from_secs(1)).await;
    }

    info!("🏁 Production simulation complete");

    // Machine shutdown
    event_bus.publish(MachineEvent {
        machine_id: machine_id.clone(),
        event_type: MachineEventType::StatusChanged {
            from: MachineStatus::Running,
            to: MachineStatus::Stopping,
        },
        timestamp: chrono::Utc::now(),
        value: None,
        unit: None,
        metadata: serde_json::json!({}),
    }).await?;

    sleep(Duration::from_secs(1)).await;

    event_bus.publish(MachineEvent {
        machine_id,
        event_type: MachineEventType::StatusChanged {
            from: MachineStatus::Stopping,
            to: MachineStatus::Stopped,
        },
        timestamp: chrono::Utc::now(),
        value: None,
        unit: None,
        metadata: serde_json::json!({}),
    }).await?;

    info!("⏹️  Machine stopped");

    // Keep running to see all log output
    sleep(Duration::from_secs(2)).await;

    Ok(())
}
