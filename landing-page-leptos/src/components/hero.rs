use leptos::*;
use crate::{Language, Theme, i18n::T};

#[component]
pub fn Hero(
    lang: ReadSignal<Language>,
    theme: ReadSignal<Theme>,
) -> impl IntoView {
    view! {
        <section class="hero">
            <div class="hero-content container">
                <div class="hero-badge">
                    <span class="badge-icon">"🏛️"</span>
                    <span>"ARX (fortress) + AXIS (engine) = ARXIS"</span>
                </div>

                <h1 class="hero-title">
                    {move || if lang.get() == Language::PtBR { "A " } else { "The " }}
                    <span class="gradient-text">{move || T::hero_title(lang.get()).split(' ').last().unwrap()}</span>
                    <br/>
                    {move || if lang.get() == Language::PtBR { "Matemática" } else { "Mathematical" }}
                </h1>

                <p class="hero-subtitle">
                    {move || T::hero_subtitle(lang.get())}
                </p>

                <p class="hero-description">
                    {move || T::hero_description(lang.get())}
                </p>

                <div class="hero-stats">
                    <div class="stat">
                        <span class="stat-number">"🌌"</span>
                        <span class="stat-label">{move || if lang.get() == Language::PtBR { "NASA/LISA Pronto" } else { "NASA/LISA Ready" }}</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"101"</span>
                        <span class="stat-label">{move || if lang.get() == Language::PtBR { "Testes Passando" } else { "Tests Passing" }}</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"⚡"</span>
                        <span class="stat-label">{move || if lang.get() == Language::PtBR { "Ultra Rápido" } else { "Blazing Fast" }}</span>
                    </div>
                </div>

                <div class="hero-cta">
                    <a href="#docs" class="btn btn-primary">
                        <span>{move || T::get_started(lang.get())}</span>
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <line x1="5" y1="12" x2="19" y2="12"></line>
                            <polyline points="12 5 19 12 12 19"></polyline>
                        </svg>
                    </a>
                    <a href="https://github.com/avilaops/arxis" class="btn btn-secondary" target="_blank">
                        {move || T::view_github(lang.get())}
                    </a>
                </div>

                <div class="hero-code">
                    <pre><code>
                        <span class="code-keyword">"use"</span>{" "}
                        <span class="code-var">"arxis_quaternions"</span>{"::"}
                        <span class="code-var">"physics"</span>{"::*;\n\n"}

                        <span class="code-comment">"// Detectar ondas gravitacionais da LISA\n"</span>
                        <span class="code-keyword">"let"</span>{" "}
                        <span class="code-var">"smbh"</span>{" = "}
                        <span class="code-type">"LISASource"</span>{"::"}
                        <span class="code-function">"smbh"</span>{"("}
                        <span class="code-number">"1e6"</span>{", "}
                        <span class="code-number">"5e5"</span>{", "}
                        <span class="code-number">"1.0"</span>{", "}
                        <span class="code-number">"0.05"</span>{");\n"}

                        <span class="code-keyword">"let"</span>{" "}
                        <span class="code-var">"snr"</span>{" = "}
                        <span class="code-var">"smbh"</span>{"."}
                        <span class="code-function">"lisa_snr"</span>{"();\n"}

                        <span class="code-function">"println!"</span>{"("}
                        <span class="code-string">"\"SNR: {:.1}\""</span>{", "}
                        <span class="code-var">"snr"</span>{");"}
                    </code></pre>
                </div>
            </div>
        </section>
    }
}
