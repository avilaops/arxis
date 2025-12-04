//! Symbology Editor

use crate::layer_manager::Layer;
use crate::ui::Message;
use iced::{
    alignment::Horizontal,
    theme,
    widget::{button, column, container, row, slider, text, Slider, Space},
    Alignment, Background, Border, Color, Element, Length, Shadow, Theme,
};
use iced::border::Radius;

pub struct SymbologyEditor;

impl SymbologyEditor {
    pub fn view(layer: Option<&Layer>) -> Element<'_, Message> {
        if let Some(layer) = layer {
            Self::render_editor(layer)
        } else {
            container(
                column![
                    text("Nenhuma camada selecionada").size(16),
                    text("Selecione uma camada no painel à esquerda para ajustar a simbologia.")
                        .size(13)
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
            .into()
        }
    }

    fn render_editor(layer: &Layer) -> Element<'_, Message> {
        let symbology = &layer.symbology;

        let stroke_layer_id = layer.id.clone();
        let symbol_layer_id = layer.id.clone();
        let opacity_layer_id = layer.id.clone();

        column![
            text(format!("Camada: {}", layer.name)).size(18),
            text(format!(
                "Tipo: {:?} • Visível: {}",
                layer.layer_type,
                if layer.visible { "Sim" } else { "Não" }
            ))
            .size(13)
            .style(secondary_text_style()),
            Space::with_height(Length::Fixed(12.0)),
            row![
                color_preview("Cor de preenchimento", symbology.fill_color),
                color_preview("Cor do contorno", symbology.stroke_color),
            ]
            .spacing(12),
            Space::with_height(Length::Fixed(16.0)),
            slider_block(
                "Espessura do traço",
                format!("{:.1} px", symbology.stroke_width),
                slider(0.5..=10.0, symbology.stroke_width, move |value| {
                    Message::UpdateSymbology {
                        layer_id: stroke_layer_id.clone(),
                        property: "stroke_width".into(),
                        value: format!("{value:.1}"),
                    }
                }),
            ),
            slider_block(
                "Tamanho do símbolo",
                format!("{:.0} px", symbology.symbol_size),
                slider(2.0..=30.0, symbology.symbol_size, move |value| {
                    Message::UpdateSymbology {
                        layer_id: symbol_layer_id.clone(),
                        property: "symbol_size".into(),
                        value: format!("{value:.0}"),
                    }
                }),
            ),
            slider_block(
                "Opacidade",
                format!("{:.0}%", layer.opacity * 100.0),
                slider(0.0..=1.0, layer.opacity, move |value| {
                    Message::UpdateSymbology {
                        layer_id: opacity_layer_id.clone(),
                        property: "opacity".into(),
                        value: format!("{value:.2}"),
                    }
                }),
            ),
            Space::with_height(Length::Fixed(16.0)),
            button(text("Aplicar ajustes"))
                .padding([10, 12])
                .style(theme::Button::Primary)
                .on_press(Message::ToggleSymbologyEditor),
        ]
        .spacing(12)
        .into()
    }
}

fn slider_block(
    label: &str,
    value: String,
    slider: Slider<'static, f32, Message>,
) -> Element<'static, Message> {
    column![
        row![
            text(label).size(13),
            horizontal_spacer(),
            text(value).size(12).style(secondary_text_style()),
        ]
        .align_items(Alignment::Center),
        slider,
    ]
    .spacing(6)
    .into()
}

fn color_preview(label: &str, color: [u8; 3]) -> Element<'static, Message> {
    column![
        text(label).size(12),
        container(
            text(format!("RGB({}, {}, {})", color[0], color[1], color[2]))
                .size(12)
                .horizontal_alignment(Horizontal::Center),
        )
        .padding(8)
        .width(Length::Fixed(140.0))
        .style(ColorSwatchStyle(Color::from_rgb8(color[0], color[1], color[2]))),
    ]
    .spacing(6)
    .into()
}

fn horizontal_spacer() -> Element<'static, Message> {
    Space::with_width(Length::Fill).into()
}

fn secondary_text_style() -> theme::Text {
    theme::Text::Color(Color::from_rgb8(170, 180, 200))
}

struct ColorSwatchStyle(Color);

impl iced::widget::container::StyleSheet for ColorSwatchStyle {
    type Style = Theme;

    fn appearance(&self, _theme: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(Background::Color(Color { a: 0.9, ..self.0 })),
            text_color: Some(Color::from_rgb8(18, 22, 28)),
            border: Border {
                radius: Radius::from(8.0),
                width: 1.0,
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.31),
            },
            shadow: Shadow::default(),
        }
    }
}

impl From<ColorSwatchStyle> for theme::Container {
    fn from(style: ColorSwatchStyle) -> Self {
        theme::Container::Custom(Box::new(style))
    }
}
