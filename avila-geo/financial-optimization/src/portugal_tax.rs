use crate::models::*;
use crate::errors::AppError;

/// Portugal Tax System Calculator
///
/// Handles IRC (Corporate Income Tax), SIFIDE (R&D Tax Credit),
/// Patent Box, and other Portugal-specific tax calculations
pub struct PortugalTaxSystem {
    pub standard_irc_rate: f64,         // 21%
    pub reduced_rate_interior: f64,     // 12.5% (interior regions)
    pub municipal_surcharge_max: f64,   // Up to 1.5%
    pub state_surcharge_tiers: Vec<(f64, f64)>,  // Progressive above €1.5M

    // Incentives
    pub sifide_base_rate: f64,          // R&D: 32.5% base
    pub sifide_incremental_rate: f64,   // R&D: 50% incremental
    pub sifide_max_rate: f64,           // R&D: up to 82.5% of expenses
    pub cfei_max_rate: f64,             // Investment: up to 25%
    pub patent_box_exemption: f64,      // IP Box: 50% exemption
}

impl Default for PortugalTaxSystem {
    fn default() -> Self {
        Self {
            standard_irc_rate: 0.21,
            reduced_rate_interior: 0.125,
            municipal_surcharge_max: 0.015,
            state_surcharge_tiers: vec![
                (1_500_000.0, 0.0),      // No surcharge up to €1.5M
                (7_500_000.0, 0.03),     // 3% between €1.5M - €7.5M
                (35_000_000.0, 0.05),    // 5% between €7.5M - €35M
                (f64::INFINITY, 0.09),   // 9% above €35M
            ],
            sifide_base_rate: 0.325,
            sifide_incremental_rate: 0.50,
            sifide_max_rate: 0.825,
            cfei_max_rate: 0.25,
            patent_box_exemption: 0.50,
        }
    }
}

