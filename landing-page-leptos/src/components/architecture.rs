use leptos::*;
use crate::{Language, Theme};

#[component]
pub fn Architecture(
    lang: ReadSignal<Language>,
    theme: ReadSignal<Theme>,
) -> impl IntoView {
    view! {
        <section id="architecture" class="architecture">
            <div class="container">
                <h2 class="section-title">
                    {move || if lang.get() == Language::PtBR { "Arquitetura" } else { "Architecture" }}
                </h2>
                <div class="arch-diagram">
                    <div class="arch-layer">
                        <div class="arch-box">"Physics Layer"</div>
                        <div class="arch-detail">"GR • LISA • Cosmology"</div>
                    </div>
                    <div class="arch-arrow">"↓"</div>
                    <div class="arch-layer">
                        <div class="arch-box">"Math Layer"</div>
                        <div class="arch-detail">"Quaternions • Tensors • FFT"</div>
                    </div>
                    <div class="arch-arrow">"↓"</div>
                    <div class="arch-layer">
                        <div class="arch-box">"Core Engine"</div>
                        <div class="arch-detail">"SIMD • Parallel • Zero-Copy"</div>
                    </div>
                </div>
            </div>
        </section>
    }
}
