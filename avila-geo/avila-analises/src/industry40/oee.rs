use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

use super::iot::ProductionTelemetry;

/// Calculadora de OEE (Overall Equipment Effectiveness)
/// OEE = Disponibilidade × Performance × Qualidade
pub struct OEECalculator {
    target_cycle_time_ms: u64,
    planned_production_time_hours: f64,
}

impl OEECalculator {
    pub fn new(target_cycle_time_ms: u64, planned_production_time_hours: f64) -> Self {
        Self {
            target_cycle_time_ms,
            planned_production_time_hours,
        }
    }

    /// Calcular OEE baseado em dados de produção
    pub fn calculate_oee(&self, production_data: &ProductionData) -> OEEMetrics {
        let availability = self.calculate_availability(production_data);
        let performance = self.calculate_performance(production_data);
        let quality = self.calculate_quality(production_data);

        let oee = availability * performance * quality;

        OEEMetrics {
            oee,
            availability,
            performance,
            quality,
            production_time_hours: production_data.actual_production_time_hours,
            downtime_hours: production_data.downtime_hours,
            total_count: production_data.total_count,
            good_count: production_data.good_count,
            reject_count: production_data.reject_count,
            target_count: self.calculate_target_count(production_data.actual_production_time_hours),
            ideal_cycle_time_ms: self.target_cycle_time_ms,
            actual_cycle_time_ms: production_data.actual_avg_cycle_time_ms,
            calculated_at: Utc::now(),
        }
    }

    /// Calcular Disponibilidade = Tempo de Produção / Tempo Planejado
    fn calculate_availability(&self, data: &ProductionData) -> f64 {
        if self.planned_production_time_hours == 0.0 {
            return 0.0;
        }
        (data.actual_production_time_hours / self.planned_production_time_hours).min(1.0)
    }

    /// Calcular Performance = (Tempo de Ciclo Ideal × Contagem Total) / Tempo de Produção
    fn calculate_performance(&self, data: &ProductionData) -> f64 {
        if data.actual_production_time_hours == 0.0 {
            return 0.0;
        }

        let ideal_time_hours = (self.target_cycle_time_ms as f64 * data.total_count as f64)
            / (1000.0 * 3600.0);

        (ideal_time_hours / data.actual_production_time_hours).min(1.0)
    }

    /// Calcular Qualidade = Peças Boas / Total Produzido
    fn calculate_quality(&self, data: &ProductionData) -> f64 {
        if data.total_count == 0 {
            return 0.0;
        }
        (data.good_count as f64 / data.total_count as f64).min(1.0)
    }

    /// Calcular contagem alvo
    fn calculate_target_count(&self, production_time_hours: f64) -> u64 {
        let production_time_ms = production_time_hours * 3600.0 * 1000.0;
        (production_time_ms / self.target_cycle_time_ms as f64) as u64
    }

    /// Analisar perdas (Six Big Losses)
    pub fn analyze_losses(&self, data: &ProductionData) -> LossAnalysis {
        let planned_time_ms = self.planned_production_time_hours * 3600.0 * 1000.0;
        let production_time_ms = data.actual_production_time_hours * 3600.0 * 1000.0;

        // 1. Perdas de Disponibilidade
        let breakdown_loss_ms = data.breakdown_time_hours * 3600.0 * 1000.0;
        let setup_loss_ms = data.setup_time_hours * 3600.0 * 1000.0;

        // 2. Perdas de Performance
        let ideal_time_ms = (self.target_cycle_time_ms * data.total_count) as f64;
        let speed_loss_ms = production_time_ms - ideal_time_ms;

        let minor_stops_loss_ms = data.minor_stops_count as f64 * 60.0 * 1000.0; // assumindo 1 min por parada

        // 3. Perdas de Qualidade
        let defect_loss_ms = (data.reject_count as f64 * self.target_cycle_time_ms as f64);

        LossAnalysis {
            breakdown_loss_percent: (breakdown_loss_ms / planned_time_ms) * 100.0,
            setup_changeover_loss_percent: (setup_loss_ms / planned_time_ms) * 100.0,
            small_stops_loss_percent: (minor_stops_loss_ms / planned_time_ms) * 100.0,
            speed_loss_percent: (speed_loss_ms.max(0.0) / planned_time_ms) * 100.0,
            defect_loss_percent: (defect_loss_ms / planned_time_ms) * 100.0,
            startup_loss_percent: 0.0, // Pode ser calculado se houver dados
        }
    }

