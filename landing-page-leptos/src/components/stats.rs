use leptos::*;
use crate::{Language, Theme};

#[component]
pub fn Stats(
    lang: ReadSignal<Language>,
    theme: ReadSignal<Theme>,
) -> impl IntoView {
    view! {
        <section class="stats">
            <div class="container">
                <div class="stats-grid">
                    <div class="stat-item">
                        <div class="stat-value">"101"</div>
                        <div class="stat-label">
                            {move || if lang.get() == Language::PtBR { "Testes" } else { "Tests" }}
                        </div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-value">"11"</div>
                        <div class="stat-label">
                            {move || if lang.get() == Language::PtBR { "Módulos" } else { "Modules" }}
                        </div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-value">"100%"</div>
                        <div class="stat-label">
                            {move || if lang.get() == Language::PtBR { "Rust" } else { "Rust" }}
                        </div>
                    </div>
                    <div class="stat-item">
                        <div class="stat-value">"0"</div>
                        <div class="stat-label">
                            {move || if lang.get() == Language::PtBR { "Dependências Unsafe" } else { "Unsafe Dependencies" }}
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
