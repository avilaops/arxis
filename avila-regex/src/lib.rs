//! Avila Regex - Regex nativo simplificado
//! Substitui regex crate - suporte básico

pub struct Regex {
    pattern: String,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Self, &'static str> {
        Ok(Self {
            pattern: pattern.to_string(),
        })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.find(text).is_some()
    }

    pub fn find(&self, text: &str) -> Option<Match> {
        // Implementação básica - suporta alguns padrões simples
        if self.pattern.starts_with('^') && self.pattern.ends_with('$') {
            // Match exato
            let pat = &self.pattern[1..self.pattern.len() - 1];
            if text == pat {
                return Some(Match { start: 0, end: text.len() });
            }
        } else if self.pattern.starts_with('^') {
            // Match no início
            let pat = &self.pattern[1..];
            if text.starts_with(pat) {
                return Some(Match { start: 0, end: pat.len() });
            }
        } else if self.pattern.ends_with('$') {
            // Match no fim
            let pat = &self.pattern[..self.pattern.len() - 1];
            if text.ends_with(pat) {
                let start = text.len() - pat.len();
                return Some(Match { start, end: text.len() });
            }
        } else {
            // Match em qualquer lugar
            if let Some(pos) = text.find(&self.pattern) {
                return Some(Match {
                    start: pos,
                    end: pos + self.pattern.len(),
                });
            }
        }
        None
    }

    pub fn captures(&self, text: &str) -> Option<Captures> {
        self.find(text).map(|m| Captures {
            text: text.to_string(),
            matches: vec![m],
        })
    }

    pub fn replace(&self, text: &str, replacement: &str) -> String {
        if let Some(m) = self.find(text) {
            let mut result = String::new();
            result.push_str(&text[..m.start]);
            result.push_str(replacement);
            result.push_str(&text[m.end..]);
            result
        } else {
            text.to_string()
        }
    }

    pub fn replace_all(&self, text: &str, replacement: &str) -> String {
        text.replace(&self.pattern, replacement)
    }
}

pub struct Match {
    pub start: usize,
    pub end: usize,
}

impl Match {
    pub fn as_str<'a>(&self, text: &'a str) -> &'a str {
        &text[self.start..self.end]
    }
}

pub struct Captures {
    text: String,
    matches: Vec<Match>,
}

impl Captures {
    pub fn get(&self, index: usize) -> Option<&str> {
        self.matches.get(index).map(|m| m.as_str(&self.text))
    }
}
