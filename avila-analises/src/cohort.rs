use crate::models::*;
use chrono::{DateTime, Datelike, Duration, Utc};
use std::collections::{HashMap, HashSet};
use tracing::info;

// ==================== Cohort Analyzer ====================

pub struct CohortAnalyzer;

impl CohortAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Criar cohorts baseados em período de cadastro
    pub fn create_cohorts(
        &self,
        users: &[UserProfile],
        period: CohortPeriod,
    ) -> Vec<Cohort> {
        let mut cohorts: HashMap<String, Cohort> = HashMap::new();

        for user in users {
            let cohort_key = self.get_cohort_key(&user.first_seen, &period);

            cohorts
                .entry(cohort_key.clone())
                .or_insert_with(|| Cohort {
                    cohort_id: cohort_key,
                    period: period.clone(),
                    users: vec![],
                    created_at: user.first_seen,
                })
                .users
                .push(user.user_id.clone());
        }

        let mut result: Vec<_> = cohorts.into_values().collect();
        result.sort_by_key(|c| c.created_at);
        result
    }

    fn get_cohort_key(&self, date: &DateTime<Utc>, period: &CohortPeriod) -> String {
        match period {
            CohortPeriod::Daily(_) => date.format("%Y-%m-%d").to_string(),
            CohortPeriod::Weekly(_, _) => {
                format!("{}-W{:02}", date.year(), date.iso_week().week())
            }
            CohortPeriod::Monthly(_, _) => {
                format!("{}-{:02}", date.year(), date.month())
            }
        }
    }

    /// Analisar múltiplos cohorts
    pub fn analyze_cohorts(
        &self,
        cohorts: &[Cohort],
        events: &[BehaviorEvent],
        periods: usize,
    ) -> CohortAnalysis {
        info!("Analyzing {} cohorts over {} periods", cohorts.len(), periods);

        let mut cohort_metrics = Vec::new();
        let mut retention_matrix = Vec::new();

        for cohort in cohorts {
            let retention = self.calculate_retention(cohort, events, periods);
            let revenue = self.calculate_revenue_by_period(cohort, events, periods);
            let engagement = self.calculate_engagement_by_period(cohort, events, periods);

            retention_matrix.push(retention.clone());

            cohort_metrics.push(CohortMetrics {
                cohort_id: cohort.cohort_id.clone(),
                size: cohort.users.len(),
                retention_by_period: retention,
                revenue_by_period: revenue,
                engagement_by_period: engagement,
            });
        }

        CohortAnalysis {
            cohorts: cohort_metrics,
            retention_rates: retention_matrix,
        }
    }

    /// Calcular retenção por período
    pub fn calculate_retention(
        &self,
        cohort: &Cohort,
        events: &[BehaviorEvent],
        periods: usize,
    ) -> Vec<f64> {
        let mut retention = vec![0.0; periods];
        let cohort_users: HashSet<_> = cohort.users.iter().collect();

        for period in 0..periods {
            let period_start = cohort.created_at + Duration::weeks(period as i64);
            let period_end = period_start + Duration::weeks(1);

            let active_users = events
                .iter()
                .filter(|e| cohort_users.contains(&e.user_id))
                .filter(|e| e.timestamp >= period_start && e.timestamp < period_end)
                .map(|e| &e.user_id)
                .collect::<HashSet<_>>()
                .len();

            retention[period] = if cohort.users.is_empty() {
                0.0
            } else {
                active_users as f64 / cohort.users.len() as f64
            };
        }

        retention
    }

    /// Calcular receita por período
    fn calculate_revenue_by_period(
        &self,
        cohort: &Cohort,
        events: &[BehaviorEvent],
        periods: usize,
    ) -> Vec<f64> {
        let mut revenue = vec![0.0; periods];
        let cohort_users: HashSet<_> = cohort.users.iter().collect();

        for period in 0..periods {
            let period_start = cohort.created_at + Duration::weeks(period as i64);
            let period_end = period_start + Duration::weeks(1);

            let period_revenue: f64 = events
                .iter()
                .filter(|e| cohort_users.contains(&e.user_id))
                .filter(|e| e.timestamp >= period_start && e.timestamp < period_end)
                .filter_map(|e| {
                    if let EventType::Purchase { amount, .. } = e.event_type {
                        Some(amount)
                    } else {
                        None
                    }
                })
                .sum();

            revenue[period] = period_revenue;
        }

        revenue
    }

    /// Calcular engajamento por período (eventos por usuário)
    fn calculate_engagement_by_period(
        &self,
        cohort: &Cohort,
        events: &[BehaviorEvent],
        periods: usize,
    ) -> Vec<f64> {
        let mut engagement = vec![0.0; periods];
        let cohort_users: HashSet<_> = cohort.users.iter().collect();

        for period in 0..periods {
            let period_start = cohort.created_at + Duration::weeks(period as i64);
            let period_end = period_start + Duration::weeks(1);

            let period_events = events
                .iter()
                .filter(|e| cohort_users.contains(&e.user_id))
                .filter(|e| e.timestamp >= period_start && e.timestamp < period_end)
                .count();

            let active_users = events
                .iter()
                .filter(|e| cohort_users.contains(&e.user_id))
                .filter(|e| e.timestamp >= period_start && e.timestamp < period_end)
                .map(|e| &e.user_id)
                .collect::<HashSet<_>>()
                .len();

            engagement[period] = if active_users > 0 {
                period_events as f64 / active_users as f64
            } else {
                0.0
            };
        }

        engagement
    }

    /// Calcular taxa de retenção média
    pub fn calculate_avg_retention(&self, analysis: &CohortAnalysis) -> Vec<f64> {
        if analysis.retention_rates.is_empty() {
            return Vec::new();
        }

        let periods = analysis.retention_rates[0].len();
        let mut avg_retention = vec![0.0; periods];

        for period in 0..periods {
            let sum: f64 = analysis
                .retention_rates
                .iter()
                .map(|rates| rates.get(period).copied().unwrap_or(0.0))
                .sum();

            avg_retention[period] = sum / analysis.retention_rates.len() as f64;
        }

        avg_retention
    }

    /// Identificar cohorts com melhor/pior desempenho
    pub fn rank_cohorts_by_retention(&self, analysis: &CohortAnalysis) -> Vec<(String, f64)> {
        let mut rankings: Vec<_> = analysis
            .cohorts
            .iter()
            .map(|cohort| {
                let avg_retention = if cohort.retention_by_period.is_empty() {
                    0.0
                } else {
                    cohort.retention_by_period.iter().sum::<f64>()
                        / cohort.retention_by_period.len() as f64
                };
                (cohort.cohort_id.clone(), avg_retention)
            })
            .collect();

        rankings.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        rankings
    }

    /// Gerar relatório de cohort
    pub fn print_cohort_report(&self, analysis: &CohortAnalysis) {
        println!("\n=== Cohort Analysis Report ===");
        println!("Total cohorts analyzed: {}", analysis.cohorts.len());

        // Retenção média por período
        let avg_retention = self.calculate_avg_retention(analysis);
        println!("\nAverage Retention by Period:");
        for (i, rate) in avg_retention.iter().enumerate() {
            println!("  Week {}: {:.2}%", i, rate * 100.0);
        }

        // Top 5 cohorts por retenção
        let rankings = self.rank_cohorts_by_retention(analysis);
        println!("\nTop 5 Cohorts by Retention:");
        for (i, (cohort_id, retention)) in rankings.iter().take(5).enumerate() {
            println!("  {}. {}: {:.2}%", i + 1, cohort_id, retention * 100.0);
        }

        // Detalhes de cada cohort
        println!("\nCohort Details:");
        for cohort in &analysis.cohorts {
            println!("\n  Cohort: {}", cohort.cohort_id);
            println!("    Size: {} users", cohort.size);
            println!("    Retention:");
            for (i, rate) in cohort.retention_by_period.iter().enumerate() {
                println!("      Week {}: {:.2}%", i, rate * 100.0);
            }

            let total_revenue: f64 = cohort.revenue_by_period.iter().sum();
            if total_revenue > 0.0 {
                println!("    Total Revenue: R$ {:.2}", total_revenue);
                println!(
                    "    Revenue per User: R$ {:.2}",
                    total_revenue / cohort.size as f64
                );
            }
        }
    }

    /// Calcular Lifetime Value (LTV) de um cohort
    pub fn calculate_ltv(&self, cohort_metrics: &CohortMetrics) -> f64 {
        cohort_metrics.revenue_by_period.iter().sum::<f64>() / cohort_metrics.size as f64
    }

    /// Calcular tempo de vida médio (quando usuário para de usar)
    pub fn calculate_avg_lifetime(&self, cohort_metrics: &CohortMetrics) -> f64 {
        // Encontrar quando retenção cai abaixo de 10%
        for (i, retention) in cohort_metrics.retention_by_period.iter().enumerate() {
            if *retention < 0.1 {
                return i as f64;
            }
        }
        cohort_metrics.retention_by_period.len() as f64
    }
}

