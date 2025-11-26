use crate::models::*;
use std::collections::{HashMap, HashSet};
use chrono::{DateTime, Utc, Duration};
use tracing::{info, debug};

// ==================== Funnel Analyzer ====================

pub struct FunnelAnalyzer {
    funnels: Vec<Funnel>,
}

impl FunnelAnalyzer {
    pub fn new() -> Self {
        Self {
            funnels: Vec::new(),
        }
    }

    pub fn add_funnel(&mut self, funnel: Funnel) {
        self.funnels.push(funnel);
    }

    pub fn analyze_funnel(&self, funnel: &Funnel, events: &[BehaviorEvent]) -> FunnelAnalysis {
        info!("Analyzing funnel: {}", funnel.name);

        let mut users_in_funnel: HashMap<String, Vec<(usize, DateTime<Utc>)>> = HashMap::new();

        // Rastrear progresso de cada usuário através do funil
        for event in events {
            for (step_idx, step) in funnel.steps.iter().enumerate() {
                if self.matches_condition(&step.condition, event) {
                    users_in_funnel
                        .entry(event.user_id.clone())
                        .or_default()
                        .push((step_idx, event.timestamp));
                }
            }
        }

        // Ordenar eventos de cada usuário por timestamp
        for user_steps in users_in_funnel.values_mut() {
            user_steps.sort_by_key(|(_, time)| *time);
        }

        // Calcular conversões para cada etapa
        let mut step_conversions = Vec::new();
        let total_entered = users_in_funnel.len();

        for step_idx in 0..funnel.steps.len() {
            let users_reached = users_in_funnel
                .values()
                .filter(|steps| steps.iter().any(|(idx, _)| *idx >= step_idx))
                .count();

            let users_completed = if step_idx < funnel.steps.len() - 1 {
                users_in_funnel
                    .values()
                    .filter(|steps| {
                        // Verificar se usuário completou esta etapa E a próxima
                        let has_current = steps.iter().any(|(idx, _)| *idx == step_idx);
                        let has_next = steps.iter().any(|(idx, _)| *idx == step_idx + 1);
                        has_current && has_next
                    })
                    .count()
            } else {
                users_reached
            };

            // Calcular tempo médio para próxima etapa
            let avg_time_to_next = if step_idx < funnel.steps.len() - 1 {
                self.calculate_avg_time_between_steps(
                    &users_in_funnel,
                    step_idx,
                    step_idx + 1,
                )
            } else {
                0.0
            };

            let conversion_rate = if users_reached > 0 {
                users_completed as f64 / users_reached as f64
            } else {
                0.0
            };

            step_conversions.push(StepConversion {
                step_index: step_idx,
                users_entered: users_reached,
                users_completed,
                conversion_rate,
                avg_time_to_next,
            });
        }

        // Identificar pontos de maior abandono
        let drop_off_points: Vec<(usize, f64)> = step_conversions
            .iter()
            .enumerate()
            .map(|(idx, conv)| (idx, 1.0 - conv.conversion_rate))
            .collect();

        // Calcular tempo médio entre todas as etapas
        let avg_time_between_steps = (0..funnel.steps.len() - 1)
            .map(|i| self.calculate_avg_time_between_steps(&users_in_funnel, i, i + 1))
            .collect();

        FunnelAnalysis {
            total_entered,
            step_conversions,
            avg_time_between_steps,
            drop_off_points,
        }
    }

