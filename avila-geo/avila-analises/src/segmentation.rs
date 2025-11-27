use crate::models::*;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use tracing::info;

// ==================== User Segmentation ====================

pub struct UserSegmentation {
    segments: Vec<Segment>,
}

impl UserSegmentation {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    pub fn with_default_segments() -> Self {
        let mut segmentation = Self::new();
        segmentation.segments = Self::create_default_segments();
        segmentation
    }

    pub fn add_segment(&mut self, segment: Segment) {
        self.segments.push(segment);
    }

    /// Classificar usuário em segmentos
    pub fn classify_user(&self, profile: &UserProfile) -> Vec<String> {
        self.segments
            .iter()
            .filter(|seg| self.matches_segment(seg, profile))
            .map(|seg| seg.id.clone())
            .collect()
    }

    fn matches_segment(&self, segment: &Segment, profile: &UserProfile) -> bool {
        segment
            .rules
            .iter()
            .all(|rule| self.matches_rule(rule, profile))
    }

    fn matches_rule(&self, rule: &SegmentRule, profile: &UserProfile) -> bool {
        match rule {
            SegmentRule::BehaviorCount {
                event_type,
                operator,
                threshold,
            } => {
                let count = match event_type.as_str() {
                    "purchase" => profile.behaviors.total_purchases,
                    "search" => profile.behaviors.search_queries.len(),
                    _ => 0,
                };
                self.compare_value(count as f64, *threshold as f64, operator)
            }

            SegmentRule::TimeRange { field, days } => {
                if field == "last_purchase" {
                    if let Some(days_since) = profile.behaviors.days_since_last_purchase {
                        days_since <= *days as i64
                    } else {
                        false
                    }
                } else if field == "first_seen" {
                    let days_since = (Utc::now() - profile.first_seen).num_days();
                    days_since <= *days as i64
                } else {
                    false
                }
            }

            SegmentRule::ValueRange { field, min, max } => {
                let value = match field.as_str() {
                    "total_spent" => profile.behaviors.total_spent,
                    "avg_order_value" => profile.behaviors.avg_order_value,
                    "engagement_score" => profile.engagement_score,
                    "loyalty_score" => profile.loyalty_score,
                    _ => 0.0,
                };
                value >= *min && value <= *max
            }

            SegmentRule::Category { field, values } => {
                if field == "segments" {
                    profile
                        .segments
                        .iter()
                        .any(|s| values.contains(s))
                } else {
                    false
                }
            }

            SegmentRule::Computed {
                score_type,
                operator,
                threshold,
            } => {
                let score = match score_type {
                    ScoreType::Engagement => profile.engagement_score,
                    ScoreType::Loyalty => profile.loyalty_score,
                    ScoreType::ChurnRisk => profile.churn_risk,
                    ScoreType::ConversionProbability => profile.conversion_probability,
                };
                self.compare_value(score, *threshold, operator)
            }
        }
    }

    fn compare_value(&self, value: f64, threshold: f64, operator: &Operator) -> bool {
        match operator {
            Operator::GreaterThan => value > threshold,
            Operator::LessThan => value < threshold,
            Operator::Equals => (value - threshold).abs() < 0.001,
            Operator::Between => false, // Requires min/max
        }
    }