// ==================== Cohort Builder ====================

pub struct CohortBuilder {
    period: CohortPeriod,
}

impl CohortBuilder {
    pub fn daily() -> Self {
        Self {
            period: CohortPeriod::Daily(Utc::now()),
        }
    }

    pub fn weekly() -> Self {
        let now = Utc::now();
        Self {
            period: CohortPeriod::Weekly(now.iso_week().week(), now.year()),
        }
    }

    pub fn monthly() -> Self {
        let now = Utc::now();
        Self {
            period: CohortPeriod::Monthly(now.month(), now.year()),
        }
    }

    pub fn build(self) -> CohortPeriod {
        self.period
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_profile(user_id: &str, first_seen: DateTime<Utc>) -> UserProfile {
        UserProfile {
            user_id: user_id.to_string(),
            first_seen,
            last_seen: Utc::now(),
            total_sessions: 10,
            total_events: 100,
            behaviors: UserBehaviors::default(),
            segments: vec![],
            engagement_score: 0.5,
            loyalty_score: 0.5,
            conversion_probability: 0.5,
            churn_risk: 0.5,
            interests: vec![],
            preferred_categories: HashMap::new(),
            browsing_patterns: BrowsingPatterns::default(),
        }
    }

    #[test]
    fn test_cohort_creation() {
        let analyzer = CohortAnalyzer::new();
        let now = Utc::now();

        let users = vec![
            create_test_profile("user1", now),
            create_test_profile("user2", now),
            create_test_profile("user3", now - Duration::weeks(1)),
        ];

        let cohorts = analyzer.create_cohorts(&users, CohortPeriod::Weekly(1, 2024));
        assert!(!cohorts.is_empty());
    }

    #[test]
    fn test_cohort_retention() {
        let analyzer = CohortAnalyzer::new();
        let now = Utc::now();

        let cohort = Cohort {
            cohort_id: "2024-W01".to_string(),
            period: CohortPeriod::Weekly(1, 2024),
            users: vec!["user1".to_string(), "user2".to_string()],
            created_at: now,
        };

        let events = vec![]; // Empty events for test
        let retention = analyzer.calculate_retention(&cohort, &events, 4);

        assert_eq!(retention.len(), 4);
    }
}
