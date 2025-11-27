use crate::models::*;
use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

// ==================== Event Store ====================

#[derive(Clone)]
pub struct EventStore {
    events: Arc<DashMap<String, Vec<BehaviorEvent>>>,
}

impl EventStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(DashMap::new()),
        }
    }

    pub async fn store(&self, event: BehaviorEvent) -> Result<()> {
        let user_id = event.user_id.clone();
        self.events
            .entry(user_id)
            .or_insert_with(Vec::new)
            .push(event);
        Ok(())
    }

    pub fn get_user_events(&self, user_id: &str) -> Vec<BehaviorEvent> {
        self.events
            .get(user_id)
            .map(|e| e.value().clone())
            .unwrap_or_default()
    }

    pub fn get_all_events(&self) -> Vec<BehaviorEvent> {
        self.events
            .iter()
            .flat_map(|entry| entry.value().clone())
            .collect()
    }

    pub fn get_events_in_range(
        &self,
        start: chrono::DateTime<Utc>,
        end: chrono::DateTime<Utc>,
    ) -> Vec<BehaviorEvent> {
        self.get_all_events()
            .into_iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect()
    }
}

// ==================== Session Manager ====================

pub struct SessionManager {
    active_sessions: Arc<DashMap<String, Session>>,
    session_timeout: Duration,
}

impl SessionManager {
    pub fn new(session_timeout_minutes: i64) -> Self {
        Self {
            active_sessions: Arc::new(DashMap::new()),
            session_timeout: Duration::minutes(session_timeout_minutes),
        }
    }

    pub async fn update_session(&self, event: &BehaviorEvent) -> Result<()> {
        let session_id = event.session_id.clone();

        if let Some(mut session) = self.active_sessions.get_mut(&session_id) {
            // Atualizar sessão existente
            session.last_activity = event.timestamp;
            session.events.push(event.clone());

            if let EventType::PageView { url, .. } = &event.event_type {
                session.page_sequence.push(url.clone());
            }

            let duration = (event.timestamp - session.start_time).num_milliseconds();
            session.total_duration_ms = duration.max(0) as u64;
        } else {
            // Criar nova sessão
            let session = Session {
                session_id: session_id.clone(),
                user_id: event.user_id.clone(),
                start_time: event.timestamp,
                last_activity: event.timestamp,
                events: vec![event.clone()],
                page_sequence: if let EventType::PageView { url, .. } = &event.event_type {
                    vec![url.clone()]
                } else {
                    Vec::new()
                },
                total_duration_ms: 0,
            };
            self.active_sessions.insert(session_id, session);
        }

        Ok(())
    }

    pub fn get_session(&self, session_id: &str) -> Option<Session> {
        self.active_sessions.get(session_id).map(|s| s.value().clone())
    }

    pub fn is_session_expired(&self, session: &Session) -> bool {
        Utc::now().signed_duration_since(session.last_activity) > self.session_timeout
    }

    pub fn calculate_session_metrics(&self, session: &Session) -> SessionMetrics {
        SessionMetrics {
            duration: session.total_duration_ms,
            page_views: session.page_sequence.len(),
            bounce: session.page_sequence.len() == 1,
            converted: session.events.iter().any(|e| {
                matches!(e.event_type, EventType::Purchase { .. })
            }),
        }
    }

    pub async fn cleanup_expired_sessions(&self) -> usize {
        let now = Utc::now();
        let mut removed = 0;

        self.active_sessions.retain(|_, session| {
            let expired = now.signed_duration_since(session.last_activity) > self.session_timeout;
            if expired {
                removed += 1;
            }
            !expired
        });

        removed
    }
}

// ==================== Real-Time Processor ====================

pub struct RealTimeProcessor {
    event_counter: Arc<RwLock<EventCounter>>,
}

#[derive(Debug, Default)]
struct EventCounter {
    total_events: usize,
    events_by_type: std::collections::HashMap<String, usize>,
    last_reset: chrono::DateTime<Utc>,
}

impl RealTimeProcessor {
    pub fn new() -> Self {
        Self {
            event_counter: Arc::new(RwLock::new(EventCounter {
                total_events: 0,
                events_by_type: std::collections::HashMap::new(),
                last_reset: Utc::now(),
            })),
        }
    }

    pub async fn process(&self, event: &BehaviorEvent) -> Result<()> {
        let mut counter = self.event_counter.write().await;
        counter.total_events += 1;

        let event_type_name = match &event.event_type {
            EventType::PageView { .. } => "page_view",
            EventType::Click { .. } => "click",
            EventType::Purchase { .. } => "purchase",
            EventType::Search { .. } => "search",
            EventType::AddToCart { .. } => "add_to_cart",
            EventType::RemoveFromCart { .. } => "remove_from_cart",
            EventType::FormSubmit { .. } => "form_submit",
            EventType::VideoPlay { .. } => "video_play",
            EventType::VideoComplete { .. } => "video_complete",
            EventType::Download { .. } => "download",
            EventType::Share { .. } => "share",
            EventType::Scroll { .. } => "scroll",
            EventType::Custom { name, .. } => name.as_str(),
        };

        *counter.events_by_type.entry(event_type_name.to_string()).or_insert(0) += 1;

        debug!("Processed event: {} for user: {}", event_type_name, event.user_id);
        Ok(())
    }

