use leptos::*;
use leptos_router::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="hero">
            <div class="container">
                <h1 class="hero-title">"Burn Reconstruction System"</h1>
                <p class="hero-subtitle">
                    "AI-powered surgical planning for facial burn reconstruction"
                </p>
                <div class="hero-features">
                    <div class="feature-card">
                        <span class="feature-icon">"📸"</span>
                        <h3>"3D Reconstruction"</h3>
                        <p>"From 2D photos to accurate 3D facial models"</p>
                    </div>
                    <div class="feature-card">
                        <span class="feature-icon">"🔬"</span>
                        <h3>"Surgical Simulation"</h3>
                        <p>"Preview surgery outcomes before operation"</p>
                    </div>
                    <div class="feature-card">
                        <span class="feature-icon">"📊"</span>
                        <h3>"Clinical Reports"</h3>
                        <p>"Comprehensive planning documentation"</p>
                    </div>
                </div>
                <div class="hero-cta">
                    <A href="/new-case" class="btn btn-primary btn-lg">
                        "Start New Case"
                    </A>
                    <A href="/cases" class="btn btn-secondary btn-lg">
                        "View Cases"
                    </A>
                </div>
            </div>
        </div>

        <section class="about">
            <div class="container">
                <h2>"About This System"</h2>
                <p>
                    "The Burn Reconstruction System uses advanced 3D morphable models (3DMM) "
                    "and machine learning to help surgeons plan reconstructive procedures for "
                    "burn victims. By creating accurate 3D models from photos, surgeons can "
                    "simulate different surgical approaches and visualize expected outcomes."
                </p>
                <div class="stats">
                    <div class="stat">
                        <span class="stat-number">"30%"</span>
                        <span class="stat-label">"Reduction in surgery time"</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"95%"</span>
                        <span class="stat-label">"Model accuracy"</span>
                    </div>
                    <div class="stat">
                        <span class="stat-number">"100%"</span>
                        <span class="stat-label">"Rust & WASM"</span>
                    </div>
                </div>
            </div>
        </section>

        <section class="workflow">
            <div class="container">
                <h2>"How It Works"</h2>
                <div class="workflow-steps">
                    <div class="step">
                        <span class="step-number">"1"</span>
                        <h3>"Upload Photos"</h3>
                        <p>"Take multiple photos of the patient from different angles"</p>
                    </div>
                    <div class="step">
                        <span class="step-number">"2"</span>
                        <h3>"3D Reconstruction"</h3>
                        <p>"AI generates a detailed 3D model of the face"</p>
                    </div>
                    <div class="step">
                        <span class="step-number">"3"</span>
                        <h3>"Plan Surgery"</h3>
                        <p>"Define the surgical procedure and affected areas"</p>
                    </div>
                    <div class="step">
                        <span class="step-number">"4"</span>
                        <h3>"Simulate Results"</h3>
                        <p>"Preview expected outcomes in 3D"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
