//! Multi-criteria scoring system

use serde::{Deserialize, Serialize};

/// Complete location score with all criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationScore {
    pub location_id: String,
    pub location_name: String,

    /// Individual criterion scores (0-100)
    pub cost_of_living: f64,
    pub market_demand: f64,
    pub competition: f64,
    pub infrastructure: f64,
    pub quality_of_life: f64,
    pub fiscal_incentives: f64,
    pub accessibility: f64,
    pub talent_availability: f64,

    /// Weighted total score
    pub total_score: f64,

    /// Rank among all locations
    pub rank: Option<usize>,
}

impl LocationScore {
    pub fn new(location_id: impl Into<String>, location_name: impl Into<String>) -> Self {
        Self {
            location_id: location_id.into(),
            location_name: location_name.into(),
            cost_of_living: 0.0,
            market_demand: 0.0,
            competition: 0.0,
            infrastructure: 0.0,
            quality_of_life: 0.0,
            fiscal_incentives: 0.0,
            accessibility: 0.0,
            talent_availability: 0.0,
            total_score: 0.0,
            rank: None,
        }
    }

    /// Calculate weighted total score
    pub fn calculate_total(&mut self, weights: &ScoringWeights) {
        self.total_score =
            self.cost_of_living * weights.cost_of_living +
            self.market_demand * weights.market_demand +
            self.competition * weights.competition +
            self.infrastructure * weights.infrastructure +
            self.quality_of_life * weights.quality_of_life +
            self.fiscal_incentives * weights.fiscal_incentives +
            self.accessibility * weights.accessibility +
            self.talent_availability * weights.talent_availability;
    }
}

/// Weights for scoring criteria (must sum to 1.0)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringWeights {
    pub cost_of_living: f64,
    pub market_demand: f64,
    pub competition: f64,
    pub infrastructure: f64,
    pub quality_of_life: f64,
    pub fiscal_incentives: f64,
    pub accessibility: f64,
    pub talent_availability: f64,
}

impl Default for ScoringWeights {
    fn default() -> Self {
        Self {
            cost_of_living: 0.20,
            market_demand: 0.25,
            competition: 0.15,
            infrastructure: 0.15,
            quality_of_life: 0.10,
            fiscal_incentives: 0.05,
            accessibility: 0.05,
            talent_availability: 0.05,
        }
    }
}

impl ScoringWeights {
    pub fn validate(&self) -> bool {
        let sum = self.cost_of_living +
                  self.market_demand +
                  self.competition +
                  self.infrastructure +
                  self.quality_of_life +
                  self.fiscal_incentives +
                  self.accessibility +
                  self.talent_availability;

        (sum - 1.0).abs() < 0.01
    }

    /// Preset for bootstrapped startup (low resources)
    pub fn bootstrap() -> Self {
        Self {
            cost_of_living: 0.35,
            market_demand: 0.25,
            competition: 0.15,
            infrastructure: 0.10,
            quality_of_life: 0.05,
            fiscal_incentives: 0.05,
            accessibility: 0.03,
            talent_availability: 0.02,
        }
    }

    /// Preset for funded startup (growth focus)
    pub fn growth_focused() -> Self {
        Self {
            cost_of_living: 0.10,
            market_demand: 0.35,
            competition: 0.10,
            infrastructure: 0.15,
            quality_of_life: 0.10,
            fiscal_incentives: 0.05,
            accessibility: 0.10,
            talent_availability: 0.05,
        }
    }

    /// Preset for remote-first company
    pub fn remote_first() -> Self {
        Self {
            cost_of_living: 0.25,
            market_demand: 0.15,
            competition: 0.10,
            infrastructure: 0.25,
            quality_of_life: 0.15,
            fiscal_incentives: 0.05,
            accessibility: 0.03,
            talent_availability: 0.02,
        }
    }
}

/// Quality of life metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityOfLife {
    /// Overall livability index (0-100)
    pub livability_index: f64,

    /// Safety score (0-100)
    pub safety_score: f64,

    /// Healthcare access score (0-100)
    pub healthcare_score: f64,

    /// Education quality score (0-100)
    pub education_score: f64,

    /// Cultural amenities score (0-100)
    pub culture_score: f64,

    /// Climate suitability score (0-100)
    pub climate_score: f64,

    /// Expat community size
    pub expat_community_size: u32,

    /// Air quality index (lower is better, 0-500)
    pub air_quality_index: f64,
}

impl QualityOfLife {
    pub fn overall_score(&self) -> f64 {
        let air_quality_score = 100.0 - (self.air_quality_index / 5.0).min(100.0);
        let expat_score = (self.expat_community_size as f64 / 100.0).min(100.0);

        (self.livability_index * 0.25 +
         self.safety_score * 0.20 +
         self.healthcare_score * 0.15 +
         self.education_score * 0.10 +
         self.culture_score * 0.10 +
         self.climate_score * 0.10 +
         expat_score * 0.05 +
         air_quality_score * 0.05)
    }
}
