use leptos::*;
use crate::{Language, Theme, i18n::T};

#[component]
pub fn Footer(
    lang: ReadSignal<Language>,
    theme: ReadSignal<Theme>,
) -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="container footer-content">
                <div class="footer-section">
                    <div class="footer-logo">
                        <span class="footer-logo-icon">"🏛️"</span>
                        <span class="footer-logo-text">"ARXIS"</span>
                    </div>
                    <p class="footer-tagline">
                        {move || T::footer_tagline(lang.get())}
                    </p>
                    <p class="footer-description">
                        {move || T::footer_description(lang.get())}
                    </p>
                </div>

                <div class="footer-section">
                    <h4>{move || T::resources(lang.get())}</h4>
                    <ul>
                        <li><a href="https://docs.rs/arxis-quaternions" target="_blank">"Documentation"</a></li>
                        <li><a href="https://github.com/avilaops/arxis/tree/main/examples" target="_blank">"Examples"</a></li>
                        <li><a href="https://github.com/avilaops/arxis" target="_blank">"GitHub"</a></li>
                        <li><a href="https://crates.io/crates/arxis-quaternions" target="_blank">"crates.io"</a></li>
                    </ul>
                </div>

                <div class="footer-section">
                    <h4>{move || T::community(lang.get())}</h4>
                    <ul>
                        <li><a href="https://github.com/avilaops/arxis/issues" target="_blank">"Issues"</a></li>
                        <li><a href="https://github.com/avilaops/arxis/discussions" target="_blank">"Discussions"</a></li>
                        <li><a href="https://github.com/avilaops" target="_blank">"Avila Organization"</a></li>
                    </ul>
                </div>

                <div class="footer-section">
                    <h4>{move || T::about(lang.get())}</h4>
                    <ul>
                        <li><a href="mailto:nicolas@avila.inc">"Contact"</a></li>
                        <li><a href="https://avila.inc" target="_blank">"Avila Inc"</a></li>
                    </ul>
                </div>
            </div>

            <div class="footer-bottom">
                <div class="container footer-bottom-content">
                    <p class="copyright">
                        "© 2025 "
                        <a href="https://avila.inc" target="_blank">"Avila"</a>
                        {move || if lang.get() == Language::PtBR { ". Todos os direitos reservados." } else { ". All rights reserved." }}
                    </p>
                    <p class="license">
                        {move || if lang.get() == Language::PtBR { "Licenciado sob " } else { "Licensed under " }}
                        <a href="https://opensource.org/licenses/MIT" target="_blank">"MIT"</a>
                        {" / "}
                        <a href="https://www.apache.org/licenses/LICENSE-2.0" target="_blank">"Apache-2.0"</a>
                    </p>
                </div>
            </div>
        </footer>
    }
}
