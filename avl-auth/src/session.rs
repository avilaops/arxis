//! Session management with distributed storage

use crate::error::{AuthError, Result};
use crate::models::Session;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct SessionManager {
    sessions: Arc<RwLock<HashMap<Uuid, Session>>>,
    config: SessionConfig,
}

#[derive(Clone)]
pub struct SessionConfig {
    pub idle_timeout: Duration,
    pub absolute_timeout: Duration,
    pub max_concurrent_sessions: u32,
    pub device_binding: bool,
    pub ip_binding: bool,
}

impl SessionManager {
    pub fn new(config: SessionConfig) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            config,
        }
    }

    pub async fn create_session(
        &self,
        user_id: Uuid,
        access_token: String,
        refresh_token: String,
        access_token_ttl: Duration,
        refresh_token_ttl: Duration,
        device_id: Option<String>,
        ip_address: Option<IpAddr>,
        user_agent: Option<String>,
        scopes: Vec<String>,
    ) -> Result<Session> {
        // Check concurrent session limit
        self.enforce_concurrent_limit(&user_id).await?;

        let now = Utc::now();
        let session = Session {
            id: Uuid::new_v4(),
            user_id,
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
            expires_at: now + access_token_ttl,
            refresh_expires_at: now + refresh_token_ttl,
            device_id,
            ip_address,
            user_agent,
            created_at: now,
            last_active_at: now,
            scopes,
        };

        let mut sessions = self.sessions.write().await;
        sessions.insert(session.id, session.clone());

        tracing::info!(
            session_id = %session.id,
            user_id = %user_id,
            "Created new session"
        );

        Ok(session)
    }

    async fn enforce_concurrent_limit(&self, user_id: &Uuid) -> Result<()> {
        let sessions = self.sessions.read().await;

        let user_sessions: Vec<_> = sessions
            .values()
            .filter(|s| s.user_id == *user_id)
            .collect();

        if user_sessions.len() >= self.config.max_concurrent_sessions as usize {
            // Get oldest session ID
            if let Some(oldest) = user_sessions
                .iter()
                .min_by_key(|s| s.created_at)
            {
                let oldest_id = oldest.id;
                drop(sessions); // Release read lock
                self.delete_session(&oldest_id).await?;
            }
        }

        Ok(())
    }    pub async fn get_session(&self, session_id: &Uuid) -> Result<Session> {
        let sessions = self.sessions.read().await;
        sessions
            .get(session_id)
            .cloned()
            .ok_or(AuthError::SessionNotFound)
    }

    pub async fn validate_session(
        &self,
        session_id: &Uuid,
        ip_address: Option<IpAddr>,
        device_id: Option<&str>,
    ) -> Result<Session> {
        let sessions = self.sessions.read().await;
        let session = sessions
            .get(session_id)
            .ok_or(AuthError::SessionNotFound)?;

        let now = Utc::now();

        // Check if session expired
        if now > session.expires_at {
            drop(sessions);
            self.delete_session(session_id).await?;
            return Err(AuthError::SessionExpired);
        }

        // Check idle timeout
        let idle_time = now - session.last_active_at;
        if idle_time > self.config.idle_timeout {
            drop(sessions);
            self.delete_session(session_id).await?;
            return Err(AuthError::SessionExpired);
        }

        // Check absolute timeout
        let session_age = now - session.created_at;
        if session_age > self.config.absolute_timeout {
            drop(sessions);
            self.delete_session(session_id).await?;
            return Err(AuthError::SessionExpired);
        }

        // Verify device binding
        if self.config.device_binding {
            if let (Some(session_device), Some(req_device)) = (&session.device_id, device_id) {
                if session_device != req_device {
                    return Err(AuthError::InvalidToken("Device mismatch".to_string()));
                }
            }
        }

        // Verify IP binding
        if self.config.ip_binding {
            if let (Some(session_ip), Some(req_ip)) = (session.ip_address, ip_address) {
                if session_ip != req_ip {
                    return Err(AuthError::InvalidToken("IP address mismatch".to_string()));
                }
            }
        }

        Ok(session.clone())
    }

    pub async fn update_activity(&self, session_id: &Uuid) -> Result<()> {
        let mut sessions = self.sessions.write().await;

        if let Some(session) = sessions.get_mut(session_id) {
            session.last_active_at = Utc::now();
            Ok(())
        } else {
            Err(AuthError::SessionNotFound)
        }
    }

    pub async fn refresh_session(
        &self,
        session_id: &Uuid,
        new_access_token: String,
        access_token_ttl: Duration,
    ) -> Result<Session> {
        let mut sessions = self.sessions.write().await;

        let session = sessions
            .get_mut(session_id)
            .ok_or(AuthError::SessionNotFound)?;

        // Check if refresh token is still valid
        if Utc::now() > session.refresh_expires_at {
            return Err(AuthError::SessionExpired);
        }

        session.access_token = new_access_token;
        session.expires_at = Utc::now() + access_token_ttl;
        session.last_active_at = Utc::now();

        Ok(session.clone())
    }

    pub async fn delete_session(&self, session_id: &Uuid) -> Result<()> {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);

        tracing::info!(session_id = %session_id, "Deleted session");
        Ok(())
    }

    pub async fn delete_user_sessions(&self, user_id: &Uuid) -> Result<usize> {
        let mut sessions = self.sessions.write().await;

        let session_ids: Vec<_> = sessions
            .iter()
            .filter(|(_, s)| s.user_id == *user_id)
            .map(|(id, _)| *id)
            .collect();

        let count = session_ids.len();

        for id in session_ids {
            sessions.remove(&id);
        }

        tracing::info!(user_id = %user_id, count, "Deleted user sessions");
        Ok(count)
    }

    pub async fn list_user_sessions(&self, user_id: &Uuid) -> Vec<Session> {
        let sessions = self.sessions.read().await;
        sessions
            .values()
            .filter(|s| s.user_id == *user_id)
            .cloned()
            .collect()
    }

    pub async fn cleanup_expired_sessions(&self) -> Result<usize> {
        let mut sessions = self.sessions.write().await;
        let now = Utc::now();

        let initial_count = sessions.len();

        sessions.retain(|_, session| {
            let expired = now > session.expires_at
                || (now - session.last_active_at) > self.config.idle_timeout;
            !expired
        });

        let removed = initial_count - sessions.len();

        if removed > 0 {
            tracing::info!("Cleaned up {} expired sessions", removed);
        }

        Ok(removed)
    }

    pub async fn get_stats(&self) -> SessionStats {
        let sessions = self.sessions.read().await;

        let total = sessions.len();
        let unique_users = sessions
            .values()
            .map(|s| s.user_id)
            .collect::<std::collections::HashSet<_>>()
            .len();

        let now = Utc::now();
        let active_last_hour = sessions
            .values()
            .filter(|s| (now - s.last_active_at) < Duration::hours(1))
            .count();

        SessionStats {
            total_sessions: total,
            unique_users,
            active_last_hour,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct SessionStats {
    pub total_sessions: usize,
    pub unique_users: usize,
    pub active_last_hour: usize,
}
