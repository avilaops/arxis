//! Observability module integrating NASA and Google Cloud best practices

use crate::{Result, TimeSeries};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Google's Four Golden Signals
#[derive(Debug, Clone)]
pub struct GoldenSignals {
    pub latency: LatencyMetrics,
    pub traffic: TrafficMetrics,
    pub errors: ErrorMetrics,
    pub saturation: SaturationMetrics,
}

#[derive(Debug, Clone)]
pub struct LatencyMetrics {
    pub p50: Duration,
    pub p95: Duration,
    pub p99: Duration,
    pub p99_9: Duration,
}

#[derive(Debug, Clone)]
pub struct TrafficMetrics {
    pub requests_per_second: f64,
    pub bytes_per_second: f64,
    pub active_connections: usize,
}

#[derive(Debug, Clone)]
pub struct ErrorMetrics {
    pub error_rate: f64,
    pub error_budget: f64,
    pub total_errors: usize,
}

#[derive(Debug, Clone)]
pub struct SaturationMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_usage: f64,
}

/// NASA Data Quality Assessment
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataQualityAssessment {
    /// Accuracy score (0-1)
    pub accuracy: f64,
    /// Completeness score (0-1)
    pub completeness: f64,
    /// Consistency score (0-1)
    pub consistency: f64,
    /// Timeliness in milliseconds
    pub timeliness_ms: u64,
    /// Validity score (0-1)
    pub validity: f64,
    /// Overall quality score
    pub overall_score: f64,
}

impl DataQualityAssessment {
    /// Check if data meets NASA standards (>= 95% overall)
    pub fn meets_nasa_standards(&self) -> bool {
        self.overall_score >= 0.95
    }

    /// Calculate overall quality score
    pub fn calculate_overall(&mut self) {
        self.overall_score = self.accuracy * 0.3
            + self.completeness * 0.25
            + self.consistency * 0.25
            + self.validity * 0.2;
    }
}

/// NASA-style Quality Control
#[derive(Debug, Clone)]
pub struct NASAQualityControl {
    pub ucl: f64, // Upper Control Limit
    pub lcl: f64, // Lower Control Limit
    pub target: f64,
    pub sigma: usize, // 3-sigma or 6-sigma
}

impl NASAQualityControl {
    pub fn new(target: f64, std_dev: f64, sigma: usize) -> Self {
        let sigma_multiplier = sigma as f64;
        Self {
            ucl: target + sigma_multiplier * std_dev,
            lcl: target - sigma_multiplier * std_dev,
            target,
            sigma,
        }
    }

    /// Check if value is within control limits
    pub fn is_in_control(&self, value: f64) -> bool {
        value >= self.lcl && value <= self.ucl
    }

    /// Apply Western Electric Rules
    pub fn apply_western_electric_rules(&self, ts: &TimeSeries) -> Vec<QualityViolation> {
        let mut violations = Vec::new();

        // Rule 1: One point beyond 3-sigma
        for (i, &value) in ts.values.iter().enumerate() {
            if !self.is_in_control(value) {
                violations.push(QualityViolation {
                    index: i,
                    value,
                    rule: WesternElectricRule::BeyondLimits,
                    severity: ViolationSeverity::Critical,
                });
            }
        }

        // Rule 2: Two out of three consecutive points beyond 2-sigma
        // Rule 3: Four out of five consecutive points beyond 1-sigma
        // ... (implement remaining rules)

        violations
    }
}

#[derive(Debug, Clone)]
pub struct QualityViolation {
    pub index: usize,
    pub value: f64,
    pub rule: WesternElectricRule,
    pub severity: ViolationSeverity,
}

#[derive(Debug, Clone, Copy)]
pub enum WesternElectricRule {
    BeyondLimits,
    TwoOfThree,
    FourOfFive,
    EightConsecutive,
}

#[derive(Debug, Clone, Copy)]
pub enum ViolationSeverity {
    Warning,
    Critical,
    Emergency,
}

/// Data Quality metrics (NASA standards)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataQualityMetrics {
    pub glitches: usize,
    pub gaps: usize,
    pub score: f64, // 0.0 - 1.0
}

/// Service Level Objective (Google SRE)
#[derive(Debug, Clone)]
pub struct ServiceLevelObjective {
    pub name: String,
    pub target: f64, // e.g., 0.999 for 99.9%
    pub window: Duration,
    pub error_budget: ErrorBudget,
}

#[derive(Debug, Clone)]
pub struct ErrorBudget {
    pub total: f64,
    pub consumed: f64,
    pub remaining: f64,
    pub burn_rate: f64,
}

impl ErrorBudget {
    pub fn new(total: f64) -> Self {
        Self {
            total,
            consumed: 0.0,
            remaining: total,
            burn_rate: 0.0,
        }
    }

    pub fn is_exhausted(&self) -> bool {
        self.remaining <= 0.0
    }

