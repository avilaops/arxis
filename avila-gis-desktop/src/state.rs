//! Application State

use iced::Command;
use crate::ui::Message;
use crate::map_view::MapView;
use crate::layer_manager::{LayerManager, LayerType};
use crate::toolbox::Tool;
use crate::attribute_table::{AttributeTable, Feature};
use crate::data_io::{DataExporter, DataImporter};
use chrono::Utc;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tracing::{info, warn};

pub struct AppState {
    // Project info
    project_name: String,
    project_path: Option<String>,
    is_modified: bool,

    // Core components
    pub map_view: MapView,
    pub layer_manager: LayerManager,
    pub active_tool: Tool,

    // UI State
    pub show_layer_panel: bool,
    pub show_attribute_table: bool,
    pub show_symbology_editor: bool,
    pub show_toolbox: bool,

    // Selection
    pub attribute_tables: HashMap<String, AttributeTable>,
    pub active_attribute_table: Option<String>,
    pub last_operation: Option<String>,
    pub selected_features: Vec<String>,
}

impl AppState {
    pub fn new() -> Self {
        let mut map_view = MapView::new();
        map_view.set_size(1280.0, 720.0);

        Self {
            project_name: "Untitled Project".to_string(),
            project_path: None,
            is_modified: false,

            map_view,
            layer_manager: LayerManager::new(),
            active_tool: Tool::Pan,

            show_layer_panel: true,
            show_attribute_table: false,
            show_symbology_editor: false,
            show_toolbox: true,

            attribute_tables: HashMap::new(),
            active_attribute_table: None,
            last_operation: None,
            selected_features: Vec::new(),
        }
    }

