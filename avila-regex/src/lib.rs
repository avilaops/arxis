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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let re = Regex::new("^test$").unwrap();
        assert!(re.is_match("test"));
        assert!(!re.is_match("testing"));
    }

    #[test]
    fn test_start_match() {
        let re = Regex::new("^hello").unwrap();
        assert!(re.is_match("hello world"));
        assert!(!re.is_match("say hello"));
    }

    #[test]
    fn test_end_match() {
        let re = Regex::new("world$").unwrap();
        assert!(re.is_match("hello world"));
        assert!(!re.is_match("world hello"));
    }

    #[test]
    fn test_contains_match() {
        let re = Regex::new("rust").unwrap();
        assert!(re.is_match("I love rust"));
        assert!(!re.is_match("I love go"));
    }

    #[test]
    fn test_replace() {
        let re = Regex::new("world").unwrap();
        let result = re.replace("hello world", "Rust");
        assert_eq!(result, "hello Rust");
    }

    #[test]
    fn test_replace_all() {
        let re = Regex::new("o").unwrap();
        let result = re.replace_all("foo bar boo", "0");
        assert_eq!(result, "f00 bar b00");
    }
}
