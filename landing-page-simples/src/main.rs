use leptos::*;
use leptos_meta::*;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    view! {
        <div class="page">
            <Hero/>
            <Problem/>
            <Solution/>
            <HowItWorks/>
            <Results/>
            <CTA/>
        </div>
    }
}

// ===============================================
// 1. HERO - Primeira coisa que a pessoa vê
// ===============================================
#[component]
fn Hero() -> impl IntoView {
    view! {
        <section class="hero">
            <h1>"Seus dados científicos estão seguros?"</h1>
            <p class="subtitle">
                "Nós ajudamos cientistas e pesquisadores a "
                <strong>"processar dados complexos sem perder informação"</strong>
                " - rápido, seguro e sem complicação."
            </p>
            <button class="cta-button">"Quero conhecer"</button>
        </section>
    }
}

// ===============================================
// 2. PROBLEMA - O que o cliente enfrenta
// ===============================================
#[component]
fn Problem() -> impl IntoView {
    view! {
        <section class="problem">
            <h2>"Você já passou por isso?"</h2>

            <div class="problem-cards">
                <div class="card">
                    <span class="emoji">"😰"</span>
                    <h3>"Perdi dados importantes"</h3>
                    <p>"Você processou terabytes de dados da sua pesquisa e algo deu errado. Semanas de trabalho perdidas."</p>
                </div>

                <div class="card">
                    <span class="emoji">"🐌"</span>
                    <h3>"Demora dias para processar"</h3>
                    <p>"Sua análise científica leva 3 dias. Você não pode esperar tanto para tomar decisões."</p>
                </div>

                <div class="card">
                    <span class="emoji">"💸"</span>
                    <h3>"Custos explodiram"</h3>
                    <p>"Você está pagando R$ 10.000/mês em nuvem estrangeira. E ainda fica lento."</p>
                </div>
            </div>
        </section>
    }
}

// ===============================================
// 3. SOLUÇÃO - Como nós resolvemos
// ===============================================
#[component]
fn Solution() -> impl IntoView {
    view! {
        <section class="solution">
            <h2>"Nós criamos uma plataforma que entende suas necessidades"</h2>

            <div class="benefits">
                <div class="benefit">
                    <span class="icon">"✅"</span>
                    <h3>"Dados seguros"</h3>
                    <p>"Seus dados nunca serão perdidos. Garantimos 99.99% de disponibilidade."</p>
                </div>

                <div class="benefit">
                    <span class="icon">"⚡"</span>
                    <h3>"10x mais rápido"</h3>
                    <p>"O que levava 3 dias agora leva 7 horas. Resultado no mesmo dia."</p>
                </div>

                <div class="benefit">
                    <span class="icon">"🇧🇷"</span>
                    <h3>"Feito no Brasil"</h3>
                    <p>"Servidores em São Paulo. Latência baixa, suporte em português, custos 40% menores."</p>
                </div>

                <div class="benefit">
                    <span class="icon">"🛡️"</span>
                    <h3>"Simples e confiável"</h3>
                    <p>"Você não precisa ser especialista em TI. Nós cuidamos da complexidade técnica."</p>
                </div>
            </div>
        </section>
    }
}

// ===============================================
// 4. COMO FUNCIONA - História simples
// ===============================================
#[component]
fn HowItWorks() -> impl IntoView {
    view! {
        <section class="how-it-works">
            <h2>"Como funciona na prática?"</h2>

            <div class="timeline">
                <div class="step">
                    <span class="step-number">"1"</span>
                    <h3>"Você nos conta seu problema"</h3>
                    <p>"Em uma conversa de 30 minutos, você explica o que precisa. Sem jargões técnicos."</p>
                </div>

                <div class="step">
                    <span class="step-number">"2"</span>
                    <h3>"Configuramos tudo para você"</h3>
                    <p>"Nossa equipe prepara o sistema. Você não mexe em servidores, não escreve código."</p>
                </div>

                <div class="step">
                    <span class="step-number">"3"</span>
                    <h3>"Você vê os resultados"</h3>
                    <p>"Seus dados processados, organizados, prontos para análise. Simples assim."</p>
                </div>
            </div>
        </section>
    }
}

// ===============================================
// 5. RESULTADOS - Histórias reais (depois)
// ===============================================
#[component]
fn Results() -> impl IntoView {
    view! {
        <section class="results">
            <h2>"Quem já usa nossa plataforma"</h2>

            <div class="testimonials">
                <div class="testimonial">
                    <p class="quote">
                        "\"Antes eu esperava 3 dias para processar dados do telescópio. "
                        "Agora faço isso em algumas horas. Mudou minha pesquisa.\""
                    </p>
                    <p class="author">"- Dra. Maria Silva, Astrônoma (IAG/USP)"</p>
                </div>

                <div class="testimonial">
                    <p class="quote">
                        "\"O custo caiu pela metade e a velocidade triplicou. "
                        "Finalmente uma solução brasileira que funciona.\""
                    </p>
                    <p class="author">"- Prof. João Santos, Física (UNICAMP)"</p>
                </div>
            </div>
        </section>
    }
}

// ===============================================
// 6. CTA - Chamada para ação
// ===============================================
#[component]
fn CTA() -> impl IntoView {
    view! {
        <section class="cta">
            <h2>"Pronto para resolver seu problema?"</h2>
            <p>"Entre em contato e vamos conversar (sem compromisso)"</p>

            <div class="contact-options">
                <a href="mailto:nicolas@avila.inc" class="contact-btn email">
                    <span class="icon">"📧"</span>
                    "nicolas@avila.inc"
                </a>

                <a href="https://wa.me/5517997811471" class="contact-btn whatsapp">
                    <span class="icon">"📱"</span>
                    "+55 17 99781-1471"
                </a>
            </div>

            <p class="guarantee">"💚 Primeira consulta gratuita - 30 minutos"</p>
        </section>
    }
}
