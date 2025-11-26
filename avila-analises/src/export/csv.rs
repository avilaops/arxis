use super::{DataExporter, ExportError, ExportFormat, ExportResult};
use crate::models::BehaviorEvent;
use async_trait::async_trait;
use serde::Serialize;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub struct CsvExporter;

impl CsvExporter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CsvExporter {
    fn default() -> Self {
        Self::new()
    }
}

/// Estrutura flat para CSV
#[derive(Debug, Serialize)]
struct CsvRecord {
    event_id: String,
    user_id: String,
    session_id: String,
    timestamp: String,
    event_type: String,
    event_data: String,
    device_type: String,
    os: String,
    browser: String,
    country: String,
    city: String,
}

impl From<&BehaviorEvent> for CsvRecord {
    fn from(event: &BehaviorEvent) -> Self {
        let event_type_name = format!("{:?}", event.event_type);
        let event_data = serde_json::to_string(&event.event_type)
            .unwrap_or_else(|_| "{}".to_string());

        CsvRecord {
            event_id: event.event_id.clone(),
            user_id: event.user_id.clone(),
            session_id: event.session_id.clone(),
            timestamp: event.timestamp.to_rfc3339(),
            event_type: event_type_name,
            event_data,
            device_type: format!("{:?}", event.context.device.device_type),
            os: event.context.device.os.clone(),
            browser: event.context.device.browser.clone(),
            country: event.context.location.country.clone(),
            city: event.context.location.city.clone().unwrap_or_default(),
        }
    }
}

#[async_trait]
impl DataExporter for CsvExporter {
    async fn export(
        &self,
        events: Vec<BehaviorEvent>,
        output_path: &Path,
    ) -> Result<ExportResult, ExportError> {
        let mut wtr = csv::Writer::from_path(output_path)?;

        let record_count = events.len();

        for event in &events {
            let record = CsvRecord::from(event);
            wtr.serialize(record)?;
        }

        wtr.flush()?;

        let file_size = tokio::fs::metadata(output_path).await?.len();

        Ok(ExportResult {
            file_path: output_path.display().to_string(),
            record_count,
            file_size_bytes: file_size,
            format: ExportFormat::Csv,
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
    async fn test_csv_export() {
        let exporter = CsvExporter::new();
        
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

        let temp_path = std::env::temp_dir().join("test_export.csv");
        let result = exporter.export(vec![event], &temp_path).await.unwrap();

        assert_eq!(result.record_count, 1);
        assert!(result.file_size_bytes > 0);
    }
}
