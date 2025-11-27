//! Financial analysis algorithms

use crate::models::FinancialProjection;
use rand::Rng;
use rand_distr::{Distribution, Normal};

/// Monte Carlo simulation for financial projections
pub struct MonteCarloSimulation {
    pub num_simulations: usize,
    pub projection: FinancialProjection,
}

impl MonteCarloSimulation {
    pub fn new(projection: FinancialProjection, num_simulations: usize) -> Self {
        Self {
            num_simulations,
            projection,
        }
    }

    /// Run simulation with uncertainty in revenue and costs
    pub fn simulate(
        &self,
        years: u32,
        revenue_std_dev: f64,
        cost_std_dev: f64,
        discount_rate: f64,
    ) -> MonteCarloResult {
        let mut rng = rand::thread_rng();
        let revenue_dist = Normal::new(1.0, revenue_std_dev).unwrap();
        let cost_dist = Normal::new(1.0, cost_std_dev).unwrap();

        let mut npvs = Vec::with_capacity(self.num_simulations);

        for _ in 0..self.num_simulations {
            let mut npv = -self.projection.initial_investment;

            for year in 1..=years {
                let revenue_multiplier = revenue_dist.sample(&mut rng).max(0.1);
                let cost_multiplier = cost_dist.sample(&mut rng).max(0.1);

                let annual_revenue = self.projection.expected_monthly_revenue * 12.0
                    * (1.0 + self.projection.revenue_growth_rate / 100.0).powi(year as i32 - 1)
                    * revenue_multiplier;

                let annual_cost = self.projection.monthly_operational_cost * 12.0 * cost_multiplier;
                let annual_profit = (annual_revenue - annual_cost) * (self.projection.profit_margin / 100.0);

                let discount_factor = (1.0 + discount_rate).powi(-(year as i32));
                npv += annual_profit * discount_factor;
            }

            npvs.push(npv);
        }

        npvs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean_npv = npvs.iter().sum::<f64>() / npvs.len() as f64;
        let variance = npvs.iter().map(|x| (x - mean_npv).powi(2)).sum::<f64>() / npvs.len() as f64;
        let std_dev = variance.sqrt();

        let percentile_5 = npvs[(npvs.len() as f64 * 0.05) as usize];
        let percentile_95 = npvs[(npvs.len() as f64 * 0.95) as usize];
        let median = npvs[npvs.len() / 2];

        let probability_positive = npvs.iter().filter(|&&x| x > 0.0).count() as f64 / npvs.len() as f64;

        MonteCarloResult {
            mean_npv,
            median_npv: median,
            std_dev,
            percentile_5,
            percentile_95,
            probability_positive,
            all_npvs: npvs,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MonteCarloResult {
    pub mean_npv: f64,
    pub median_npv: f64,
    pub std_dev: f64,
    pub percentile_5: f64,
    pub percentile_95: f64,
    pub probability_positive: f64,
    pub all_npvs: Vec<f64>,
}

/// Tax optimization for multi-jurisdiction setup
pub struct TaxOptimizer {
    /// Available tax jurisdictions with rates
    pub jurisdictions: Vec<(String, f64)>,

    /// Expected revenue distribution
    pub revenue_sources: Vec<(String, f64)>,
}

impl TaxOptimizer {
    pub fn optimize(&self, total_revenue: f64) -> TaxOptimizationResult {
        // Simple optimization: allocate revenue to lowest tax jurisdiction
        let min_tax = self.jurisdictions
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap();

        let tax_amount = total_revenue * (min_tax.1 / 100.0);
        let after_tax = total_revenue - tax_amount;

        TaxOptimizationResult {
            optimal_jurisdiction: min_tax.0.clone(),
            tax_rate: min_tax.1,
            tax_amount,
            after_tax_revenue: after_tax,
            tax_savings: self.calculate_savings(total_revenue, min_tax.1),
        }
    }

    fn calculate_savings(&self, revenue: f64, optimal_rate: f64) -> f64 {
        let avg_rate = self.jurisdictions.iter().map(|(_, r)| r).sum::<f64>()
            / self.jurisdictions.len() as f64;

        revenue * ((avg_rate - optimal_rate) / 100.0)
    }
}

#[derive(Debug, Clone)]
pub struct TaxOptimizationResult {
    pub optimal_jurisdiction: String,
    pub tax_rate: f64,
    pub tax_amount: f64,
    pub after_tax_revenue: f64,
    pub tax_savings: f64,
}

/// Sensitivity analysis for key parameters
pub struct SensitivityAnalysis {
    pub base_projection: FinancialProjection,
}

impl SensitivityAnalysis {
    pub fn analyze(&self, years: u32, discount_rate: f64) -> SensitivityResult {
        let base_npv = self.base_projection.npv(years, discount_rate);

        // Analyze sensitivity to revenue
        let revenue_plus_10 = FinancialProjection {
            expected_monthly_revenue: self.base_projection.expected_monthly_revenue * 1.1,
            ..self.base_projection.clone()
        };
        let revenue_npv_change = revenue_plus_10.npv(years, discount_rate) - base_npv;

        // Analyze sensitivity to cost
        let cost_plus_10 = FinancialProjection {
            monthly_operational_cost: self.base_projection.monthly_operational_cost * 1.1,
            ..self.base_projection.clone()
        };
        let cost_npv_change = cost_plus_10.npv(years, discount_rate) - base_npv;

        // Analyze sensitivity to growth rate
        let growth_plus_1 = FinancialProjection {
            revenue_growth_rate: self.base_projection.revenue_growth_rate + 1.0,
            ..self.base_projection.clone()
        };
        let growth_npv_change = growth_plus_1.npv(years, discount_rate) - base_npv;

        SensitivityResult {
            base_npv,
            revenue_sensitivity: revenue_npv_change / base_npv * 100.0,
            cost_sensitivity: cost_npv_change.abs() / base_npv * 100.0,
            growth_sensitivity: growth_npv_change / base_npv * 100.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SensitivityResult {
    pub base_npv: f64,
    pub revenue_sensitivity: f64,
    pub cost_sensitivity: f64,
    pub growth_sensitivity: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monte_carlo() {
        let projection = FinancialProjection {
            initial_investment: 50000.0,
            monthly_operational_cost: 5000.0,
            expected_monthly_revenue: 10000.0,
            revenue_growth_rate: 20.0,
            profit_margin: 30.0,
        };

        let mc = MonteCarloSimulation::new(projection, 1000);
        let result = mc.simulate(5, 0.1, 0.05, 0.1);

        assert!(result.probability_positive >= 0.0 && result.probability_positive <= 1.0);
        assert!(result.mean_npv != 0.0);
    }
}
