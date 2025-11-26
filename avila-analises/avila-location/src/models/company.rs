//! Company and competition analysis models

use super::{Sector, CompanySize};
use serde::{Deserialize, Serialize};

/// Represents a competitor in the region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competitor {
    pub name: String,
    pub size: CompanySize,
    pub specializations: Vec<String>,
    pub estimated_revenue: f64,
    pub employee_count: u32,
    pub years_in_market: u32,
    pub client_sectors: Vec<Sector>,
}

/// Competition analysis for a region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetitionData {
    /// Number of direct competitors
    pub direct_competitors: u32,

    /// Number of indirect competitors
    pub indirect_competitors: u32,

    /// Average competitor age (years)
    pub avg_competitor_age: f64,

    /// Market concentration (HHI index)
    pub market_concentration_hhi: f64,

    /// Average pricing (monthly service cost)
    pub avg_pricing: f64,

    /// Service gaps (unmet demand areas)
    pub service_gaps: Vec<String>,

    /// Top competitors
    pub top_competitors: Vec<Competitor>,
}

impl CompetitionData {
    /// Calculate Porter's Five Forces score
    pub fn porter_five_forces(&self) -> PorterFiveForces {
        // Threat of new entrants (lower is better for us as new entrant)
        let new_entrants = if self.direct_competitors < 5 {
            30.0
        } else if self.direct_competitors < 15 {
            60.0
        } else {
            90.0
        };

        // Threat of substitutes
        let substitutes = (self.indirect_competitors as f64 / 10.0).min(100.0);

        // Bargaining power of buyers (more competitors = more power)
        let buyer_power = (self.direct_competitors as f64 * 5.0).min(100.0);

        // Bargaining power of suppliers (tech talent availability)
        let supplier_power = 50.0; // Mid-range default

        // Competitive rivalry (based on concentration)
        let rivalry = self.market_concentration_hhi;

        PorterFiveForces {
            threat_of_new_entrants: new_entrants,
            threat_of_substitutes: substitutes,
            bargaining_power_buyers: buyer_power,
            bargaining_power_suppliers: supplier_power,
            competitive_rivalry: rivalry,
        }
    }

    pub fn competition_score(&self) -> f64 {
        // Lower competition = higher score
        let density_score = 100.0 - (self.direct_competitors as f64 * 3.0).min(100.0);
        let concentration_score = 100.0 - self.market_concentration_hhi;
        let gaps_score = (self.service_gaps.len() as f64 * 10.0).min(100.0);

        (density_score * 0.40 + concentration_score * 0.35 + gaps_score * 0.25)
    }
}

/// Porter's Five Forces analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PorterFiveForces {
    pub threat_of_new_entrants: f64,
    pub threat_of_substitutes: f64,
    pub bargaining_power_buyers: f64,
    pub bargaining_power_suppliers: f64,
    pub competitive_rivalry: f64,
}

impl PorterFiveForces {
    pub fn overall_intensity(&self) -> f64 {
        (self.threat_of_new_entrants * 0.20 +
         self.threat_of_substitutes * 0.15 +
         self.bargaining_power_buyers * 0.25 +
         self.bargaining_power_suppliers * 0.15 +
         self.competitive_rivalry * 0.25)
    }

    /// Lower intensity is better for new business
    pub fn attractiveness_score(&self) -> f64 {
        100.0 - self.overall_intensity()
    }
}

/// SWOT Analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwotAnalysis {
    pub strengths: Vec<String>,
    pub weaknesses: Vec<String>,
    pub opportunities: Vec<String>,
    pub threats: Vec<String>,
}

impl SwotAnalysis {
    pub fn score(&self) -> f64 {
        let positive = (self.strengths.len() + self.opportunities.len()) as f64;
        let negative = (self.weaknesses.len() + self.threats.len()) as f64;

        if negative == 0.0 {
            100.0
        } else {
            (positive / (positive + negative) * 100.0).min(100.0)
        }
    }
}
