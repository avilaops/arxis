use leptos::*;
use crate::{Language, Theme, i18n::T};

#[component]
pub fn Features(
    lang: ReadSignal<Language>,
    theme: ReadSignal<Theme>,
) -> impl IntoView {
    view! {
        <section id="features" class="features">
            <div class="container">
                <h2 class="section-title">
                    {move || T::features_title(lang.get())}
                </h2>

                <div class="features-grid">
                    <FeatureCard
                        icon="🌌"
                        title=move || T::nasa_lisa_title(lang.get())
                        description=move || T::nasa_lisa_desc(lang.get())
                        features=vec![
                            "✅ Matched filtering",
                            "✅ Template banks (MBHB, EMRI)",
                            "✅ MCMC parameter estimation",
                            "✅ TDI channels & whitening",
                        ]
                    />

                    <FeatureCard
                        icon="🔄"
                        title=move || T::quaternions_title(lang.get())
                        description=move || T::quaternions_desc(lang.get())
                        features=vec![
                            "✅ 3D/4D quaternions (SO(3), SO(4))",
                            "✅ Dual quaternions",
                            "✅ 4D polytopes (Tesseract, 24-Cell)",
                            "✅ SLERP interpolation",
                        ]
                    />

                    <FeatureCard
                        icon="📊"
                        title=move || T::tensors_title(lang.get())
                        description=move || T::tensors_desc(lang.get())
                        features=vec![
                            "✅ 0D-4D tensor operations",
                            "✅ Matrix algebra (inverse, det)",
                            "✅ Convolution & pooling",
                            "✅ Batch normalization",
                        ]
                    />

                    <FeatureCard
                        icon="🕳️"
                        title=move || T::gr_title(lang.get())
                        description=move || T::gr_desc(lang.get())
                        features=vec![
                            "✅ Schwarzschild, Kerr, FLRW",
                            "✅ Geodesic integration",
                            "✅ Riemann curvature",
                            "✅ Gravitational lensing",
                        ]
                    />
                </div>
            </div>
        </section>
    }
}

#[component]
fn FeatureCard(
    icon: &'static str,
    title: impl Fn() -> &'static str + 'static,
    description: impl Fn() -> &'static str + 'static,
    features: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="feature-card">
            <div class="feature-icon">{icon}</div>
            <h3>{title}</h3>
            <p>{description}</p>
            <ul class="feature-list">
                {features.into_iter().map(|f| view! { <li>{f}</li> }).collect_view()}
            </ul>
        </div>
    }
}
