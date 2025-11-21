use leptos::*;
use crate::{Language, Theme, i18n::T};

#[component]
pub fn Contact(
    lang: ReadSignal<Language>,
    theme: ReadSignal<Theme>,
) -> impl IntoView {
    view! {
        <section id="contact" class="contact">
            <div class="container">
                <h2 class="section-title">
                    {move || T::contact_title(lang.get())}
                </h2>

                <div class="contact-links">
                    <a href="https://github.com/avilaops/arxis" target="_blank" class="contact-card">
                        <span class="contact-icon">"📦"</span>
                        <span class="contact-text">"GitHub"</span>
                    </a>
                    <a href="https://crates.io/crates/arxis-quaternions" target="_blank" class="contact-card">
                        <span class="contact-icon">"🦀"</span>
                        <span class="contact-text">"crates.io"</span>
                    </a>
                    <a href="https://docs.rs/arxis-quaternions" target="_blank" class="contact-card">
                        <span class="contact-icon">"📚"</span>
                        <span class="contact-text">"docs.rs"</span>
                    </a>
                    <a href="mailto:nicolas@avila.inc" class="contact-card">
                        <span class="contact-icon">"✉️"</span>
                        <span class="contact-text">"Email"</span>
                    </a>
                </div>
            </div>
        </section>
    }
}
