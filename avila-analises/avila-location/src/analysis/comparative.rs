//! Comparative analysis between locations

use crate::data::RegionData;
use crate::models::{LocationScore, ScoringWeights};
use crate::scoring::rank_locations;

/// Comparative analysis result
#[derive(Debug, Clone)]
pub struct ComparativeAnalysis {
    pub scores: Vec<LocationScore>,
    pub best_for_bootstrap: Option<usize>,
    pub best_for_growth: Option<usize>,
    pub best_for_remote: Option<usize>,
    pub best_overall: Option<usize>,
}

/// Analyze and compare multiple regions
pub fn analyze_regions(regions: &[RegionData], weights: &ScoringWeights) -> ComparativeAnalysis {
    let mut scores: Vec<LocationScore> = regions
        .iter()
        .map(|r| r.calculate_score(weights))
        .collect();

    rank_locations(&mut scores);

    // Find best for different scenarios
    let best_overall = if !scores.is_empty() { Some(0) } else { None };

    // Best for bootstrap (low cost focus)
    let bootstrap_weights = ScoringWeights::bootstrap();
    let mut bootstrap_scores: Vec<LocationScore> = regions
        .iter()
        .map(|r| r.calculate_score(&bootstrap_weights))
        .collect();
    rank_locations(&mut bootstrap_scores);
    let best_for_bootstrap = if !bootstrap_scores.is_empty() { Some(0) } else { None };

    // Best for growth (market focus)
    let growth_weights = ScoringWeights::growth_focused();
    let mut growth_scores: Vec<LocationScore> = regions
        .iter()
        .map(|r| r.calculate_score(&growth_weights))
        .collect();
    rank_locations(&mut growth_scores);
    let best_for_growth = if !growth_scores.is_empty() { Some(0) } else { None };

    // Best for remote (infrastructure focus)
    let remote_weights = ScoringWeights::remote_first();
    let mut remote_scores: Vec<LocationScore> = regions
        .iter()
        .map(|r| r.calculate_score(&remote_weights))
        .collect();
    rank_locations(&mut remote_scores);
    let best_for_remote = if !remote_scores.is_empty() { Some(0) } else { None };

    ComparativeAnalysis {
        scores,
        best_for_bootstrap,
        best_for_growth,
        best_for_remote,
        best_overall,
    }
}

/// Compare two specific regions
pub fn compare_two_regions(region1: &RegionData, region2: &RegionData) -> RegionComparison {
    let weights = ScoringWeights::default();
    let score1 = region1.calculate_score(&weights);
    let score2 = region2.calculate_score(&weights);

    RegionComparison {
        region1_name: region1.region.location.name.clone(),
        region2_name: region2.region.location.name.clone(),
        region1_score: score1.total_score,
        region2_score: score2.total_score,
        cost_difference: region2.economic.cost_of_living_index - region1.economic.cost_of_living_index,
        market_difference: score2.market_demand - score1.market_demand,
        competition_difference: score2.competition - score1.competition,
        winner: if score1.total_score > score2.total_score { 1 } else { 2 },
    }
}

#[derive(Debug, Clone)]
pub struct RegionComparison {
    pub region1_name: String,
    pub region2_name: String,
    pub region1_score: f64,
    pub region2_score: f64,
    pub cost_difference: f64,
    pub market_difference: f64,
    pub competition_difference: f64,
    pub winner: u8,
}

/// Filter regions by criteria
pub fn filter_regions(
    regions: &[RegionData],
    max_cost_index: Option<f64>,
    min_market_score: Option<f64>,
    max_competition: Option<u32>,
    min_infrastructure: Option<f64>,
) -> Vec<&RegionData> {
    regions
        .iter()
        .filter(|r| {
            if let Some(max_cost) = max_cost_index {
                if r.economic.cost_of_living_index > max_cost {
                    return false;
                }
            }

            if let Some(min_market) = min_market_score {
                if r.market.market_score() < min_market {
                    return false;
                }
            }

            if let Some(max_comp) = max_competition {
                if r.competition.direct_competitors > max_comp {
                    return false;
                }
            }

            if let Some(min_infra) = min_infrastructure {
                if r.infrastructure.infrastructure_score() < min_infra {
                    return false;
                }
            }

            true
        })
        .collect()
}

/// Calculate ROI comparison between regions
pub fn calculate_roi_comparison(
    regions: &[RegionData],
    investment: f64,
    expected_revenue: f64,
    years: u32,
) -> Vec<ROIComparison> {
    regions
        .iter()
        .map(|region| {
            let monthly_cost = region.economic.total_monthly_cost_estimate(50.0); // 50m² office
            let annual_cost = monthly_cost * 12.0;
            let tax_rate = region.fiscal.effective_corporate_tax_rate() / 100.0;

            let mut total_profit = 0.0;
            for year in 1..=years {
                let revenue = expected_revenue * (1.08_f64).powi(year as i32 - 1);
                let profit_before_tax = revenue - annual_cost;
                let profit_after_tax = profit_before_tax * (1.0 - tax_rate);
                total_profit += profit_after_tax;
            }

            let roi = ((total_profit - investment) / investment) * 100.0;

            ROIComparison {
                location: region.region.location.name.clone(),
                total_profit,
                roi,
                annual_cost,
                effective_tax_rate: tax_rate * 100.0,
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct ROIComparison {
    pub location: String,
    pub total_profit: f64,
    pub roi: f64,
    pub annual_cost: f64,
    pub effective_tax_rate: f64,
}
