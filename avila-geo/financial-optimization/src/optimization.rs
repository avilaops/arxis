use crate::models::*;
use crate::errors::AppError;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;
use std::collections::HashMap;

/// Resource allocation using linear programming
pub struct ResourceAllocator;

impl ResourceAllocator {
    /// Optimize budget allocation across cities to maximize expected revenue
    ///
    /// This is a simplified version - in production, use good_lp crate
    /// for proper linear programming optimization
    pub fn optimize_allocation(
        budget: f64,
        cities: &[City],
        expected_revenue_per_unit: &HashMap<String, f64>,
    ) -> Result<HashMap<String, f64>, AppError> {
        if budget <= 0.0 {
            return Err(AppError::ValidationError("Budget must be positive".to_string()));
        }

        if cities.is_empty() {
            return Err(AppError::ValidationError("Must provide at least one city".to_string()));
        }

        // Greedy allocation based on revenue-to-cost ratio
        let mut allocations: HashMap<String, f64> = HashMap::new();
        let mut remaining_budget = budget;

        // Calculate ROI for each city
        let mut city_roi: Vec<_> = cities.iter()
            .map(|city| {
                let revenue = expected_revenue_per_unit.get(&city.name).unwrap_or(&0.0);
                let roi = revenue / city.cost_index;
                (city.name.clone(), roi, city.cost_index)
            })
            .collect();

        // Sort by ROI descending
        city_roi.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Allocate proportionally based on ROI
        let total_roi: f64 = city_roi.iter().map(|(_, roi, _)| roi).sum();

        for (city_name, roi, _) in city_roi {
            let allocation = budget * (roi / total_roi);
            allocations.insert(city_name, allocation.min(remaining_budget));
            remaining_budget -= allocation;

            if remaining_budget <= 0.0 {
                break;
            }
        }

        Ok(allocations)
    }
}

/// Break-even analysis calculator
pub struct BreakEvenAnalyzer;

impl BreakEvenAnalyzer {
    /// Calculate break-even point
    pub fn calculate_break_even(
        fixed_costs: f64,
        variable_cost_per_unit: f64,
        price_per_unit: f64,
    ) -> Result<BreakEvenAnalysis, AppError> {
        if fixed_costs < 0.0 {
            return Err(AppError::ValidationError("Fixed costs cannot be negative".to_string()));
        }

        if variable_cost_per_unit < 0.0 {
            return Err(AppError::ValidationError("Variable cost cannot be negative".to_string()));
        }

        if price_per_unit <= variable_cost_per_unit {
            return Err(AppError::ValidationError(
                "Price must be greater than variable cost".to_string()
            ));
        }

        let contribution_margin = price_per_unit - variable_cost_per_unit;
        let break_even_units = fixed_costs / contribution_margin;
        let break_even_revenue = break_even_units * price_per_unit;

        // Contribution margin ratio
        let contribution_margin_ratio = contribution_margin / price_per_unit;

        // Margin of safety (assume target is 2x break-even)
        let target_revenue = break_even_revenue * 2.0;
        let margin_of_safety = (target_revenue - break_even_revenue) / target_revenue;

        Ok(BreakEvenAnalysis {
            break_even_units: break_even_units.ceil() as usize,
            break_even_revenue,
            contribution_margin_ratio,
            margin_of_safety,
        })
    }

