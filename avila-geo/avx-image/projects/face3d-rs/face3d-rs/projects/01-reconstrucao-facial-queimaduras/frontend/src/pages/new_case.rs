use leptos::*;
use crate::api;

#[component]
pub fn NewCasePage() -> impl IntoView {
    let (patient_id, set_patient_id) = create_signal(String::new());
    let (is_creating, set_is_creating) = create_signal(false);
    let (error, set_error) = create_signal(Option::<String>::None);

    let create_case_action = create_action(move |patient_id: &String| {
        let patient_id = patient_id.clone();
        async move {
            set_is_creating.set(true);
            set_error.set(None);

            match api::create_case(patient_id).await {
                Ok(case) => {
                    set_is_creating.set(false);
                    // Navigate to case detail page
                    let navigate = leptos_router::use_navigate();
                    navigate(&format!("/case/{}", case.id), Default::default());
                }
                Err(e) => {
                    set_error.set(Some(e));
                    set_is_creating.set(false);
                }
            }
        }
    });

    let on_submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let id = patient_id.get();
        if !id.is_empty() {
            create_case_action.dispatch(id);
        }
    };

    view! {
        <div class="container page-container">
            <div class="page-header">
                <h1>"Create New Case"</h1>
                <p>"Start a new facial reconstruction case by entering patient information"</p>
            </div>

            <div class="card">
                <form on:submit=on_submit class="form">
                    <div class="form-group">
                        <label for="patient-id" class="form-label">
                            "Patient ID" <span class="required">"*"</span>
                        </label>
                        <input
                            type="text"
                            id="patient-id"
                            class="form-input"
                            placeholder="Enter patient identifier (e.g., PT-2025-001)"
                            value=patient_id
                            on:input=move |ev| set_patient_id.set(event_target_value(&ev))
                            required
                        />
                        <p class="form-hint">
                            "This ID will be used to reference the case. Use a unique, "
                            "de-identified identifier to maintain patient privacy (LGPD compliance)."
                        </p>
                    </div>

                    {move || {
                        error.get().map(|err| {
                            view! {
                                <div class="alert alert-error">
                                    <strong>"Error: "</strong> {err}
                                </div>
                            }
                        })
                    }}

                    <div class="form-actions">
                        <button
                            type="submit"
                            class="btn btn-primary"
                            disabled=move || is_creating.get() || patient_id.get().is_empty()
                        >
                            {move || if is_creating.get() {
                                "Creating..."
                            } else {
                                "Create Case"
                            }}
                        </button>
                        <a href="/cases" class="btn btn-secondary">"Cancel"</a>
                    </div>
                </form>
            </div>

            <div class="info-section">
                <h2>"What happens next?"</h2>
                <ol class="steps-list">
                    <li>"Upload photos of the patient from multiple angles"</li>
                    <li>"System will generate a 3D reconstruction"</li>
                    <li>"Define the surgical plan and affected areas"</li>
                    <li>"Simulate the surgery and preview results"</li>
                    <li>"Export 3D models and reports for medical records"</li>
                </ol>
            </div>
        </div>
    }
}