    /// Classificar OEE
    pub fn classify_oee(&self, oee: f64) -> OEEClassification {
        if oee >= 0.85 {
            OEEClassification::WorldClass
        } else if oee >= 0.60 {
            OEEClassification::Good
        } else if oee >= 0.40 {
            OEEClassification::Average
        } else {
            OEEClassification::Poor
        }
    }
}

/// Dados de produção para cálculo de OEE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionData {
    pub device_id: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub actual_production_time_hours: f64,
    pub downtime_hours: f64,
    pub breakdown_time_hours: f64,
    pub setup_time_hours: f64,
    pub total_count: u64,
    pub good_count: u64,
    pub reject_count: u64,
    pub actual_avg_cycle_time_ms: u64,
    pub minor_stops_count: u32,
}

/// Métricas de OEE
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OEEMetrics {
    pub oee: f64,
    pub availability: f64,
    pub performance: f64,
    pub quality: f64,
    pub production_time_hours: f64,
    pub downtime_hours: f64,
    pub total_count: u64,
    pub good_count: u64,
    pub reject_count: u64,
    pub target_count: u64,
    pub ideal_cycle_time_ms: u64,
    pub actual_cycle_time_ms: u64,
    pub calculated_at: DateTime<Utc>,
}

impl OEEMetrics {
    /// Formatar OEE como percentual
    pub fn oee_percent(&self) -> f64 {
        self.oee * 100.0
    }

    /// Formatar disponibilidade como percentual
    pub fn availability_percent(&self) -> f64 {
        self.availability * 100.0
    }

    /// Formatar performance como percentual
    pub fn performance_percent(&self) -> f64 {
        self.performance * 100.0
    }

    /// Formatar qualidade como percentual
    pub fn quality_percent(&self) -> f64 {
        self.quality * 100.0
    }

    /// Calcular taxa de defeitos
    pub fn defect_rate(&self) -> f64 {
        if self.total_count == 0 {
            return 0.0;
        }
        (self.reject_count as f64 / self.total_count as f64) * 100.0
    }
}

/// Classificação de OEE
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OEEClassification {
    WorldClass,  // >= 85%
    Good,        // >= 60%
    Average,     // >= 40%
    Poor,        // < 40%
}

/// Análise de perdas (Six Big Losses)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LossAnalysis {
    pub breakdown_loss_percent: f64,
    pub setup_changeover_loss_percent: f64,
    pub small_stops_loss_percent: f64,
    pub speed_loss_percent: f64,
    pub defect_loss_percent: f64,
    pub startup_loss_percent: f64,
}

impl LossAnalysis {
    /// Obter perda total
    pub fn total_loss_percent(&self) -> f64 {
        self.breakdown_loss_percent
            + self.setup_changeover_loss_percent
            + self.small_stops_loss_percent
            + self.speed_loss_percent
            + self.defect_loss_percent
            + self.startup_loss_percent
    }

    /// Obter maior perda
    pub fn biggest_loss(&self) -> (&'static str, f64) {
        let losses = vec![
            ("Breakdown", self.breakdown_loss_percent),
            ("Setup/Changeover", self.setup_changeover_loss_percent),
            ("Small Stops", self.small_stops_loss_percent),
            ("Speed Loss", self.speed_loss_percent),
            ("Defects", self.defect_loss_percent),
            ("Startup", self.startup_loss_percent),
        ];

        losses
            .into_iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
    }
}

