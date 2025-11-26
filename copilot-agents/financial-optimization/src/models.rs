use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Location {
    Lisbon,
    Porto,
    Interior,
    Madeira,
    Azores,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Country {
    Portugal,
    EU(String),
    NonEU(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    Software,
    Consulting,
    Hardware,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incentive {
    pub name: String,
    pub description: String,
    pub value_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakEvenAnalysis {
    pub break_even_units: usize,
    pub break_even_revenue: f64,
    pub contribution_margin_ratio: f64,
    pub margin_of_safety: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResults {
    pub mean: f64,
    pub median: f64,
    pub percentile_5: f64,
    pub percentile_95: f64,
    pub probability_loss: f64,
    pub standard_deviation: f64,
    pub coefficient_of_variation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DCFResult {
    pub npv: f64,
    pub terminal_value: f64,
    pub enterprise_value: f64,
    pub pv_by_year: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxOptimizationResult {
    pub current_effective_rate: f64,
    pub optimized_rate: f64,
    pub annual_savings: f64,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VATStrategy {
    ReverseCharge,
    DestinationPrinciple(f64),
    NoVAT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorporateStructure {
    SingleEntity {
        location: Location,
    },
    Holding {
        holding_company: Location,
        operating_companies: Vec<Location>,
    },
    International {
        headquarters: Location,
        subsidiaries: Vec<Subsidiary>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subsidiary {
    pub location: Country,
    pub revenue_allocation: f64,
    pub employees: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationObjectives {
    pub minimize_tax: bool,
    pub minimize_compliance_cost: bool,
    pub maximize_flexibility: bool,
    pub weights: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructureScore {
    pub structure: CorporateStructure,
    pub effective_tax_rate: f64,
    pub compliance_cost: f64,
    pub total_score: f64,
    pub pros: Vec<String>,
    pub cons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Expense {
    pub description: String,
    pub amount: f64,
    pub vat_rate: f64,
    pub deductible: bool,
}

impl Expense {
    pub fn is_vat_deductible(&self) -> bool {
        self.deductible
    }

    pub fn vat_amount(&self) -> f64 {
        self.amount * self.vat_rate
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub name: String,
    pub country: String,
    pub cost_index: f64,
}
