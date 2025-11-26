use crate::errors::AppError;
use crate::models::*;

/// DCF (Discounted Cash Flow) valuation model
#[derive(Debug, Clone)]
pub struct DCFModel {
    pub discount_rate: f64,
    pub terminal_growth_rate: f64,
    pub forecast_years: usize,
}

impl DCFModel {
    pub fn new(discount_rate: f64, terminal_growth_rate: f64, forecast_years: usize) -> Self {
        Self {
            discount_rate,
            terminal_growth_rate,
            forecast_years,
        }
    }

    /// Calculate NPV (Net Present Value) of future cash flows
    pub fn calculate_npv(&self, cash_flows: &[f64]) -> Result<f64, AppError> {
        if cash_flows.is_empty() {
            return Err(AppError::ValidationError("Cash flows cannot be empty".to_string()));
        }

        let pv_operating = cash_flows.iter()
            .enumerate()
            .map(|(year, cf)| {
                cf / (1.0 + self.discount_rate).powi((year + 1) as i32)
            })
            .sum::<f64>();

        let terminal_value = self.calculate_terminal_value(cash_flows)?;

        Ok(pv_operating + terminal_value)
    }

    /// Calculate terminal value using perpetuity growth model
    pub fn calculate_terminal_value(&self, cash_flows: &[f64]) -> Result<f64, AppError> {
        if cash_flows.is_empty() {
            return Err(AppError::ValidationError("Cash flows cannot be empty".to_string()));
        }

        if self.discount_rate <= self.terminal_growth_rate {
            return Err(AppError::ValidationError(
                "Discount rate must be greater than terminal growth rate".to_string()
            ));
        }

        let last_cf = cash_flows.last().unwrap();
        let terminal_cf = last_cf * (1.0 + self.terminal_growth_rate);
        let terminal_value = terminal_cf / (self.discount_rate - self.terminal_growth_rate);

        // Discount to present value
        let pv_terminal = terminal_value / (1.0 + self.discount_rate).powi(self.forecast_years as i32);

        Ok(pv_terminal)
    }

    /// Full DCF valuation
    pub fn calculate_dcf(
        &self,
        cash_flows: &[f64],
        debt: f64,
        cash: f64,
        shares_outstanding: f64,
    ) -> Result<DCFResult, AppError> {
        let npv = self.calculate_npv(cash_flows)?;
        let terminal_value = self.calculate_terminal_value(cash_flows)?;

        let enterprise_value = npv;
        let equity_value = enterprise_value - debt + cash;
        let value_per_share = if shares_outstanding > 0.0 {
            equity_value / shares_outstanding
        } else {
            0.0
        };

        let pv_by_year: Vec<f64> = cash_flows.iter()
            .enumerate()
            .map(|(year, cf)| {
                cf / (1.0 + self.discount_rate).powi((year + 1) as i32)
            })
            .collect();

        Ok(DCFResult {
            npv: equity_value,
            terminal_value,
            enterprise_value,
            pv_by_year,
        })
    }

    /// Sensitivity to WACC (discount rate)
    pub fn sensitivity_to_wacc(
        &self,
        cash_flows: &[f64],
        wacc_range: (f64, f64),
        step: f64,
    ) -> Result<Vec<(f64, f64)>, AppError> {
        let mut results = Vec::new();
        let mut wacc = wacc_range.0;

        while wacc <= wacc_range.1 {
            let mut model = self.clone();
            model.discount_rate = wacc;
            let npv = model.calculate_npv(cash_flows)?;
            results.push((wacc, npv));
            wacc += step;
        }

        Ok(results)
    }

    /// Sensitivity to terminal growth rate
    pub fn sensitivity_to_growth(
        &self,
        cash_flows: &[f64],
        growth_range: (f64, f64),
        step: f64,
    ) -> Result<Vec<(f64, f64)>, AppError> {
        let mut results = Vec::new();
        let mut growth = growth_range.0;

        while growth <= growth_range.1 && growth < self.discount_rate {
            let mut model = self.clone();
            model.terminal_growth_rate = growth;
            let npv = model.calculate_npv(cash_flows)?;
            results.push((growth, npv));
            growth += step;
        }

        Ok(results)
    }
}

