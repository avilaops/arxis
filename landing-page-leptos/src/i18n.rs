use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    PtBR,
    En,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::PtBR => "pt-BR",
            Language::En => "en",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pt-BR" => Some(Language::PtBR),
            "en" => Some(Language::En),
            _ => None,
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Language::PtBR => Language::En,
            Language::En => Language::PtBR,
        }
    }

    pub fn flag(&self) -> &'static str {
        match self {
            Language::PtBR => "🇧🇷",
            Language::En => "🇬🇧",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::PtBR => "PT",
            Language::En => "EN",
        }
    }
}

// Translations
pub struct T;

impl T {
    pub fn hero_title(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Detectando Colisões de Buracos Negros",
            Language::En => "Detecting Black Hole Collisions",
        }
    }

    pub fn hero_subtitle(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Quando dois buracos negros colidem, eles criam ondas no próprio tecido do espaço. Nós ajudamos cientistas a detectar e estudar esses eventos cósmicos.",
            Language::En => "When two black holes collide, they create waves in the very fabric of space. We help scientists detect and study these cosmic events.",
        }
    }

    pub fn hero_description(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "É como jogar uma pedra num lago - as ondas se espalham. Só que no universo, são eventos milhões de vezes mais massivos que o Sol criando essas ondas.",
            Language::En => "It's like throwing a stone in a lake - the waves spread out. Except in the universe, these are events millions of times more massive than the Sun creating these waves.",
        }
    }

    pub fn get_started(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Entenda como funciona",
            Language::En => "Understand how it works",
        }
    }

    pub fn view_github(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Por que isso importa",
            Language::En => "Why this matters",
        }
    }

    pub fn features_title(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "O que fazemos",
            Language::En => "What we do",
        }
    }

    pub fn nasa_lisa_title(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Detectamos Ondas Gravitacionais",
            Language::En => "We Detect Gravitational Waves",
        }
    }

    pub fn nasa_lisa_desc(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Imagine o universo como um oceano. Quando eventos massivos acontecem - como dois buracos negros colidindo - eles criam \"ondas\" no espaço-tempo. Nós construímos as ferramentas que detectam essas ondas.",
            Language::En => "Imagine the universe as an ocean. When massive events happen - like two black holes colliding - they create \"waves\" in spacetime. We build the tools that detect these waves.",
        }
    }

    pub fn quaternions_title(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Identificamos o que Aconteceu",
            Language::En => "We Identify What Happened",
        }
    }

    pub fn quaternions_desc(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Cada onda gravitacional conta uma história: dois buracos negros se chocando, uma estrela sendo engolida, eventos que nunca vimos antes. Nossa tecnologia identifica qual evento causou cada onda.",
            Language::En => "Each gravitational wave tells a story: two black holes colliding, a star being swallowed, events we've never seen before. Our technology identifies which event caused each wave.",
        }
    }

    pub fn tensors_title(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Trabalhamos com a NASA",
            Language::En => "We Work with NASA",
        }
    }

    pub fn tensors_desc(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "A missão LISA da NASA vai colocar detectores no espaço para ouvir o universo. Nosso sistema ajuda a processar esses sinais. Participamos de uma missão espacial real.",
            Language::En => "NASA's LISA mission will place detectors in space to listen to the universe. Our system helps process these signals. We're part of a real space mission.",
        }
    }

    pub fn gr_title(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Relatividade Geral",
            Language::En => "General Relativity",
        }
    }

    pub fn gr_desc(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Kit completo de RG com métricas, geodésicas e tensores de curvatura.",
            Language::En => "Comprehensive GR toolkit with metrics, geodesics, and curvature tensors.",
        }
    }

    pub fn contact_title(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Entre em Contato",
            Language::En => "Get in Touch",
        }
    }

    pub fn footer_tagline(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "A Cidadela Matemática",
            Language::En => "The Mathematical Citadel",
        }
    }

    pub fn footer_description(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Física e matemática de nível científico em Rust",
            Language::En => "Research-grade physics & mathematics in Rust",
        }
    }

    pub fn resources(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Recursos",
            Language::En => "Resources",
        }
    }

    pub fn community(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Comunidade",
            Language::En => "Community",
        }
    }

    pub fn about(lang: Language) -> &'static str {
        match lang {
            Language::PtBR => "Sobre",
            Language::En => "About",
        }
    }
}
