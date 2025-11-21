use leptos::*;
use crate::{Theme, Language};

#[component]
pub fn Navbar(
    theme: ReadSignal<Theme>,
    set_theme: WriteSignal<Theme>,
    lang: ReadSignal<Language>,
    set_lang: WriteSignal<Language>,
) -> impl IntoView {
    let toggle_theme = move |_| {
        set_theme.update(|t| *t = t.toggle());
    };

    let toggle_lang = move |_| {
        set_lang.update(|l| *l = l.toggle());
    };

    view! {
        <nav class="navbar">
            <div class="container nav-container">
                <div class="logo">
                    <Logo/>
                    <span class="logo-text">"ARXIS"</span>
                </div>

                <div class="nav-links">
                    <a href="#features">{move || if lang.get() == Language::PtBR { "Recursos" } else { "Features" }}</a>
                    <a href="#architecture">{move || if lang.get() == Language::PtBR { "Arquitetura" } else { "Architecture" }}</a>
                    <a href="#docs">{move || if lang.get() == Language::PtBR { "Documentação" } else { "Documentation" }}</a>
                    <a href="#contact">{move || if lang.get() == Language::PtBR { "Contato" } else { "Contact" }}</a>

                    <a href="https://github.com/avilaops/arxis" target="_blank" class="btn-github">
                        <svg width="20" height="20" viewBox="0 0 16 16" fill="currentColor">
                            <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"/>
                        </svg>
                        "GitHub"
                    </a>

                    // Theme toggle
                    <button class="theme-toggle" on:click=toggle_theme title="Toggle theme">
                        {move || theme.get().icon()}
                    </button>

                    // Language toggle
                    <button class="lang-toggle" on:click=toggle_lang title="Toggle language">
                        <span>{move || lang.get().flag()}</span>
                        <span class="lang-code">{move || lang.get().code()}</span>
                    </button>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Logo() -> impl IntoView {
    view! {
        <div class="logo-icon">
            <svg viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg">
                <defs>
                    <linearGradient id="logoGradient" x1="0%" y1="0%" x2="100%" y2="100%">
                        <stop offset="0%" style="stop-color:#FFD700;stop-opacity:1"/>
                        <stop offset="50%" style="stop-color:#FFA500;stop-opacity:1"/>
                        <stop offset="100%" style="stop-color:#FF8C00;stop-opacity:1"/>
                    </linearGradient>
                </defs>
                <circle cx="50" cy="25" r="8" fill="url(#logoGradient)"/>
                <circle cx="75" cy="50" r="8" fill="url(#logoGradient)"/>
                <circle cx="50" cy="75" r="8" fill="url(#logoGradient)"/>
                <circle cx="25" cy="50" r="8" fill="url(#logoGradient)"/>
                <line x1="50" y1="25" x2="75" y2="50" stroke="url(#logoGradient)" stroke-width="3"/>
                <line x1="75" y1="50" x2="50" y2="75" stroke="url(#logoGradient)" stroke-width="3"/>
                <line x1="50" y1="75" x2="25" y2="50" stroke="url(#logoGradient)" stroke-width="3"/>
                <line x1="25" y1="50" x2="50" y2="25" stroke="url(#logoGradient)" stroke-width="3"/>
            </svg>
        </div>
    }
}