impl PortugalTaxSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Calculate effective tax rate considering location, R&D, and IP income
    pub fn calculate_effective_rate(
        &self,
        taxable_income: f64,
        location: &Location,
        rd_expenses: f64,
        ip_income: f64,
        nexus_ratio: f64,
    ) -> Result<f64, AppError> {
        if taxable_income <= 0.0 {
            return Err(AppError::ValidationError("Taxable income must be positive".to_string()));
        }

        let base_rate = match location {
            Location::Interior => self.reduced_rate_interior,
            Location::Madeira => 0.14,  // Reduced rate for Madeira
            Location::Azores => 0.14,   // Reduced rate for Azores
            _ => self.standard_irc_rate,
        };

        let surcharge = self.calculate_state_surcharge(taxable_income);
        let municipal = self.municipal_surcharge_max; // Assume maximum

        let gross_tax = taxable_income * (base_rate + surcharge + municipal);

        let sifide_credit = self.calculate_sifide_credit(rd_expenses, 0.0)?;
        let patent_box_saving = self.calculate_patent_box_saving(ip_income, nexus_ratio, base_rate)?;

        let net_tax = (gross_tax - sifide_credit - patent_box_saving).max(0.0);

        Ok(net_tax / taxable_income)
    }

    /// Calculate state surcharge based on income tiers
    pub fn calculate_state_surcharge(&self, taxable_income: f64) -> f64 {
        if taxable_income <= 1_500_000.0 {
            return 0.0;
        }

        let mut total_surcharge = 0.0;
        let mut remaining_income = taxable_income;

        for i in 0..self.state_surcharge_tiers.len() - 1 {
            let (current_threshold, _) = self.state_surcharge_tiers[i];
            let (next_threshold, rate) = self.state_surcharge_tiers[i + 1];

            if remaining_income > current_threshold {
                let taxable_in_tier = (remaining_income - current_threshold)
                    .min(next_threshold - current_threshold);
                total_surcharge += taxable_in_tier * rate;
            }
        }

        total_surcharge / taxable_income
    }

    /// Calculate SIFIDE (R&D Tax Credit)
    ///
    /// Base rate: 32.5% of eligible expenses
    /// Incremental: 50% on increases over previous 2 years average
    /// Maximum: 82.5% of eligible expenses
    pub fn calculate_sifide_credit(
        &self,
        current_rd_expenses: f64,
        previous_2_years_avg: f64,
    ) -> Result<f64, AppError> {
        if current_rd_expenses < 0.0 {
            return Err(AppError::ValidationError("R&D expenses cannot be negative".to_string()));
        }

        // Base credit: 32.5% of all eligible expenses
        let base_credit = current_rd_expenses * self.sifide_base_rate;

        // Incremental credit: 50% on increase over previous average
        let incremental = if current_rd_expenses > previous_2_years_avg {
            (current_rd_expenses - previous_2_years_avg) * self.sifide_incremental_rate
        } else {
            0.0
        };

        // Total credit (capped at 82.5% of expenses)
        let total_credit = (base_credit + incremental)
            .min(current_rd_expenses * self.sifide_max_rate);

        Ok(total_credit)
    }

    /// Calculate Patent Box (IP Box) tax saving
    ///
    /// 50% exemption on qualified IP income
    /// Must satisfy nexus ratio (substantial R&D activity)
    pub fn calculate_patent_box_saving(
        &self,
        ip_income: f64,
        nexus_ratio: f64,
        base_tax_rate: f64,
    ) -> Result<f64, AppError> {
        if ip_income < 0.0 {
            return Err(AppError::ValidationError("IP income cannot be negative".to_string()));
        }

        if !(0.0..=1.0).contains(&nexus_ratio) {
            return Err(AppError::ValidationError("Nexus ratio must be between 0 and 1".to_string()));
        }

        // Qualified IP income based on nexus approach
        let qualified_income = ip_income * nexus_ratio;

        // 50% exemption
        let exempt_income = qualified_income * self.patent_box_exemption;

        // Tax saving
        let saving = exempt_income * base_tax_rate;

        Ok(saving)
    }

    /// Optimize corporate structure for Portugal
    pub fn optimize_portugal_structure(
        &self,
        annual_revenue: f64,
        rd_expenses: f64,
        ip_income: f64,
    ) -> Result<TaxOptimizationResult, AppError> {
        // Evaluate different location options
        let locations = vec![
            Location::Lisbon,
            Location::Porto,
            Location::Interior,
            Location::Madeira,
        ];

        let mut best_rate = f64::MAX;
        let mut best_location = &locations[0];

        for location in &locations {
            let rate = self.calculate_effective_rate(
                annual_revenue,
                location,
                rd_expenses,
                ip_income,
                0.8, // Assume 80% nexus ratio
            )?;

            if rate < best_rate {
                best_rate = rate;
                best_location = location;
            }
        }

        let current_rate = self.calculate_effective_rate(
            annual_revenue,
            &Location::Lisbon,
            0.0,
            0.0,
            0.0,
        )?;

        let annual_savings = annual_revenue * (current_rate - best_rate);

        let recommendations = vec![
            format!("Consider establishing operations in {:?} region", best_location),
            if rd_expenses > 0.0 {
                format!("Maximize SIFIDE credit: Potential €{:.2} annual tax credit",
                    self.calculate_sifide_credit(rd_expenses, 0.0)?)
            } else {
                "Increase R&D activities to qualify for SIFIDE (up to 82.5% tax credit)".to_string()
            },
            if ip_income > 0.0 {
                "Apply Patent Box regime for 50% exemption on IP income".to_string()
            } else {
                "Develop patentable IP to benefit from Patent Box (50% exemption)".to_string()
            },
            "Optimize municipal surcharge (negotiate with local municipalities)".to_string(),
        ];

        Ok(TaxOptimizationResult {
            current_effective_rate: current_rate,
            optimized_rate: best_rate,
            annual_savings,
            recommendations,
        })
    }
}

/// Portugal-specific incentives
pub struct PortugalIncentives;

