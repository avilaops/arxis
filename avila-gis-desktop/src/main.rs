//! 🗺️ AvilaGIS Desktop
//!
//! Desktop GIS Application with ArcGIS-like interface

use iced::{Application, Command, Element, Settings, Theme, executor, window};

mod ui;
mod state;
mod map_view;
mod layer_manager;
mod symbology_editor;
mod toolbox;
mod attribute_table;
mod data_io;

use state::AppState;
use ui::Message;

pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    AvilaGIS::run(Settings {
        window: window::Settings {
            size: iced::Size::new(1400.0, 900.0),
            position: window::Position::Centered,
            min_size: Some(iced::Size::new(1024.0, 768.0)),
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    })
}

struct AvilaGIS {
    state: AppState,
}

impl Application for AvilaGIS {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                state: AppState::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        format!("AvilaGIS Desktop - {}", self.state.project_name())
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.state.update(message)
    }

    fn view(&self) -> Element<'_, Message> {
        ui::view(&self.state)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
