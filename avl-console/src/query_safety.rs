//! Query Safety and Validation Layer
//!
//! Prevents dangerous SQL operations and enforces execution policies.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    static ref DANGEROUS_KEYWORDS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("DROP");
        set.insert("TRUNCATE");
        set.insert("DELETE");
        set.insert("ALTER");
        set.insert("CREATE");
        set.insert("GRANT");
        set.insert("REVOKE");
        set.insert("EXECUTE");
        set.insert("EXEC");
        set.insert("SHUTDOWN");
        set.insert("KILL");
        set
    };

    static ref SQL_INJECTION_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"(?i)(union\s+select)").unwrap(),
        Regex::new(r"(?i)(;\s*drop)").unwrap(),
        Regex::new(r"(?i)(;\s*delete)").unwrap(),
        Regex::new(r"(?i)(;\s*update)").unwrap(),
        Regex::new(r"(?i)(--\s*$)").unwrap(),
        Regex::new(r"(?i)(/\*.*\*/)").unwrap(),
        Regex::new(r"(?i)(xp_cmdshell)").unwrap(),
        Regex::new(r"(?i)(sp_executesql)").unwrap(),
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyAnalysis {
    pub risk_level: RiskLevel,
    pub is_allowed: bool,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
    pub sanitized_query: Option<String>,
}

impl SafetyAnalysis {
    pub fn is_dangerous(&self) -> bool {
        matches!(self.risk_level, RiskLevel::High | RiskLevel::Critical)
    }
}

#[derive(Debug, Clone)]
pub struct QueryPolicy {
    pub allow_mutations: bool,
    pub allow_schema_changes: bool,
    pub allow_admin_operations: bool,
    pub max_result_rows: usize,
    pub require_where_clause: bool,
}

impl Default for QueryPolicy {
    fn default() -> Self {
        Self {
            allow_mutations: false,
            allow_schema_changes: false,
            allow_admin_operations: false,
            max_result_rows: 1000,
            require_where_clause: true,
        }
    }
}

impl QueryPolicy {
    pub fn read_only() -> Self {
        Self {
            allow_mutations: false,
            allow_schema_changes: false,
            allow_admin_operations: false,
            max_result_rows: 1000,
            require_where_clause: false,
        }
    }

    pub fn developer() -> Self {
        Self {
            allow_mutations: true,
            allow_schema_changes: false,
            allow_admin_operations: false,
            max_result_rows: 10000,
            require_where_clause: false,
        }
    }

    pub fn admin() -> Self {
        Self {
            allow_mutations: true,
            allow_schema_changes: true,
            allow_admin_operations: true,
            max_result_rows: 100000,
            require_where_clause: false,
        }
    }
}

pub struct QueryValidator {
    policy: QueryPolicy,
}

impl QueryValidator {
    pub fn new(policy: QueryPolicy) -> Self {
        Self { policy }
    }

    pub fn read_only() -> Self {
        Self::new(QueryPolicy::read_only())
    }

    pub fn developer() -> Self {
        Self::new(QueryPolicy::developer())
    }

    pub fn admin() -> Self {
        Self::new(QueryPolicy::admin())
    }

    pub fn validate(&self, query: &str) -> SafetyAnalysis {
        let mut violations = Vec::new();
        let mut warnings = Vec::new();
        let mut risk_level = RiskLevel::Safe;

        let normalized = query.to_uppercase();
        let tokens: Vec<&str> = normalized.split_whitespace().collect();

        // Check for dangerous keywords
        for keyword in DANGEROUS_KEYWORDS.iter() {
            if tokens.contains(keyword) {
                match *keyword {
                    "DROP" | "TRUNCATE" => {
                        if !self.policy.allow_schema_changes {
                            violations.push(format!("Operação {} não permitida", keyword));
                            risk_level = RiskLevel::Critical;
                        }
                    }
                    "DELETE" | "UPDATE" => {
                        if !self.policy.allow_mutations {
                            violations.push(format!("Operação {} não permitida", keyword));
                            risk_level = RiskLevel::High;
                        } else if !normalized.contains("WHERE") {
                            violations.push(format!("{} sem cláusula WHERE é perigoso", keyword));
                            risk_level = RiskLevel::High;
                        }
                    }
                    "ALTER" | "CREATE" => {
                        if !self.policy.allow_schema_changes {
                            violations.push(format!("Alteração de schema não permitida"));
                            risk_level = RiskLevel::High;
                        }
                    }
                    "GRANT" | "REVOKE" | "SHUTDOWN" | "KILL" => {
                        if !self.policy.allow_admin_operations {
                            violations.push(format!("Operação administrativa não permitida"));
                            risk_level = RiskLevel::Critical;
                        }
                    }
                    _ => {
                        warnings.push(format!("Palavra-chave sensível detectada: {}", keyword));
                        if risk_level == RiskLevel::Safe {
                            risk_level = RiskLevel::Low;
                        }
                    }
                }
            }
        }

        // Check for SQL injection patterns
        for pattern in SQL_INJECTION_PATTERNS.iter() {
            if pattern.is_match(query) {
                violations.push("Padrão de SQL injection detectado".to_string());
                risk_level = RiskLevel::Critical;
            }
        }

        // Check for SELECT * without LIMIT
        if normalized.contains("SELECT *") && !normalized.contains("LIMIT") {
            warnings.push("SELECT * sem LIMIT pode retornar muitos dados".to_string());
            if risk_level == RiskLevel::Safe {
                risk_level = RiskLevel::Low;
            }
        }

        // Check for missing LIMIT on large tables
        if normalized.starts_with("SELECT") && !normalized.contains("LIMIT") {
            warnings.push(format!("Considere adicionar LIMIT {} para limitar resultados", self.policy.max_result_rows));
        }

        // Sanitize and add LIMIT if needed
        let sanitized_query = if normalized.starts_with("SELECT") && !normalized.contains("LIMIT") {
            Some(format!("{} LIMIT {}", query, self.policy.max_result_rows))
        } else {
            None
        };

        SafetyAnalysis {
            risk_level,
            is_allowed: violations.is_empty(),
            violations,
            warnings,
            sanitized_query,
        }
    }

