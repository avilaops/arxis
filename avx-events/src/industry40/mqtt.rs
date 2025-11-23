//! MQTT bridge for Industry 4.0 sensor integration
//!
//! Bridges MQTT topics to EventBus, commonly used for IoT sensors in manufacturing.

use crate::{EventBus, Result};
use rumqttc::{AsyncClient, Event as MqttEvent, EventLoop, MqttOptions, QoS};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::{debug, error, info, warn};

/// MQTT bridge configuration
#[derive(Debug, Clone)]
pub struct MqttBridgeConfig {
    pub broker_host: String,
    pub broker_port: u16,
    pub client_id: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub keep_alive: Duration,
    pub qos: QoS,
}

impl Default for MqttBridgeConfig {
    fn default() -> Self {
        Self {
            broker_host: "localhost".into(),
            broker_port: 1883,
            client_id: "avx-events-mqtt-bridge".into(),
            username: None,
            password: None,
            keep_alive: Duration::from_secs(30),
            qos: QoS::AtLeastOnce,
        }
    }
}

/// MQTT to EventBus bridge
pub struct MqttBridge {
    client: AsyncClient,
    event_loop: EventLoop,
    event_bus: Arc<EventBus>,
    config: MqttBridgeConfig,
}

impl MqttBridge {
    /// Create a new MQTT bridge
    pub fn new(event_bus: Arc<EventBus>, config: MqttBridgeConfig) -> Result<Self> {
        let mut mqtt_options = MqttOptions::new(
            &config.client_id,
            &config.broker_host,
            config.broker_port,
        );

        mqtt_options.set_keep_alive(config.keep_alive);

        if let (Some(username), Some(password)) = (&config.username, &config.password) {
            mqtt_options.set_credentials(username, password);
        }

        let (client, event_loop) = AsyncClient::new(mqtt_options, 100);

        info!(
            broker = %format!("{}:{}", config.broker_host, config.broker_port),
            client_id = %config.client_id,
            "MQTT bridge created"
        );

        Ok(Self {
            client,
            event_loop,
            event_bus,
            config,
        })
    }

    /// Subscribe to MQTT topic and forward to EventBus
    pub async fn subscribe(&self, topic: impl Into<String>) -> Result<()> {
        let topic = topic.into();
        self.client
            .subscribe(&topic, self.config.qos)
            .await
            .map_err(|e| crate::Error::internal(format!("MQTT subscribe error: {}", e)))?;

        info!(topic = %topic, "Subscribed to MQTT topic");
        Ok(())
    }

    /// Publish EventBus event to MQTT topic
    pub async fn publish<T: Serialize>(
        &self,
        topic: impl Into<String>,
        payload: &T,
    ) -> Result<()> {
        let topic = topic.into();
        let payload = serde_json::to_vec(payload)
            .map_err(|e| crate::Error::serialization(e.to_string()))?;

        self.client
            .publish(&topic, self.config.qos, false, payload)
            .await
            .map_err(|e| crate::Error::internal(format!("MQTT publish error: {}", e)))?;

        debug!(topic = %topic, "Published to MQTT");
        Ok(())
    }

    /// Start the bridge event loop
    pub fn start(mut self) -> JoinHandle<()> {
        let event_bus = self.event_bus.clone();

        tokio::spawn(async move {
            info!("MQTT bridge event loop started");

            loop {
                match self.event_loop.poll().await {
                    Ok(notification) => {
                        if let MqttEvent::Incoming(packet) = notification {
                            Self::handle_mqtt_packet_static(&event_bus, packet).await;
                        }
                    }
                    Err(e) => {
                        error!(error = %e, "MQTT event loop error");
                        tokio::time::sleep(Duration::from_secs(5)).await;
                    }
                }
            }
        })
    }

    async fn handle_mqtt_packet_static(event_bus: &Arc<EventBus>, packet: rumqttc::Packet) {
        use rumqttc::Packet;

        match packet {
            Packet::Publish(publish) => {
                let topic = publish.topic.clone();
                let payload = publish.payload.to_vec();

                debug!(topic = %topic, size = payload.len(), "Received MQTT message");

                // Try to parse as JSON and publish to EventBus
                if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&payload) {
                    // Create a generic MQTT event
                    let event = MqttMessage {
                        topic: topic.clone(),
                        payload: json,
                        timestamp: chrono::Utc::now(),
                    };

                    if let Err(e) = event_bus.publish(event).await {
                        error!(topic = %topic, error = %e, "Failed to publish MQTT message to EventBus");
                    }
                } else {
                    warn!(topic = %topic, "Failed to parse MQTT payload as JSON");
                }
            }
            Packet::ConnAck(_) => {
                info!("Connected to MQTT broker");
            }
            Packet::Disconnect => {
                warn!("Disconnected from MQTT broker");
            }
            _ => {}
        }
    }
}

/// Generic MQTT message event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqttMessage {
    pub topic: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl crate::Event for MqttMessage {
    fn event_type(&self) -> &'static str {
        "mqtt.message"
    }

    fn aggregate_id(&self) -> String {
        self.topic.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mqtt_config_default() {
        let config = MqttBridgeConfig::default();
        assert_eq!(config.broker_host, "localhost");
        assert_eq!(config.broker_port, 1883);
    }
}
