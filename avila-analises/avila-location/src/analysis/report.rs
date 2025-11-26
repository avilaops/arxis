//! Report generation

use crate::analysis::ComparativeAnalysis;
use crate::data::RegionData;
use crate::models::LocationScore;
use serde::{Deserialize, Serialize};

/// Comprehensive location report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationReport {
    pub title: String,
    pub generated_at: String,
    pub analysis: AnalysisSummary,
    pub recommendations: Vec<String>,
    pub top_locations: Vec<LocationSummary>,
    pub scenario_analysis: ScenarioAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisSummary {
    pub total_locations_analyzed: usize,
    pub countries_covered: Vec<String>,
    pub best_overall: String,
    pub best_value: String,
    pub best_market: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationSummary {
    pub rank: usize,
    pub name: String,
    pub country: String,
    pub total_score: f64,
    pub cost_index: f64,
    pub market_score: f64,
    pub competition_score: f64,
    pub infrastructure_score: f64,
    pub key_strengths: Vec<String>,
    pub key_weaknesses: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioAnalysis {
    pub bootstrap_best: String,
    pub growth_best: String,
    pub remote_best: String,
}

/// Generate comprehensive report
pub fn generate_report(
    regions: &[RegionData],
    analysis: &ComparativeAnalysis,
) -> LocationReport {
    let now = chrono::Utc::now();

    let mut countries: Vec<String> = regions
        .iter()
        .map(|r| r.region.location.country.name().to_string())
        .collect();
    countries.sort();
    countries.dedup();

    let top_locations: Vec<LocationSummary> = analysis
        .scores
        .iter()
        .take(10)
        .enumerate()
        .map(|(i, score)| {
            let region = regions
                .iter()
                .find(|r| r.region.location.id == score.location_id)
                .unwrap();

            let key_strengths = identify_strengths(score);
            let key_weaknesses = identify_weaknesses(score);

            LocationSummary {
                rank: i + 1,
                name: score.location_name.clone(),
                country: region.region.location.country.name().to_string(),
                total_score: score.total_score,
                cost_index: score.cost_of_living,
                market_score: score.market_demand,
                competition_score: score.competition,
                infrastructure_score: score.infrastructure,
                key_strengths,
                key_weaknesses,
            }
        })
        .collect();

    let best_overall = analysis.scores.first()
        .map(|s| s.location_name.clone())
        .unwrap_or_else(|| "N/A".to_string());

    let best_value = analysis.scores
        .iter()
        .max_by(|a, b| a.cost_of_living.partial_cmp(&b.cost_of_living).unwrap())
        .map(|s| s.location_name.clone())
        .unwrap_or_else(|| "N/A".to_string());

    let best_market = analysis.scores
        .iter()
        .max_by(|a, b| a.market_demand.partial_cmp(&b.market_demand).unwrap())
        .map(|s| s.location_name.clone())
        .unwrap_or_else(|| "N/A".to_string());

    let recommendations = generate_recommendations(&analysis.scores, regions);

    let scenario_analysis = ScenarioAnalysis {
        bootstrap_best: analysis.best_for_bootstrap
            .and_then(|i| regions.get(i))
            .map(|r| r.region.location.name.clone())
            .unwrap_or_else(|| "N/A".to_string()),
        growth_best: analysis.best_for_growth
            .and_then(|i| regions.get(i))
            .map(|r| r.region.location.name.clone())
            .unwrap_or_else(|| "N/A".to_string()),
        remote_best: analysis.best_for_remote
            .and_then(|i| regions.get(i))
            .map(|r| r.region.location.name.clone())
            .unwrap_or_else(|| "N/A".to_string()),
    };

    LocationReport {
        title: "Location Intelligence Report".to_string(),
        generated_at: now.to_rfc3339(),
        analysis: AnalysisSummary {
            total_locations_analyzed: regions.len(),
            countries_covered: countries,
            best_overall,
            best_value,
            best_market,
        },
        recommendations,
        top_locations,
        scenario_analysis,
    }
}

fn identify_strengths(score: &LocationScore) -> Vec<String> {
    let mut strengths = Vec::new();

    if score.cost_of_living > 70.0 {
        strengths.push("Excellent cost of living".to_string());
    }
    if score.market_demand > 80.0 {
        strengths.push("Strong market demand".to_string());
    }
    if score.infrastructure > 85.0 {
        strengths.push("World-class infrastructure".to_string());
    }
    if score.competition > 70.0 {
        strengths.push("Low competition".to_string());
    }
    if score.fiscal_incentives > 75.0 {
        strengths.push("Attractive tax incentives".to_string());
    }
    if score.quality_of_life > 85.0 {
        strengths.push("High quality of life".to_string());
    }

    strengths
}

fn identify_weaknesses(score: &LocationScore) -> Vec<String> {
    let mut weaknesses = Vec::new();

    if score.cost_of_living < 40.0 {
        weaknesses.push("High cost of living".to_string());
    }
    if score.market_demand < 50.0 {
        weaknesses.push("Limited market size".to_string());
    }
    if score.infrastructure < 60.0 {
        weaknesses.push("Infrastructure challenges".to_string());
    }
    if score.competition < 40.0 {
        weaknesses.push("High competition".to_string());
    }
    if score.accessibility < 50.0 {
        weaknesses.push("Limited accessibility".to_string());
    }

    weaknesses
}

fn generate_recommendations(scores: &[LocationScore], regions: &[RegionData]) -> Vec<String> {
    let mut recs = Vec::new();

    if let Some(top) = scores.first() {
        recs.push(format!(
            "Top recommendation: {} with a score of {:.1}/100",
            top.location_name, top.total_score
        ));
    }

    // Cost-conscious recommendation
    if let Some(best_cost) = scores.iter().max_by(|a, b| {
        a.cost_of_living.partial_cmp(&b.cost_of_living).unwrap()
    }) {
        if let Some(region) = regions.iter().find(|r| r.region.location.id == best_cost.location_id) {
            recs.push(format!(
                "Best value: {} - Low cost ({}% below average) with good fundamentals",
                best_cost.location_name,
                (100.0 - region.economic.cost_of_living_index) as i32
            ));
        }
    }

    // Market opportunity recommendation
    if let Some(best_market) = scores.iter().max_by(|a, b| {
        a.market_demand.partial_cmp(&b.market_demand).unwrap()
    }) {
        recs.push(format!(
            "Best market opportunity: {} - Strong demand and growth potential",
            best_market.location_name
        ));
    }

    // Strategic insights
    recs.push("Consider starting in a lower-cost location to bootstrap, then expand to major markets".to_string());
    recs.push("Free zones in Dubai offer 0% tax but require specific compliance".to_string());
    recs.push("Portugal's interior regions offer significant tax incentives (50% reduction)".to_string());

    recs
}

/// Export report to JSON
pub fn export_json(report: &LocationReport) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

/// Export report to CSV (simplified)
pub fn export_csv(scores: &[LocationScore]) -> String {
    let mut csv = String::from("Rank,Location,Total Score,Cost,Market,Competition,Infrastructure,Quality of Life\n");

    for score in scores {
        csv.push_str(&format!(
            "{},{},{:.2},{:.2},{:.2},{:.2},{:.2},{:.2}\n",
            score.rank.unwrap_or(0),
            score.location_name,
            score.total_score,
            score.cost_of_living,
            score.market_demand,
            score.competition,
            score.infrastructure,
            score.quality_of_life
        ));
    }

    csv
}
