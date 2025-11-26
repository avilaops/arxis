use crate::models::*;
use crate::errors::AppError;
use std::collections::HashMap;

/// Corporate structure optimizer
pub struct StructureOptimizer;

impl StructureOptimizer {
    /// Optimize corporate structure based on objectives
    pub fn optimize_structure(
        revenue_by_country: &HashMap<String, f64>,
        objectives: &OptimizationObjectives,
    ) -> Result<Vec<StructureScore>, AppError> {
        let structures = vec![
            Self::single_entity_structure(),
            Self::holding_structure(),
            Self::international_structure(),
        ];

        let mut scored_structures = Vec::new();

        for structure in structures {
            let score = Self::score_structure(&structure, revenue_by_country, objectives)?;
            scored_structures.push(score);
        }

        // Sort by total score descending
        scored_structures.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

        Ok(scored_structures)
    }

    fn single_entity_structure() -> CorporateStructure {
        CorporateStructure::SingleEntity {
            location: Location::Lisbon,
        }
    }

    fn holding_structure() -> CorporateStructure {
        CorporateStructure::Holding {
            holding_company: Location::Madeira, // Lower tax
            operating_companies: vec![
                Location::Lisbon,
                Location::Porto,
            ],
        }
    }

    fn international_structure() -> CorporateStructure {
        CorporateStructure::International {
            headquarters: Location::Lisbon,
            subsidiaries: vec![
                Subsidiary {
                    location: Country::EU("Ireland".to_string()),
                    revenue_allocation: 0.3,
                    employees: 20,
                },
                Subsidiary {
                    location: Country::EU("Netherlands".to_string()),
                    revenue_allocation: 0.2,
                    employees: 10,
                },
            ],
        }
    }

    fn score_structure(
        structure: &CorporateStructure,
        revenue_by_country: &HashMap<String, f64>,
        objectives: &OptimizationObjectives,
    ) -> Result<StructureScore, AppError> {
        let effective_tax_rate = Self::calculate_effective_tax_rate(structure, revenue_by_country)?;
        let compliance_cost = Self::estimate_compliance_cost(structure);

        // Calculate weighted score
        let tax_weight = objectives.weights.get("tax").unwrap_or(&0.5);
        let compliance_weight = objectives.weights.get("compliance").unwrap_or(&0.3);
        let flexibility_weight = objectives.weights.get("flexibility").unwrap_or(&0.2);

        let flexibility_score = Self::flexibility_score(structure);

        let total_score =
            (1.0 - effective_tax_rate) * tax_weight +
            (1.0 - compliance_cost / 100_000.0) * compliance_weight +
            flexibility_score * flexibility_weight;

        let (pros, cons) = Self::structure_pros_cons(structure);

        Ok(StructureScore {
            structure: structure.clone(),
            effective_tax_rate,
            compliance_cost,
            total_score,
            pros,
            cons,
        })
    }

    fn calculate_effective_tax_rate(
        structure: &CorporateStructure,
        revenue_by_country: &HashMap<String, f64>,
    ) -> Result<f64, AppError> {
        match structure {
            CorporateStructure::SingleEntity { location } => {
                Ok(Self::location_tax_rate(location))
            },
            CorporateStructure::Holding { holding_company, operating_companies } => {
                // Holding companies typically benefit from participation exemption
                // Operating companies pay local rates
                let holding_rate = Self::location_tax_rate(holding_company) * 0.5; // Participation exemption
                let operating_rate: f64 = operating_companies.iter()
                    .map(|loc| Self::location_tax_rate(loc))
                    .sum::<f64>() / operating_companies.len() as f64;

                Ok((holding_rate + operating_rate) / 2.0)
            },
            CorporateStructure::International { headquarters, subsidiaries } => {
                let hq_rate = Self::location_tax_rate(headquarters);

                let sub_rate: f64 = subsidiaries.iter()
                    .map(|sub| Self::country_tax_rate(&sub.location) * sub.revenue_allocation)
                    .sum();

                Ok(hq_rate * 0.4 + sub_rate * 0.6) // Weighted average
            },
        }
    }

    fn estimate_compliance_cost(structure: &CorporateStructure) -> f64 {
        match structure {
            CorporateStructure::SingleEntity { .. } => 10_000.0,
            CorporateStructure::Holding { operating_companies, .. } => {
                15_000.0 + (operating_companies.len() as f64 * 5_000.0)
            },
            CorporateStructure::International { subsidiaries, .. } => {
                25_000.0 + (subsidiaries.len() as f64 * 15_000.0)
            },
        }
    }