/// Agregador de OEE ao longo do tempo
pub struct OEEAggregator {
    metrics_history: Vec<OEEMetrics>,
}

impl OEEAggregator {
    pub fn new() -> Self {
        Self {
            metrics_history: Vec::new(),
        }
    }

    /// Adicionar métrica
    pub fn add_metric(&mut self, metric: OEEMetrics) {
        self.metrics_history.push(metric);
    }

    /// Calcular OEE médio
    pub fn average_oee(&self) -> f64 {
        if self.metrics_history.is_empty() {
            return 0.0;
        }

        let sum: f64 = self.metrics_history.iter().map(|m| m.oee).sum();
        sum / self.metrics_history.len() as f64
    }

    /// Calcular tendência de OEE
    pub fn oee_trend(&self) -> Trend {
        if self.metrics_history.len() < 2 {
            return Trend::Stable;
        }

        let recent = self.metrics_history.last().unwrap().oee;
        let previous = self.metrics_history[self.metrics_history.len() - 2].oee;

        let change = recent - previous;

        if change > 0.05 {
            Trend::Improving
        } else if change < -0.05 {
            Trend::Declining
        } else {
            Trend::Stable
        }
    }

    /// Obter estatísticas
    pub fn get_statistics(&self) -> OEEStatistics {
        if self.metrics_history.is_empty() {
            return OEEStatistics::default();
        }

        let oee_values: Vec<f64> = self.metrics_history.iter().map(|m| m.oee).collect();

        let min = oee_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = oee_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let avg = oee_values.iter().sum::<f64>() / oee_values.len() as f64;

        // Calcular desvio padrão
        let variance = oee_values.iter()
            .map(|v| (v - avg).powi(2))
            .sum::<f64>() / oee_values.len() as f64;
        let std_dev = variance.sqrt();

        OEEStatistics {
            count: self.metrics_history.len(),
            min_oee: min,
            max_oee: max,
            avg_oee: avg,
            std_dev_oee: std_dev,
            trend: self.oee_trend(),
        }
    }
}

impl Default for OEEAggregator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Trend {
    Improving,
    Stable,
    Declining,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OEEStatistics {
    pub count: usize,
    pub min_oee: f64,
    pub max_oee: f64,
    pub avg_oee: f64,
    pub std_dev_oee: f64,
    pub trend: Trend,
}

impl Default for OEEStatistics {
    fn default() -> Self {
        Self {
            count: 0,
            min_oee: 0.0,
            max_oee: 0.0,
            avg_oee: 0.0,
            std_dev_oee: 0.0,
            trend: Trend::Stable,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oee_calculation() {
        let calculator = OEECalculator::new(1000, 8.0); // 1s cycle time, 8h planned

        let data = ProductionData {
            device_id: "machine-1".to_string(),
            period_start: Utc::now(),
            period_end: Utc::now(),
            actual_production_time_hours: 7.0, // 7h production
            downtime_hours: 1.0,
            breakdown_time_hours: 0.5,
            setup_time_hours: 0.5,
            total_count: 20000, // 20k pieces
            good_count: 19000,  // 19k good
            reject_count: 1000, // 1k rejects
            actual_avg_cycle_time_ms: 1200, // 1.2s actual
            minor_stops_count: 10,
        };

        let metrics = calculator.calculate_oee(&data);

        assert!(metrics.availability > 0.0);
        assert!(metrics.performance > 0.0);
        assert!(metrics.quality > 0.0);
        assert!(metrics.oee > 0.0);
        assert_eq!(metrics.quality, 0.95); // 19000/20000
    }

    #[test]
    fn test_oee_classification() {
        let calculator = OEECalculator::new(1000, 8.0);

        assert_eq!(calculator.classify_oee(0.90), OEEClassification::WorldClass);
        assert_eq!(calculator.classify_oee(0.70), OEEClassification::Good);
        assert_eq!(calculator.classify_oee(0.50), OEEClassification::Average);
        assert_eq!(calculator.classify_oee(0.30), OEEClassification::Poor);
    }
}
