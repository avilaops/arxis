use leptos::*;
use crate::{Language, Theme};

#[component]
pub fn CodeExamples(
    lang: ReadSignal<Language>,
    theme: ReadSignal<Theme>,
) -> impl IntoView {
    view! {
        <section id="docs" class="code-examples">
            <div class="container">
                <h2 class="section-title">
                    {move || if lang.get() == Language::PtBR { "Exemplos de Código" } else { "Code Examples" }}
                </h2>

                <pre><code>
                    <span class="code-comment">"// Relatividade Geral: Órbita ao redor de buraco negro\n"</span>
                    <span class="code-keyword">"use"</span>{" "}
                    <span class="code-var">"arxis_quaternions"</span>{"::"}
                    <span class="code-var">"gr"</span>{"::*;\n\n"}

                    <span class="code-keyword">"let"</span>{" "}
                    <span class="code-var">"bh"</span>{" = "}
                    <span class="code-type">"Schwarzschild"</span>{"::"}
                    <span class="code-function">"new"</span>{"("}
                    <span class="code-number">"1e6"</span>{");\n"}

                    <span class="code-keyword">"let"</span>{" "}
                    <span class="code-var">"orbit"</span>{" = "}
                    <span class="code-var">"bh"</span>{"."}
                    <span class="code-function">"geodesic"</span>{"(\n    "}
                    <span class="code-var">"r"</span>{": "}
                    <span class="code-number">"10.0"</span>{",\n    "}
                    <span class="code-var">"theta"</span>{": "}
                    <span class="code-var">"PI"</span>{"/"}
                    <span class="code-number">"2.0"</span>{",\n    "}
                    <span class="code-var">"E"</span>{": "}
                    <span class="code-number">"0.95"</span>{",\n);\n\n"}

                    <span class="code-function">"println!"</span>{"("}
                    <span class="code-string">"\"Periapsis: {:.2} M\""</span>{", "}
                    <span class="code-var">"orbit"</span>{"."}
                    <span class="code-function">"periapsis"</span>{"());"}
                </code></pre>
            </div>
        </section>
    }
}
