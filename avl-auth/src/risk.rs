//! Risk-based authentication and anomaly detection

use crate::error::Result;
use crate::models::{RiskAction, RiskAssessment, RiskFactor, RiskLevel, User};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct RiskEngine {
    config: RiskConfig,
    user_behavior: Arc<RwLock<HashMap<String, UserBehaviorProfile>>>,
}

#[derive(Clone)]
pub struct RiskConfig {
    pub mfa_threshold: u8,
    pub block_threshold: u8,
    pub geo_velocity_enabled: bool,
    pub max_travel_speed_kmh: f64,
}

#[derive(Clone)]
struct UserBehaviorProfile {
    usual_locations: Vec<Location>,
    usual_devices: Vec<String>,
    usual_login_times: Vec<chrono::NaiveTime>,
    last_location: Option<Location>,
    last_login: Option<chrono::DateTime<chrono::Utc>>,
    successful_logins: u64,
    failed_logins: u64,
}

#[derive(Clone)]
struct Location {
    latitude: f64,
    longitude: f64,
    city: Option<String>,
    country: Option<String>,
}

impl RiskEngine {
    pub fn new(config: RiskConfig) -> Self {
        Self {
            config,
            user_behavior: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn assess_risk(
        &self,
        user: &User,
        ip_address: Option<IpAddr>,
        device_id: Option<&str>,
        _user_agent: Option<&str>,
    ) -> Result<RiskAssessment> {
        let mut factors = Vec::new();
        let mut total_score = 0u8;

        // Factor 1: Account status
        if user.status != crate::models::UserStatus::Active {
            factors.push(RiskFactor {
                name: "Account Status".to_string(),
                score: 100,
                reason: "Account is not active".to_string(),
            });
            total_score = 100;
        }

        // Factor 2: Failed login attempts
        if user.failed_login_attempts > 0 {
            let score = (user.failed_login_attempts * 10).min(50) as u8;
            factors.push(RiskFactor {
                name: "Failed Attempts".to_string(),
                score,
                reason: format!("{} recent failed login attempts", user.failed_login_attempts),
            });
            total_score = total_score.saturating_add(score);
        }

        // Factor 3: Account age
        let account_age = chrono::Utc::now() - user.created_at;
        if account_age < chrono::Duration::days(1) {
            factors.push(RiskFactor {
                name: "New Account".to_string(),
                score: 30,
                reason: "Account created less than 24 hours ago".to_string(),
            });
            total_score = total_score.saturating_add(30);
        }

        // Factor 4: Unusual location (if IP available)
        if let Some(ip) = ip_address {
            if let Some(location_score) = self.check_location_risk(user, &ip).await {
                factors.push(location_score.clone());
                total_score = total_score.saturating_add(location_score.score);
            }
        }

        // Factor 5: Unknown device
        if let Some(device) = device_id {
            if let Some(device_score) = self.check_device_risk(user, device).await {
                factors.push(device_score.clone());
                total_score = total_score.saturating_add(device_score.score);
            }
        }

        // Factor 6: Unusual time
        if let Some(time_score) = self.check_time_risk(user).await {
            factors.push(time_score.clone());
            total_score = total_score.saturating_add(time_score.score);
        }

        // Factor 7: Geo-velocity check
        if self.config.geo_velocity_enabled {
            if let (Some(_ip), Some(velocity_score)) = (ip_address, self.check_geo_velocity(user, &ip_address.unwrap()).await) {
                factors.push(velocity_score.clone());
                total_score = total_score.saturating_add(velocity_score.score);
            }
        }

        // Determine risk level and recommended action
        let level = match total_score {
            0..=30 => RiskLevel::Low,
            31..=60 => RiskLevel::Medium,
            61..=85 => RiskLevel::High,
            _ => RiskLevel::Critical,
        };

        let recommended_action = if total_score >= self.config.block_threshold {
            RiskAction::Deny
        } else if total_score >= self.config.mfa_threshold {
            RiskAction::RequireMfa
        } else if total_score >= 40 {
            RiskAction::Challenge
        } else {
            RiskAction::Allow
        };

        Ok(RiskAssessment {
            score: total_score,
            level,
            factors,
            recommended_action,
        })
    }

    async fn check_location_risk(&self, user: &User, ip: &IpAddr) -> Option<RiskFactor> {
        // Simplified: In production, use a geo-IP service
        let location = self.get_location_from_ip(ip)?;

        let behavior = self.user_behavior.read().await;
        let profile = behavior.get(&user.id.to_string())?;

        let is_usual = profile.usual_locations.iter().any(|loc| {
            self.distance_km(loc, &location) < 100.0
        });

        if !is_usual {
            Some(RiskFactor {
                name: "Unusual Location".to_string(),
                score: 25,
                reason: format!("Login from unfamiliar location: {:?}", location.city),
            })
        } else {
            None
        }
    }

    async fn check_device_risk(&self, user: &User, device_id: &str) -> Option<RiskFactor> {
        let behavior = self.user_behavior.read().await;
        let profile = behavior.get(&user.id.to_string())?;

        if !profile.usual_devices.contains(&device_id.to_string()) {
            Some(RiskFactor {
                name: "Unknown Device".to_string(),
                score: 20,
                reason: "Login from unrecognized device".to_string(),
            })
        } else {
            None
        }
    }

    async fn check_time_risk(&self, user: &User) -> Option<RiskFactor> {
        let current_time = chrono::Utc::now().time();

        let behavior = self.user_behavior.read().await;
        let profile = behavior.get(&user.id.to_string())?;

        // Check if current time is within usual login patterns (±2 hours)
        let is_usual_time = profile.usual_login_times.iter().any(|usual| {
            let diff = if current_time >= *usual {
                (current_time - *usual).num_hours()
            } else {
                (*usual - current_time).num_hours()
            };
            diff <= 2
        });

        if !is_usual_time && !profile.usual_login_times.is_empty() {
            Some(RiskFactor {
                name: "Unusual Time".to_string(),
                score: 15,
                reason: "Login at unusual time of day".to_string(),
            })
        } else {
            None
        }
    }

    async fn check_geo_velocity(&self, user: &User, ip: &IpAddr) -> Option<RiskFactor> {
        let current_location = self.get_location_from_ip(ip)?;

        let behavior = self.user_behavior.read().await;
        let profile = behavior.get(&user.id.to_string())?;

        let last_location = profile.last_location.as_ref()?;
        let last_login = profile.last_login?;

        let distance = self.distance_km(last_location, &current_location);
        let time_diff = (chrono::Utc::now() - last_login).num_hours() as f64;

        if time_diff > 0.0 {
            let velocity = distance / time_diff;

            if velocity > self.config.max_travel_speed_kmh {
                return Some(RiskFactor {
                    name: "Impossible Travel".to_string(),
                    score: 40,
                    reason: format!(
                        "Travel speed of {:.0} km/h exceeds maximum",
                        velocity
                    ),
                });
            }
        }

        None
    }

    fn get_location_from_ip(&self, _ip: &IpAddr) -> Option<Location> {
        // Placeholder: In production, integrate with MaxMind GeoIP2 or similar
        Some(Location {
            latitude: -23.5505,
            longitude: -46.6333,
            city: Some("São Paulo".to_string()),
            country: Some("Brazil".to_string()),
        })
    }

    fn distance_km(&self, loc1: &Location, loc2: &Location) -> f64 {
        // Haversine formula
        let r = 6371.0; // Earth radius in km

        let lat1 = loc1.latitude.to_radians();
        let lat2 = loc2.latitude.to_radians();
        let delta_lat = (loc2.latitude - loc1.latitude).to_radians();
        let delta_lon = (loc2.longitude - loc1.longitude).to_radians();

        let a = (delta_lat / 2.0).sin().powi(2)
            + lat1.cos() * lat2.cos() * (delta_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

        r * c
    }

    pub async fn update_behavior_profile(
        &self,
        user_id: &uuid::Uuid,
        ip_address: Option<IpAddr>,
        device_id: Option<String>,
        success: bool,
    ) {
        let mut behavior = self.user_behavior.write().await;
        let profile = behavior.entry(user_id.to_string()).or_insert_with(|| UserBehaviorProfile {
            usual_locations: Vec::new(),
            usual_devices: Vec::new(),
            usual_login_times: Vec::new(),
            last_location: None,
            last_login: None,
            successful_logins: 0,
            failed_logins: 0,
        });

        if success {
            profile.successful_logins += 1;

            if let Some(ip) = ip_address {
                if let Some(location) = self.get_location_from_ip(&ip) {
                    profile.last_location = Some(location.clone());

                    // Add to usual locations if not already there
                    if !profile.usual_locations.iter().any(|l| self.distance_km(l, &location) < 50.0) {
                        profile.usual_locations.push(location);
                    }
                }
            }

            if let Some(device) = device_id {
                if !profile.usual_devices.contains(&device) {
                    profile.usual_devices.push(device);
                }
            }

            let current_time = chrono::Utc::now().time();
            profile.usual_login_times.push(current_time);
            profile.last_login = Some(chrono::Utc::now());
        } else {
            profile.failed_logins += 1;
        }
    }
}
