use axum::{extract::{Path, State}, Json};
use serde::{Deserialize, Serialize};

use crate::{
    api::{error::ApiError, response::ApiResponse, state::AppState, ApiResult},
    models::UserProfile,
    prediction::BehaviorPredictor,
    segmentation::UserSegmentation,
    storage::QueryOptions,
};

/// Resposta de perfil de usuário
#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub user_id: String,
    pub engagement_score: f64,
    pub loyalty_score: f64,
    pub churn_risk: f64,
    pub conversion_probability: f64,
    pub total_sessions: usize,
    pub total_events: usize,
    pub total_spent: f64,
    pub segments: Vec<String>,
}

/// Resposta de segmento
#[derive(Debug, Serialize)]
pub struct UserSegmentResponse {
    pub user_id: String,
    pub segment: String,
    pub segment_description: String,
    pub confidence: f64,
}

/// Resposta de predições
#[derive(Debug, Serialize)]
pub struct UserPredictionsResponse {
    pub user_id: String,
    pub churn_probability: f64,
    pub conversion_probability: f64,
    pub predicted_ltv: f64,
    pub recommended_products: Vec<ProductRecommendation>,
}

#[derive(Debug, Serialize)]
pub struct ProductRecommendation {
    pub product_id: String,
    pub score: f64,
    pub reason: String,
}

/// Resposta de lista de segmentos
#[derive(Debug, Serialize)]
pub struct SegmentsListResponse {
    pub segments: Vec<SegmentInfo>,
    pub total_users_analyzed: usize,
}

#[derive(Debug, Serialize)]
pub struct SegmentInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub user_count: usize,
    pub percentage: f64,
}

