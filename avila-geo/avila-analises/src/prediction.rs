use crate::models::*;
use std::collections::HashMap;
use tracing::info;

// ==================== Behavior Predictor ====================

pub struct BehaviorPredictor {
    churn_model: ChurnModel,
    conversion_model: ConversionModel,
    recommendation_model: RecommendationModel,
}

impl BehaviorPredictor {
    pub fn new() -> Self {
        Self {
            churn_model: ChurnModel::new(),
            conversion_model: ConversionModel::new(),
            recommendation_model: RecommendationModel::new(),
        }
    }

    pub fn predict_churn(&self, profile: &UserProfile) -> f64 {
        self.churn_model.predict(profile)
    }

    pub fn predict_conversion(&self, profile: &UserProfile) -> f64 {
        self.conversion_model.predict(profile)
    }

    pub fn recommend_products(&self, user_id: &str, n: usize) -> Vec<(String, f64)> {
        self.recommendation_model.recommend(user_id, n)
    }

    pub fn train_recommendation_model(&mut self, events: &[BehaviorEvent]) {
        self.recommendation_model.train(events);
    }
}

// ==================== Churn Prediction Model ====================

pub struct ChurnModel {
    weights: Vec<f64>,
    bias: f64,
}

impl ChurnModel {
    pub fn new() -> Self {
        // Pesos pré-treinados (simplificado)
        Self {
            weights: vec![
                0.05,  // days_since_last_purchase
                -0.02, // avg_session_duration
                -0.01, // total_sessions
                -0.03, // total_purchases
                0.15,  // bounce_rate
                -0.08, // engagement_score
            ],
            bias: 0.5,
        }
    }

    pub fn predict(&self, profile: &UserProfile) -> f64 {
        let features = self.extract_features(profile);

        let score: f64 = features
            .iter()
            .zip(self.weights.iter())
            .map(|(f, w)| f * w)
            .sum::<f64>()
            + self.bias;

        // Sigmoid para normalizar entre 0-1
        self.sigmoid(score)
    }

    fn extract_features(&self, profile: &UserProfile) -> Vec<f64> {
        vec![
            profile
                .behaviors
                .days_since_last_purchase
                .unwrap_or(999) as f64,
            profile.behaviors.avg_session_duration_seconds / 60.0, // Normalizar para minutos
            (profile.total_sessions as f64).ln_1p(),               // Log transform
            (profile.behaviors.total_purchases as f64).ln_1p(),
            profile.behaviors.bounce_rate,
            profile.engagement_score,
        ]
    }

    fn sigmoid(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    /// Treinar modelo com dados históricos (simplificado)
    pub fn train(&mut self, profiles: &[UserProfile], labels: &[f64]) {
        if profiles.is_empty() || profiles.len() != labels.len() {
            return;
        }

        let learning_rate = 0.01;
        let epochs = 100;

        for _ in 0..epochs {
            let mut weight_gradients = vec![0.0; self.weights.len()];
            let mut bias_gradient = 0.0;

            for (profile, &label) in profiles.iter().zip(labels.iter()) {
                let prediction = self.predict(profile);
                let error = prediction - label;

                let features = self.extract_features(profile);
                for (i, &feature) in features.iter().enumerate() {
                    weight_gradients[i] += error * feature;
                }
                bias_gradient += error;
            }

            // Atualizar pesos
            let n = profiles.len() as f64;
            for (i, weight) in self.weights.iter_mut().enumerate() {
                *weight -= learning_rate * (weight_gradients[i] / n);
            }
            self.bias -= learning_rate * (bias_gradient / n);
        }

        info!("Churn model trained with {} samples", profiles.len());
    }
}

// ==================== Conversion Prediction Model ====================

pub struct ConversionModel {
    weights: Vec<f64>,
    bias: f64,
}

impl ConversionModel {
    pub fn new() -> Self {
        Self {
            weights: vec![
                0.08,  // engagement_score
                0.06,  // avg_session_duration
                0.04,  // pages_per_session
                -0.10, // bounce_rate
                0.05,  // search_queries_count
                0.03,  // clicked_products_count
            ],
            bias: -0.3,
        }
    }

    pub fn predict(&self, profile: &UserProfile) -> f64 {
        let features = self.extract_features(profile);

        let score: f64 = features
            .iter()
            .zip(self.weights.iter())
            .map(|(f, w)| f * w)
            .sum::<f64>()
            + self.bias;

        self.sigmoid(score)
    }

    fn extract_features(&self, profile: &UserProfile) -> Vec<f64> {
        vec![
            profile.engagement_score,
            profile.behaviors.avg_session_duration_seconds / 60.0,
            profile.behaviors.avg_pages_per_session,
            profile.behaviors.bounce_rate,
            (profile.behaviors.search_queries.len() as f64).ln_1p(),
            (profile.behaviors.clicked_products.len() as f64).ln_1p(),
        ]
    }

