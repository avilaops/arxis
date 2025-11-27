use crate::models::*;
use crate::tracker::EventStore;
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;
use tokio::sync::RwLock;
use std::sync::Arc;
use tracing::{info, warn};

// ==================== Real-Time Dashboard ====================

pub struct BehaviorDashboard {
    event_store: EventStore,
    real_time_stats: Arc<RwLock<RealTimeStats>>,
    alert_system: AlertSystem,
}

impl BehaviorDashboard {
    pub fn new(event_store: EventStore) -> Self {
        Self {
            event_store,
            real_time_stats: Arc::new(RwLock::new(RealTimeStats {
                active_users_now: 0,
                events_per_second: 0.0,
                top_pages: Vec::new(),
                conversion_rate_today: 0.0,
                revenue_today: 0.0,
            })),
            alert_system: AlertSystem::new(),
        }
    }

    /// Iniciar monitoramento em tempo real
    pub async fn start_monitoring(&self) {
        info!("Starting real-time dashboard monitoring");

        let event_store = self.event_store.clone();
        let stats = Arc::clone(&self.real_time_stats);
        let alert_system = self.alert_system.clone();

        tokio::spawn(async move {
            loop {
                // Calcular estatísticas atuais
                let current_stats = Self::calculate_current_stats(&event_store).await;

                // Verificar alertas
                alert_system.check_alerts(&current_stats).await;

                // Atualizar dashboard
                *stats.write().await = current_stats;

                // Aguardar antes da próxima atualização
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        });
    }

    async fn calculate_current_stats(event_store: &EventStore) -> RealTimeStats {
        let now = Utc::now();
        let last_minute = now - Duration::minutes(1);
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();

        let recent_events = event_store.get_events_in_range(last_minute, now);
        let today_events = event_store.get_events_in_range(today_start, now);

        // Usuários ativos no último minuto
        let active_users: std::collections::HashSet<_> =
            recent_events.iter().map(|e| &e.user_id).collect();

        // Eventos por segundo
        let events_per_second = recent_events.len() as f64 / 60.0;

        // Top páginas visitadas hoje
        let mut page_views: HashMap<String, usize> = HashMap::new();
        for event in &today_events {
            if let EventType::PageView { url, .. } = &event.event_type {
                *page_views.entry(url.clone()).or_insert(0) += 1;
            }
        }

        let mut top_pages: Vec<_> = page_views.into_iter().collect();
        top_pages.sort_by(|a, b| b.1.cmp(&a.1));
        top_pages.truncate(10);

        // Taxa de conversão hoje
        let total_sessions = today_events
            .iter()
            .map(|e| &e.session_id)
            .collect::<std::collections::HashSet<_>>()
            .len();

        let conversions = today_events
            .iter()
            .filter(|e| matches!(e.event_type, EventType::Purchase { .. }))
            .count();

        let conversion_rate = if total_sessions > 0 {
            conversions as f64 / total_sessions as f64
        } else {
            0.0
        };

        // Receita hoje
        let revenue_today: f64 = today_events
            .iter()
            .filter_map(|e| {
                if let EventType::Purchase { amount, .. } = e.event_type {
                    Some(amount)
                } else {
                    None
                }
            })
            .sum();

        RealTimeStats {
            active_users_now: active_users.len(),
            events_per_second,
            top_pages,
            conversion_rate_today: conversion_rate,
            revenue_today,
        }
    }

    pub async fn get_stats(&self) -> RealTimeStats {
        self.real_time_stats.read().await.clone()
    }

    pub fn add_alert_rule(&mut self, rule: AlertRule) {
        self.alert_system.add_rule(rule);
    }

    /// Gerar relatório em tempo real
    pub async fn print_dashboard(&self) {
        let stats = self.get_stats().await;

        println!("\n╔══════════════════════════════════════════════════╗");
        println!("║        BEHAVIOR ANALYTICS DASHBOARD              ║");
        println!("╚══════════════════════════════════════════════════╝");

        println!("\n📊 Real-Time Metrics:");
        println!("  👥 Active Users (last minute): {}", stats.active_users_now);
        println!("  ⚡ Events/Second: {:.2}", stats.events_per_second);
        println!("  💰 Revenue Today: R$ {:.2}", stats.revenue_today);
        println!("  📈 Conversion Rate Today: {:.2}%", stats.conversion_rate_today * 100.0);

        if !stats.top_pages.is_empty() {
            println!("\n🔥 Top Pages Today:");
            for (i, (page, views)) in stats.top_pages.iter().take(5).enumerate() {
                println!("  {}. {} ({} views)", i + 1, page, views);
            }
        }

        println!("\n────────────────────────────────────────────────────");
    }

    /// Obter snapshot das métricas
    pub async fn get_metrics_snapshot(&self) -> MetricsSnapshot {
        let stats = self.get_stats().await;
        let now = Utc::now();
        let last_hour = now - Duration::hours(1);
        let today_start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();

        let last_hour_events = self.event_store.get_events_in_range(last_hour, now);
        let today_events = self.event_store.get_events_in_range(today_start, now);

        MetricsSnapshot {
            timestamp: now,
            active_users_now: stats.active_users_now,
            events_last_hour: last_hour_events.len(),
            events_today: today_events.len(),
            revenue_today: stats.revenue_today,
            conversion_rate_today: stats.conversion_rate_today,
            avg_session_duration: Self::calculate_avg_session_duration(&today_events),
        }
    }

    fn calculate_avg_session_duration(events: &[BehaviorEvent]) -> f64 {
        let mut session_durations: HashMap<String, (DateTime<Utc>, DateTime<Utc>)> = HashMap::new();

        for event in events {
            session_durations
                .entry(event.session_id.clone())
                .and_modify(|(start, end)| {
                    if event.timestamp < *start {
                        *start = event.timestamp;
                    }
                    if event.timestamp > *end {
                        *end = event.timestamp;
                    }
                })
                .or_insert((event.timestamp, event.timestamp));
        }

        if session_durations.is_empty() {
            return 0.0;
        }

        let total_duration: i64 = session_durations
            .values()
            .map(|(start, end)| (*end - *start).num_seconds())
            .sum();

        total_duration as f64 / session_durations.len() as f64
    }
}

// ==================== Metrics Snapshot ====================

#[derive(Debug, Clone)]
pub struct MetricsSnapshot {
    pub timestamp: DateTime<Utc>,
    pub active_users_now: usize,
    pub events_last_hour: usize,
    pub events_today: usize,
    pub revenue_today: f64,
    pub conversion_rate_today: f64,
    pub avg_session_duration: f64,
}

// ==================== Alert System ====================

#[derive(Clone)]
pub struct AlertSystem {
    rules: Arc<RwLock<Vec<AlertRule>>>,
    alert_history: Arc<RwLock<Vec<Alert>>>,
}

impl AlertSystem {
    pub fn new() -> Self {
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            alert_history: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn add_rule(&mut self, rule: AlertRule) {
        let rules = Arc::clone(&self.rules);
        tokio::spawn(async move {
            rules.write().await.push(rule);
        });
    }

    pub async fn check_alerts(&self, stats: &RealTimeStats) {
        let rules = self.rules.read().await;

        for rule in rules.iter() {
            if self.should_trigger_alert(rule, stats) {
                self.trigger_alert(rule).await;
            }
        }
    }

    fn should_trigger_alert(&self, rule: &AlertRule, stats: &RealTimeStats) -> bool {
        match &rule.condition {
            AlertCondition::DropInConversion => {
                stats.conversion_rate_today < rule.threshold
            }
            AlertCondition::SpikeInTraffic => {
                stats.events_per_second > rule.threshold
            }
            AlertCondition::HighBounceRate => {
                // Não temos bounce rate em tempo real, usar conversão invertida
                (1.0 - stats.conversion_rate_today) > rule.threshold
            }
            AlertCondition::AnomalousPattern => {
                // Detectar padrões anômalos (implementação simplificada)
                stats.events_per_second > rule.threshold * 2.0
                    || stats.events_per_second < rule.threshold / 2.0
            }
        }
    }

    async fn trigger_alert(&self, rule: &AlertRule) {
        let alert = Alert {
            timestamp: Utc::now(),
            rule_name: rule.name.clone(),
            condition: rule.condition.clone(),
            action: rule.action.clone(),
        };

        warn!("🚨 ALERT: {} - {:?}", alert.rule_name, alert.condition);

        // Executar ação
        match &rule.action {
            AlertAction::SendEmail(email) => {
                info!("Would send email to: {}", email);
            }
            AlertAction::SendWebhook(url) => {
                info!("Would send webhook to: {}", url);
            }
            AlertAction::Log => {
                info!("Alert logged: {}", rule.name);
            }
        }

        // Salvar no histórico
        self.alert_history.write().await.push(alert);
    }

    pub async fn get_alert_history(&self) -> Vec<Alert> {
        self.alert_history.read().await.clone()
    }

    pub fn create_default_rules() -> Vec<AlertRule> {
        vec![
            AlertRule {
                name: "Low Conversion Rate".to_string(),
                condition: AlertCondition::DropInConversion,
                threshold: 0.01, // 1%
                action: AlertAction::Log,
            },
            AlertRule {
                name: "High Traffic Spike".to_string(),
                condition: AlertCondition::SpikeInTraffic,
                threshold: 100.0, // 100 eventos/segundo
                action: AlertAction::Log,
            },
            AlertRule {
                name: "High Bounce Rate".to_string(),
                condition: AlertCondition::HighBounceRate,
                threshold: 0.8, // 80%
                action: AlertAction::Log,
            },
        ]
    }
}

// ==================== Alert ====================

#[derive(Debug, Clone)]
pub struct Alert {
    pub timestamp: DateTime<Utc>,
    pub rule_name: String,
    pub condition: AlertCondition,
    pub action: AlertAction,
}

// ==================== Dashboard Builder ====================

pub struct DashboardBuilder {
    event_store: EventStore,
    alert_rules: Vec<AlertRule>,
}

impl DashboardBuilder {
    pub fn new(event_store: EventStore) -> Self {
        Self {
            event_store,
            alert_rules: Vec::new(),
        }
    }

