//! Role-Based and Attribute-Based Access Control

use crate::error::{AuthError, Result};
use crate::models::{Permission, Policy, PolicyCondition, PolicyEffect, Role, User};
use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct PermissionManager {
    roles: Arc<RwLock<HashMap<String, Role>>>,
    permissions: Arc<RwLock<HashMap<String, Permission>>>,
    policies: Arc<RwLock<Vec<Policy>>>,
}

impl PermissionManager {
    pub fn new() -> Self {
        Self {
            roles: Arc::new(RwLock::new(HashMap::new())),
            permissions: Arc::new(RwLock::new(HashMap::new())),
            policies: Arc::new(RwLock::new(Vec::new())),
        }
    }

    // ==================== Role Management ====================

    pub async fn create_role(&self, role: Role) -> Result<()> {
        let mut roles = self.roles.write().await;

        if roles.contains_key(&role.id) {
            return Err(AuthError::Internal(format!("Role already exists: {}", role.id)));
        }

        roles.insert(role.id.clone(), role);
        Ok(())
    }

    pub async fn get_role(&self, role_id: &str) -> Result<Role> {
        let roles = self.roles.read().await;
        roles
            .get(role_id)
            .cloned()
            .ok_or_else(|| AuthError::PermissionDenied(format!("Role not found: {}", role_id)))
    }

    pub async fn update_role(&self, role: Role) -> Result<()> {
        let mut roles = self.roles.write().await;
        roles.insert(role.id.clone(), role);
        Ok(())
    }

    pub async fn delete_role(&self, role_id: &str) -> Result<()> {
        let mut roles = self.roles.write().await;
        roles.remove(role_id);
        Ok(())
    }

    // ==================== Permission Management ====================

    pub async fn create_permission(&self, permission: Permission) -> Result<()> {
        let mut permissions = self.permissions.write().await;

        if permissions.contains_key(&permission.id) {
            return Err(AuthError::Internal(format!("Permission already exists: {}", permission.id)));
        }

        permissions.insert(permission.id.clone(), permission);
        Ok(())
    }

    pub async fn get_permission(&self, permission_id: &str) -> Result<Permission> {
        let permissions = self.permissions.read().await;
        permissions
            .get(permission_id)
            .cloned()
            .ok_or_else(|| AuthError::PermissionDenied(format!("Permission not found: {}", permission_id)))
    }

    // ==================== Policy Management (ABAC) ====================

    pub async fn create_policy(&self, policy: Policy) -> Result<()> {
        let mut policies = self.policies.write().await;
        policies.push(policy);

        // Sort by priority (higher priority first)
        policies.sort_by(|a, b| b.priority.cmp(&a.priority));
        Ok(())
    }

    pub async fn get_policies(&self) -> Vec<Policy> {
        let policies = self.policies.read().await;
        policies.clone()
    }

    pub async fn delete_policy(&self, policy_id: &uuid::Uuid) -> Result<()> {
        let mut policies = self.policies.write().await;
        policies.retain(|p| &p.id != policy_id);
        Ok(())
    }

    // ==================== Access Control ====================

    pub async fn check_permission(
        &self,
        user: &User,
        resource: &str,
        action: &str,
    ) -> Result<bool> {
        // First check direct user permissions
        let permission_id = format!("{}:{}", resource, action);
        if user.permissions.contains(&permission_id) {
            return Ok(true);
        }

        // Then check role-based permissions
        let roles = self.roles.read().await;
        let mut resolved_permissions = HashSet::new();

        for role_id in &user.roles {
            if let Some(role) = roles.get(role_id) {
                self.resolve_role_permissions(role, &roles, &mut resolved_permissions);
            }
        }

        Ok(resolved_permissions.contains(&permission_id))
    }

    fn resolve_role_permissions(
        &self,
        role: &Role,
        all_roles: &HashMap<String, Role>,
        resolved: &mut HashSet<String>,
    ) {
        // Add direct permissions
        for perm in &role.permissions {
            resolved.insert(perm.clone());
        }

        // Recursively resolve inherited roles
        for inherited_role_id in &role.inherits {
            if let Some(inherited_role) = all_roles.get(inherited_role_id) {
                self.resolve_role_permissions(inherited_role, all_roles, resolved);
            }
        }
    }