    pub fn time_to_exhaustion(&self) -> Option<Duration> {
        if self.burn_rate > 0.0 {
            Some(Duration::from_secs_f64(self.remaining / self.burn_rate))
        } else {
            None
        }
    }
}

/// Structured log entry (Google Cloud Logging format)
#[derive(Debug, Clone)]
pub struct StructuredLog {
    pub timestamp: DateTime<Utc>,
    pub severity: LogSeverity,
    pub message: String,
    pub labels: HashMap<String, String>,
    pub trace_id: Option<String>,
    pub span_id: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum LogSeverity {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}

impl StructuredLog {
    pub fn new(severity: LogSeverity, message: impl Into<String>) -> Self {
        Self {
            timestamp: Utc::now(),
            severity,
            message: message.into(),
            labels: HashMap::new(),
            trace_id: None,
            span_id: None,
        }
    }

    pub fn with_label(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.labels.insert(key.into(), value.into());
        self
    }

    pub fn with_trace(mut self, trace_id: String, span_id: String) -> Self {
        self.trace_id = Some(trace_id);
        self.span_id = Some(span_id);
        self
    }
}

/// Alert levels (NASA-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlertLevel {
    Green,  // Normal operations
    Yellow, // Caution
    Red,    // Warning - immediate attention
    Black,  // Emergency - system critical
}

#[derive(Debug, Clone)]
pub struct Alert {
    pub timestamp: DateTime<Utc>,
    pub level: AlertLevel,
    pub message: String,
    pub metric: String,
    pub value: f64,
    pub threshold: f64,
    pub recommended_action: String,
}

/// Performance metrics tracker
pub struct PerformanceTracker {
    _start_time: Instant,
    measurements: Vec<Duration>,
}

impl PerformanceTracker {
    pub fn new() -> Self {
        Self {
            _start_time: Instant::now(),
            measurements: Vec::new(),
        }
    }

    pub fn measure<F, T>(&mut self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        let start = Instant::now();
        let result = f();
        let duration = start.elapsed();
        self.measurements.push(duration);
        result
    }

    pub fn calculate_percentiles(&self) -> LatencyMetrics {
        let mut sorted = self.measurements.clone();
        sorted.sort();

        let len = sorted.len();
        if len == 0 {
            return LatencyMetrics {
                p50: Duration::ZERO,
                p95: Duration::ZERO,
                p99: Duration::ZERO,
                p99_9: Duration::ZERO,
            };
        }

        LatencyMetrics {
            p50: sorted[len * 50 / 100],
            p95: sorted[len * 95 / 100],
            p99: sorted[len * 99 / 100],
            p99_9: sorted[(len * 999 / 1000).min(len - 1)],
        }
    }
}

impl Default for PerformanceTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for data quality assessment
pub trait DataQuality {
    fn assess_quality(&self) -> DataQualityAssessment;
}

impl DataQuality for TimeSeries {
    fn assess_quality(&self) -> DataQualityAssessment {
        let stats = self.statistics();

        // Accuracy: based on std deviation (lower is better)
        let accuracy = if stats.std_dev > 0.0 {
            (1.0 / (1.0 + stats.std_dev / stats.mean.abs().max(1.0))).min(1.0)
        } else {
            1.0
        };

        // Completeness: no NaN or infinite values
        let valid_count = self.values.iter().filter(|v| v.is_finite()).count();
        let completeness = valid_count as f64 / self.values.len() as f64;

        // Consistency: check for sudden jumps
        let diffs = self.diff();
        let mean_diff = diffs.iter().sum::<f64>() / diffs.len() as f64;
        let sudden_jumps = diffs
            .iter()
            .filter(|&&d| (d - mean_diff).abs() > 3.0 * stats.std_dev)
            .count();
        let consistency = 1.0 - (sudden_jumps as f64 / diffs.len() as f64);

        // Validity: values within reasonable range
        let validity = if stats.min >= 0.0 && stats.max < f64::MAX {
            1.0
        } else {
            0.5
        };

        let mut dqa = DataQualityAssessment {
            accuracy,
            completeness,
            consistency,
            validity,
            timeliness_ms: 0, // Would be set from actual latency measurement
            overall_score: 0.0,
        };

        dqa.calculate_overall();
        dqa
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quality_control() {
        let qc = NASAQualityControl::new(10.0, 2.0, 3);
        assert!(qc.is_in_control(10.0));
        assert!(qc.is_in_control(12.0));
        assert!(!qc.is_in_control(20.0));
    }

    #[test]
    fn test_error_budget() {
        let mut budget = ErrorBudget::new(0.001);
        assert!(!budget.is_exhausted());

        budget.consumed = 0.001;
        budget.remaining = 0.0;
        assert!(budget.is_exhausted());
    }

    #[test]
    fn test_data_quality() {
        let ts = TimeSeries::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let dqa = ts.assess_quality();

        assert!(dqa.accuracy > 0.0);
        assert!(dqa.completeness == 1.0);
        assert!(dqa.overall_score > 0.0);
    }
}