    fn sigmoid(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }
}

// ==================== Recommendation Model ====================

pub struct RecommendationModel {
    user_item_matrix: HashMap<String, HashMap<String, f64>>,
    item_popularity: HashMap<String, usize>,
}

impl RecommendationModel {
    pub fn new() -> Self {
        Self {
            user_item_matrix: HashMap::new(),
            item_popularity: HashMap::new(),
        }
    }

    /// Treinar modelo com eventos históricos
    pub fn train(&mut self, events: &[BehaviorEvent]) {
        self.user_item_matrix.clear();
        self.item_popularity.clear();

        for event in events {
            let item_id = match &event.event_type {
                EventType::PageView { url, .. } => Some(url.clone()),
                EventType::Purchase { product_id, .. } => Some(product_id.clone()),
                EventType::AddToCart { product_id, .. } => Some(product_id.clone()),
                EventType::Click { element_id, .. } => Some(element_id.clone()),
                _ => None,
            };

            if let Some(item) = item_id {
                // Calcular score baseado no tipo de interação
                let score = match &event.event_type {
                    EventType::Purchase { .. } => 5.0,
                    EventType::AddToCart { .. } => 3.0,
                    EventType::PageView { duration_ms, .. } => {
                        // Score baseado no tempo de visualização
                        (*duration_ms as f64 / 10000.0).min(2.0)
                    }
                    EventType::Click { .. } => 1.0,
                    _ => 0.5,
                };

                // Atualizar matriz usuário-item
                self.user_item_matrix
                    .entry(event.user_id.clone())
                    .or_insert_with(HashMap::new)
                    .entry(item.clone())
                    .and_modify(|s| *s += score)
                    .or_insert(score);

                // Atualizar popularidade
                *self.item_popularity.entry(item).or_insert(0) += 1;
            }
        }

        info!(
            "Recommendation model trained with {} users and {} items",
            self.user_item_matrix.len(),
            self.item_popularity.len()
        );
    }

    /// Recomendar itens para um usuário
    pub fn recommend(&self, user_id: &str, n: usize) -> Vec<(String, f64)> {
        let mut scores: HashMap<String, f64> = HashMap::new();

        // Encontrar usuários similares
        let similar_users = self.find_similar_users(user_id, 10);

        if similar_users.is_empty() {
            // Fallback: retornar itens mais populares
            return self.get_popular_items(n);
        }

        // Agregar recomendações de usuários similares
        for (similar_user, similarity) in similar_users {
            if let Some(items) = self.user_item_matrix.get(&similar_user) {
                for (item, score) in items {
                    // Não recomendar itens que o usuário já interagiu
                    if let Some(user_items) = self.user_item_matrix.get(user_id) {
                        if user_items.contains_key(item) {
                            continue;
                        }
                    }

                    *scores.entry(item.clone()).or_insert(0.0) += score * similarity;
                }
            }
        }

        // Ordenar e retornar top N
        let mut recommendations: Vec<_> = scores.into_iter().collect();
        recommendations.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        recommendations.truncate(n);

        recommendations
    }

    /// Encontrar usuários similares usando similaridade de cosseno
    fn find_similar_users(&self, user_id: &str, k: usize) -> Vec<(String, f64)> {
        let mut similarities = Vec::new();

        if let Some(user_vector) = self.user_item_matrix.get(user_id) {
            for (other_user, other_vector) in &self.user_item_matrix {
                if other_user != user_id {
                    let similarity = self.cosine_similarity(user_vector, other_vector);
                    if similarity > 0.0 {
                        similarities.push((other_user.clone(), similarity));
                    }
                }
            }
        }

        similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similarities.truncate(k);
        similarities
    }

    /// Calcular similaridade de cosseno entre dois vetores
    fn cosine_similarity(
        &self,
        a: &HashMap<String, f64>,
        b: &HashMap<String, f64>,
    ) -> f64 {
        let dot_product: f64 = a
            .iter()
            .filter_map(|(k, v)| b.get(k).map(|bv| v * bv))
            .sum();

        let norm_a: f64 = a.values().map(|v| v * v).sum::<f64>().sqrt();
        let norm_b: f64 = b.values().map(|v| v * v).sum::<f64>().sqrt();

        if norm_a == 0.0 || norm_b == 0.0 {
            0.0
        } else {
            dot_product / (norm_a * norm_b)
        }
    }

    /// Retornar itens mais populares
    fn get_popular_items(&self, n: usize) -> Vec<(String, f64)> {
        let mut items: Vec<_> = self
            .item_popularity
            .iter()
            .map(|(item, count)| (item.clone(), *count as f64))
            .collect();

        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        items.truncate(n);
        items
    }

