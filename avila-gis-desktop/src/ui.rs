//! Main UI Layout

use crate::attribute_table::Feature;
use crate::layer_manager::{Layer, LayerType};
use crate::state::AppState;
use crate::symbology_editor::SymbologyEditor;
use crate::toolbox::{Tool, ToolCategory};
use iced::{
    alignment::Horizontal,
    border::Radius,
    theme,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, scrollable, text,
        text_input, vertical_rule, Column, Row, Space,
    },
    Alignment, Background, Border, Color, Element, Length, Shadow, Theme, Vector,
};

type UiElement<'a> = Element<'a, Message>;
type MaybeUiElement<'a> = Option<Element<'a, Message>>;

#[derive(Debug, Clone)]
pub enum Message {
    // File Menu
    NewProject,
    OpenProject,
    SaveProject,
    SaveProjectAs,
    Exit,

    // View Menu
    ToggleLayerPanel,
    ToggleAttributeTable,
    ToggleSymbologyEditor,
    ToggleToolbox,

    // Tools
    SelectTool(Tool),

    // Map Navigation
    ZoomIn,
    ZoomOut,
    ZoomToExtent,
    Pan(f32, f32),

    // Layer Operations
    AddLayer(LayerType),
    RemoveLayer(String),
    ToggleLayerVisibility(String),
    SelectLayer(String),

    // Data I/O
    ImportData(String),
    ExportMap(String),

    // Symbology
    UpdateSymbology {
        layer_id: String,
        property: String,
        value: String,
    },

    // Attribute Table
    OpenAttributeTable(String),
    SelectFeature(String),
    FilterFeatures(String),

    // Geoprocessing
    Buffer { distance: f64 },
    Clip,
    Union,
    Intersect,
}

pub fn view(state: &AppState) -> UiElement<'_> {
    let chrome = column![
        header(state),
        column![
            menu_bar(state),
            toolbar(state),
        ]
        .spacing(8),
        main_area(state),
        status_bar(state),
    ]
    .spacing(16)
    .height(Length::Fill);

    container(chrome)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding([16, 20, 20, 20])
        .into()
}

fn header(state: &AppState) -> UiElement<'_> {
    let (center_x, center_y) = state.map_view.center();
    let (width, height) = state.map_view.size();

    let header_row = row![
        column![
            text("AvilaGIS Desktop").size(28),
            text(format!("Projeto ativo: {}", state.project_name()))
                .size(16)
                .style(secondary_text_style()),
        ]
        .spacing(4),
        horizontal_space(),
        row![
            info_badge("Camadas", state.layer_manager.layer_count().to_string()),
            info_badge("Ferramenta", state.active_tool.name().to_string()),
            info_badge("Centro", format!("{:.2}, {:.2}", center_x, center_y)),
            info_badge("Canvas", format!("{:.0}×{:.0}", width, height)),
        ]
        .spacing(12),
    ]
    .align_items(Alignment::Center);

    container(header_row)
        .padding(16)
        .style(PanelStyle)
        .into()
}

fn menu_bar(state: &AppState) -> UiElement<'_> {
    let menu_row = row![
        quick_action("Novo Projeto", Message::NewProject),
        quick_action("Abrir", Message::OpenProject),
        quick_action("Salvar", Message::SaveProject),
        quick_action("Salvar como", Message::SaveProjectAs),
        Space::with_width(Length::Fixed(20.0)),
        text("Painéis").size(13).style(secondary_text_style()),
        menu_toggle_button("Camadas", state.show_layer_panel, Message::ToggleLayerPanel),
        menu_toggle_button(
            "Tabela",
            state.show_attribute_table,
            Message::ToggleAttributeTable,
        ),
        menu_toggle_button(
            "Simbologia",
            state.show_symbology_editor,
            Message::ToggleSymbologyEditor,
        ),
        menu_toggle_button("Toolbox", state.show_toolbox, Message::ToggleToolbox),
        horizontal_space(),
        quick_action("Importar CSV", Message::ImportData("csv".into())),
        quick_action("Exportar PDF", Message::ExportMap("pdf".into())),
        quick_action("Sair", Message::Exit),
        Space::with_width(Length::Fixed(12.0)),
        text("Ctrl+Shift+G")
            .size(12)
            .style(secondary_text_style()),
    ]
    .spacing(10)
    .align_items(Alignment::Center);

    container(menu_row)
        .padding([10, 16])
        .style(PanelStyle)
        .into()
}

