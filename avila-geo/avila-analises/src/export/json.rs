use super::{DataExporter, ExportError, ExportFormat, ExportResult};
use crate::models::BehaviorEvent;
use async_trait::async_trait;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub struct JsonExporter {
    pretty: bool,
    jsonlines: bool,
}

impl JsonExporter {
    pub fn new() -> Self {
        Self {
            pretty: false,
            jsonlines: false,
        }
    }

    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }

    pub fn jsonlines(mut self) -> Self {
        self.jsonlines = true;
        self
    }
}

impl Default for JsonExporter {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DataExporter for JsonExporter {
    async fn export(
        &self,
        events: Vec<BehaviorEvent>,
        output_path: &Path,
    ) -> Result<ExportResult, ExportError> {
        let mut file = tokio::fs::File::create(output_path).await?;

        let record_count = events.len();

        if self.jsonlines {
            // JSON Lines format (one JSON object per line)
            for event in &events {
                let json = serde_json::to_string(event)?;
                file.write_all(json.as_bytes()).await?;
                file.write_all(b"\n").await?;
            }
        } else {
            // Standard JSON array
            let json = if self.pretty {
                serde_json::to_string_pretty(&events)?
            } else {
                serde_json::to_string(&events)?
            };
            file.write_all(json.as_bytes()).await?;
        }

        file.flush().await?;

        let file_size = tokio::fs::metadata(output_path).await?.len();

        Ok(ExportResult {
            file_path: output_path.display().to_string(),
            record_count,
            file_size_bytes: file_size,
            format: if self.jsonlines {
                ExportFormat::JsonLines
            } else {
                ExportFormat::Json
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;
    use chrono::Utc;
    use std::collections::HashMap;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_json_export() {
        let exporter = JsonExporter::new();
        
        let event = BehaviorEvent {
            event_id: Uuid::new_v4().to_string(),
            user_id: "user1".to_string(),
            session_id: "session1".to_string(),
            timestamp: Utc::now(),
            event_type: EventType::PageView {
                url: "/test".to_string(),
                title: "Test".to_string(),
                duration_ms: 1000,
            },
            metadata: HashMap::new(),
            context: EventContext {
                device: DeviceInfo {
                    device_type: DeviceType::Desktop,
                    os: "Windows".to_string(),
                    browser: "Chrome".to_string(),
                    screen_resolution: (1920, 1080),
                },
                location: LocationInfo {
                    country: "BR".to_string(),
                    city: Some("São Paulo".to_string()),
                    timezone: "America/Sao_Paulo".to_string(),
                    ip_address: "127.0.0.1".to_string(),
                },
                referrer: None,
                user_agent: "test".to_string(),
                viewport: Viewport { width: 1920, height: 1080 },
            },
        };

        let temp_path = std::env::temp_dir().join("test_export.json");
        let result = exporter.export(vec![event], &temp_path).await.unwrap();

        assert_eq!(result.record_count, 1);
        assert!(result.file_size_bytes > 0);
    }
}
