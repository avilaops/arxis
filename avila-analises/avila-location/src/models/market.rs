//! Market analysis data structures

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Market potential and demand analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    /// Total number of companies in the region
    pub total_companies: u64,

    /// Companies by sector
    pub companies_by_sector: HashMap<Sector, u64>,

    /// Companies by size
    pub companies_by_size: HashMap<CompanySize, u64>,

    /// Digital maturity index (0-100)
    pub digital_maturity_index: f64,

    /// Tech adoption rate (0-100)
    pub tech_adoption_rate: f64,

    /// Annual IT spending per company (in local currency)
    pub avg_it_spending: f64,

    /// Market growth rate (annual %)
    pub market_growth_rate: f64,

    /// Number of tech events per year
    pub tech_events_annual: u32,

    /// Number of tech communities/meetups
    pub tech_communities: u32,
}

impl MarketData {
    pub fn potential_customers(&self, target_sectors: &[Sector]) -> u64 {
        target_sectors
            .iter()
            .filter_map(|s| self.companies_by_sector.get(s))
            .sum()
    }

    pub fn market_score(&self) -> f64 {
        let size_score = (self.total_companies as f64 / 10000.0).min(100.0);
        let maturity_score = self.digital_maturity_index;
        let adoption_score = self.tech_adoption_rate;
        let growth_score = (self.market_growth_rate * 10.0).min(100.0);

        (size_score * 0.30 +
         maturity_score * 0.25 +
         adoption_score * 0.25 +
         growth_score * 0.20)
    }
}

/// Industry sectors
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Sector {
    Technology,
    FinTech,
    ECommerce,
    Retail,
    Healthcare,
    Education,
    Manufacturing,
    Logistics,
    Tourism,
    RealEstate,
    Construction,
    Consulting,
    Marketing,
    Legal,
    Other,
}

impl Sector {
    pub fn tech_demand_multiplier(&self) -> f64 {
        match self {
            Sector::Technology => 2.0,
            Sector::FinTech => 1.8,
            Sector::ECommerce => 1.6,
            Sector::Healthcare => 1.4,
            Sector::Education => 1.3,
            Sector::Consulting => 1.5,
            Sector::Marketing => 1.4,
            Sector::Retail => 1.2,
            Sector::Logistics => 1.3,
            Sector::Manufacturing => 1.1,
            _ => 1.0,
        }
    }
}

/// Company size classification
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CompanySize {
    /// 1-9 employees
    Micro,

    /// 10-49 employees
    Small,

    /// 50-249 employees
    Medium,

    /// 250+ employees
    Large,
}

impl CompanySize {
    pub fn it_budget_range(&self) -> (f64, f64) {
        match self {
            CompanySize::Micro => (1000.0, 10000.0),
            CompanySize::Small => (10000.0, 50000.0),
            CompanySize::Medium => (50000.0, 500000.0),
            CompanySize::Large => (500000.0, 10000000.0),
        }
    }
}

/// Demand analysis metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandMetrics {
    /// Estimated addressable market (annual revenue potential)
    pub addressable_market: f64,

    /// Serviceable market (realistic capture)
    pub serviceable_market: f64,

    /// Target market (initial focus)
    pub target_market: f64,

    /// Customer acquisition cost estimate
    pub customer_acquisition_cost: f64,

    /// Average deal size
    pub avg_deal_size: f64,

    /// Sales cycle length (days)
    pub sales_cycle_days: u32,
}