fn toolbar(state: &AppState) -> UiElement<'_> {
    let active_tool = state.active_tool;

    let mut toolbar = Row::new()
        .spacing(8)
        .align_items(Alignment::Center);

    for action in [
        tool_button_optional("🆕", "Novo", false, Some(Message::NewProject)),
        tool_button_optional("📁", "Abrir", false, Some(Message::OpenProject)),
        tool_button_optional("💾", "Salvar", false, Some(Message::SaveProject)),
    ] {
        toolbar = toolbar.push(action);
    }

    toolbar = toolbar.push(Space::with_width(Length::Fixed(18.0)));

    for action in [
        tool_button_optional("🔍+", "Zoom +", false, Some(Message::ZoomIn)),
        tool_button_optional("🔍-", "Zoom -", false, Some(Message::ZoomOut)),
        tool_button_optional("🌍", "Extenso", false, Some(Message::ZoomToExtent)),
        tool_button_optional(
            "✋",
            "Pan",
            active_tool == Tool::Pan,
            Some(Message::SelectTool(Tool::Pan)),
        ),
        tool_button_optional(
            "🖱️",
            "Seleção",
            active_tool == Tool::Select,
            Some(Message::SelectTool(Tool::Select)),
        ),
        tool_button_optional(
            "📏",
            "Medir",
            active_tool == Tool::Measure,
            Some(Message::SelectTool(Tool::Measure)),
        ),
        tool_button_optional(
            "ℹ️",
            "Identificar",
            active_tool == Tool::Identify,
            Some(Message::SelectTool(Tool::Identify)),
        ),
    ] {
        toolbar = toolbar.push(action);
    }

    toolbar = toolbar.push(Space::with_width(Length::Fixed(18.0)));

    toolbar = toolbar.push(tool_button_optional(
        "➕",
        "Nova Camada",
        false,
        Some(Message::AddLayer(LayerType::Point)),
    ));

    let remove_message = state
        .layer_manager
        .selected_layer()
        .map(|layer| Message::RemoveLayer(layer.id.clone()));

    toolbar = toolbar.push(tool_button_optional("🗑️", "Remover", false, remove_message));

    container(toolbar)
        .padding([12, 16])
        .style(PanelStyle)
        .into()
}

fn main_area(state: &AppState) -> UiElement<'_> {
    let mut sections = Vec::new();

    if let Some(left) = left_sidebar(state) {
        sections.push(left);
    }

    sections.push(map_panel(state));

    if let Some(right) = right_sidebar(state) {
        sections.push(right);
    }

    Row::with_children(sections)
        .spacing(16)
        .height(Length::Fill)
        .into()
}

fn left_sidebar(state: &AppState) -> MaybeUiElement<'_> {
    if !state.show_layer_panel && !state.show_toolbox {
        return None;
    }

    let mut stack = Column::new().spacing(12).height(Length::Fill);

    if state.show_layer_panel {
        let height = if state.show_toolbox { 3 } else { 5 };
        let panel_view: UiElement<'_> = panel("Camadas", layer_list(state))
            .height(Length::FillPortion(height as u16))
            .into();
        stack = stack.push(panel_view);
    }

    if state.show_toolbox {
        let height = if state.show_layer_panel { 2 } else { 5 };
        let panel_view: UiElement<'_> = panel("Toolbox", toolbox_content(state))
            .height(Length::FillPortion(height as u16))
            .into();
        stack = stack.push(panel_view);
    }

    Some(
        container(stack)
            .width(Length::Fixed(320.0))
            .height(Length::Fill)
            .into(),
    )
}

