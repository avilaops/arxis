use super::{EventStore, EventFilter, QueryOptions, OrderBy, StorageStats, StorageError};
use crate::models::BehaviorEvent;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::Arc;

/// Implementação in-memory do EventStore (para desenvolvimento/testes)
#[derive(Clone)]
pub struct InMemoryStore {
    events: Arc<DashMap<String, BehaviorEvent>>,
    user_index: Arc<DashMap<String, Vec<String>>>,
    session_index: Arc<DashMap<String, Vec<String>>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            events: Arc::new(DashMap::new()),
            user_index: Arc::new(DashMap::new()),
            session_index: Arc::new(DashMap::new()),
        }
    }

    pub fn get_all_events(&self) -> Vec<BehaviorEvent> {
        self.events
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    fn build_indexes(&self, event: &BehaviorEvent) {
        // Index por user_id
        self.user_index
            .entry(event.user_id.clone())
            .or_insert_with(Vec::new)
            .push(event.event_id.clone());

        // Index por session_id
        self.session_index
            .entry(event.session_id.clone())
            .or_insert_with(Vec::new)
            .push(event.event_id.clone());
    }

    fn filter_events(&self, events: Vec<BehaviorEvent>, filter: &EventFilter) -> Vec<BehaviorEvent> {
        events
            .into_iter()
            .filter(|event| {
                // Filtro por user_ids
                if let Some(ref user_ids) = filter.user_ids {
                    if !user_ids.contains(&event.user_id) {
                        return false;
                    }
                }

                // Filtro por session_ids
                if let Some(ref session_ids) = filter.session_ids {
                    if !session_ids.contains(&event.session_id) {
                        return false;
                    }
                }

                // Filtro por event_types
                if let Some(ref event_types) = filter.event_types {
                    let event_type_name = match &event.event_type {
                        crate::models::EventType::PageView { .. } => "PageView",
                        crate::models::EventType::Click { .. } => "Click",
                        crate::models::EventType::Purchase { .. } => "Purchase",
                        crate::models::EventType::AddToCart { .. } => "AddToCart",
                        crate::models::EventType::Search { .. } => "Search",
                        _ => "Other",
                    };
                    if !event_types.contains(&event_type_name.to_string()) {
                        return false;
                    }
                }

                // Filtro por tempo
                if let Some(start) = filter.start_time {
                    if event.timestamp < start {
                        return false;
                    }
                }
                if let Some(end) = filter.end_time {
                    if event.timestamp > end {
                        return false;
                    }
                }

                // Filtro por country
                if let Some(ref countries) = filter.countries {
                    if !countries.contains(&event.context.location.country) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    fn sort_events(&self, mut events: Vec<BehaviorEvent>, order: &OrderBy) -> Vec<BehaviorEvent> {
        match order {
            OrderBy::TimestampDesc => {
                events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            }
            OrderBy::TimestampAsc => {
                events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            }
            OrderBy::UserIdAsc => {
                events.sort_by(|a, b| a.user_id.cmp(&b.user_id));
            }
            OrderBy::EventTypeAsc => {
                events.sort_by(|a, b| {
                    format!("{:?}", a.event_type).cmp(&format!("{:?}", b.event_type))
                });
            }
        }
        events
    }

    fn apply_pagination(&self, events: Vec<BehaviorEvent>, options: &QueryOptions) -> Vec<BehaviorEvent> {
        let offset = options.offset.unwrap_or(0);
        let limit = options.limit.unwrap_or(100);

        events.into_iter().skip(offset).take(limit).collect()
    }
}

impl Default for InMemoryStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventStore for InMemoryStore {
    async fn store(&self, event: BehaviorEvent) -> Result<(), StorageError> {
        self.build_indexes(&event);
        self.events.insert(event.event_id.clone(), event);
        Ok(())
    }

    async fn store_batch(&self, events: Vec<BehaviorEvent>) -> Result<(), StorageError> {
        for event in events {
            self.store(event).await?;
        }
        Ok(())
    }

    async fn query(&self, filter: EventFilter) -> Result<Vec<BehaviorEvent>, StorageError> {
        let all_events = self.get_all_events();
        let filtered = self.filter_events(all_events, &filter);
        Ok(filtered)
    }

    async fn get_by_user(&self, user_id: &str, options: QueryOptions) -> Result<Vec<BehaviorEvent>, StorageError> {
        let event_ids = self.user_index.get(user_id);

        let events: Vec<BehaviorEvent> = match event_ids {
            Some(ids) => ids
                .iter()
                .filter_map(|id| self.events.get(id).map(|e| e.clone()))
                .collect(),
            None => Vec::new(),
        };

        let sorted = self.sort_events(events, &options.order_by);
        let paginated = self.apply_pagination(sorted, &options);

        Ok(paginated)
    }

    async fn get_by_session(&self, session_id: &str) -> Result<Vec<BehaviorEvent>, StorageError> {
        let event_ids = self.session_index.get(session_id);

        let events: Vec<BehaviorEvent> = match event_ids {
            Some(ids) => ids
                .iter()
                .filter_map(|id| self.events.get(id).map(|e| e.clone()))
                .collect(),
            None => Vec::new(),
        };

        Ok(events)
    }

    async fn get_by_time_range(
        &self,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
        options: QueryOptions,
    ) -> Result<Vec<BehaviorEvent>, StorageError> {
        let all_events = self.get_all_events();

        let filtered: Vec<BehaviorEvent> = all_events
            .into_iter()
            .filter(|e| e.timestamp >= start && e.timestamp <= end)
            .collect();

        let sorted = self.sort_events(filtered, &options.order_by);
        let paginated = self.apply_pagination(sorted, &options);

        Ok(paginated)
    }

    async fn count(&self, filter: Option<EventFilter>) -> Result<usize, StorageError> {
        match filter {
            Some(f) => {
                let events = self.query(f).await?;
                Ok(events.len())
            }
            None => Ok(self.events.len()),
        }
    }

    async fn count_by_type(&self) -> Result<HashMap<String, usize>, StorageError> {
        let mut counts: HashMap<String, usize> = HashMap::new();

        for entry in self.events.iter() {
            let event_type = match &entry.value().event_type {
                crate::models::EventType::PageView { .. } => "PageView",
                crate::models::EventType::Click { .. } => "Click",
                crate::models::EventType::Purchase { .. } => "Purchase",
                crate::models::EventType::AddToCart { .. } => "AddToCart",
                crate::models::EventType::RemoveFromCart { .. } => "RemoveFromCart",
                crate::models::EventType::Search { .. } => "Search",
                crate::models::EventType::Scroll { .. } => "Scroll",
                crate::models::EventType::FormSubmit { .. } => "FormSubmit",
                crate::models::EventType::VideoPlay { .. } => "VideoPlay",
                crate::models::EventType::VideoComplete { .. } => "VideoComplete",
                crate::models::EventType::Download { .. } => "Download",
                crate::models::EventType::Share { .. } => "Share",
                crate::models::EventType::Custom { .. } => "Custom",
            };

            *counts.entry(event_type.to_string()).or_insert(0) += 1;
        }

        Ok(counts)
    }

    async fn delete_older_than(&self, timestamp: DateTime<Utc>) -> Result<usize, StorageError> {
        let to_delete: Vec<String> = self
            .events
            .iter()
            .filter(|entry| entry.value().timestamp < timestamp)
            .map(|entry| entry.key().clone())
            .collect();

        let count = to_delete.len();

        for event_id in to_delete {
            self.events.remove(&event_id);
        }

        Ok(count)
    }

    async fn get_stats(&self) -> Result<StorageStats, StorageError> {
        let total_events = self.events.len();
        let total_users = self.user_index.len();
        let total_sessions = self.session_index.len();

        let mut oldest: Option<DateTime<Utc>> = None;
        let mut newest: Option<DateTime<Utc>> = None;

        for entry in self.events.iter() {
            let ts = entry.value().timestamp;
            if oldest.is_none() || ts < oldest.unwrap() {
                oldest = Some(ts);
            }
            if newest.is_none() || ts > newest.unwrap() {
                newest = Some(ts);
            }
        }

        let events_by_type = self.count_by_type().await?;

        Ok(StorageStats {
            total_events,
            total_users,
            total_sessions,
            storage_size_bytes: total_events * 1024, // estimativa
            oldest_event: oldest,
            newest_event: newest,
            events_by_type,
        })
    }

    async fn health_check(&self) -> Result<(), StorageError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{EventType, EventContext, DeviceInfo, LocationInfo, Viewport, DeviceType};
    use uuid::Uuid;

    fn create_test_event(user_id: &str, event_type: EventType) -> BehaviorEvent {
        BehaviorEvent {
            event_id: Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            session_id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
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
        }
    }

    #[tokio::test]
    async fn test_store_and_retrieve() {
        let store = InMemoryStore::new();
        let event = create_test_event("user1", EventType::PageView {
            url: "/home".to_string(),
            title: "Home".to_string(),
            duration_ms: 5000,
        });

        store.store(event.clone()).await.unwrap();

        let events = store.get_by_user("user1", QueryOptions::default()).await.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].user_id, "user1");
    }

    #[tokio::test]
    async fn test_count_by_type() {
        let store = InMemoryStore::new();

        store.store(create_test_event("user1", EventType::PageView {
            url: "/home".to_string(),
            title: "Home".to_string(),
            duration_ms: 5000,
        })).await.unwrap();

        store.store(create_test_event("user2", EventType::Purchase {
            product_id: "prod1".to_string(),
            amount: 100.0,
            currency: "BRL".to_string(),
        })).await.unwrap();

        let counts = store.count_by_type().await.unwrap();
        assert_eq!(counts.get("PageView"), Some(&1));
        assert_eq!(counts.get("Purchase"), Some(&1));
    }
}