/// IRR (Internal Rate of Return) calculator
pub struct IRRCalculator;

impl IRRCalculator {
    /// Calculate IRR using Newton-Raphson method
    pub fn calculate_irr(cash_flows: &[f64], initial_guess: f64) -> Result<f64, AppError> {
        if cash_flows.is_empty() {
            return Err(AppError::ValidationError("Cash flows cannot be empty".to_string()));
        }

        const MAX_ITERATIONS: usize = 100;
        const TOLERANCE: f64 = 1e-6;

        let mut rate = initial_guess;

        for _ in 0..MAX_ITERATIONS {
            let npv = Self::npv_at_rate(cash_flows, rate);
            let npv_derivative = Self::npv_derivative(cash_flows, rate);

            if npv_derivative.abs() < 1e-10 {
                return Err(AppError::CalculationError("IRR calculation did not converge".to_string()));
            }

            let new_rate = rate - npv / npv_derivative;

            if (new_rate - rate).abs() < TOLERANCE {
                return Ok(new_rate);
            }

            rate = new_rate;
        }

        Err(AppError::CalculationError("IRR calculation did not converge".to_string()))
    }

    fn npv_at_rate(cash_flows: &[f64], rate: f64) -> f64 {
        cash_flows.iter()
            .enumerate()
            .map(|(year, cf)| cf / (1.0 + rate).powi(year as i32))
            .sum()
    }

    fn npv_derivative(cash_flows: &[f64], rate: f64) -> f64 {
        cash_flows.iter()
            .enumerate()
            .skip(1)
            .map(|(year, cf)| {
                -(year as f64) * cf / (1.0 + rate).powi((year + 1) as i32)
            })
            .sum()
    }
}

/// WACC (Weighted Average Cost of Capital) calculator
pub struct WACCCalculator;

impl WACCCalculator {
    /// Calculate WACC
    pub fn calculate_wacc(
        equity_value: f64,
        debt_value: f64,
        cost_of_equity: f64,
        cost_of_debt: f64,
        tax_rate: f64,
    ) -> Result<f64, AppError> {
        if equity_value < 0.0 || debt_value < 0.0 {
            return Err(AppError::ValidationError("Values cannot be negative".to_string()));
        }

        let total_value = equity_value + debt_value;
        if total_value == 0.0 {
            return Err(AppError::ValidationError("Total value cannot be zero".to_string()));
        }

        let equity_weight = equity_value / total_value;
        let debt_weight = debt_value / total_value;

        let wacc = equity_weight * cost_of_equity
                 + debt_weight * cost_of_debt * (1.0 - tax_rate);

        Ok(wacc)
    }

    /// Calculate cost of equity using CAPM
    pub fn cost_of_equity_capm(
        risk_free_rate: f64,
        beta: f64,
        market_return: f64,
    ) -> f64 {
        risk_free_rate + beta * (market_return - risk_free_rate)
    }
}

/// Payback period calculator
pub struct PaybackCalculator;

impl PaybackCalculator {
    /// Calculate simple payback period
    pub fn simple_payback(initial_investment: f64, annual_cash_flow: f64) -> Result<f64, AppError> {
        if initial_investment <= 0.0 {
            return Err(AppError::ValidationError("Initial investment must be positive".to_string()));
        }

        if annual_cash_flow <= 0.0 {
            return Err(AppError::ValidationError("Annual cash flow must be positive".to_string()));
        }

        Ok(initial_investment / annual_cash_flow)
    }