fn right_sidebar(state: &AppState) -> MaybeUiElement<'_> {
    if !state.show_symbology_editor && !state.show_attribute_table {
        return None;
    }

    let mut stack = Column::new().spacing(12).height(Length::Fill);

    if state.show_symbology_editor {
        let height = if state.show_attribute_table { 3 } else { 5 };
        let panel_view: UiElement<'_> = panel(
            "Simbologia",
            SymbologyEditor::view(state.layer_manager.selected_layer()),
        )
        .height(Length::FillPortion(height as u16))
        .into();
        stack = stack.push(panel_view);
    }

    if state.show_attribute_table {
        let height = if state.show_symbology_editor { 2 } else { 5 };
        let panel_view: UiElement<'_> = panel("Tabela de Atributos", attribute_table_view(state))
            .height(Length::FillPortion(height as u16))
            .into();
        stack = stack.push(panel_view);
    }

    Some(
        container(stack)
            .width(Length::Fixed(340.0))
            .height(Length::Fill)
            .into(),
    )
}

fn map_panel(state: &AppState) -> UiElement<'_> {
    let (center_x, center_y) = state.map_view.center();

    let header = row![
        text("Visualização de Mapa").size(16),
        horizontal_space(),
        text(format!("Zoom: {}", state.map_view.zoom_level()))
            .size(12)
            .style(secondary_text_style()),
        vertical_rule(1),
        text(format!("Centro: {:.2}, {:.2}", center_x, center_y))
            .size(12)
            .style(secondary_text_style()),
        vertical_rule(1),
        text(format!("Ferramenta: {}", state.active_tool.name()))
            .size(12)
            .style(secondary_text_style()),
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    let canvas: UiElement<'_> = container(
        column![
            text("🗺️ Renderização cartográfica em breve").size(20),
            text("Adicione camadas para visualizar dados geoespaciais.")
                .size(14)
                .style(secondary_text_style()),
        ]
        .spacing(8)
        .align_items(Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x()
    .center_y()
    .style(theme::Container::Transparent)
    .into();

    panel(
        "Mapa",
        column![
            header,
            map_navigation_controls(),
            canvas,
        ]
        .spacing(16)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn status_bar(state: &AppState) -> UiElement<'_> {
    let (center_x, center_y) = state.map_view.center();

    let status_row = row![
        status_item("Camadas", state.layer_manager.layer_count().to_string()),
        vertical_rule(1),
        status_item("Ferramenta", state.active_tool.name().to_string()),
        vertical_rule(1),
        status_item("Centro", format!("{:.2}, {:.2}", center_x, center_y)),
        horizontal_space(),
        text("© 2025 Avila Research Labs")
            .size(12)
            .style(secondary_text_style()),
    ]
    .spacing(12)
    .align_items(Alignment::Center);

    container(status_row)
        .padding([8, 16])
        .style(PanelStyle)
        .into()
}

fn layer_list(state: &AppState) -> UiElement<'_> {
    let layers = state.layer_manager.get_layers();
    let selected_id = state
        .layer_manager
        .selected_layer()
        .map(|layer| layer.id.clone());

    let mut list = Column::new().spacing(8).width(Length::Fill);

    list = list.push(layer_creation_controls());

    if layers.is_empty() {
        list = list.push(
            container(
                column![
                    text("Nenhuma camada carregada.").size(14),
                    text("Use \"Nova Camada\" para iniciar.")
                        .size(12)
                        .style(secondary_text_style()),
                ]
                .spacing(4)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .padding(16)
            .style(theme::Container::Transparent),
        );
    } else {
        for layer in layers {
            let is_selected = selected_id
                .as_ref()
                .map(|id| id == &layer.id)
                .unwrap_or(false);
            list = list.push(layer_entry(layer, is_selected));
        }
    }

    scrollable(list).height(Length::Fill).into()
}

fn layer_creation_controls() -> UiElement<'static> {
    let controls = row![
        small_layer_button("Ponto", Message::AddLayer(LayerType::Point)),
        small_layer_button("Linha", Message::AddLayer(LayerType::Line)),
        small_layer_button("Polígono", Message::AddLayer(LayerType::Polygon)),
        small_layer_button("Raster", Message::AddLayer(LayerType::Raster)),
    ]
    .spacing(6)
    .align_items(Alignment::Center);

    container(controls)
        .width(Length::Fill)
        .padding([4, 8])
        .style(theme::Container::Transparent)
        .into()
}

fn small_layer_button(label: &str, message: Message) -> UiElement<'static> {
    button(text(label).size(12))
        .padding([4, 8])
        .style(theme::Button::Secondary)
        .on_press(message)
        .into()
}

