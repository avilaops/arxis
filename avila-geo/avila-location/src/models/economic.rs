//! Economic and fiscal data models

use super::Country;
use serde::{Deserialize, Serialize};

/// Economic data for a region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicData {
    /// GDP per capita (local currency)
    pub gdp_per_capita: f64,

    /// GDP growth rate (annual %)
    pub gdp_growth_rate: f64,

    /// Cost of living index (0-100, relative)
    pub cost_of_living_index: f64,

    /// Average rent for office space (per m² monthly)
    pub office_rent_per_m2: f64,

    /// Average residential rent (monthly)
    pub residential_rent: f64,

    /// Food cost index
    pub food_cost_index: f64,

    /// Transportation cost (monthly)
    pub transportation_cost: f64,

    /// Healthcare quality score (0-100)
    pub healthcare_quality: f64,

    /// Foreign direct investment (annual, millions)
    pub fdi_annual_millions: f64,
}

impl EconomicData {
    pub fn total_monthly_cost_estimate(&self, office_size_m2: f64) -> f64 {
        let office_cost = self.office_rent_per_m2 * office_size_m2;
        let living_cost = self.residential_rent +
                         self.food_cost_index * 5.0 +
                         self.transportation_cost;

        office_cost + living_cost
    }

    pub fn economic_score(&self) -> f64 {
        let growth_score = (self.gdp_growth_rate * 20.0).min(100.0);
        let cost_score = 100.0 - self.cost_of_living_index;
        let investment_score = (self.fdi_annual_millions / 100.0).min(100.0);
        let healthcare_score = self.healthcare_quality;

        (growth_score * 0.25 +
         cost_score * 0.30 +
         investment_score * 0.25 +
         healthcare_score * 0.20)
    }
}

/// Tax and fiscal data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalData {
    pub country: Country,

    /// Corporate income tax rate (%)
    pub corporate_tax_rate: f64,

    /// VAT/Sales tax rate (%)
    pub vat_rate: f64,

    /// Social security rate (% of salary)
    pub social_security_rate: f64,

    /// Personal income tax (top rate %)
    pub personal_income_tax: f64,

    /// Capital gains tax (%)
    pub capital_gains_tax: f64,

    /// Dividend tax (%)
    pub dividend_tax: f64,

    /// Available tax incentives
    pub incentives: Vec<TaxIncentive>,

    /// Ease of doing business rank (lower is better)
    pub ease_of_business_rank: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxIncentive {
    pub name: String,
    pub description: String,
    pub tax_reduction_percent: f64,
    pub duration_years: u32,
    pub requirements: Vec<String>,
}

impl FiscalData {
    /// Calculate effective tax rate considering incentives
    pub fn effective_corporate_tax_rate(&self) -> f64 {
        let base_rate = self.corporate_tax_rate;
        let max_reduction = self.incentives
            .iter()
            .map(|i| i.tax_reduction_percent)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(0.0);

        base_rate * (1.0 - max_reduction / 100.0)
    }

    pub fn total_tax_burden(&self) -> f64 {
        self.effective_corporate_tax_rate() +
        self.vat_rate +
        self.social_security_rate
    }

    pub fn fiscal_score(&self) -> f64 {
        let tax_score = 100.0 - (self.total_tax_burden() / 2.0);
        let ease_score = 100.0 - (self.ease_of_business_rank as f64 / 2.0).min(100.0);
        let incentive_score = (self.incentives.len() as f64 * 10.0).min(100.0);

        (tax_score * 0.50 + ease_score * 0.30 + incentive_score * 0.20)
    }
}

/// Financial projections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialProjection {
    /// Initial investment required
    pub initial_investment: f64,

    /// Monthly operational cost
    pub monthly_operational_cost: f64,

    /// Expected monthly revenue (Year 1)
    pub expected_monthly_revenue: f64,

    /// Revenue growth rate (annual %)
    pub revenue_growth_rate: f64,

    /// Profit margin (%)
    pub profit_margin: f64,
}

impl FinancialProjection {
    /// Calculate Net Present Value over N years
    pub fn npv(&self, years: u32, discount_rate: f64) -> f64 {
        let mut npv = -self.initial_investment;

        for year in 1..=years {
            let annual_revenue = self.expected_monthly_revenue * 12.0
                * (1.0 + self.revenue_growth_rate / 100.0).powi(year as i32 - 1);
            let annual_cost = self.monthly_operational_cost * 12.0;
            let annual_profit = (annual_revenue - annual_cost) * (self.profit_margin / 100.0);

            let discount_factor = (1.0 + discount_rate).powi(-(year as i32));
            npv += annual_profit * discount_factor;
        }

        npv
    }

    /// Calculate Internal Rate of Return
    pub fn irr(&self, years: u32) -> f64 {
        // Simple IRR approximation using binary search
        let mut low = 0.0;
        let mut high = 1.0;
        let tolerance = 0.0001;

        for _ in 0..100 {
            let mid = (low + high) / 2.0;
            let npv = self.npv(years, mid);

            if npv.abs() < tolerance {
                return mid * 100.0; // Return as percentage
            }

            if npv > 0.0 {
                low = mid;
            } else {
                high = mid;
            }
        }

        ((low + high) / 2.0) * 100.0
    }

    /// Calculate break-even point (months)
    pub fn break_even_months(&self) -> f64 {
        let monthly_profit = self.expected_monthly_revenue * (self.profit_margin / 100.0)
            - self.monthly_operational_cost;

        if monthly_profit <= 0.0 {
            f64::INFINITY
        } else {
            self.initial_investment / monthly_profit
        }
    }
}
