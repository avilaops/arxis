use crate::models::*;
use crate::errors::AppError;

/// VAT Optimizer for cross-border transactions
///
/// Handles VAT optimization for Portugal and EU cross-border scenarios
pub struct VATOptimizer {
    pub portugal_standard_rate: f64,    // 23%
    pub portugal_reduced_rate: f64,     // 13% (some goods/services)
    pub portugal_super_reduced: f64,    // 6% (essentials)
    pub madeira_rate: f64,              // 22%
    pub azores_rate: f64,               // 18%
}

impl Default for VATOptimizer {
    fn default() -> Self {
        Self {
            portugal_standard_rate: 0.23,
            portugal_reduced_rate: 0.13,
            portugal_super_reduced: 0.06,
            madeira_rate: 0.22,
            azores_rate: 0.18,
        }
    }
}

impl VATOptimizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Optimize VAT for cross-border services
    pub fn optimize_cross_border_vat(
        &self,
        revenue: f64,
        customer_location: Country,
        service_type: ServiceType,
        is_business_customer: bool,
    ) -> Result<(VATStrategy, f64), AppError> {
        if revenue < 0.0 {
            return Err(AppError::ValidationError("Revenue cannot be negative".to_string()));
        }

        let strategy = match (&customer_location, is_business_customer) {
            // B2B in EU: Reverse charge (0% VAT)
            (Country::EU(_), true) => VATStrategy::ReverseCharge,

            // B2C in EU: Customer's country rate
            (Country::EU(country), false) => {
                let vat_rate = self.get_eu_country_vat_rate(country);
                VATStrategy::DestinationPrinciple(vat_rate)
            },

            // Portugal domestic
            (Country::Portugal, _) => {
                let vat_rate = self.get_portugal_vat_rate(&service_type);
                VATStrategy::DestinationPrinciple(vat_rate)
            },

            // Non-EU: No VAT
            (Country::NonEU(_), _) => VATStrategy::NoVAT,
        };

        let vat_amount = match &strategy {
            VATStrategy::ReverseCharge => 0.0,
            VATStrategy::DestinationPrinciple(rate) => revenue * rate,
            VATStrategy::NoVAT => 0.0,
        };

        Ok((strategy, vat_amount))
    }

    /// Optimize VAT recovery from expenses
    pub fn optimize_vat_recovery(&self, expenses: &[Expense]) -> Result<f64, AppError> {
        let total_recoverable: f64 = expenses.iter()
            .filter(|e| e.is_vat_deductible())
            .map(|e| e.vat_amount())
            .sum();

        Ok(total_recoverable)
    }

    /// Get Portugal VAT rate based on service/product type
    fn get_portugal_vat_rate(&self, service_type: &ServiceType) -> f64 {
        match service_type {
            ServiceType::Software => self.portugal_standard_rate,
            ServiceType::Consulting => self.portugal_standard_rate,
            ServiceType::Hardware => self.portugal_standard_rate,
            ServiceType::Other(_) => self.portugal_standard_rate,
        }
    }

    /// Get EU country VAT rate (simplified - in production, use a database)
    fn get_eu_country_vat_rate(&self, country: &str) -> f64 {
        match country.to_lowercase().as_str() {
            "portugal" | "pt" => 0.23,
            "spain" | "es" => 0.21,
            "france" | "fr" => 0.20,
            "germany" | "de" => 0.19,
            "italy" | "it" => 0.22,
            "netherlands" | "nl" => 0.21,
            "belgium" | "be" => 0.21,
            "ireland" | "ie" => 0.23,
            "poland" | "pl" => 0.23,
            "sweden" | "se" => 0.25,
            "denmark" | "dk" => 0.25,
            "finland" | "fi" => 0.24,
            "austria" | "at" => 0.20,
            "greece" | "gr" => 0.24,
            "czech republic" | "cz" => 0.21,
            "romania" | "ro" => 0.19,
            "hungary" | "hu" => 0.27,
            _ => 0.20, // Default EU average
        }
    }

    /// Calculate VAT on cross-border e-commerce
    /// OSS (One-Stop Shop) threshold: €10,000
    pub fn calculate_oss_threshold_impact(
        &self,
        annual_eu_b2c_sales: f64,
        sales_by_country: &std::collections::HashMap<String, f64>,
    ) -> Result<(bool, f64), AppError> {
        const OSS_THRESHOLD: f64 = 10_000.0;

        let uses_oss = annual_eu_b2c_sales > OSS_THRESHOLD;

        let total_vat = if uses_oss {
            // Apply destination country VAT rates
            sales_by_country.iter()
                .map(|(country, sales)| {
                    let rate = self.get_eu_country_vat_rate(country);
                    sales * rate
                })
                .sum()
        } else {
            // Apply Portugal VAT rate
            annual_eu_b2c_sales * self.portugal_standard_rate
        };

        Ok((uses_oss, total_vat))
    }

    /// Calculate intra-community supply savings
    pub fn calculate_intracom_savings(
        &self,
        goods_value: f64,
        from_country: Country,
        to_country: Country,
    ) -> Result<f64, AppError> {
        // Intra-community supplies are VAT-exempt (0%)
        // Customer self-assesses VAT in their country (reverse charge)

        match (&from_country, &to_country) {
            (Country::EU(_), Country::EU(_)) |
            (Country::Portugal, Country::EU(_)) => {
                // VAT-exempt transaction
                // Savings = Portugal VAT that would have been charged
                Ok(goods_value * self.portugal_standard_rate)
            },
            _ => Ok(0.0), // No savings for non-EU
        }
    }

    /// Analyze VAT group registration benefits
    pub fn analyze_vat_group_benefits(
        &self,
        intragroup_transactions: f64,
        external_purchases: f64,
    ) -> Result<VATGroupBenefits, AppError> {
        // VAT group = single VAT registration for multiple entities
        // Benefits:
        // 1. Intra-group transactions are out of scope of VAT
        // 2. Simplified compliance
        // 3. Better cash flow

        let vat_saved_on_intragroup = intragroup_transactions * self.portugal_standard_rate;
        let compliance_cost_reduction = 5_000.0; // Per entity per year (estimated)

        Ok(VATGroupBenefits {
            vat_saved: vat_saved_on_intragroup,
            compliance_savings: compliance_cost_reduction,
            cash_flow_improvement: vat_saved_on_intragroup * 0.1, // 10% of VAT as cash flow benefit
            total_annual_benefit: vat_saved_on_intragroup + compliance_cost_reduction,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VATGroupBenefits {
    pub vat_saved: f64,
    pub compliance_savings: f64,
    pub cash_flow_improvement: f64,
    pub total_annual_benefit: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b2b_eu_reverse_charge() {
        let optimizer = VATOptimizer::new();

        let (strategy, vat) = optimizer.optimize_cross_border_vat(
            100_000.0,
            Country::EU("Spain".to_string()),
            ServiceType::Software,
            true,
        ).unwrap();

        match strategy {
            VATStrategy::ReverseCharge => assert_eq!(vat, 0.0),
            _ => panic!("Expected ReverseCharge"),
        }
    }

    #[test]
    fn test_b2c_eu_destination_principle() {
        let optimizer = VATOptimizer::new();

        let (strategy, vat) = optimizer.optimize_cross_border_vat(
            100_000.0,
            Country::EU("Germany".to_string()),
            ServiceType::Software,
            false,
        ).unwrap();

        match strategy {
            VATStrategy::DestinationPrinciple(rate) => {
                assert_eq!(rate, 0.19); // Germany VAT
                assert_eq!(vat, 100_000.0 * 0.19);
            },
            _ => panic!("Expected DestinationPrinciple"),
        }
    }

    #[test]
    fn test_vat_recovery() {
        let optimizer = VATOptimizer::new();

        let expenses = vec![
            Expense {
                description: "Office supplies".to_string(),
                amount: 1000.0,
                vat_rate: 0.23,
                deductible: true,
            },
            Expense {
                description: "Client entertainment".to_string(),
                amount: 500.0,
                vat_rate: 0.23,
                deductible: false, // Not deductible
            },
        ];

        let recovery = optimizer.optimize_vat_recovery(&expenses).unwrap();
        assert_eq!(recovery, 1000.0 * 0.23); // Only first expense
    }
}
