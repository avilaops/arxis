//! Billing and usage tracking

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct BillingManager {
    rate_table: RateTable,
}

impl BillingManager {
    pub fn new() -> Self {
        Self {
            rate_table: RateTable::default_pricing(),
        }
    }

    pub fn calculate_instance_cost(&self, instance_type: &str, hours: f64) -> f64 {
        let rate = self.rate_table.get_compute_rate(instance_type);
        rate * hours
    }

    pub fn calculate_storage_cost(&self, gb_months: f64) -> f64 {
        self.rate_table.storage_rate_per_gb * gb_months
    }

    pub fn get_pricing(&self) -> &RateTable {
        &self.rate_table
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateTable {
    pub compute_rates: HashMap<String, f64>,
    pub storage_rate_per_gb: f64,
    pub network_rate_per_gb: f64,
}

impl RateTable {
    pub fn default_pricing() -> Self {
        let mut compute_rates = HashMap::new();

        compute_rates.insert("t3.micro".to_string(), 0.0104);
        compute_rates.insert("t3.small".to_string(), 0.0208);
        compute_rates.insert("t3.medium".to_string(), 0.0416);
        compute_rates.insert("c6.large".to_string(), 0.085);
        compute_rates.insert("m6.large".to_string(), 0.096);
        compute_rates.insert("r6.large".to_string(), 0.126);

        Self {
            compute_rates,
            storage_rate_per_gb: 0.023,
            network_rate_per_gb: 0.09,
        }
    }

    pub fn get_compute_rate(&self, instance_type: &str) -> f64 {
        *self.compute_rates.get(instance_type).unwrap_or(&0.05)
    }
}