    pub fn sanitize(&self, query: &str) -> Result<String, String> {
        let analysis = self.validate(query);

        if !analysis.is_allowed {
            return Err(format!("Query bloqueada: {}", analysis.violations.join(", ")));
        }

        Ok(analysis.sanitized_query.unwrap_or_else(|| query.to_string()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: String,
    pub user_id: String,
    pub query: String,
    pub risk_level: RiskLevel,
    pub allowed: bool,
    pub execution_time_ms: Option<u64>,
    pub rows_affected: Option<usize>,
}

pub struct AuditLogger {
    entries: Vec<AuditLogEntry>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, entry: AuditLogEntry) {
        self.entries.push(entry);
    }

    pub fn get_recent(&self, limit: usize) -> Vec<&AuditLogEntry> {
        self.entries.iter().rev().take(limit).collect()
    }

    pub fn get_by_user(&self, user_id: &str) -> Vec<&AuditLogEntry> {
        self.entries.iter().filter(|e| e.user_id == user_id).collect()
    }

    pub fn get_high_risk(&self) -> Vec<&AuditLogEntry> {
        self.entries
            .iter()
            .filter(|e| matches!(e.risk_level, RiskLevel::High | RiskLevel::Critical))
            .collect()
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_select() {
        let validator = QueryValidator::new(QueryPolicy::read_only());
        let analysis = validator.validate("SELECT id, name FROM users LIMIT 10");
        assert_eq!(analysis.risk_level, RiskLevel::Safe);
        assert!(analysis.is_allowed);
        assert!(analysis.violations.is_empty());
    }

    #[test]
    fn test_dangerous_drop() {
        let validator = QueryValidator::new(QueryPolicy::read_only());
        let analysis = validator.validate("DROP TABLE users");
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
        assert!(!analysis.is_allowed);
        assert!(!analysis.violations.is_empty());
    }

    #[test]
    fn test_delete_without_where() {
        let validator = QueryValidator::new(QueryPolicy::developer());
        let analysis = validator.validate("DELETE FROM users");
        assert_eq!(analysis.risk_level, RiskLevel::High);
        assert!(!analysis.is_allowed);
    }

    #[test]
    fn test_delete_with_where() {
        let validator = QueryValidator::new(QueryPolicy::developer());
        let analysis = validator.validate("DELETE FROM users WHERE id = 123");
        assert!(analysis.is_allowed);
    }

    #[test]
    fn test_sql_injection_union() {
        let validator = QueryValidator::new(QueryPolicy::read_only());
        let analysis = validator.validate("SELECT * FROM users WHERE id = 1 UNION SELECT password FROM admin");
        assert_eq!(analysis.risk_level, RiskLevel::Critical);
        assert!(!analysis.is_allowed);
    }

    #[test]
    fn test_select_star_warning() {
        let validator = QueryValidator::new(QueryPolicy::read_only());
        let analysis = validator.validate("SELECT * FROM users");
        assert!(analysis.is_allowed);
        assert!(!analysis.warnings.is_empty());
    }

    #[test]
    fn test_sanitize_adds_limit() {
        let validator = QueryValidator::new(QueryPolicy::read_only());
        let sanitized = validator.sanitize("SELECT * FROM users").unwrap();
        assert!(sanitized.contains("LIMIT"));
    }

    #[test]
    fn test_policy_read_only() {
        let policy = QueryPolicy::read_only();
        assert!(!policy.allow_mutations);
        assert!(!policy.allow_schema_changes);
    }

    #[test]
    fn test_policy_admin() {
        let policy = QueryPolicy::admin();
        assert!(policy.allow_mutations);
        assert!(policy.allow_schema_changes);
        assert!(policy.allow_admin_operations);
    }

    #[test]
    fn test_audit_logger() {
        let mut logger = AuditLogger::new();
        logger.log(AuditLogEntry {
            timestamp: "2024-11-23T10:00:00Z".to_string(),
            user_id: "user1".to_string(),
            query: "SELECT * FROM users".to_string(),
            risk_level: RiskLevel::Safe,
            allowed: true,
            execution_time_ms: Some(150),
            rows_affected: Some(100),
        });

        assert_eq!(logger.get_recent(10).len(), 1);
        assert_eq!(logger.get_by_user("user1").len(), 1);
    }
}
