use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use gloo_storage::{LocalStorage, Storage};

mod components;
mod i18n;
mod theme;

use components::*;
use i18n::*;
use theme::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // Theme state (light/dark)
    let (theme, set_theme) = create_signal(load_theme());

    // Language state (pt-BR/en)
    let (lang, set_lang) = create_signal(load_language());

    // Save theme to localStorage
    create_effect(move |_| {
        let theme_value = theme.get();
        let _ = LocalStorage::set("theme", theme_value.as_str());
        update_theme_class(&theme_value);
    });

    // Save language to localStorage
    create_effect(move |_| {
        let lang_value = lang.get();
        let _ = LocalStorage::set("language", lang_value.as_str());
    });

    view! {
        <Router>
            <div class=move || format!("app {}", theme.get().as_str())>
                <Title text="Arxis - The Mathematical Citadel"/>
                <Meta name="description" content="Research-Grade Physics & Mathematics in Rust"/>

                <Navbar theme=theme set_theme=set_theme lang=lang set_lang=set_lang/>

                <Routes>
                    <Route path="" view=move || view! {
                        <Hero lang=lang theme=theme/>
                        <Features lang=lang theme=theme/>
                        <Architecture lang=lang theme=theme/>
                        <CodeExamples lang=lang theme=theme/>
                        <Stats lang=lang theme=theme/>
                        <Contact lang=lang theme=theme/>
                        <Footer lang=lang theme=theme/>
                    }/>
                </Routes>
            </div>
        </Router>
    }
}

fn load_theme() -> Theme {
    LocalStorage::get("theme")
        .ok()
        .and_then(|s: String| Theme::from_str(&s))
        .unwrap_or(Theme::Light)
}

fn load_language() -> Language {
    LocalStorage::get("language")
        .ok()
        .and_then(|s: String| Language::from_str(&s))
        .unwrap_or(Language::PtBR)
}

fn update_theme_class(theme: &Theme) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                let _ = body.set_attribute("data-theme", theme.as_str());
            }
        }
    }
}
