use crate::errors::AppError;
use crate::optimization::MonteCarloSimulator;
use crate::models::SimulationResults;

/// Financial forecasting and sensitivity analysis
pub struct FinancialForecaster;

impl FinancialForecaster {
    /// Forecast revenue using linear regression
    pub fn forecast_revenue(
        historical_data: &[f64],
        periods_ahead: usize,
    ) -> Result<Vec<f64>, AppError> {
        if historical_data.len() < 2 {
            return Err(AppError::ValidationError(
                "Need at least 2 historical data points".to_string()
            ));
        }

        // Simple linear regression
        let n = historical_data.len() as f64;
        let x_values: Vec<f64> = (1..=historical_data.len()).map(|i| i as f64).collect();

        let x_mean = x_values.iter().sum::<f64>() / n;
        let y_mean = historical_data.iter().sum::<f64>() / n;

        let mut numerator = 0.0;
        let mut denominator = 0.0;

        for (x, y) in x_values.iter().zip(historical_data.iter()) {
            numerator += (x - x_mean) * (y - y_mean);
            denominator += (x - x_mean).powi(2);
        }

        let slope = numerator / denominator;
        let intercept = y_mean - slope * x_mean;

        // Generate forecasts
        let mut forecasts = Vec::new();
        for i in 1..=periods_ahead {
            let x = (historical_data.len() + i) as f64;
            let forecast = intercept + slope * x;
            forecasts.push(forecast.max(0.0)); // No negative revenue
        }

        Ok(forecasts)
    }

    /// Forecast expenses based on revenue
    pub fn forecast_expenses(
        revenue_forecast: &[f64],
        variable_cost_ratio: f64,
        fixed_costs: f64,
    ) -> Result<Vec<f64>, AppError> {
        if variable_cost_ratio < 0.0 || variable_cost_ratio > 1.0 {
            return Err(AppError::ValidationError(
                "Variable cost ratio must be between 0 and 1".to_string()
            ));
        }

        let expenses: Vec<f64> = revenue_forecast.iter()
            .map(|revenue| revenue * variable_cost_ratio + fixed_costs)
            .collect();

        Ok(expenses)
    }

