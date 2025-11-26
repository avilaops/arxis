use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::errors::AppError;
use crate::models::*;
use crate::portugal_tax::*;
use crate::vat_optimizer::*;
use crate::optimization::*;
use crate::financial_models::*;
use crate::corporate_structure::*;
use crate::simulators::*;

// ============================================================================
// Request/Response Types
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct CalculateIRCRequest {
    pub taxable_income: f64,
    pub location: Location,
}

#[derive(Debug, Serialize)]
pub struct CalculateIRCResponse {
    pub taxable_income: f64,
    pub base_rate: f64,
    pub state_surcharge: f64,
    pub municipal_surcharge: f64,
    pub total_tax: f64,
    pub effective_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct CalculateSIFIDERequest {
    pub current_rd_expenses: f64,
    pub previous_2_years_avg: f64,
}

#[derive(Debug, Serialize)]
pub struct CalculateSIFIDEResponse {
    pub base_credit: f64,
    pub incremental_credit: f64,
    pub total_credit: f64,
    pub credit_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct CalculatePatentBoxRequest {
    pub ip_income: f64,
    pub nexus_ratio: f64,
}

#[derive(Debug, Serialize)]
pub struct CalculatePatentBoxResponse {
    pub ip_income: f64,
    pub qualified_income: f64,
    pub exempt_income: f64,
    pub tax_saving: f64,
}

#[derive(Debug, Deserialize)]
pub struct EffectiveTaxRateRequest {
    pub taxable_income: f64,
    pub location: Location,
    pub rd_expenses: f64,
    pub ip_income: f64,
    pub nexus_ratio: f64,
}

#[derive(Debug, Deserialize)]
pub struct CrossBorderVATRequest {
    pub revenue: f64,
    pub customer_location: Country,
    pub service_type: ServiceType,
    pub is_business_customer: bool,
}

#[derive(Debug, Serialize)]
pub struct CrossBorderVATResponse {
    pub strategy: String,
    pub vat_amount: f64,
    pub effective_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct VATRecoveryRequest {
    pub expenses: Vec<Expense>,
}

#[derive(Debug, Serialize)]
pub struct VATRecoveryResponse {
    pub total_vat_recoverable: f64,
    pub num_deductible_expenses: usize,
}

#[derive(Debug, Deserialize)]
pub struct LinearProgrammingRequest {
    pub budget: f64,
    pub cities: Vec<City>,
    pub expected_revenue_per_unit: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct BreakEvenRequest {
    pub fixed_costs: f64,
    pub variable_cost_per_unit: f64,
    pub price_per_unit: f64,
}

#[derive(Debug, Deserialize)]
pub struct DCFRequest {
    pub cash_flows: Vec<f64>,
    pub discount_rate: f64,
    pub terminal_growth_rate: f64,
    pub debt: f64,
    pub cash: f64,
    pub shares_outstanding: f64,
}

#[derive(Debug, Deserialize)]
pub struct NPVRequest {
    pub cash_flows: Vec<f64>,
    pub discount_rate: f64,
}

#[derive(Debug, Deserialize)]
pub struct IRRRequest {
    pub cash_flows: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct IRRResponse {
    pub irr: f64,
    pub irr_percentage: String,
}

#[derive(Debug, Deserialize)]
pub struct OptimizeStructureRequest {
    pub revenue_by_country: HashMap<String, f64>,
    pub objectives: OptimizationObjectives,
}

#[derive(Debug, Deserialize)]
pub struct MonteCarloRequest {
    pub base_revenue: f64,
    pub base_costs: f64,
    pub revenue_volatility: f64,
    pub cost_volatility: f64,
    pub iterations: usize,
}

#[derive(Debug, Deserialize)]
pub struct RevenueForecastRequest {
    pub historical_data: Vec<f64>,
    pub periods_ahead: usize,
}

#[derive(Debug, Serialize)]
pub struct RevenueForecastResponse {
    pub forecast: Vec<f64>,
    pub historical: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct SensitivityRequest {
    pub base_revenue: f64,
    pub base_costs: f64,
    pub revenue_variation: f64,
    pub cost_variation: f64,
}

#[derive(Debug, Serialize)]
pub struct SensitivityResponse {
    pub scenarios: Vec<SensitivityScenario>,
}

#[derive(Debug, Serialize)]
pub struct SensitivityScenario {
    pub name: String,
    pub revenue: f64,
    pub costs: f64,
    pub profit: f64,
    pub margin: f64,
}

// ============================================================================
// API Handlers - Tax Optimization
// ============================================================================

pub async fn calculate_irc(
    Json(req): Json<CalculateIRCRequest>,
) -> Result<Json<CalculateIRCResponse>, AppError> {
    let system = PortugalTaxSystem::new();

    let base_rate = match &req.location {
        Location::Interior => system.reduced_rate_interior,
        Location::Madeira => 0.14,
        Location::Azores => 0.14,
        _ => system.standard_irc_rate,
    };

    let state_surcharge = system.calculate_state_surcharge(req.taxable_income);
    let municipal_surcharge = system.municipal_surcharge_max;

    let total_tax = req.taxable_income * (base_rate + state_surcharge + municipal_surcharge);
    let effective_rate = total_tax / req.taxable_income;

    Ok(Json(CalculateIRCResponse {
        taxable_income: req.taxable_income,
        base_rate,
        state_surcharge,
        municipal_surcharge,
        total_tax,
        effective_rate,
    }))
}

pub async fn calculate_sifide(
    Json(req): Json<CalculateSIFIDERequest>,
) -> Result<Json<CalculateSIFIDEResponse>, AppError> {
    let system = PortugalTaxSystem::new();

    let base_credit = req.current_rd_expenses * system.sifide_base_rate;

    let incremental_credit = if req.current_rd_expenses > req.previous_2_years_avg {
        (req.current_rd_expenses - req.previous_2_years_avg) * system.sifide_incremental_rate
    } else {
        0.0
    };

    let total_credit = (base_credit + incremental_credit)
        .min(req.current_rd_expenses * system.sifide_max_rate);

    let credit_rate = total_credit / req.current_rd_expenses;

    Ok(Json(CalculateSIFIDEResponse {
        base_credit,
        incremental_credit,
        total_credit,
        credit_rate,
    }))
}

pub async fn calculate_patent_box(
    Json(req): Json<CalculatePatentBoxRequest>,
) -> Result<Json<CalculatePatentBoxResponse>, AppError> {
    let system = PortugalTaxSystem::new();

    let qualified_income = req.ip_income * req.nexus_ratio;
    let exempt_income = qualified_income * system.patent_box_exemption;
    let tax_saving = exempt_income * system.standard_irc_rate;

    Ok(Json(CalculatePatentBoxResponse {
        ip_income: req.ip_income,
        qualified_income,
        exempt_income,
        tax_saving,
    }))
}

pub async fn calculate_effective_tax_rate(
    Json(req): Json<EffectiveTaxRateRequest>,
) -> Result<Json<TaxOptimizationResult>, AppError> {
    let system = PortugalTaxSystem::new();

    let result = system.optimize_portugal_structure(
        req.taxable_income,
        req.rd_expenses,
        req.ip_income,
    )?;

    Ok(Json(result))
}

// ============================================================================
// API Handlers - VAT Optimization
// ============================================================================

pub async fn optimize_cross_border_vat(
    Json(req): Json<CrossBorderVATRequest>,
) -> Result<Json<CrossBorderVATResponse>, AppError> {
    let optimizer = VATOptimizer::new();

    let (strategy, vat_amount) = optimizer.optimize_cross_border_vat(
        req.revenue,
        req.customer_location,
        req.service_type,
        req.is_business_customer,
    )?;

    let strategy_name = match strategy {
        VATStrategy::ReverseCharge => "Reverse Charge".to_string(),
        VATStrategy::DestinationPrinciple(_) => "Destination Principle".to_string(),
        VATStrategy::NoVAT => "No VAT".to_string(),
    };

    let effective_rate = if req.revenue > 0.0 {
        vat_amount / req.revenue
    } else {
        0.0
    };

    Ok(Json(CrossBorderVATResponse {
        strategy: strategy_name,
        vat_amount,
        effective_rate,
    }))
}

pub async fn optimize_vat_recovery(
    Json(req): Json<VATRecoveryRequest>,
) -> Result<Json<VATRecoveryResponse>, AppError> {
    let optimizer = VATOptimizer::new();

    let total_recoverable = optimizer.optimize_vat_recovery(&req.expenses)?;
    let num_deductible = req.expenses.iter().filter(|e| e.is_vat_deductible()).count();

    Ok(Json(VATRecoveryResponse {
        total_vat_recoverable: total_recoverable,
        num_deductible_expenses: num_deductible,
    }))
}

// ============================================================================
// API Handlers - Cost Optimization
// ============================================================================

pub async fn linear_programming_allocation(
    Json(req): Json<LinearProgrammingRequest>,
) -> Result<Json<HashMap<String, f64>>, AppError> {
    let allocations = ResourceAllocator::optimize_allocation(
        req.budget,
        &req.cities,
        &req.expected_revenue_per_unit,
    )?;

    Ok(Json(allocations))
}

pub async fn break_even_analysis(
    Json(req): Json<BreakEvenRequest>,
) -> Result<Json<BreakEvenAnalysis>, AppError> {
    let analysis = BreakEvenAnalyzer::calculate_break_even(
        req.fixed_costs,
        req.variable_cost_per_unit,
        req.price_per_unit,
    )?;

    Ok(Json(analysis))
}

// ============================================================================
// API Handlers - Financial Models
// ============================================================================

pub async fn dcf_valuation(
    Json(req): Json<DCFRequest>,
) -> Result<Json<DCFResult>, AppError> {
    let model = DCFModel::new(
        req.discount_rate,
        req.terminal_growth_rate,
        req.cash_flows.len(),
    );

    let result = model.calculate_dcf(
        &req.cash_flows,
        req.debt,
        req.cash,
        req.shares_outstanding,
    )?;

    Ok(Json(result))
}

pub async fn npv_calculation(
    Json(req): Json<NPVRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let model = DCFModel::new(req.discount_rate, 0.03, req.cash_flows.len());
    let npv = model.calculate_npv(&req.cash_flows)?;

    Ok(Json(serde_json::json!({
        "npv": npv,
        "discount_rate": req.discount_rate,
        "positive": npv > 0.0,
    })))
}

pub async fn irr_calculation(
    Json(req): Json<IRRRequest>,
) -> Result<Json<IRRResponse>, AppError> {
    let irr = IRRCalculator::calculate_irr(&req.cash_flows, 0.10)?;

    Ok(Json(IRRResponse {
        irr,
        irr_percentage: format!("{:.2}%", irr * 100.0),
    }))
}

// ============================================================================
// API Handlers - Corporate Structure
// ============================================================================

pub async fn optimize_corporate_structure(
    Json(req): Json<OptimizeStructureRequest>,
) -> Result<Json<Vec<StructureScore>>, AppError> {
    let scores = StructureOptimizer::optimize_structure(
        &req.revenue_by_country,
        &req.objectives,
    )?;

    Ok(Json(scores))
}

pub async fn evaluate_structure(
    Json(structure): Json<CorporateStructure>,
) -> Result<Json<serde_json::Value>, AppError> {
    // Simplified evaluation
    let compliance_cost = match &structure {
        CorporateStructure::SingleEntity { .. } => 10_000.0,
        CorporateStructure::Holding { .. } => 25_000.0,
        CorporateStructure::International { .. } => 50_000.0,
    };

    Ok(Json(serde_json::json!({
        "structure": structure,
        "estimated_compliance_cost": compliance_cost,
    })))
}

// ============================================================================
// API Handlers - Simulation & Forecasting
// ============================================================================

pub async fn monte_carlo_simulation(
    Json(req): Json<MonteCarloRequest>,
) -> Result<Json<SimulationResults>, AppError> {
    let results = MonteCarloSimulator::simulate_outcomes(
        req.base_revenue,
        req.base_costs,
        req.revenue_volatility,
        req.cost_volatility,
        req.iterations,
    )?;

    Ok(Json(results))
}

pub async fn revenue_forecast(
    Json(req): Json<RevenueForecastRequest>,
) -> Result<Json<RevenueForecastResponse>, AppError> {
    let forecast = FinancialForecaster::forecast_revenue(
        &req.historical_data,
        req.periods_ahead,
    )?;

    Ok(Json(RevenueForecastResponse {
        forecast,
        historical: req.historical_data,
    }))
}

pub async fn sensitivity_analysis(
    Json(req): Json<SensitivityRequest>,
) -> Result<Json<SensitivityResponse>, AppError> {
    let mut scenarios = Vec::new();

    // Base case
    scenarios.push(SensitivityScenario {
        name: "Base Case".to_string(),
        revenue: req.base_revenue,
        costs: req.base_costs,
        profit: req.base_revenue - req.base_costs,
        margin: (req.base_revenue - req.base_costs) / req.base_revenue,
    });

    // Revenue variations
    for mult in [-req.revenue_variation, req.revenue_variation] {
        let revenue = req.base_revenue * (1.0 + mult);
        let profit = revenue - req.base_costs;
        scenarios.push(SensitivityScenario {
            name: format!("Revenue {:+.0}%", mult * 100.0),
            revenue,
            costs: req.base_costs,
            profit,
            margin: profit / revenue,
        });
    }

    // Cost variations
    for mult in [-req.cost_variation, req.cost_variation] {
        let costs = req.base_costs * (1.0 + mult);
        let profit = req.base_revenue - costs;
        scenarios.push(SensitivityScenario {
            name: format!("Costs {:+.0}%", mult * 100.0),
            revenue: req.base_revenue,
            costs,
            profit,
            margin: profit / req.base_revenue,
        });
    }

    Ok(Json(SensitivityResponse { scenarios }))
}