    fn matches_condition(&self, condition: &FunnelCondition, event: &BehaviorEvent) -> bool {
        match condition {
            FunnelCondition::PageView(url) => {
                if let EventType::PageView { url: event_url, .. } = &event.event_type {
                    event_url.contains(url)
                } else {
                    false
                }
            }
            FunnelCondition::EventType(event_type) => {
                let type_name = match &event.event_type {
                    EventType::PageView { .. } => "page_view",
                    EventType::Click { .. } => "click",
                    EventType::Search { .. } => "search",
                    EventType::Purchase { .. } => "purchase",
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
                type_name == event_type
            }
            FunnelCondition::PurchaseCompleted => {
                matches!(event.event_type, EventType::Purchase { .. })
            }
            FunnelCondition::AddedToCart => {
                matches!(event.event_type, EventType::AddToCart { .. })
            }
        }
    }

    fn calculate_avg_time_between_steps(
        &self,
        users_in_funnel: &HashMap<String, Vec<(usize, DateTime<Utc>)>>,
        from_step: usize,
        to_step: usize,
    ) -> f64 {
        let mut times = Vec::new();

        for user_steps in users_in_funnel.values() {
            if let Some((_, from_time)) = user_steps.iter().find(|(idx, _)| *idx == from_step) {
                if let Some((_, to_time)) = user_steps
                    .iter()
                    .find(|(idx, time)| *idx == to_step && time > from_time)
                {
                    let duration = (*to_time - *from_time).num_seconds();
                    times.push(duration as f64);
                }
            }
        }

        if times.is_empty() {
            0.0
        } else {
            times.iter().sum::<f64>() / times.len() as f64
        }
    }

    pub fn create_ecommerce_funnel() -> Funnel {
        Funnel {
            name: "E-commerce Purchase Funnel".to_string(),
            steps: vec![
                FunnelStep {
                    name: "Product Page View".to_string(),
                    condition: FunnelCondition::PageView("/product".to_string()),
                },
                FunnelStep {
                    name: "Add to Cart".to_string(),
                    condition: FunnelCondition::AddedToCart,
                },
                FunnelStep {
                    name: "Checkout Page".to_string(),
                    condition: FunnelCondition::PageView("/checkout".to_string()),
                },
                FunnelStep {
                    name: "Purchase Complete".to_string(),
                    condition: FunnelCondition::PurchaseCompleted,
                },
            ],
        }
    }

    pub fn print_funnel_report(&self, analysis: &FunnelAnalysis) {
        println!("\n=== Funnel Analysis Report ===");
        println!("Total users entered: {}", analysis.total_entered);
        println!("\nStep-by-step conversion:");

        for step in &analysis.step_conversions {
            println!(
                "  Step {}: {} users → {} completed ({:.2}% conversion)",
                step.step_index + 1,
                step.users_entered,
                step.users_completed,
                step.conversion_rate * 100.0
            );
            if step.avg_time_to_next > 0.0 {
                println!(
                    "    Avg time to next step: {:.0} seconds",
                    step.avg_time_to_next
                );
            }
        }

        println!("\nDrop-off analysis:");
        let mut drop_offs: Vec<_> = analysis.drop_off_points.iter().collect();
        drop_offs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (step, rate) in drop_offs.iter().take(3) {
            if *rate > 0.01 {
                println!("  Step {}: {:.2}% drop-off", step + 1, rate * 100.0);
            }
        }
    }
}

// ==================== Funnel Builder ====================

pub struct FunnelBuilder {
    name: String,
    steps: Vec<FunnelStep>,
}

impl FunnelBuilder {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            steps: Vec::new(),
        }
    }

    pub fn add_page_view(mut self, name: impl Into<String>, url: impl Into<String>) -> Self {
        self.steps.push(FunnelStep {
            name: name.into(),
            condition: FunnelCondition::PageView(url.into()),
        });
        self
    }

    pub fn add_event(mut self, name: impl Into<String>, event_type: impl Into<String>) -> Self {
        self.steps.push(FunnelStep {
            name: name.into(),
            condition: FunnelCondition::EventType(event_type.into()),
        });
        self
    }

    pub fn add_purchase(mut self, name: impl Into<String>) -> Self {
        self.steps.push(FunnelStep {
            name: name.into(),
            condition: FunnelCondition::PurchaseCompleted,
        });
        self
    }

    pub fn add_cart(mut self, name: impl Into<String>) -> Self {
        self.steps.push(FunnelStep {
            name: name.into(),
            condition: FunnelCondition::AddedToCart,
        });
        self
    }

    pub fn build(self) -> Funnel {
        Funnel {
            name: self.name,
            steps: self.steps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    fn create_test_event(
        user_id: &str,
        event_type: EventType,
        timestamp: DateTime<Utc>,
    ) -> BehaviorEvent {
        BehaviorEvent {
            event_id: uuid::Uuid::new_v4().to_string(),
            user_id: user_id.to_string(),
            session_id: "test_session".to_string(),
            timestamp,
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
                    ip_address: "192.168.1.1".to_string(),
                },
                referrer: None,
                user_agent: "Mozilla/5.0".to_string(),
                viewport: Viewport {
                    width: 1920,
                    height: 1080,
                },
            },
        }
    }

    #[test]
    fn test_funnel_analysis() {
        let analyzer = FunnelAnalyzer::new();
        let funnel = FunnelAnalyzer::create_ecommerce_funnel();

        let now = Utc::now();
        let events = vec![
            create_test_event(
                "user1",
                EventType::PageView {
                    url: "/product/123".to_string(),
                    title: "Product".to_string(),
                    duration_ms: 5000,
                },
                now,
            ),
            create_test_event(
                "user1",
                EventType::AddToCart {
                    product_id: "123".to_string(),
                    quantity: 1,
                },
                now + Duration::seconds(30),
            ),
            create_test_event(
                "user1",
                EventType::PageView {
                    url: "/checkout".to_string(),
                    title: "Checkout".to_string(),
                    duration_ms: 10000,
                },
                now + Duration::seconds(60),
            ),
        ];

        let analysis = analyzer.analyze_funnel(&funnel, &events);
        assert!(analysis.total_entered > 0);
        assert!(!analysis.step_conversions.is_empty());
    }

    #[test]
    fn test_funnel_builder() {
        let funnel = FunnelBuilder::new("Test Funnel")
            .add_page_view("Landing", "/landing")
            .add_cart("Add to Cart")
            .add_purchase("Complete Purchase")
            .build();

        assert_eq!(funnel.name, "Test Funnel");
        assert_eq!(funnel.steps.len(), 3);
    }
}