    /// Generate profit & loss projection
    pub fn generate_pl_projection(
        revenue_forecast: &[f64],
        expense_forecast: &[f64],
        tax_rate: f64,
    ) -> Result<Vec<PLProjection>, AppError> {
        if revenue_forecast.len() != expense_forecast.len() {
            return Err(AppError::ValidationError(
                "Revenue and expense forecasts must have same length".to_string()
            ));
        }

        let mut projections = Vec::new();

        for (year, (revenue, expenses)) in revenue_forecast.iter()
            .zip(expense_forecast.iter())
            .enumerate()
        {
            let ebitda = revenue - expenses;
            let depreciation = revenue * 0.05; // Assume 5% depreciation
            let ebit = ebitda - depreciation;
            let tax = if ebit > 0.0 { ebit * tax_rate } else { 0.0 };
            let net_income = ebit - tax;

            projections.push(PLProjection {
                year: year + 1,
                revenue: *revenue,
                expenses: *expenses,
                ebitda,
                depreciation,
                ebit,
                tax,
                net_income,
            });
        }

        Ok(projections)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PLProjection {
    pub year: usize,
    pub revenue: f64,
    pub expenses: f64,
    pub ebitda: f64,
    pub depreciation: f64,
    pub ebit: f64,
    pub tax: f64,
    pub net_income: f64,
}

/// Sensitivity analyzer
pub struct SensitivityAnalyzer;

impl SensitivityAnalyzer {
    /// One-way sensitivity analysis
    pub fn one_way_sensitivity(
        base_value: f64,
        input_range: (f64, f64),
        calculate_output: impl Fn(f64) -> f64,
        steps: usize,
    ) -> Result<Vec<(f64, f64)>, AppError> {
        if steps == 0 {
            return Err(AppError::ValidationError("Steps must be greater than 0".to_string()));
        }

        let step_size = (input_range.1 - input_range.0) / steps as f64;
        let mut results = Vec::new();

        for i in 0..=steps {
            let input = input_range.0 + (i as f64 * step_size);
            let output = calculate_output(input);
            results.push((input, output));
        }

        Ok(results)
    }

    /// Two-way sensitivity analysis (data table)
    pub fn two_way_sensitivity(
        input1_range: (f64, f64),
        input2_range: (f64, f64),
        calculate_output: impl Fn(f64, f64) -> f64,
        steps: usize,
    ) -> Result<SensitivityTable, AppError> {
        if steps == 0 {
            return Err(AppError::ValidationError("Steps must be greater than 0".to_string()));
        }

        let step1 = (input1_range.1 - input1_range.0) / steps as f64;
        let step2 = (input2_range.1 - input2_range.0) / steps as f64;

        let mut input1_values = Vec::new();
        let mut input2_values = Vec::new();
        let mut output_matrix = Vec::new();

        for i in 0..=steps {
            input1_values.push(input1_range.0 + (i as f64 * step1));
            input2_values.push(input2_range.0 + (i as f64 * step2));
        }

        for &input1 in &input1_values {
            let mut row = Vec::new();
            for &input2 in &input2_values {
                row.push(calculate_output(input1, input2));
            }
            output_matrix.push(row);
        }

        Ok(SensitivityTable {
            input1_values,
            input2_values,
            output_matrix,
        })
    }

    /// Scenario analysis
    pub fn scenario_analysis(
        scenarios: Vec<ScenarioInput>,
        calculate_outcome: impl Fn(&ScenarioInput) -> f64,
    ) -> Result<Vec<ScenarioResult>, AppError> {
        let results: Vec<ScenarioResult> = scenarios.iter()
            .map(|scenario| {
                let outcome = calculate_outcome(scenario);
                ScenarioResult {
                    name: scenario.name.clone(),
                    probability: scenario.probability,
                    outcome,
                }
            })
            .collect();

        Ok(results)
    }

    /// Calculate expected value from scenarios
    pub fn expected_value(scenarios: &[ScenarioResult]) -> Result<f64, AppError> {
        let total_prob: f64 = scenarios.iter().map(|s| s.probability).sum();

        if (total_prob - 1.0).abs() > 0.01 {
            return Err(AppError::ValidationError(
                "Scenario probabilities must sum to 1.0".to_string()
            ));
        }

        let ev = scenarios.iter()
            .map(|s| s.outcome * s.probability)
            .sum();

        Ok(ev)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SensitivityTable {
    pub input1_values: Vec<f64>,
    pub input2_values: Vec<f64>,
    pub output_matrix: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScenarioInput {
    pub name: String,
    pub probability: f64,
    pub revenue: f64,
    pub costs: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScenarioResult {
    pub name: String,
    pub probability: f64,
    pub outcome: f64,
}

/// Risk analyzer using Value at Risk (VaR)
pub struct RiskAnalyzer;

impl RiskAnalyzer {
    /// Calculate Value at Risk (VaR) using historical simulation
    pub fn calculate_var(
        returns: &[f64],
        confidence_level: f64,
    ) -> Result<f64, AppError> {
        if returns.is_empty() {
            return Err(AppError::ValidationError("Returns cannot be empty".to_string()));
        }

        if confidence_level <= 0.0 || confidence_level >= 1.0 {
            return Err(AppError::ValidationError(
                "Confidence level must be between 0 and 1".to_string()
            ));
        }

        let mut sorted_returns = returns.to_vec();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let index = ((1.0 - confidence_level) * sorted_returns.len() as f64) as usize;
        let var = sorted_returns[index];

        Ok(-var) // VaR is reported as a positive number
    }

    /// Calculate Conditional Value at Risk (CVaR)
    pub fn calculate_cvar(
        returns: &[f64],
        confidence_level: f64,
    ) -> Result<f64, AppError> {
        let var = Self::calculate_var(returns, confidence_level)?;

        let mut sorted_returns = returns.to_vec();
        sorted_returns.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let cutoff_index = ((1.0 - confidence_level) * sorted_returns.len() as f64) as usize;
        let tail_returns = &sorted_returns[..cutoff_index];

        if tail_returns.is_empty() {
            return Ok(var);
        }

        let cvar = -tail_returns.iter().sum::<f64>() / tail_returns.len() as f64;

        Ok(cvar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revenue_forecast() {
        let historical = vec![100.0, 120.0, 140.0, 160.0];
        let forecast = FinancialForecaster::forecast_revenue(&historical, 3).unwrap();

        // Should be increasing trend
        assert!(forecast[0] > 160.0);
        assert!(forecast[1] > forecast[0]);
        assert!(forecast[2] > forecast[1]);
    }

    #[test]
    fn test_pl_projection() {
        let revenue = vec![1_000_000.0, 1_200_000.0, 1_400_000.0];
        let expenses = vec![600_000.0, 700_000.0, 800_000.0];

        let projections = FinancialForecaster::generate_pl_projection(
            &revenue,
            &expenses,
            0.21, // 21% tax
        ).unwrap();

        assert_eq!(projections.len(), 3);
        assert!(projections[0].net_income > 0.0);
    }

    #[test]
    fn test_var_calculation() {
        let returns = vec![-0.05, -0.02, 0.01, 0.03, 0.05, 0.07, 0.10];
        let var = RiskAnalyzer::calculate_var(&returns, 0.95).unwrap();

        // 95% VaR should be around 5%
        assert!(var > 0.04 && var < 0.06);
    }
}