fn toolbox_content(state: &AppState) -> UiElement<'_> {
    let active_tool = state.active_tool;
    let categories = [
        ToolCategory::Navigation,
        ToolCategory::Selection,
        ToolCategory::Measurement,
        ToolCategory::Editing,
        ToolCategory::Analysis,
    ];

    let mut content = Column::new().spacing(12).width(Length::Fill);

    for category in categories {
        let mut row = Row::new().spacing(8);

        for tool in category.tools() {
            let button_element: UiElement<'static> = button(
                column![
                    text(tool.icon())
                        .size(18)
                        .horizontal_alignment(Horizontal::Center),
                    text(tool.name())
                        .size(12)
                        .horizontal_alignment(Horizontal::Center),
                ]
                .spacing(4)
                .align_items(Alignment::Center),
            )
            .padding([6, 10])
            .style(if active_tool == tool {
                theme::Button::Primary
            } else {
                theme::Button::Secondary
            })
            .on_press(Message::SelectTool(tool))
            .into();

            row = row.push(button_element);
        }

        content = content.push(
            column![
                text(category.name()).size(14),
                row,
            ]
            .spacing(6),
        );
    }

    content = content.push(horizontal_rule(1));
    content = content.push(
        column![
            text("Geoprocessamento").size(14),
            row![
                small_layer_button("Buffer 100m", Message::Buffer { distance: 100.0 }),
                small_layer_button("Clip", Message::Clip),
                small_layer_button("Union", Message::Union),
                small_layer_button("Intersect", Message::Intersect),
            ]
            .spacing(6),
        ]
        .spacing(8),
    );

    scrollable(content).height(Length::Fill).into()
}