impl PortugalIncentives {
    /// Get incentives for interior regions
    pub fn interior_incentives() -> Vec<Incentive> {
        vec![
            Incentive {
                name: "Reduced IRC Rate".to_string(),
                description: "12.5% vs 21% standard rate".to_string(),
                value_percent: 40.5, // (21-12.5)/21
            },
            Incentive {
                name: "EU Structural Funds".to_string(),
                description: "Up to 45% grant for investment in interior regions".to_string(),
                value_percent: 45.0,
            },
            Incentive {
                name: "Social Security Reductions".to_string(),
                description: "Up to 50% reduction for new hires in interior regions".to_string(),
                value_percent: 50.0,
            },
            Incentive {
                name: "RFAI - Incentive Tax Regime".to_string(),
                description: "Additional 10% deduction on eligible investments".to_string(),
                value_percent: 10.0,
            },
        ]
    }

    /// Get startup incentives
    pub fn startup_incentives() -> Vec<Incentive> {
        vec![
            Incentive {
                name: "Startup Visa".to_string(),
                description: "Fast-track residence permit for founders".to_string(),
                value_percent: 0.0,
            },
            Incentive {
                name: "Portugal Tech Visa".to_string(),
                description: "Expedited work permits for tech talent".to_string(),
                value_percent: 0.0,
            },
            Incentive {
                name: "Startup Portugal+".to_string(),
                description: "Access to funding, mentorship, and co-working".to_string(),
                value_percent: 0.0,
            },
            Incentive {
                name: "IRS Young Tax Regime".to_string(),
                description: "50% exemption on employment income (first 5 years, under 35)".to_string(),
                value_percent: 50.0,
            },
        ]
    }

    /// Get R&D incentives
    pub fn rd_incentives() -> Vec<Incentive> {
        vec![
            Incentive {
                name: "SIFIDE II".to_string(),
                description: "Up to 82.5% tax credit on R&D expenses".to_string(),
                value_percent: 82.5,
            },
            Incentive {
                name: "Patent Box".to_string(),
                description: "50% exemption on IP-related income".to_string(),
                value_percent: 50.0,
            },
            Incentive {
                name: "Horizon Europe".to_string(),
                description: "EU funding for R&D projects".to_string(),
                value_percent: 0.0,
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_surcharge() {
        let system = PortugalTaxSystem::new();

        // No surcharge under €1.5M
        assert_eq!(system.calculate_state_surcharge(1_000_000.0), 0.0);

        // 3% surcharge between €1.5M - €7.5M
        let surcharge = system.calculate_state_surcharge(2_000_000.0);
        assert!(surcharge > 0.0 && surcharge < 0.03);
    }

    #[test]
    fn test_sifide_credit() {
        let system = PortugalTaxSystem::new();

        let credit = system.calculate_sifide_credit(100_000.0, 0.0).unwrap();
        assert_eq!(credit, 100_000.0 * 0.325); // Base rate

        // With incremental
        let credit_incr = system.calculate_sifide_credit(150_000.0, 100_000.0).unwrap();
        assert!(credit_incr > 100_000.0 * 0.325);
    }

    #[test]
    fn test_patent_box() {
        let system = PortugalTaxSystem::new();

        let saving = system.calculate_patent_box_saving(
            100_000.0,  // IP income
            0.8,        // Nexus ratio
            0.21,       // Tax rate
        ).unwrap();

        // 50% exemption on 80% qualified = 40% exemption
        assert_eq!(saving, 100_000.0 * 0.8 * 0.5 * 0.21);
    }

    #[test]
    fn test_effective_rate_interior() {
        let system = PortugalTaxSystem::new();

        let rate = system.calculate_effective_rate(
            1_000_000.0,
            &Location::Interior,
            0.0,
            0.0,
            0.0,
        ).unwrap();

        // Should be around 12.5% + municipal surcharge
        assert!(rate >= 0.125 && rate <= 0.15);
    }
}