    /// Calculate discounted payback period
    pub fn discounted_payback(
        initial_investment: f64,
        cash_flows: &[f64],
        discount_rate: f64,
    ) -> Result<Option<f64>, AppError> {
        if initial_investment <= 0.0 {
            return Err(AppError::ValidationError("Initial investment must be positive".to_string()));
        }

        let mut cumulative_pv = 0.0;

        for (year, cf) in cash_flows.iter().enumerate() {
            let pv = cf / (1.0 + discount_rate).powi((year + 1) as i32);
            cumulative_pv += pv;

            if cumulative_pv >= initial_investment {
                // Interpolate to get fractional year
                let previous_cumulative = cumulative_pv - pv;
                let fraction = (initial_investment - previous_cumulative) / pv;
                return Ok(Some((year as f64) + fraction));
            }
        }

        Ok(None) // Never pays back
    }
}

/// ROI (Return on Investment) calculator
pub struct ROICalculator;

impl ROICalculator {
    /// Calculate simple ROI
    pub fn calculate_roi(gain: f64, cost: f64) -> Result<f64, AppError> {
        if cost == 0.0 {
            return Err(AppError::ValidationError("Cost cannot be zero".to_string()));
        }

        Ok((gain - cost) / cost)
    }

    /// Calculate annualized ROI
    pub fn annualized_roi(total_return: f64, years: f64) -> Result<f64, AppError> {
        if years <= 0.0 {
            return Err(AppError::ValidationError("Years must be positive".to_string()));
        }

        Ok((1.0 + total_return).powf(1.0 / years) - 1.0)
    }

    /// Calculate ROAS (Return on Ad Spend)
    pub fn calculate_roas(revenue: f64, ad_spend: f64) -> Result<f64, AppError> {
        if ad_spend == 0.0 {
            return Err(AppError::ValidationError("Ad spend cannot be zero".to_string()));
        }

        Ok(revenue / ad_spend)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dcf_npv() {
        let model = DCFModel::new(0.10, 0.03, 5);
        let cash_flows = vec![100.0, 110.0, 120.0, 130.0, 140.0];

        let npv = model.calculate_npv(&cash_flows).unwrap();

        // Manual calculation: 100/1.1 + 110/1.21 + 120/1.331 + 130/1.4641 + 140/1.61051 + TV
        assert!(npv > 0.0);
    }

    #[test]
    fn test_irr_calculation() {
        let cash_flows = vec![-1000.0, 300.0, 400.0, 500.0, 200.0];

        let irr = IRRCalculator::calculate_irr(&cash_flows, 0.10).unwrap();

        // IRR should be around 14-15%
        assert!(irr > 0.10 && irr < 0.20);
    }

    #[test]
    fn test_wacc() {
        let wacc = WACCCalculator::calculate_wacc(
            6_000_000.0,  // Equity value
            4_000_000.0,  // Debt value
            0.12,         // Cost of equity
            0.06,         // Cost of debt
            0.21,         // Tax rate
        ).unwrap();

        // WACC = 0.6 * 0.12 + 0.4 * 0.06 * (1-0.21) = 0.072 + 0.01896 = 0.09096
        assert!((wacc - 0.09096).abs() < 0.0001);
    }

    #[test]
    fn test_capm() {
        let cost_of_equity = WACCCalculator::cost_of_equity_capm(
            0.03,  // Risk-free rate
            1.2,   // Beta
            0.10,  // Market return
        );

        // CAPM = 0.03 + 1.2 * (0.10 - 0.03) = 0.03 + 0.084 = 0.114
        assert_eq!(cost_of_equity, 0.114);
    }

    #[test]
    fn test_simple_payback() {
        let payback = PaybackCalculator::simple_payback(100_000.0, 25_000.0).unwrap();
        assert_eq!(payback, 4.0); // 4 years
    }

    #[test]
    fn test_roi() {
        let roi = ROICalculator::calculate_roi(150_000.0, 100_000.0).unwrap();
        assert_eq!(roi, 0.5); // 50% ROI
    }
}