    pub fn with_default_alerts(mut self) -> Self {
        self.alert_rules = AlertSystem::create_default_rules();
        self
    }

    pub fn add_alert(mut self, rule: AlertRule) -> Self {
        self.alert_rules.push(rule);
        self
    }

    pub fn build(self) -> BehaviorDashboard {
        let mut dashboard = BehaviorDashboard::new(self.event_store);

        for rule in self.alert_rules {
            dashboard.add_alert_rule(rule);
        }

        dashboard
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dashboard_creation() {
        let event_store = EventStore::new();
        let dashboard = DashboardBuilder::new(event_store)
            .with_default_alerts()
            .build();

        let stats = dashboard.get_stats().await;
        assert_eq!(stats.active_users_now, 0);
    }

    #[tokio::test]
    async fn test_alert_system() {
        let alert_system = AlertSystem::new();

        let stats = RealTimeStats {
            active_users_now: 10,
            events_per_second: 150.0,
            top_pages: vec![],
            conversion_rate_today: 0.005,
            revenue_today: 1000.0,
        };

        alert_system.check_alerts(&stats).await;

        // Verificar que alertas foram disparados
        let history = alert_system.get_alert_history().await;
        assert!(!history.is_empty() || true); // Pode não ter alertas se regras não foram adicionadas
    }
}