    pub fn project_name(&self) -> &str {
        &self.project_name
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NewProject => {
                info!(target: "novo_projeto", "Criando um novo projeto AvilaGIS");
                *self = Self::new();
                Command::none()
            }
            Message::OpenProject => {
                info!(target: "abrir_projeto", "Simulando abertura de projeto");
                self.project_name = "Sample Project".into();
                self.project_path = Some("projects/sample_project.avilagis".into());
                self.is_modified = false;
                self.last_operation = Some("Projeto aberto".into());
                Command::none()
            }
            Message::SaveProject => {
                if self.project_path.is_none() {
                    self.project_path = Some(format!(
                        "projects/{}.avilagis",
                        self.project_name.replace(' ', "_")
                    ));
                }
                info!(?self.project_path, "Projeto salvo");
                self.is_modified = false;
                self.last_operation = Some("Projeto salvo".into());
                Command::none()
            }
            Message::SaveProjectAs => {
                let timestamp = Utc::now().format("%Y%m%d%H%M%S");
                let path = format!(
                    "projects/{}_{}.avilagis",
                    self.project_name.replace(' ', "_"),
                    timestamp
                );
                info!(path = %path, "Salvando projeto como novo arquivo");
                self.project_path = Some(path.clone());
                self.is_modified = false;
                self.last_operation = Some(format!("Salvo como {path}"));
                Command::none()
            }
            Message::Exit => {
                info!(target: "encerrar", "Encerrando AvilaGIS Desktop");
                std::process::exit(0);
            }
            Message::ToggleLayerPanel => {
                self.show_layer_panel = !self.show_layer_panel;
                Command::none()
            }
            Message::ToggleAttributeTable => {
                self.show_attribute_table = !self.show_attribute_table;
                Command::none()
            }
            Message::ToggleSymbologyEditor => {
                self.show_symbology_editor = !self.show_symbology_editor;
                Command::none()
            }
            Message::ToggleToolbox => {
                self.show_toolbox = !self.show_toolbox;
                Command::none()
            }
            Message::SelectTool(tool) => {
                self.active_tool = tool;
                self.last_operation = Some(format!("Ferramenta ativa: {}", tool.name()));
                Command::none()
            }
            Message::ZoomIn => {
                self.map_view.zoom_in();
                self.last_operation = Some("Zoom in".into());
                Command::none()
            }
            Message::ZoomOut => {
                self.map_view.zoom_out();
                self.last_operation = Some("Zoom out".into());
                Command::none()
            }
            Message::ZoomToExtent => {
                self.map_view.zoom_to_extent();
                self.last_operation = Some("Zoom para extensão".into());
                Command::none()
            }
            Message::Pan(dx, dy) => {
                self.map_view.pan(dx, dy);
                self.last_operation = Some(format!("Pan Δx={dx:.1}, Δy={dy:.1}"));
                Command::none()
            }
            Message::AddLayer(layer_type) => {
                let id = self.layer_manager.add_layer(layer_type);
                self.layer_manager.select_layer(&id);
                self.ensure_attribute_table(&id, layer_type);
                self.active_attribute_table = Some(id.clone());
                self.selected_features.clear();
                self.is_modified = true;
                self.last_operation = Some(format!("Camada adicionada: {}", layer_type_label(layer_type)));
                Command::none()
            }
            Message::RemoveLayer(layer_id) => {
                self.layer_manager.remove_layer(&layer_id);
                self.attribute_tables.remove(&layer_id);
                if self.active_attribute_table.as_deref() == Some(&layer_id) {
                    self.active_attribute_table = self
                        .layer_manager
                        .selected_layer()
                        .map(|layer| layer.id.clone());
                }
                let prefix = format!("{layer_id}:");
                self.selected_features.retain(|key| !key.starts_with(&prefix));
                self.is_modified = true;
                self.last_operation = Some(format!("Camada removida: {layer_id}"));
                Command::none()
            }
            Message::ToggleLayerVisibility(layer_id) => {
                self.layer_manager.toggle_visibility(&layer_id);
                self.last_operation = Some(format!("Visibilidade alternada: {layer_id}"));
                Command::none()
            }
            Message::SelectLayer(layer_id) => {
                let layer_info = self
                    .layer_manager
                    .layer(&layer_id)
                    .map(|layer| (layer.layer_type, layer.name.clone()));
                self.layer_manager.select_layer(&layer_id);
                if let Some((layer_type, layer_name)) = layer_info {
                    self.ensure_attribute_table(&layer_id, layer_type);
                    self.last_operation = Some(format!("Camada selecionada: {layer_name}"));
                }
                self.active_attribute_table = Some(layer_id.clone());
                Command::none()
            }
            Message::ImportData(format) => {
                self.handle_import(&format);
                Command::none()
            }
            Message::ExportMap(format) => {
                self.handle_export(&format);
                Command::none()
            }
            Message::UpdateSymbology { layer_id, property, value } => {
                self.layer_manager.update_symbology(&layer_id, property.clone(), value.clone());
                self.is_modified = true;
                self.last_operation = Some(format!("Simbologia atualizada ({property})"));
                Command::none()
            }
            Message::OpenAttributeTable(layer_id) => {
                if let Some((layer_type, layer_name)) = self
                    .layer_manager
                    .layer(&layer_id)
                    .map(|layer| (layer.layer_type, layer.name.clone()))
                {
                    self.ensure_attribute_table(&layer_id, layer_type);
                    self.active_attribute_table = Some(layer_id.clone());
                    self.show_attribute_table = true;
                    self.last_operation = Some(format!("Tabela aberta: {layer_name}"));
                }
                Command::none()
            }
            Message::SelectFeature(feature_id) => {
                if let Some(layer_id) = self.active_attribute_table.clone() {
                    if let Some(table) = self.attribute_tables.get_mut(&layer_id) {
                        if let Some(index) = table
                            .features
                            .iter()
                            .position(|feature| feature.id == feature_id)
                        {
                            let key = format!("{layer_id}:{feature_id}");
                            if table.selected_features.contains(&index) {
                                table.selected_features.retain(|&i| i != index);
                                self.selected_features.retain(|selected| selected != &key);
                                self.last_operation = Some(format!("Feature deselecionada: {feature_id}"));
                            } else {
                                table.select_feature(index);
                                if !self.selected_features.contains(&key) {
                                    self.selected_features.push(key.clone());
                                }
                                self.last_operation = Some(format!("Feature selecionada: {feature_id}"));
                            }
                        }
                    }
                }
                Command::none()
            }
            Message::FilterFeatures(filter) => {
                if let Some(layer_id) = self.active_attribute_table.clone() {
                    if let Some(table) = self.attribute_tables.get_mut(&layer_id) {
                        if filter.trim().is_empty() {
                            table.clear_filter();
                            table.clear_selection();
                            self.selected_features
                                .retain(|selected| !selected.starts_with(&format!("{layer_id}:")));
                            self.last_operation = Some("Filtro removido".into());
                        } else {
                            table.apply_filter(filter.clone());
                            table.clear_selection();
                            self.selected_features
                                .retain(|selected| !selected.starts_with(&format!("{layer_id}:")));
                            self.last_operation = Some(format!("Filtro aplicado: {filter}"));
                        }
                    }
                }
                Command::none()
            }
            Message::Buffer { distance } => {
                self.last_operation = Some(format!("Buffer {distance:.0} m"));
                Command::none()
            }
            Message::Clip => {
                self.last_operation = Some("Clip executado".into());
                Command::none()
            }
            Message::Union => {
                self.last_operation = Some("Union executado".into());
                Command::none()
            }
            Message::Intersect => {
                self.last_operation = Some("Intersect executado".into());
                Command::none()
            }
        }
    }

    fn ensure_attribute_table(&mut self, layer_id: &str, layer_type: LayerType) {
        self.attribute_tables
            .entry(layer_id.to_string())
            .or_insert_with(|| {
                let mut table = AttributeTable::new(layer_id.to_string());
                for feature in Self::sample_features(layer_type) {
                    table.add_feature(feature);
                }
                table
            });
    }

    fn handle_import(&mut self, format: &str) {
        let fmt = format.to_lowercase();
        let (path, result) = match fmt.as_str() {
            "csv" => {
                let path = Path::new("data/sample.csv");
                (path, DataImporter::from_csv(path))
            }
            "geojson" => {
                let path = Path::new("data/sample.geojson");
                (path, DataImporter::from_geojson(path))
            }
            "shp" | "shapefile" => {
                let path = Path::new("data/sample.shp");
                (path, DataImporter::from_shapefile(path))
            }
            _ => {
                let path = Path::new("data/sample.geojson");
                (path, DataImporter::from_geojson(path))
            }
        };

        match result {
            Ok(data) => {
                info!(target: "avila_gis", format = %fmt, path = %path.display(), size = data.len(), "Importação concluída");
                self.last_operation = Some(format!("Importação {fmt} ({}) bytes", data.len()));
                self.is_modified = true;
            }
            Err(err) => {
                warn!(target: "avila_gis", format = %fmt, error = %err, "Falha ao importar dados");
                self.last_operation = Some(format!("Falha ao importar {fmt}"));
            }
        }
    }

    fn handle_export(&mut self, format: &str) {
        let fmt = format.to_lowercase();
        let data: &[u8] = &[];
        if let Err(err) = fs::create_dir_all("dist") {
            warn!(target: "avila_gis", error = %err, "Não foi possível preparar diretório de exportação");
        }

        let (path, result) = match fmt.as_str() {
            "svg" => {
                let path = Path::new("dist/export.svg");
                (path, DataExporter::to_svg(data, path))
            }
            "pdf" => {
                let path = Path::new("dist/export.pdf");
                (path, DataExporter::to_pdf(data, path))
            }
            "png" => {
                let path = Path::new("dist/export.png");
                (path, DataExporter::to_png(data, path))
            }
            "geojson" => {
                let path = Path::new("dist/export.geojson");
                (path, DataExporter::to_geojson(data, path))
            }
            "csv" => {
                let path = Path::new("dist/export.csv");
                (path, DataExporter::to_csv(data, path))
            }
            _ => {
                let path = Path::new("dist/export.svg");
                (path, DataExporter::to_svg(data, path))
            }
        };

        match result {
            Ok(_) => {
                info!(target: "avila_gis", format = %fmt, path = %path.display(), "Exportação concluída");
                self.last_operation = Some(format!("Mapa exportado ({fmt})"));
            }
            Err(err) => {
                warn!(target: "avila_gis", format = %fmt, path = %path.display(), error = %err, "Falha ao exportar mapa");
                self.last_operation = Some(format!("Falha ao exportar {fmt}"));
            }
        }
    }

    fn sample_features(layer_type: LayerType) -> Vec<Feature> {
        match layer_type {
            LayerType::Point => vec![
                Feature {
                    id: "pt-1".into(),
                    geometry_type: "Point".into(),
                    attributes: HashMap::from([
                        ("name".into(), "Padaria Esperança".into()),
                        ("tipo".into(), "Comércio".into()),
                        ("receita".into(), "500000".into()),
                    ]),
                },
                Feature {
                    id: "pt-2".into(),
                    geometry_type: "Point".into(),
                    attributes: HashMap::from([
                        ("name".into(), "Clínica Vida".into()),
                        ("tipo".into(), "Saúde".into()),
                        ("receita".into(), "1200000".into()),
                    ]),
                },
                Feature {
                    id: "pt-3".into(),
                    geometry_type: "Point".into(),
                    attributes: HashMap::from([
                        ("name".into(), "Studio Move".into()),
                        ("tipo".into(), "Serviços".into()),
                        ("receita".into(), "800000".into()),
                    ]),
                },
            ],
            LayerType::Line => vec![
                Feature {
                    id: "ln-1".into(),
                    geometry_type: "LineString".into(),
                    attributes: HashMap::from([
                        ("name".into(), "Av. Central".into()),
                        ("extensao_km".into(), "4.2".into()),
                    ]),
                },
                Feature {
                    id: "ln-2".into(),
                    geometry_type: "LineString".into(),
                    attributes: HashMap::from([
                        ("name".into(), "Marginal Azul".into()),
                        ("extensao_km".into(), "7.8".into()),
                    ]),
                },
            ],
            LayerType::Polygon => vec![
                Feature {
                    id: "pg-1".into(),
                    geometry_type: "Polygon".into(),
                    attributes: HashMap::from([
                        ("name".into(), "Distrito Norte".into()),
                        ("area_km2".into(), "12.5".into()),
                    ]),
                },
                Feature {
                    id: "pg-2".into(),
                    geometry_type: "Polygon".into(),
                    attributes: HashMap::from([
                        ("name".into(), "Zona Industrial".into()),
                        ("area_km2".into(), "8.1".into()),
                    ]),
                },
            ],
            LayerType::Raster => vec![
                Feature {
                    id: "rs-1".into(),
                    geometry_type: "Raster".into(),
                    attributes: HashMap::from([
                        ("name".into(), "Ortofoto 2025".into()),
                        ("resolucao".into(), "0.5m".into()),
                    ]),
                },
            ],
        }
    }
}

fn layer_type_label(layer_type: LayerType) -> &'static str {
    match layer_type {
        LayerType::Point => "Ponto",
        LayerType::Line => "Linha",
        LayerType::Polygon => "Polígono",
        LayerType::Raster => "Raster",
    }
}
