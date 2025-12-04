use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="font-family: Arial, sans-serif; padding: 20px;">
            <h1 style="color: #3b82f6;">{"🚀 Vizzio - Substituto para Augin"}</h1>
            <p>{"Aplicação web desenvolvida em Rust + Avila Framework"}</p>
            <p>{"Recursos principais:"}</p>
            <ul>
                <li>{"Interface moderna e responsiva"}</li>
                <li>{"Criptografia avançada com Avila Crypto"}</li>
                <li>{"Detecção de objetos com YOLO"}</li>
                <li>{"Processamento geoespacial"}</li>
            </ul>
            <button style="background: #3b82f6; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer;">
                {"Começar"}
            </button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