fn attribute_table_view(state: &AppState) -> UiElement<'_> {
    let Some(active_layer_id) = state.active_attribute_table.as_ref() else {
        return container(
            column![
                text("Nenhuma camada ativa.").size(14),
                text("Selecione uma camada e abra a tabela de atributos para visualizar os registros.")
                    .size(12)
                    .style(secondary_text_style()),
            ]
            .spacing(6)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(theme::Container::Transparent)
        .into();
    };

    let Some(table) = state.attribute_tables.get(active_layer_id) else {
        return container(
            column![
                text("Tabela inexistente.").size(14),
                text("A camada ativa não possui dados de atributos carregados.")
                    .size(12)
                    .style(secondary_text_style()),
            ]
            .spacing(6)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .style(theme::Container::Transparent)
        .into();
    };

    let filter_value = table.filter.clone().unwrap_or_default();
    let total_records = table.feature_count();
    let filtered_features = table.get_filtered_features();
    let filtered_count = filtered_features.len();
    let selected_count = table.selected_count();

    let stats_row = row![
        text(format!("Camada: {}", table.layer_id)).size(16),
        horizontal_space(),
        text(format!(
            "Registros: {} • Visíveis: {} • Selecionados: {}",
            total_records, filtered_count, selected_count
        ))
        .size(12)
        .style(secondary_text_style()),
    ]
    .align_items(Alignment::Center);

    let filter_row = row![
        text_input(
            "Filtrar por identificador, tipo ou atributo...",
            &filter_value,
        )
        .padding([6, 10])
        .size(14)
        .on_input(Message::FilterFeatures),
        button(text("Limpar").size(12))
            .padding([6, 12])
            .style(theme::Button::Secondary)
            .on_press(Message::FilterFeatures(String::new())),
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    let actions_row = row![
        text("Operações rápidas").size(13).style(secondary_text_style()),
        horizontal_space(),
        small_layer_button("Importar CSV", Message::ImportData("csv".into())),
        small_layer_button("Exportar GeoJSON", Message::ExportMap("geojson".into())),
        small_layer_button("Buffer 250m", Message::Buffer { distance: 250.0 }),
        small_layer_button("Clip", Message::Clip),
        small_layer_button("Union", Message::Union),
        small_layer_button("Intersect", Message::Intersect),
    ]
    .spacing(6)
    .align_items(Alignment::Center);

    let mut rows = Column::new().spacing(6).width(Length::Fill);

    if filtered_features.is_empty() {
        rows = rows.push(
            container(
                column![
                    text("Nenhum registro encontrado").size(14),
                    text("Ajuste o filtro ou importe novos dados.")
                        .size(12)
                        .style(secondary_text_style()),
                ]
                .spacing(4)
                .align_items(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([24, 16])
            .style(theme::Container::Transparent),
        );
    } else {
        for (index, feature) in filtered_features {
            let is_selected = table.selected_features.contains(&index);
            rows = rows.push(feature_row(active_layer_id, feature, index, is_selected));
        }
    }

    column![
        stats_row,
        filter_row,
        actions_row,
        horizontal_rule(1),
        scrollable(rows).height(Length::Fill),
    ]
    .spacing(12)
    .into()
}

fn feature_row(layer_id: &str, feature: &Feature, index: usize, is_selected: bool) -> UiElement<'static> {
    let mut badges = Row::new().spacing(6);
    badges = badges.push(attribute_badge("Tipo", &feature.geometry_type));

    for (key, value) in feature.attributes.iter().take(4) {
        badges = badges.push(attribute_badge(key, value));
    }

    let title = format!("ID {}", feature.id);
    let subtitle = format!("Registro {}", index + 1);
    let layer_hint = format!("Camada {layer_id}");

    button(
        column![
            row![
                text(title).size(14),
                horizontal_space(),
                text(layer_hint).size(12).style(secondary_text_style()),
            ],
            text(subtitle).size(12).style(secondary_text_style()),
            badges,
        ]
        .spacing(6)
        .width(Length::Fill),
    )
    .padding([8, 12])
    .style(if is_selected {
        theme::Button::Primary
    } else {
        theme::Button::Secondary
    })
    .on_press(Message::SelectFeature(feature.id.clone()))
    .into()
}

fn attribute_badge(label: &str, value: impl ToString) -> UiElement<'static> {
    let value_str = value.to_string();
    container(
        row![
            text(label).size(11).style(secondary_text_style()),
            text(value_str).size(12),
        ]
        .spacing(4),
    )
    .padding([4, 8])
    .style(BadgeStyle)
    .into()
}

fn map_navigation_controls() -> UiElement<'static> {
    let step = 120.0;
    let controls = row![
        small_layer_button("⬅️", Message::Pan(-step, 0.0)),
        small_layer_button("⬆️", Message::Pan(0.0, -step)),
        small_layer_button("⬇️", Message::Pan(0.0, step)),
        small_layer_button("➡️", Message::Pan(step, 0.0)),
    ]
    .spacing(8)
    .align_items(Alignment::Center);

    container(controls)
        .width(Length::Fill)
        .style(theme::Container::Transparent)
        .into()
}

fn info_badge(label: &str, value: String) -> UiElement<'static> {
    container(
        column![
            text(label).size(12).style(secondary_text_style()),
            text(value).size(18),
        ]
        .spacing(4),
    )
    .padding([4, 12])
    .style(theme::Container::Transparent)
    .into()
}

fn quick_action(label: &str, msg: Message) -> UiElement<'static> {
    button(text(label).size(14))
        .padding([6, 12])
        .style(theme::Button::Secondary)
        .on_press(msg)
        .into()
}

fn menu_toggle_button(label: &str, active: bool, msg: Message) -> UiElement<'static> {
    let indicator = if active { "●" } else { "○" };
    let style = if active {
        theme::Button::Primary
    } else {
        theme::Button::Secondary
    };

    button(
        row![
            text(indicator).size(12),
            text(label).size(13),
        ]
        .spacing(6)
        .align_items(Alignment::Center),
    )
    .padding([6, 12])
    .style(style)
    .on_press(msg)
    .into()
}