    pub async fn evaluate_policies(
        &self,
        user: &User,
        resource: &str,
        action: &str,
        context: &AccessContext,
    ) -> Result<PolicyDecision> {
        let policies = self.policies.read().await;

        let mut allow = false;
        let mut deny = false;
        let mut matched_policies = Vec::new();

        for policy in policies.iter() {
            if !policy.enabled {
                continue;
            }

            // Check if policy applies to this resource and action
            if !self.matches_resource(resource, &policy.resources) {
                continue;
            }

            if !self.matches_action(action, &policy.actions) {
                continue;
            }

            // Evaluate conditions
            if !self.evaluate_conditions(&policy.conditions, user, context).await {
                continue;
            }

            matched_policies.push(policy.name.clone());

            match policy.effect {
                PolicyEffect::Allow => allow = true,
                PolicyEffect::Deny => {
                    deny = true;
                    break; // Deny takes precedence
                }
            }
        }

        let decision = if deny {
            PolicyDecision::Deny
        } else if allow {
            PolicyDecision::Allow
        } else {
            PolicyDecision::Implicit
        };

        Ok(decision)
    }

    fn matches_resource(&self, resource: &str, patterns: &[String]) -> bool {
        for pattern in patterns {
            if pattern == "*" || pattern == resource {
                return true;
            }

            // Support wildcard matching
            if pattern.ends_with("*") {
                let prefix = &pattern[..pattern.len() - 1];
                if resource.starts_with(prefix) {
                    return true;
                }
            }
        }
        false
    }

    fn matches_action(&self, action: &str, patterns: &[String]) -> bool {
        patterns.contains(&action.to_string()) || patterns.contains(&"*".to_string())
    }

    async fn evaluate_conditions(
        &self,
        conditions: &[PolicyCondition],
        user: &User,
        context: &AccessContext,
    ) -> bool {
        for condition in conditions {
            if !self.evaluate_condition(condition, user, context) {
                return false;
            }
        }
        true
    }

    fn evaluate_condition(
        &self,
        condition: &PolicyCondition,
        user: &User,
        context: &AccessContext,
    ) -> bool {
        match condition {
            PolicyCondition::IpRange { cidrs } => {
                if let Some(ip) = context.ip_address {
                    self.ip_in_ranges(&ip, cidrs)
                } else {
                    false
                }
            }
            PolicyCondition::TimeWindow { start, end } => {
                self.time_in_window(start, end)
            }
            PolicyCondition::UserAttribute { key, value } => {
                if let Some(user_value) = user.metadata.get(key) {
                    user_value == value
                } else {
                    false
                }
            }
            PolicyCondition::RiskScore { max } => {
                context.risk_score <= *max
            }
        }
    }

    fn ip_in_ranges(&self, ip: &IpAddr, cidrs: &[String]) -> bool {
        // Simplified CIDR matching
        // In production, use a proper CIDR library
        for cidr in cidrs {
            if cidr.contains(&ip.to_string()) {
                return true;
            }
        }
        false
    }

    fn time_in_window(&self, start: &str, end: &str) -> bool {
        use chrono::{NaiveTime, Utc};

        let now = Utc::now().time();

        if let (Ok(start_time), Ok(end_time)) = (
            NaiveTime::parse_from_str(start, "%H:%M"),
            NaiveTime::parse_from_str(end, "%H:%M"),
        ) {
            if start_time <= end_time {
                now >= start_time && now <= end_time
            } else {
                // Handle overnight window
                now >= start_time || now <= end_time
            }
        } else {
            false
        }
    }

    // ==================== Authorization Check ====================

    pub async fn authorize(
        &self,
        user: &User,
        resource: &str,
        action: &str,
        context: &AccessContext,
    ) -> Result<bool> {
        // 1. Check explicit deny policies first
        let policy_decision = self.evaluate_policies(user, resource, action, context).await?;

        if matches!(policy_decision, PolicyDecision::Deny) {
            return Ok(false);
        }

        // 2. Check RBAC permissions
        let has_permission = self.check_permission(user, resource, action).await?;

        if has_permission {
            return Ok(true);
        }

        // 3. Check if policies explicitly allow
        if matches!(policy_decision, PolicyDecision::Allow) {
            return Ok(true);
        }

        // 4. Default deny
        Ok(false)
    }
}

#[derive(Debug, Clone)]
pub struct AccessContext {
    pub ip_address: Option<IpAddr>,
    pub user_agent: Option<String>,
    pub risk_score: u8,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allow,
    Deny,
    Implicit, // No explicit policy matched
}

impl Default for PermissionManager {
    fn default() -> Self {
        Self::new()
    }
}