    pub async fn get_stats(&self) -> (usize, std::collections::HashMap<String, usize>) {
        let counter = self.event_counter.read().await;
        (counter.total_events, counter.events_by_type.clone())
    }

    pub async fn reset_stats(&self) {
        let mut counter = self.event_counter.write().await;
        counter.total_events = 0;
        counter.events_by_type.clear();
        counter.last_reset = Utc::now();
    }
}

// ==================== Behavior Tracker ====================

pub struct BehaviorTracker {
    event_store: EventStore,
    session_manager: SessionManager,
    real_time_processor: RealTimeProcessor,
}

impl BehaviorTracker {
    pub fn new(session_timeout_minutes: i64) -> Self {
        Self {
            event_store: EventStore::new(),
            session_manager: SessionManager::new(session_timeout_minutes),
            real_time_processor: RealTimeProcessor::new(),
        }
    }

    pub async fn track_event(&mut self, mut event: BehaviorEvent) -> Result<()> {
        // Validar evento
        self.validate_event(&event)?;

        // Enriquecer com contexto adicional
        event = self.enrich_event(event).await?;

        info!("Tracking event: {:?} for user: {}",
              std::mem::discriminant(&event.event_type), event.user_id);

        // Processar em tempo real
        self.real_time_processor.process(&event).await?;

        // Armazenar
        self.event_store.store(event.clone()).await?;

        // Atualizar sessão
        self.session_manager.update_session(&event).await?;

        Ok(())
    }

    fn validate_event(&self, event: &BehaviorEvent) -> Result<()> {
        if event.user_id.is_empty() {
            anyhow::bail!("user_id cannot be empty");
        }
        if event.session_id.is_empty() {
            anyhow::bail!("session_id cannot be empty");
        }
        Ok(())
    }

    async fn enrich_event(&self, mut event: BehaviorEvent) -> Result<BehaviorEvent> {
        // Adicionar timestamp se não existir
        if event.event_id.is_empty() {
            event.event_id = Uuid::new_v4().to_string();
        }

        // Enriquecer com dados de sessão
        if let Some(session) = self.session_manager.get_session(&event.session_id) {
            event.metadata.insert(
                "session_start".to_string(),
                session.start_time.to_rfc3339(),
            );
            event.metadata.insert(
                "session_duration".to_string(),
                session.total_duration_ms.to_string(),
            );
        }

        Ok(event)
    }

    pub fn get_event_store(&self) -> &EventStore {
        &self.event_store
    }

    pub fn get_session_manager(&self) -> &SessionManager {
        &self.session_manager
    }

    pub async fn get_real_time_stats(&self) -> (usize, std::collections::HashMap<String, usize>) {
        self.real_time_processor.get_stats().await
    }
}

// ==================== User Agent Parser ====================

pub fn parse_user_agent(user_agent: &str) -> DeviceInfo {
    let ua = user_agent.to_lowercase();

    let device_type = if ua.contains("mobile") || ua.contains("android") || ua.contains("iphone") {
        DeviceType::Mobile
    } else if ua.contains("tablet") || ua.contains("ipad") {
        DeviceType::Tablet
    } else if ua.contains("tv") || ua.contains("smarttv") {
        DeviceType::Tv
    } else if ua.contains("windows") || ua.contains("macintosh") || ua.contains("linux") {
        DeviceType::Desktop
    } else {
        DeviceType::Unknown
    };

    let os = if ua.contains("windows") {
        "Windows".to_string()
    } else if ua.contains("macintosh") || ua.contains("mac os") {
        "macOS".to_string()
    } else if ua.contains("android") {
        "Android".to_string()
    } else if ua.contains("iphone") || ua.contains("ipad") {
        "iOS".to_string()
    } else if ua.contains("linux") {
        "Linux".to_string()
    } else {
        "Unknown".to_string()
    };

    let browser = if ua.contains("chrome") && !ua.contains("edg") {
        "Chrome".to_string()
    } else if ua.contains("firefox") {
        "Firefox".to_string()
    } else if ua.contains("safari") && !ua.contains("chrome") {
        "Safari".to_string()
    } else if ua.contains("edg") {
        "Edge".to_string()
    } else if ua.contains("opera") || ua.contains("opr") {
        "Opera".to_string()
    } else {
        "Unknown".to_string()
    };

    DeviceInfo {
        device_type,
        os,
        browser,
        screen_resolution: (1920, 1080), // Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_tracking() {
        let mut tracker = BehaviorTracker::new(30);

        let event = BehaviorEvent {
            event_id: Uuid::new_v4().to_string(),
            user_id: "user123".to_string(),
            session_id: "session456".to_string(),
            timestamp: Utc::now(),
            event_type: EventType::PageView {
                url: "/home".to_string(),
                title: "Home".to_string(),
                duration_ms: 5000,
            },
            metadata: std::collections::HashMap::new(),
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
                    ip_address: "192.168.1.1".to_string(),
                },
                referrer: None,
                user_agent: "Mozilla/5.0".to_string(),
                viewport: Viewport {
                    width: 1920,
                    height: 1080,
                },
            },
        };

        let result = tracker.track_event(event).await;
        assert!(result.is_ok());

        let events = tracker.get_event_store().get_user_events("user123");
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_user_agent_parsing() {
        let ua = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36";
        let device = parse_user_agent(ua);

        assert_eq!(device.device_type, DeviceType::Desktop);
        assert_eq!(device.os, "Windows");
        assert_eq!(device.browser, "Chrome");
    }
}