    /// Sensitivity analysis for break-even
    pub fn sensitivity_analysis(
        base_case: &BreakEvenAnalysis,
        fixed_costs: f64,
        variable_cost_per_unit: f64,
        price_per_unit: f64,
        variation_percent: f64,
    ) -> Result<Vec<Scenario>, AppError> {
        let mut scenarios = Vec::new();

        // Base case
        scenarios.push(Scenario {
            name: "Base Case".to_string(),
            break_even_units: base_case.break_even_units,
            break_even_revenue: base_case.break_even_revenue,
        });

        // Price variations
        for mult in [-variation_percent, variation_percent] {
            let new_price = price_per_unit * (1.0 + mult);
            let analysis = Self::calculate_break_even(
                fixed_costs,
                variable_cost_per_unit,
                new_price,
            )?;

            scenarios.push(Scenario {
                name: format!("Price {:+.0}%", mult * 100.0),
                break_even_units: analysis.break_even_units,
                break_even_revenue: analysis.break_even_revenue,
            });
        }

        // Cost variations
        for mult in [-variation_percent, variation_percent] {
            let new_variable_cost = variable_cost_per_unit * (1.0 + mult);
            if new_variable_cost >= price_per_unit {
                continue; // Skip invalid scenarios
            }

            let analysis = Self::calculate_break_even(
                fixed_costs,
                new_variable_cost,
                price_per_unit,
            )?;

            scenarios.push(Scenario {
                name: format!("Variable Cost {:+.0}%", mult * 100.0),
                break_even_units: analysis.break_even_units,
                break_even_revenue: analysis.break_even_revenue,
            });
        }

        Ok(scenarios)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Scenario {
    pub name: String,
    pub break_even_units: usize,
    pub break_even_revenue: f64,
}

/// Monte Carlo simulator for financial risk analysis
pub struct MonteCarloSimulator;

impl MonteCarloSimulator {
    /// Simulate financial outcomes using Monte Carlo method
    pub fn simulate_outcomes(
        base_revenue: f64,
        base_costs: f64,
        revenue_volatility: f64,
        cost_volatility: f64,
        iterations: usize,
    ) -> Result<SimulationResults, AppError> {
        if iterations == 0 {
            return Err(AppError::ValidationError("Iterations must be greater than 0".to_string()));
        }

        let mut rng = thread_rng();

        let revenue_dist = Normal::new(base_revenue, revenue_volatility)
            .map_err(|e| AppError::CalculationError(format!("Invalid revenue distribution: {}", e)))?;

        let cost_dist = Normal::new(base_costs, cost_volatility)
            .map_err(|e| AppError::CalculationError(format!("Invalid cost distribution: {}", e)))?;

        let mut profits = Vec::with_capacity(iterations);

        for _ in 0..iterations {
            let revenue = revenue_dist.sample(&mut rng).max(0.0);
            let costs = cost_dist.sample(&mut rng).max(0.0);
            let profit = revenue - costs;
            profits.push(profit);
        }

        profits.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mean = profits.iter().sum::<f64>() / profits.len() as f64;
        let median = profits[profits.len() / 2];
        let percentile_5 = profits[profits.len() * 5 / 100];
        let percentile_95 = profits[profits.len() * 95 / 100];

        let probability_loss = profits.iter()
            .filter(|&&p| p < 0.0)
            .count() as f64 / profits.len() as f64;

        // Calculate standard deviation
        let variance = profits.iter()
            .map(|p| (p - mean).powi(2))
            .sum::<f64>() / profits.len() as f64;
        let std_dev = variance.sqrt();

        // Coefficient of variation
        let cv = if mean != 0.0 { std_dev / mean.abs() } else { 0.0 };

        Ok(SimulationResults {
            mean,
            median,
            percentile_5,
            percentile_95,
            probability_loss,
            standard_deviation: std_dev,
            coefficient_of_variation: cv,
        })
    }

    /// Project-specific Monte Carlo simulation
    pub fn simulate_project_outcomes(
        expected_revenue: Vec<f64>,
        expected_costs: Vec<f64>,
        volatility: f64,
        iterations: usize,
    ) -> Result<Vec<SimulationResults>, AppError> {
        if expected_revenue.len() != expected_costs.len() {
            return Err(AppError::ValidationError(
                "Revenue and cost vectors must have same length".to_string()
            ));
        }

        let mut results = Vec::new();

        for (year, (revenue, cost)) in expected_revenue.iter().zip(expected_costs.iter()).enumerate() {
            let result = Self::simulate_outcomes(
                *revenue,
                *cost,
                revenue * volatility,
                cost * volatility,
                iterations,
            )?;

            results.push(result);
        }

        Ok(results)
    }
}

/// Portfolio optimization using Markowitz model
pub struct PortfolioOptimizer;

impl PortfolioOptimizer {
    /// Calculate Sharpe ratio
    pub fn sharpe_ratio(
        expected_return: f64,
        risk_free_rate: f64,
        standard_deviation: f64,
    ) -> Result<f64, AppError> {
        if standard_deviation == 0.0 {
            return Err(AppError::CalculationError("Standard deviation cannot be zero".to_string()));
        }

        Ok((expected_return - risk_free_rate) / standard_deviation)
    }

    /// Calculate Sortino ratio (downside risk only)
    pub fn sortino_ratio(
        expected_return: f64,
        risk_free_rate: f64,
        downside_deviation: f64,
    ) -> Result<f64, AppError> {
        if downside_deviation == 0.0 {
            return Err(AppError::CalculationError("Downside deviation cannot be zero".to_string()));
        }

        Ok((expected_return - risk_free_rate) / downside_deviation)
    }

    /// Calculate portfolio expected return
    pub fn portfolio_return(
        weights: &[f64],
        expected_returns: &[f64],
    ) -> Result<f64, AppError> {
        if weights.len() != expected_returns.len() {
            return Err(AppError::ValidationError(
                "Weights and returns must have same length".to_string()
            ));
        }

        let total_weight: f64 = weights.iter().sum();
        if (total_weight - 1.0).abs() > 0.001 {
            return Err(AppError::ValidationError(
                "Weights must sum to 1.0".to_string()
            ));
        }

        let return_val = weights.iter()
            .zip(expected_returns.iter())
            .map(|(w, r)| w * r)
            .sum();

        Ok(return_val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_even_calculation() {
        let result = BreakEvenAnalyzer::calculate_break_even(
            100_000.0,  // Fixed costs
            20.0,       // Variable cost per unit
            50.0,       // Price per unit
        ).unwrap();

        assert_eq!(result.break_even_units, 3334); // 100000 / (50-20) = 3333.33
        assert_eq!(result.contribution_margin_ratio, 0.6); // (50-20)/50
    }

    #[test]
    fn test_monte_carlo_simulation() {
        let result = MonteCarloSimulator::simulate_outcomes(
            1_000_000.0,  // Base revenue
            600_000.0,    // Base costs
            100_000.0,    // Revenue volatility
            50_000.0,     // Cost volatility
            10_000,       // Iterations
        ).unwrap();

        // Mean profit should be around 400k
        assert!(result.mean > 350_000.0 && result.mean < 450_000.0);

        // Should have some probability of loss (though small)
        assert!(result.probability_loss >= 0.0 && result.probability_loss <= 0.1);
    }

    #[test]
    fn test_sharpe_ratio() {
        let sharpe = PortfolioOptimizer::sharpe_ratio(
            0.12,  // 12% expected return
            0.03,  // 3% risk-free rate
            0.15,  // 15% standard deviation
        ).unwrap();

        assert_eq!(sharpe, 0.6); // (0.12 - 0.03) / 0.15
    }

    #[test]
    fn test_portfolio_return() {
        let weights = vec![0.4, 0.3, 0.3];
        let returns = vec![0.10, 0.15, 0.08];

        let portfolio_ret = PortfolioOptimizer::portfolio_return(&weights, &returns).unwrap();

        // 0.4*0.10 + 0.3*0.15 + 0.3*0.08 = 0.109
        assert!((portfolio_ret - 0.109).abs() < 0.001);
    }
}
