use leptos::*;
use leptos_meta::*;
use leptos_router::*;

mod api;
mod components;
mod models;
mod pages;

use pages::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/burn-reconstruction-frontend.css"/>
        <Title text="Burn Reconstruction System"/>
        <Meta name="description" content="Medical AI for facial reconstruction planning"/>

        <Router>
            <nav class="navbar">
                <div class="container">
                    <h1 class="logo">"🏥 Burn Reconstruction System"</h1>
                    <div class="nav-links">
                        <A href="/">"Home"</A>
                        <A href="/cases">"Cases"</A>
                        <A href="/new-case">"New Case"</A>
                    </div>
                </div>
            </nav>

            <main class="main-content">
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/cases" view=CasesListPage/>
                    <Route path="/new-case" view=NewCasePage/>
                    <Route path="/case/:id" view=CaseDetailPage/>
                    <Route path="/case/:id/reconstruct" view=ReconstructionPage/>
                    <Route path="/case/:id/simulate" view=SimulationPage/>
                </Routes>
            </main>

            <footer class="footer">
                <div class="container">
                    <p>"Built with ❤️ using 100% Rust | AVL Cloud Platform"</p>
                    <p class="disclaimer">
                        "⚠️ For medical professional use only. Not a substitute for clinical judgment."
                    </p>
                </div>
            </footer>
        </Router>
    }
}

fn main() {
    // Set up console error panic hook for better error messages
    console_error_panic_hook::set_once();

    // Initialize tracing
    tracing_wasm::set_as_global_default();

    mount_to_body(|| view! { <App/> })
}