/// GET /api/v1/users/:user_id
/// Obter perfil completo do usuário
pub async fn get_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiResult<Json<ApiResponse<UserProfileResponse>>> {
    // Buscar eventos do usuário
    let events = state
        .event_store
        .get_by_user(&user_id, QueryOptions::default())
        .await?;

    if events.is_empty() {
        return Err(ApiError::NotFound(format!("User {} not found", user_id)));
    }

    // Gerar perfil
    let profiles = crate::generate_user_profiles(&events);
    let profile = profiles
        .into_iter()
        .find(|p| p.user_id == user_id)
        .ok_or_else(|| ApiError::NotFound(format!("Profile for user {} not found", user_id)))?;

    let response = UserProfileResponse {
        user_id: profile.user_id,
        engagement_score: profile.engagement_score,
        loyalty_score: profile.loyalty_score,
        churn_risk: profile.churn_risk,
        conversion_probability: profile.conversion_probability,
        total_sessions: profile.total_sessions,
        total_events: profile.total_events,
        total_spent: profile.behaviors.total_spent,
        segments: profile.segments,
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /api/v1/users/:user_id/segment
/// Obter segmento do usuário
pub async fn get_user_segment(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiResult<Json<ApiResponse<UserSegmentResponse>>> {
    // Buscar eventos e gerar perfil
    let events = state
        .event_store
        .get_by_user(&user_id, QueryOptions::default())
        .await?;

    if events.is_empty() {
        return Err(ApiError::NotFound(format!("User {} not found", user_id)));
    }

    let profiles = crate::generate_user_profiles(&events);
    let profile = profiles
        .into_iter()
        .find(|p| p.user_id == user_id)
        .ok_or_else(|| ApiError::NotFound(format!("Profile for user {} not found", user_id)))?;

    // Determinar segmento
    let segmentation = UserSegmentation::with_default_segments();
    let segment = determine_primary_segment(&profile);

    let response = UserSegmentResponse {
        user_id: profile.user_id,
        segment: segment.clone(),
        segment_description: get_segment_description(&segment),
        confidence: 0.85, // TODO: Calcular confiança real
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /api/v1/users/:user_id/predictions
/// Obter predições para o usuário
pub async fn get_user_predictions(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiResult<Json<ApiResponse<UserPredictionsResponse>>> {
    // Buscar eventos do usuário
    let events = state
        .event_store
        .get_by_user(&user_id, QueryOptions::default())
        .await?;

    if events.is_empty() {
        return Err(ApiError::NotFound(format!("User {} not found", user_id)));
    }

    let profiles = crate::generate_user_profiles(&events);
    let profile = profiles
        .into_iter()
        .find(|p| p.user_id == user_id)
        .ok_or_else(|| ApiError::NotFound(format!("Profile for user {} not found", user_id)))?;

    // Fazer predições
    let mut predictor = BehaviorPredictor::new();

    // Treinar modelo com todos os eventos (em produção, usar modelo pré-treinado)
    let all_events = state.event_store.query(crate::storage::EventFilter::default()).await?;
    predictor.train_recommendation_model(&all_events);

    let churn_score = predictor.predict_churn(&profile);
    let conversion_score = predictor.predict_conversion(&profile);
    let recommendations = predictor.recommend_products(&profile.user_id, 5);

    // Calcular LTV previsto
    let predicted_ltv = calculate_predicted_ltv(&profile);

    let recommended_products: Vec<ProductRecommendation> = recommendations
        .into_iter()
        .map(|(product_id, score)| ProductRecommendation {
            product_id: product_id.clone(),
            score,
            reason: format!("Based on your browsing history and similar users"),
        })
        .collect();

    let response = UserPredictionsResponse {
        user_id: profile.user_id,
        churn_probability: churn_score,
        conversion_probability: conversion_score,
        predicted_ltv,
        recommended_products,
    };

    Ok(Json(ApiResponse::ok(response)))
}

/// GET /api/v1/users/segments
/// Listar todos os segmentos com estatísticas
pub async fn list_segments(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<SegmentsListResponse>>> {
    // Buscar todos os eventos e gerar perfis
    let events = state.event_store.query(crate::storage::EventFilter::default()).await?;
    let profiles = crate::generate_user_profiles(&events);

    let total_users = profiles.len();

    // Contar usuários por segmento
    let mut segment_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for profile in &profiles {
        let segment = determine_primary_segment(profile);
        *segment_counts.entry(segment).or_insert(0) += 1;
    }

    // Construir resposta
    let segments: Vec<SegmentInfo> = segment_counts
        .into_iter()
        .map(|(segment, count)| {
            let percentage = if total_users > 0 {
                (count as f64 / total_users as f64) * 100.0
            } else {
                0.0
            };

            SegmentInfo {
                id: segment.clone(),
                name: segment.clone(),
                description: get_segment_description(&segment),
                user_count: count,
                percentage,
            }
        })
        .collect();

    let response = SegmentsListResponse {
        segments,
        total_users_analyzed: total_users,
    };

    Ok(Json(ApiResponse::ok(response)))
}

// ========== Funções auxiliares ==========

fn determine_primary_segment(profile: &UserProfile) -> String {
    // Lógica de segmentação RFM simplificada
    let recency_score = if let Some(days) = profile.behaviors.days_since_last_purchase {
        if days <= 30 {
            3
        } else if days <= 90 {
            2
        } else {
            1
        }
    } else {
        0
    };

    let frequency_score = if profile.behaviors.total_purchases >= 5 {
        3
    } else if profile.behaviors.total_purchases >= 2 {
        2
    } else if profile.behaviors.total_purchases >= 1 {
        1
    } else {
        0
    };

    let monetary_score = if profile.behaviors.total_spent >= 1000.0 {
        3
    } else if profile.behaviors.total_spent >= 500.0 {
        2
    } else if profile.behaviors.total_spent > 0.0 {
        1
    } else {
        0
    };

    // Determinar segmento baseado nos scores
    match (recency_score, frequency_score, monetary_score) {
        (3, 3, 3) => "Champions".to_string(),
        (3, 2..=3, 2..=3) | (2, 3, 2..=3) => "Loyal Customers".to_string(),
        (3, _, _) => "Recent Customers".to_string(),
        (2, 2..=3, _) => "Potential Loyalists".to_string(),
        (2, _, _) => "Promising".to_string(),
        (1, 2..=3, 2..=3) => "At Risk".to_string(),
        (1, 2..=3, _) => "Need Attention".to_string(),
        (1, 1, _) => "Hibernating".to_string(),
        (0, _, _) => "New Users".to_string(),
        _ => "Other".to_string(),
    }
}

fn get_segment_description(segment: &str) -> String {
    match segment {
        "Champions" => "Best customers with high recency, frequency, and monetary value".to_string(),
        "Loyal Customers" => "Consistent purchasers with good monetary value".to_string(),
        "Recent Customers" => "Recent purchasers who show potential".to_string(),
        "Potential Loyalists" => "Recent customers with above-average frequency".to_string(),
        "Promising" => "Recent shoppers with average frequency".to_string(),
        "At Risk" => "Previously good customers who haven't purchased recently".to_string(),
        "Need Attention" => "Below average recency, frequency, and monetary value".to_string(),
        "Hibernating" => "Haven't purchased in a long time".to_string(),
        "New Users" => "Brand new users with no purchase history".to_string(),
        _ => "Other segment".to_string(),
    }
}

fn calculate_predicted_ltv(profile: &UserProfile) -> f64 {
    // Cálculo simplificado de LTV previsto
    let avg_purchase_value = profile.behaviors.avg_order_value;
    let purchase_frequency = profile.behaviors.total_purchases as f64 / 365.0; // por dia
    let customer_lifespan_days = 730.0; // 2 anos

    avg_purchase_value * purchase_frequency * customer_lifespan_days
}
