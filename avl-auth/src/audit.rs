//! Comprehensive audit logging system

use crate::error::Result;
use crate::models::{AuditLog, AuditResult};
use chrono::Utc;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct AuditManager {
    logs: Arc<RwLock<Vec<AuditLog>>>,
    retention_days: u32,
}

impl AuditManager {
    pub fn new(retention_days: u32) -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            retention_days,
        }
    }

    pub async fn log(
        &self,
        user_id: Option<Uuid>,
        session_id: Option<Uuid>,
        action: String,
        resource: String,
        result: AuditResult,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        metadata: HashMap<String, serde_json::Value>,
        risk_score: u8,
    ) {
        let log = AuditLog {
            id: Uuid::new_v4(),
            user_id,
            session_id,
            action,
            resource,
            result,
            ip_address,
            user_agent,
            metadata,
            risk_score,
            timestamp: Utc::now(),
        };

        let mut logs = self.logs.write().await;
        logs.push(log.clone());

        // Persist to database (would normally be async)
        self.persist_log(&log).await;

        // Send to telemetry
        self.send_to_telemetry(&log);
    }

    async fn persist_log(&self, log: &AuditLog) {
        // In production, write to AvilaDB
        tracing::debug!("Persisting audit log: {:?}", log.id);
    }

    fn send_to_telemetry(&self, log: &AuditLog) {
        tracing::info!(
            user_id = ?log.user_id,
            action = %log.action,
            resource = %log.resource,
            result = ?log.result,
            risk_score = log.risk_score,
            "Audit event"
        );
    }

    pub async fn query(
        &self,
        user_id: Option<Uuid>,
        action: Option<String>,
        start_time: Option<chrono::DateTime<Utc>>,
        end_time: Option<chrono::DateTime<Utc>>,
        limit: usize,
    ) -> Vec<AuditLog> {
        let logs = self.logs.read().await;

        logs.iter()
            .filter(|log| {
                if let Some(uid) = user_id {
                    if log.user_id != Some(uid) {
                        return false;
                    }
                }

                if let Some(ref act) = action {
                    if &log.action != act {
                        return false;
                    }
                }

                if let Some(start) = start_time {
                    if log.timestamp < start {
                        return false;
                    }
                }

                if let Some(end) = end_time {
                    if log.timestamp > end {
                        return false;
                    }
                }

                true
            })
            .take(limit)
            .cloned()
            .collect()
    }

    pub async fn cleanup_old_logs(&self) -> Result<usize> {
        let cutoff = Utc::now() - chrono::Duration::days(self.retention_days as i64);

        let mut logs = self.logs.write().await;
        let initial_count = logs.len();

        logs.retain(|log| log.timestamp > cutoff);

        let removed = initial_count - logs.len();

        tracing::info!("Cleaned up {} old audit logs", removed);
        Ok(removed)
    }

    pub async fn get_user_activity(&self, user_id: &Uuid, days: u32) -> UserActivity {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let logs = self.logs.read().await;

        let user_logs: Vec<_> = logs
            .iter()
            .filter(|log| log.user_id == Some(*user_id) && log.timestamp > since)
            .collect();

        let total_actions = user_logs.len();
        let successful = user_logs.iter().filter(|l| l.result == AuditResult::Success).count();
        let failed = user_logs.iter().filter(|l| l.result == AuditResult::Failure).count();
        let blocked = user_logs.iter().filter(|l| l.result == AuditResult::Blocked).count();

        let unique_ips: std::collections::HashSet<_> = user_logs
            .iter()
            .filter_map(|l| l.ip_address)
            .collect();

        let action_breakdown: HashMap<String, usize> = user_logs
            .iter()
            .fold(HashMap::new(), |mut acc, log| {
                *acc.entry(log.action.clone()).or_insert(0) += 1;
                acc
            });

        UserActivity {
            user_id: *user_id,
            total_actions,
            successful,
            failed,
            blocked,
            unique_ips: unique_ips.len(),
            action_breakdown,
            avg_risk_score: if total_actions > 0 {
                user_logs.iter().map(|l| l.risk_score as f64).sum::<f64>() / total_actions as f64
            } else {
                0.0
            },
        }
    }

    // Compliance reporting for LGPD/GDPR
    pub async fn generate_compliance_report(
        &self,
        user_id: &Uuid,
    ) -> ComplianceReport {
        let logs = self.logs.read().await;

        let user_logs: Vec<_> = logs
            .iter()
            .filter(|log| log.user_id == Some(*user_id))
            .cloned()
            .collect();

        let data_accesses = user_logs
            .iter()
            .filter(|l| l.action.contains("read") || l.action.contains("access"))
            .count();

        let data_modifications = user_logs
            .iter()
            .filter(|l| l.action.contains("update") || l.action.contains("delete"))
            .count();

        let data_exports = user_logs
            .iter()
            .filter(|l| l.action.contains("export"))
            .count();

        ComplianceReport {
            user_id: *user_id,
            report_date: Utc::now(),
            total_events: user_logs.len(),
            data_accesses,
            data_modifications,
            data_exports,
            logs: user_logs,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UserActivity {
    pub user_id: Uuid,
    pub total_actions: usize,
    pub successful: usize,
    pub failed: usize,
    pub blocked: usize,
    pub unique_ips: usize,
    pub action_breakdown: HashMap<String, usize>,
    pub avg_risk_score: f64,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ComplianceReport {
    pub user_id: Uuid,
    pub report_date: chrono::DateTime<Utc>,
    pub total_events: usize,
    pub data_accesses: usize,
    pub data_modifications: usize,
    pub data_exports: usize,
    pub logs: Vec<AuditLog>,
}