    /// Criar segmentos RFM padrão (Recency, Frequency, Monetary)
    pub fn create_default_segments() -> Vec<Segment> {
        vec![
            // Champions: Compraram recentemente, compram frequentemente, gastam muito
            Segment {
                id: "champions".to_string(),
                name: "Champions".to_string(),
                description: "Best customers - high engagement, recent purchases".to_string(),
                rules: vec![
                    SegmentRule::TimeRange {
                        field: "last_purchase".to_string(),
                        days: 30,
                    },
                    SegmentRule::BehaviorCount {
                        event_type: "purchase".to_string(),
                        operator: Operator::GreaterThan,
                        threshold: 5,
                    },
                    SegmentRule::ValueRange {
                        field: "total_spent".to_string(),
                        min: 1000.0,
                        max: f64::INFINITY,
                    },
                ],
            },
            // Loyal Customers: Compram regularmente
            Segment {
                id: "loyal".to_string(),
                name: "Loyal Customers".to_string(),
                description: "Regular customers with consistent purchases".to_string(),
                rules: vec![
                    SegmentRule::BehaviorCount {
                        event_type: "purchase".to_string(),
                        operator: Operator::GreaterThan,
                        threshold: 3,
                    },
                    SegmentRule::Computed {
                        score_type: ScoreType::Loyalty,
                        operator: Operator::GreaterThan,
                        threshold: 0.7,
                    },
                ],
            },
            // At Risk: Compraram antes mas há muito tempo
            Segment {
                id: "at_risk".to_string(),
                name: "At Risk".to_string(),
                description: "Previously active customers at risk of churning".to_string(),
                rules: vec![
                    SegmentRule::TimeRange {
                        field: "last_purchase".to_string(),
                        days: 90,
                    },
                    SegmentRule::BehaviorCount {
                        event_type: "purchase".to_string(),
                        operator: Operator::GreaterThan,
                        threshold: 2,
                    },
                    SegmentRule::Computed {
                        score_type: ScoreType::ChurnRisk,
                        operator: Operator::GreaterThan,
                        threshold: 0.5,
                    },
                ],
            },
            // New Customers: Cadastrados recentemente
            Segment {
                id: "new_customers".to_string(),
                name: "New Customers".to_string(),
                description: "Recently acquired users".to_string(),
                rules: vec![SegmentRule::TimeRange {
                    field: "first_seen".to_string(),
                    days: 30,
                }],
            },
            // High Spenders: Gastam muito por compra
            Segment {
                id: "high_spenders".to_string(),
                name: "High Spenders".to_string(),
                description: "Customers with high average order value".to_string(),
                rules: vec![SegmentRule::ValueRange {
                    field: "avg_order_value".to_string(),
                    min: 500.0,
                    max: f64::INFINITY,
                }],
            },
            // Window Shoppers: Navegam muito mas não compram
            Segment {
                id: "window_shoppers".to_string(),
                name: "Window Shoppers".to_string(),
                description: "High engagement but low conversion".to_string(),
                rules: vec![
                    SegmentRule::Computed {
                        score_type: ScoreType::Engagement,
                        operator: Operator::GreaterThan,
                        threshold: 0.7,
                    },
                    SegmentRule::BehaviorCount {
                        event_type: "purchase".to_string(),
                        operator: Operator::LessThan,
                        threshold: 2,
                    },
                ],
            },
            // Lost: Não aparecem há muito tempo
            Segment {
                id: "lost".to_string(),
                name: "Lost Customers".to_string(),
                description: "Inactive for extended period".to_string(),
                rules: vec![
                    SegmentRule::TimeRange {
                        field: "last_purchase".to_string(),
                        days: 180,
                    },
                    SegmentRule::Computed {
                        score_type: ScoreType::ChurnRisk,
                        operator: Operator::GreaterThan,
                        threshold: 0.8,
                    },
                ],
            },
        ]
    }

    /// Calcular distribuição de usuários por segmento
    pub fn segment_distribution(&self, profiles: &[UserProfile]) -> HashMap<String, usize> {
        let mut distribution = HashMap::new();

        for profile in profiles {
            let segments = self.classify_user(profile);
            for segment in segments {
                *distribution.entry(segment).or_insert(0) += 1;
            }
        }

        distribution
    }

    /// Gerar relatório de segmentação
    pub fn print_segmentation_report(&self, profiles: &[UserProfile]) {
        println!("\n=== User Segmentation Report ===");
        println!("Total users analyzed: {}", profiles.len());

        let distribution = self.segment_distribution(profiles);

        println!("\nSegment Distribution:");
        let mut sorted: Vec<_> = distribution.iter().collect();
        sorted.sort_by_key(|(_, count)| std::cmp::Reverse(*count));

        for (segment_id, count) in sorted {
            let percentage = (*count as f64 / profiles.len() as f64) * 100.0;
            let segment_name = self
                .segments
                .iter()
                .find(|s| &s.id == segment_id)
                .map(|s| s.name.as_str())
                .unwrap_or(segment_id);

            println!("  {}: {} users ({:.2}%)", segment_name, count, percentage);
        }

        // Estatísticas por segmento
        println!("\nSegment Statistics:");
        for segment in &self.segments {
            let segment_users: Vec<_> = profiles
                .iter()
                .filter(|p| self.matches_segment(segment, p))
                .collect();

            if segment_users.is_empty() {
                continue;
            }

            let avg_engagement: f64 =
                segment_users.iter().map(|u| u.engagement_score).sum::<f64>()
                    / segment_users.len() as f64;

            let avg_spent: f64 = segment_users
                .iter()
                .map(|u| u.behaviors.total_spent)
                .sum::<f64>()
                / segment_users.len() as f64;

            println!("\n  {}:", segment.name);
            println!("    Size: {} users", segment_users.len());
            println!("    Avg Engagement Score: {:.2}", avg_engagement);
            println!("    Avg Total Spent: R$ {:.2}", avg_spent);
        }
    }
}

