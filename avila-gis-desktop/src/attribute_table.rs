//! Attribute Table

use std::collections::HashMap;

pub struct AttributeTable {
    pub layer_id: String,
    pub features: Vec<Feature>,
    pub selected_features: Vec<usize>,
    pub filter: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Feature {
    pub id: String,
    pub geometry_type: String,
    pub attributes: HashMap<String, String>,
}

impl AttributeTable {
    pub fn new(layer_id: String) -> Self {
        Self {
            layer_id,
            features: Vec::new(),
            selected_features: Vec::new(),
            filter: None,
        }
    }

    pub fn add_feature(&mut self, feature: Feature) {
        self.features.push(feature);
    }

    pub fn select_feature(&mut self, index: usize) {
        if !self.selected_features.contains(&index) {
            self.selected_features.push(index);
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_features.clear();
    }

    pub fn apply_filter(&mut self, filter: String) {
        self.filter = Some(filter);
    }

    pub fn clear_filter(&mut self) {
        self.filter = None;
    }

    pub fn get_filtered_features(&self) -> Vec<(usize, &Feature)> {
        let normalized = self
            .filter
            .as_ref()
            .map(|filter| filter.trim().to_lowercase())
            .filter(|filter| !filter.is_empty());

        match normalized {
            Some(filter) => self
                .features
                .iter()
                .enumerate()
                .filter(|(_, feature)| feature.matches(&filter))
                .collect(),
            None => self.features.iter().enumerate().collect(),
        }
    }

    pub fn feature_count(&self) -> usize {
        self.features.len()
    }

    pub fn selected_count(&self) -> usize {
        self.selected_features.len()
    }
}

impl Feature {
    fn matches(&self, filter: &str) -> bool {
        if self.id.to_lowercase().contains(filter) {
            return true;
        }
        if self.geometry_type.to_lowercase().contains(filter) {
            return true;
        }

        self.attributes.iter().any(|(key, value)| {
            key.to_lowercase().contains(filter) || value.to_lowercase().contains(filter)
        })
    }
}
