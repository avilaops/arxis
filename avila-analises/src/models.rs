use chrono::{DateTime, Datelike, Utc, IsoWeek};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

// ==================== Eventos de Comportamento ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorEvent {
    pub event_id: String,
    pub user_id: String,
    pub session_id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub metadata: HashMap<String, String>,
    pub context: EventContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventType {
    PageView {
        url: String,
        title: String,
        duration_ms: u64,
    },
    Click {
        element_id: String,
        element_class: String,
        x: i32,
        y: i32,
    },
    Scroll {
        depth_percent: f32,
        max_depth: f32,
    },
    Search {
        query: String,
        results_count: usize,
    },
    Purchase {
        product_id: String,
        amount: f64,
        currency: String,
    },
    AddToCart {
        product_id: String,
        quantity: u32,
    },
    RemoveFromCart {
        product_id: String,
    },
    FormSubmit {
        form_id: String,
        fields: Vec<String>,
    },
    VideoPlay {
        video_id: String,
        position_ms: u64,
    },
    VideoComplete {
        video_id: String,
        completion_rate: f32,
    },
    Download {
        file_id: String,
        file_type: String,
    },
    Share {
        content_id: String,
        platform: String,
    },
    Custom {
        name: String,
        properties: serde_json::Value,
    },
}

// ==================== Contexto ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventContext {
    pub device: DeviceInfo,
    pub location: LocationInfo,
    pub referrer: Option<String>,
    pub user_agent: String,
    pub viewport: Viewport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub device_type: DeviceType,
    pub os: String,
    pub browser: String,
    pub screen_resolution: (u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DeviceType {
    Desktop,
    Mobile,
    Tablet,
    Tv,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationInfo {
    pub country: String,
    pub city: Option<String>,
    pub timezone: String,
    pub ip_address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

// ==================== Perfil de Usuário ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub user_id: String,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
    pub total_sessions: usize,
    pub total_events: usize,

    pub behaviors: UserBehaviors,
    pub segments: Vec<String>,

    // Scores calculados
    pub engagement_score: f64,
    pub loyalty_score: f64,
    pub conversion_probability: f64,
    pub churn_risk: f64,

    pub interests: Vec<Interest>,
    pub preferred_categories: HashMap<String, f64>,
    pub browsing_patterns: BrowsingPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBehaviors {
    pub avg_session_duration_seconds: f64,
    pub avg_pages_per_session: f64,
    pub bounce_rate: f64,
    pub conversion_rate: f64,

    pub most_active_hours: Vec<u8>,
    pub most_active_days: Vec<u8>,

    pub total_purchases: usize,
    pub total_spent: f64,
    pub avg_order_value: f64,
    pub days_since_last_purchase: Option<i64>,

    pub pages_viewed: HashSet<String>,
    pub search_queries: Vec<String>,
    pub clicked_products: Vec<String>,
}

impl Default for UserBehaviors {
    fn default() -> Self {
        Self {
            avg_session_duration_seconds: 0.0,
            avg_pages_per_session: 0.0,
            bounce_rate: 0.0,
            conversion_rate: 0.0,
            most_active_hours: Vec::new(),
            most_active_days: Vec::new(),
            total_purchases: 0,
            total_spent: 0.0,
            avg_order_value: 0.0,
            days_since_last_purchase: None,
            pages_viewed: HashSet::new(),
            search_queries: Vec::new(),
            clicked_products: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interest {
    pub category: String,
    pub score: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowsingPatterns {
    pub typical_path: Vec<String>,
    pub entry_pages: HashMap<String, usize>,
    pub exit_pages: HashMap<String, usize>,
    pub navigation_style: NavigationStyle,
}

impl Default for BrowsingPatterns {
    fn default() -> Self {
        Self {
            typical_path: Vec::new(),
            entry_pages: HashMap::new(),
            exit_pages: HashMap::new(),
            navigation_style: NavigationStyle::Explorer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NavigationStyle {
    Explorer,    // Navega muito, explora várias páginas
    Focused,     // Vai direto ao ponto
    Researcher,  // Pesquisa muito antes de comprar
    Impulsive,   // Compra rápido
}

// ==================== Sessão ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub session_id: String,
    pub user_id: String,
    pub start_time: DateTime<Utc>,
    pub last_activity: DateTime<Utc>,
    pub events: Vec<BehaviorEvent>,
    pub page_sequence: Vec<String>,
    pub total_duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetrics {
    pub duration: u64,
    pub page_views: usize,
    pub bounce: bool,
    pub converted: bool,
}

// ==================== Funnel ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Funnel {
    pub name: String,
    pub steps: Vec<FunnelStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunnelStep {
    pub name: String,
    pub condition: FunnelCondition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FunnelCondition {
    PageView(String),
    EventType(String),
    PurchaseCompleted,
    AddedToCart,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FunnelAnalysis {
    pub total_entered: usize,
    pub step_conversions: Vec<StepConversion>,
    pub avg_time_between_steps: Vec<f64>,
    pub drop_off_points: Vec<(usize, f64)>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StepConversion {
    pub step_index: usize,
    pub users_entered: usize,
    pub users_completed: usize,
    pub conversion_rate: f64,
    pub avg_time_to_next: f64,
}

// ==================== Cohort ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cohort {
    pub cohort_id: String,
    pub period: CohortPeriod,
    pub users: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CohortPeriod {
    Daily(DateTime<Utc>),
    Weekly(u32, i32),
    Monthly(u32, i32),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CohortAnalysis {
    pub cohorts: Vec<CohortMetrics>,
    pub retention_rates: Vec<Vec<f64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CohortMetrics {
    pub cohort_id: String,
    pub size: usize,
    pub retention_by_period: Vec<f64>,
    pub revenue_by_period: Vec<f64>,
    pub engagement_by_period: Vec<f64>,
}

// ==================== Segmentação ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Segment {
    pub id: String,
    pub name: String,
    pub description: String,
    pub rules: Vec<SegmentRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SegmentRule {
    BehaviorCount {
        event_type: String,
        operator: Operator,
        threshold: usize,
    },
    TimeRange {
        field: String,
        days: u32,
    },
    ValueRange {
        field: String,
        min: f64,
        max: f64,
    },
    Category {
        field: String,
        values: Vec<String>,
    },
    Computed {
        score_type: ScoreType,
        operator: Operator,
        threshold: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    GreaterThan,
    LessThan,
    Equals,
    Between,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScoreType {
    Engagement,
    Loyalty,
    ChurnRisk,
    ConversionProbability,
}

// ==================== Dashboard ====================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealTimeStats {
    pub active_users_now: usize,
    pub events_per_second: f64,
    pub top_pages: Vec<(String, usize)>,
    pub conversion_rate_today: f64,
    pub revenue_today: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub name: String,
    pub condition: AlertCondition,
    pub threshold: f64,
    pub action: AlertAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertCondition {
    DropInConversion,
    SpikeInTraffic,
    HighBounceRate,
    AnomalousPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertAction {
    SendEmail(String),
    SendWebhook(String),
    Log,
}