// ==================== Segment Builder ====================

pub struct SegmentBuilder {
    id: String,
    name: String,
    description: String,
    rules: Vec<SegmentRule>,
}

impl SegmentBuilder {
    pub fn new(id: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            rules: Vec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_purchase_count(mut self, operator: Operator, threshold: usize) -> Self {
        self.rules.push(SegmentRule::BehaviorCount {
            event_type: "purchase".to_string(),
            operator,
            threshold,
        });
        self
    }

    pub fn with_recency(mut self, days: u32) -> Self {
        self.rules.push(SegmentRule::TimeRange {
            field: "last_purchase".to_string(),
            days,
        });
        self
    }

    pub fn with_spending_range(mut self, min: f64, max: f64) -> Self {
        self.rules.push(SegmentRule::ValueRange {
            field: "total_spent".to_string(),
            min,
            max,
        });
        self
    }

    pub fn with_churn_risk(mut self, operator: Operator, threshold: f64) -> Self {
        self.rules.push(SegmentRule::Computed {
            score_type: ScoreType::ChurnRisk,
            operator,
            threshold,
        });
        self
    }

    pub fn with_engagement(mut self, operator: Operator, threshold: f64) -> Self {
        self.rules.push(SegmentRule::Computed {
            score_type: ScoreType::Engagement,
            operator,
            threshold,
        });
        self
    }

    pub fn build(self) -> Segment {
        Segment {
            id: self.id,
            name: self.name,
            description: self.description,
            rules: self.rules,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn create_test_profile(
        user_id: &str,
        total_purchases: usize,
        total_spent: f64,
        days_since_last_purchase: Option<i64>,
    ) -> UserProfile {
        UserProfile {
            user_id: user_id.to_string(),
            first_seen: Utc::now() - Duration::days(100),
            last_seen: Utc::now(),
            total_sessions: 10,
            total_events: 100,
            behaviors: UserBehaviors {
                total_purchases,
                total_spent,
                avg_order_value: if total_purchases > 0 {
                    total_spent / total_purchases as f64
                } else {
                    0.0
                },
                days_since_last_purchase,
                ..Default::default()
            },
            segments: vec![],
            engagement_score: 0.5,
            loyalty_score: 0.5,
            conversion_probability: 0.5,
            churn_risk: 0.3,
            interests: vec![],
            preferred_categories: HashMap::new(),
            browsing_patterns: BrowsingPatterns::default(),
        }
    }

    #[test]
    fn test_segment_classification() {
        let segmentation = UserSegmentation::with_default_segments();

        let champion = create_test_profile("user1", 10, 2000.0, Some(15));
        let segments = segmentation.classify_user(&champion);

        assert!(!segments.is_empty());
    }

    #[test]
    fn test_segment_builder() {
        let segment = SegmentBuilder::new("vip", "VIP Customers")
            .description("High value customers")
            .with_purchase_count(Operator::GreaterThan, 10)
            .with_spending_range(5000.0, f64::INFINITY)
            .build();

        assert_eq!(segment.id, "vip");
        assert_eq!(segment.rules.len(), 2);
    }

    #[test]
    fn test_segment_distribution() {
        let segmentation = UserSegmentation::with_default_segments();

        let profiles = vec![
            create_test_profile("user1", 10, 2000.0, Some(15)),
            create_test_profile("user2", 2, 300.0, Some(120)),
            create_test_profile("user3", 0, 0.0, None),
        ];

        let distribution = segmentation.segment_distribution(&profiles);
        assert!(!distribution.is_empty());
    }
}
