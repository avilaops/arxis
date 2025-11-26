//! Pre-populated data for analysis

pub mod portugal;
pub mod uae;

pub use portugal::*;
pub use uae::*;

use crate::models::*;

/// Load all available region data
pub fn load_all_regions() -> Vec<RegionData> {
    let mut regions = Vec::new();
    regions.extend(portugal::load_portugal_regions());
    regions.extend(uae::load_uae_regions());
    regions
}

/// Complete region data bundle
#[derive(Debug, Clone)]
pub struct RegionData {
    pub region: Region,
    pub market: MarketData,
    pub competition: CompetitionData,
    pub infrastructure: InfrastructureData,
    pub talent: TalentData,
    pub economic: EconomicData,
    pub fiscal: FiscalData,
    pub quality_of_life: QualityOfLife,
}

impl RegionData {
    /// Generate comprehensive location score
    pub fn calculate_score(&self, weights: &ScoringWeights) -> LocationScore {
        let mut score = LocationScore::new(&self.region.location.id, &self.region.location.name);

        // Cost of living (lower is better)
        score.cost_of_living = 100.0 - self.economic.cost_of_living_index;

        // Market demand (higher is better)
        score.market_demand = self.market.market_score();

        // Competition (lower competition = higher score)
        score.competition = self.competition.competition_score();

        // Infrastructure
        score.infrastructure = self.infrastructure.infrastructure_score();

        // Quality of life
        score.quality_of_life = self.quality_of_life.overall_score();

        // Fiscal incentives
        score.fiscal_incentives = self.fiscal.fiscal_score();

        // Accessibility
        score.accessibility = self.region.accessibility.overall_score();

        // Talent availability
        score.talent_availability = self.talent.talent_score();

        // Calculate weighted total
        score.calculate_total(weights);

        score
    }
}