    /// Calcular precisão das recomendações
    pub fn evaluate_precision(
        &self,
        test_events: &[BehaviorEvent],
        k: usize,
    ) -> f64 {
        let mut total_precision = 0.0;
        let mut count = 0;

        // Agrupar eventos por usuário
        let mut user_test_items: HashMap<String, Vec<String>> = HashMap::new();
        for event in test_events {
            if let EventType::Purchase { product_id, .. } = &event.event_type {
                user_test_items
                    .entry(event.user_id.clone())
                    .or_insert_with(Vec::new)
                    .push(product_id.clone());
            }
        }

        for (user_id, actual_items) in user_test_items {
            let recommendations = self.recommend(&user_id, k);
            let recommended_items: Vec<_> = recommendations.iter().map(|(item, _)| item).collect();

            let hits = actual_items
                .iter()
                .filter(|item| recommended_items.contains(&item))
                .count();

            total_precision += hits as f64 / k as f64;
            count += 1;
        }

        if count > 0 {
            total_precision / count as f64
        } else {
            0.0
        }
    }
}

// ==================== Next Purchase Predictor ====================

pub struct NextPurchasePredictor;

impl NextPurchasePredictor {
    /// Prever quando será a próxima compra (em dias)
    pub fn predict_next_purchase(profile: &UserProfile) -> Option<i64> {
        if profile.behaviors.total_purchases < 2 {
            return None;
        }

        // Calcular média de dias entre compras
        let avg_days_between = if let Some(days_since_last) = profile.behaviors.days_since_last_purchase {
            // Estimativa simples baseada no histórico
            days_since_last / 2
        } else {
            return None;
        };

        Some(avg_days_between)
    }

    /// Calcular probabilidade de compra nos próximos N dias
    pub fn purchase_probability(profile: &UserProfile, days: i64) -> f64 {
        if let Some(expected_days) = Self::predict_next_purchase(profile) {
            // Usar distribuição exponencial
            let lambda = 1.0 / expected_days as f64;
            1.0 - (-lambda * days as f64).exp()
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn create_test_profile() -> UserProfile {
        UserProfile {
            user_id: "test_user".to_string(),
            first_seen: chrono::Utc::now() - chrono::Duration::days(100),
            last_seen: chrono::Utc::now(),
            total_sessions: 20,
            total_events: 200,
            behaviors: UserBehaviors {
                avg_session_duration_seconds: 300.0,
                avg_pages_per_session: 5.0,
                bounce_rate: 0.3,
                conversion_rate: 0.1,
                most_active_hours: vec![],
                most_active_days: vec![],
                total_purchases: 5,
                total_spent: 1000.0,
                avg_order_value: 200.0,
                days_since_last_purchase: Some(30),
                pages_viewed: HashSet::new(),
                search_queries: vec!["laptop".to_string()],
                clicked_products: vec!["prod1".to_string()],
            },
            segments: vec![],
            engagement_score: 0.7,
            loyalty_score: 0.6,
            conversion_probability: 0.5,
            churn_risk: 0.4,
            interests: vec![],
            preferred_categories: HashMap::new(),
            browsing_patterns: BrowsingPatterns::default(),
        }
    }

    #[test]
    fn test_churn_prediction() {
        let model = ChurnModel::new();
        let profile = create_test_profile();
        let churn_score = model.predict(&profile);

        assert!(churn_score >= 0.0 && churn_score <= 1.0);
    }

    #[test]
    fn test_conversion_prediction() {
        let model = ConversionModel::new();
        let profile = create_test_profile();
        let conversion_score = model.predict(&profile);

        assert!(conversion_score >= 0.0 && conversion_score <= 1.0);
    }

    #[test]
    fn test_recommendation_model() {
        let mut model = RecommendationModel::new();

        let events = vec![
            BehaviorEvent {
                event_id: "1".to_string(),
                user_id: "user1".to_string(),
                session_id: "session1".to_string(),
                timestamp: chrono::Utc::now(),
                event_type: EventType::Purchase {
                    product_id: "prod1".to_string(),
                    amount: 100.0,
                    currency: "BRL".to_string(),
                },
                metadata: HashMap::new(),
                context: create_test_context(),
            },
        ];

        model.train(&events);
        let recommendations = model.recommend("user2", 5);

        assert!(recommendations.len() <= 5);
    }

    fn create_test_context() -> EventContext {
        EventContext {
            device: DeviceInfo {
                device_type: DeviceType::Desktop,
                os: "Windows".to_string(),
                browser: "Chrome".to_string(),
                screen_resolution: (1920, 1080),
            },
            location: LocationInfo {
                country: "BR".to_string(),
                city: Some("São Paulo".to_string()),
                timezone: "America/Sao_Paulo".to_string(),
                ip_address: "192.168.1.1".to_string(),
            },
            referrer: None,
            user_agent: "Mozilla/5.0".to_string(),
            viewport: Viewport {
                width: 1920,
                height: 1080,
            },
        }
    }
}
