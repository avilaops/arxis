//! Layer Manager

use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerType {
    Point,
    Line,
    Polygon,
    Raster,
}

#[derive(Debug, Clone)]
pub struct Layer {
    pub id: String,
    pub name: String,
    pub layer_type: LayerType,
    pub visible: bool,
    pub opacity: f32,
    pub symbology: Symbology,
}

#[derive(Debug, Clone)]
pub struct Symbology {
    pub fill_color: [u8; 3],
    pub stroke_color: [u8; 3],
    pub stroke_width: f32,
    pub symbol_size: f32,
}

impl Default for Symbology {
    fn default() -> Self {
        Self {
            fill_color: [52, 152, 219],
            stroke_color: [0, 0, 0],
            stroke_width: 1.0,
            symbol_size: 8.0,
        }
    }
}

pub struct LayerManager {
    layers: HashMap<String, Layer>,
    layer_order: Vec<String>,
    selected_layer: Option<String>,
}

impl LayerManager {
    pub fn new() -> Self {
        Self {
            layers: HashMap::new(),
            layer_order: Vec::new(),
            selected_layer: None,
        }
    }

    pub fn add_layer(&mut self, layer_type: LayerType) -> String {
        let id = Uuid::new_v4().to_string();
        let name = format!("New {:?} Layer", layer_type);

        let layer = Layer {
            id: id.clone(),
            name,
            layer_type,
            visible: true,
            opacity: 1.0,
            symbology: Symbology::default(),
        };

        self.layers.insert(id.clone(), layer);
        self.layer_order.push(id.clone());
        id
    }

    pub fn remove_layer(&mut self, layer_id: &str) {
        self.layers.remove(layer_id);
        self.layer_order.retain(|id| id != layer_id);

        if self.selected_layer.as_deref() == Some(layer_id) {
            self.selected_layer = None;
        }
    }

    pub fn toggle_visibility(&mut self, layer_id: &str) {
        if let Some(layer) = self.layers.get_mut(layer_id) {
            layer.visible = !layer.visible;
        }
    }

    pub fn select_layer(&mut self, layer_id: &str) {
        self.selected_layer = Some(layer_id.to_string());
    }

    pub fn update_symbology(&mut self, layer_id: &str, property: String, value: String) {
        if let Some(layer) = self.layers.get_mut(layer_id) {
            // Parse and update symbology based on property
            match property.as_str() {
                "fill_color" => {
                    // TODO: Parse color from value
                }
                "stroke_width" => {
                    if let Ok(width) = value.parse::<f32>() {
                        layer.symbology.stroke_width = width;
                    }
                }
                "symbol_size" => {
                    if let Ok(size) = value.parse::<f32>() {
                        layer.symbology.symbol_size = size;
                    }
                }
                "opacity" => {
                    if let Ok(opacity) = value.parse::<f32>() {
                        layer.opacity = opacity.clamp(0.0, 1.0);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }

    pub fn get_layers(&self) -> Vec<&Layer> {
        self.layer_order
            .iter()
            .filter_map(|id| self.layers.get(id))
            .collect()
    }

    pub fn selected_layer(&self) -> Option<&Layer> {
        self.selected_layer
            .as_ref()
            .and_then(|id| self.layers.get(id))
    }

    pub fn layer(&self, layer_id: &str) -> Option<&Layer> {
        self.layers.get(layer_id)
    }
}
