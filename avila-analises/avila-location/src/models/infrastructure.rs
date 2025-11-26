//! Infrastructure and digital capabilities

use serde::{Deserialize, Serialize};

/// Digital infrastructure metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfrastructureData {
    /// Average internet speed (Mbps)
    pub avg_internet_speed_mbps: f64,

    /// Fiber optic coverage (%)
    pub fiber_coverage_percent: f64,

    /// 5G coverage (%)
    pub five_g_coverage_percent: f64,

    /// Average latency to major datacenters (ms)
    pub avg_datacenter_latency_ms: f64,

    /// Number of coworking spaces
    pub coworking_spaces: u32,

    /// Number of tech hubs/incubators
    pub tech_hubs: u32,

    /// Number of universities with CS programs
    pub universities_with_cs: u32,

    /// Annual CS graduates
    pub cs_graduates_annual: u32,

    /// Power reliability score (0-100)
    pub power_reliability: f64,
}

impl InfrastructureData {
    pub fn infrastructure_score(&self) -> f64 {
        let internet_score = (self.avg_internet_speed_mbps / 5.0).min(100.0);
        let coverage_score = (self.fiber_coverage_percent + self.five_g_coverage_percent) / 2.0;
        let latency_score = 100.0 - (self.avg_datacenter_latency_ms / 2.0).min(100.0);
        let workspace_score = ((self.coworking_spaces + self.tech_hubs) as f64 * 5.0).min(100.0);
        let talent_score = (self.cs_graduates_annual as f64 / 10.0).min(100.0);

        (internet_score * 0.25 +
         coverage_score * 0.20 +
         latency_score * 0.15 +
         workspace_score * 0.15 +
         talent_score * 0.15 +
         self.power_reliability * 0.10)
    }
}

/// Talent availability metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TalentData {
    /// Total tech workforce in region
    pub tech_workforce: u32,

    /// Average developer salary (monthly)
    pub avg_dev_salary: f64,

    /// Salary vs market (percentage)
    pub salary_vs_market_percent: f64,

    /// Unemployment rate (%)
    pub unemployment_rate: f64,

    /// Tech job openings
    pub tech_job_openings: u32,

    /// English proficiency score (0-100)
    pub english_proficiency: f64,

    /// Skills availability by technology
    pub skills: Vec<SkillAvailability>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAvailability {
    pub technology: String,
    pub available_professionals: u32,
    pub avg_experience_years: f64,
}

impl TalentData {
    pub fn talent_score(&self) -> f64 {
        let availability_score = (self.tech_workforce as f64 / 100.0).min(100.0);
        let cost_score = 100.0 - (self.salary_vs_market_percent - 100.0).abs();
        let employability_score = 100.0 - (self.unemployment_rate * 10.0);
        let language_score = self.english_proficiency;

        (availability_score * 0.30 +
         cost_score * 0.25 +
         employability_score * 0.20 +
         language_score * 0.25)
    }
}