fn tool_button_optional(
    icon: &str,
    label: &str,
    active: bool,
    message: Option<Message>,
) -> UiElement<'static> {
    let mut btn = button(
        column![
            text(icon)
                .size(22)
                .horizontal_alignment(Horizontal::Center),
            text(label)
                .size(12)
                .horizontal_alignment(Horizontal::Center),
        ]
        .spacing(4)
        .align_items(Alignment::Center),
    )
    .padding([8, 12])
    .style(if active {
        theme::Button::Primary
    } else {
        theme::Button::Secondary
    });

    if let Some(message) = message {
        btn = btn.on_press(message);
    }

    btn.into()
}

fn layer_entry<'a>(layer: &'a Layer, is_selected: bool) -> UiElement<'a> {
    let visibility_icon = if layer.visible { "👁️" } else { "🚫" };

    let visibility_button: UiElement<'static> = button(text(visibility_icon).size(14))
        .padding([4, 8])
        .style(theme::Button::Secondary)
        .on_press(Message::ToggleLayerVisibility(layer.id.clone()))
        .into();

        let table_button: UiElement<'static> = button(text("📋").size(14))
            .padding([4, 8])
            .style(theme::Button::Secondary)
            .on_press(Message::OpenAttributeTable(layer.id.clone()))
            .into();

    let metadata = format!(
        "{:?} • {} • Opacidade: {}%",
        layer.layer_type,
        if layer.visible { "Visível" } else { "Oculto" },
        (layer.opacity * 100.0) as i32
    );

    let content = row![
        text(if is_selected { "●" } else { "○" }).size(12),
        column![
            text(&layer.name).size(15),
            text(metadata).size(12).style(secondary_text_style()),
        ]
        .spacing(2),
        horizontal_space(),
            table_button,
            visibility_button,
    ]
    .spacing(12)
    .align_items(Alignment::Center);

    let mut card = button(content)
        .padding([8, 12])
        .style(if is_selected {
            theme::Button::Positive
        } else {
            theme::Button::Secondary
        });

    card = card.on_press(Message::SelectLayer(layer.id.clone()));

    card.into()
}

fn status_item(label: &str, value: String) -> UiElement<'static> {
    column![
        text(label).size(11).style(secondary_text_style()),
        text(value).size(13),
    ]
    .spacing(2)
    .into()
}

fn panel<'a>(title: &str, body: impl Into<Element<'a, Message>>) -> iced::widget::Container<'a, Message> {
    container(
        column![
            row![
                text(title).size(16),
                horizontal_space(),
            ],
            horizontal_rule(1),
            body.into(),
        ]
        .spacing(12)
        .width(Length::Fill),
    )
    .padding(16)
    .style(PanelStyle)
}

fn secondary_text_style() -> theme::Text {
    theme::Text::Color(Color::from_rgb8(172, 182, 205))
}

struct BadgeStyle;

impl iced::widget::container::StyleSheet for BadgeStyle {
    type Style = Theme;

    fn appearance(&self, theme: &Self::Style) -> iced::widget::container::Appearance {
        let palette = theme.extended_palette();
        let background = palette.secondary.weak.color;
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color { a: 0.85, ..background })),
            text_color: None,
            border: Border {
                radius: Radius::from(8.0),
                width: 0.5,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.12),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                offset: Vector::new(0.0, 0.5),
                blur_radius: 4.0,
            },
        }
    }
}

impl From<BadgeStyle> for theme::Container {
    fn from(style: BadgeStyle) -> Self {
        theme::Container::Custom(Box::new(style))
    }
}

struct PanelStyle;

impl iced::widget::container::StyleSheet for PanelStyle {
    type Style = Theme;

    fn appearance(&self, theme: &Self::Style) -> iced::widget::container::Appearance {
        let palette = theme.extended_palette();
        let background = palette.background.weak.color;
        let background_color = Color { a: 0.92, ..background };

        iced::widget::container::Appearance {
            background: Some(Background::Color(background_color)),
            text_color: None,
            border: Border {
                radius: Radius::from(12.0),
                width: 1.0,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.45),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 10.0,
            },
        }
    }
}

impl From<PanelStyle> for theme::Container {
    fn from(style: PanelStyle) -> Self {
        theme::Container::Custom(Box::new(style))
    }
}