    fn flexibility_score(structure: &CorporateStructure) -> f64 {
        match structure {
            CorporateStructure::SingleEntity { .. } => 0.5,
            CorporateStructure::Holding { .. } => 0.7,
            CorporateStructure::International { .. } => 0.9,
        }
    }

    fn structure_pros_cons(structure: &CorporateStructure) -> (Vec<String>, Vec<String>) {
        match structure {
            CorporateStructure::SingleEntity { .. } => (
                vec![
                    "Simple to manage".to_string(),
                    "Low compliance costs".to_string(),
                    "Easy accounting".to_string(),
                ],
                vec![
                    "Limited tax optimization".to_string(),
                    "No IP protection structure".to_string(),
                    "Single jurisdiction risk".to_string(),
                ],
            ),
            CorporateStructure::Holding { .. } => (
                vec![
                    "Tax-efficient dividend flow".to_string(),
                    "IP can be held at holding level".to_string(),
                    "Participation exemption benefits".to_string(),
                    "Risk segregation".to_string(),
                ],
                vec![
                    "More complex structure".to_string(),
                    "Higher compliance costs".to_string(),
                    "Transfer pricing requirements".to_string(),
                ],
            ),
            CorporateStructure::International { .. } => (
                vec![
                    "Maximum tax optimization".to_string(),
                    "Geographic diversification".to_string(),
                    "Access to multiple markets".to_string(),
                    "IP protection across jurisdictions".to_string(),
                ],
                vec![
                    "Most complex structure".to_string(),
                    "Highest compliance costs".to_string(),
                    "Transfer pricing complexity".to_string(),
                    "CFC rules consideration".to_string(),
                ],
            ),
        }
    }

    fn location_tax_rate(location: &Location) -> f64 {
        match location {
            Location::Lisbon | Location::Porto => 0.21,
            Location::Interior => 0.125,
            Location::Madeira => 0.14,
            Location::Azores => 0.14,
        }
    }

    fn country_tax_rate(country: &Country) -> f64 {
        match country {
            Country::Portugal => 0.21,
            Country::EU(name) => match name.to_lowercase().as_str() {
                "ireland" => 0.125,
                "netherlands" => 0.258,
                "luxembourg" => 0.245,
                "cyprus" => 0.125,
                "malta" => 0.35,
                "spain" => 0.25,
                "france" => 0.25,
                "germany" => 0.30,
                _ => 0.20, // Default EU
            },
            Country::NonEU(name) => match name.to_lowercase().as_str() {
                "switzerland" => 0.115,
                "united kingdom" => 0.19,
                "usa" => 0.21,
                _ => 0.25, // Default
            },
        }
    }

    /// Evaluate transfer pricing between entities
    pub fn evaluate_transfer_pricing(
        revenue: f64,
        cost_base: f64,
        markup_percentage: f64,
    ) -> Result<TransferPricingAnalysis, AppError> {
        if markup_percentage < 0.0 || markup_percentage > 1.0 {
            return Err(AppError::ValidationError(
                "Markup percentage must be between 0 and 1".to_string()
            ));
        }

        let transfer_price = cost_base * (1.0 + markup_percentage);
        let profit_at_opco = transfer_price - cost_base;
        let profit_at_holdco = revenue - transfer_price;

        Ok(TransferPricingAnalysis {
            transfer_price,
            profit_at_opco,
            profit_at_holdco,
            markup_percentage,
            arms_length_compliant: markup_percentage >= 0.05 && markup_percentage <= 0.15,
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransferPricingAnalysis {
    pub transfer_price: f64,
    pub profit_at_opco: f64,
    pub profit_at_holdco: f64,
    pub markup_percentage: f64,
    pub arms_length_compliant: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_entity_tax_rate() {
        let structure = CorporateStructure::SingleEntity {
            location: Location::Interior,
        };

        let rate = StructureOptimizer::calculate_effective_tax_rate(
            &structure,
            &HashMap::new(),
        ).unwrap();

        assert_eq!(rate, 0.125);
    }

    #[test]
    fn test_transfer_pricing() {
        let analysis = StructureOptimizer::evaluate_transfer_pricing(
            1_000_000.0,  // Revenue
            600_000.0,    // Cost base
            0.10,         // 10% markup
        ).unwrap();

        assert_eq!(analysis.transfer_price, 660_000.0);
        assert_eq!(analysis.profit_at_opco, 60_000.0);
        assert_eq!(analysis.profit_at_holdco, 340_000.0);
        assert!(analysis.arms_length_compliant);
    }
}
