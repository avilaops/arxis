use leptos::*;

#[component]
pub fn CasesListPage() -> impl IntoView {
    view! {
        <div class="container page-container">
            <div class="page-header">
                <h1>"Cases"</h1>
                <a href="/new-case" class="btn btn-primary">"+ New Case"</a>
            </div>

            <div class="cases-grid">
                <p class="text-muted">"No cases yet. Create your first case to get started."</p>
            </div>
        </div>
    }
}

#[component]
pub fn CaseDetailPage() -> impl IntoView {
    let params = leptos_router::use_params_map();
    let case_id = move || params.with(|p| p.get("id").cloned().unwrap_or_default());

    view! {
        <div class="container page-container">
            <h1>"Case Details"</h1>
            <p>"Case ID: " {case_id}</p>
        </div>
    }
}

#[component]
pub fn ReconstructionPage() -> impl IntoView {
    view! {
        <div class="container page-container">
            <h1>"3D Reconstruction"</h1>
            <p>"Upload photos and generate 3D model"</p>
        </div>
    }
}

#[component]
pub fn SimulationPage() -> impl IntoView {
    view! {
        <div class="container page-container">
            <h1>"Surgery Simulation"</h1>
            <p>"Define surgical plan and simulate results"</p>
        </div>
    }
}
