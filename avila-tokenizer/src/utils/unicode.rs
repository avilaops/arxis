use unicode_normalization::{UnicodeNormalization, char::is_combining_mark};

/// Normalize text to NFC (Canonical Composition)
/// Example: é (U+0065 U+0301) -> é (U+00E9)
pub fn normalize_nfc(text: &str) -> String {
    text.nfc().collect()
}

/// Normalize text to NFD (Canonical Decomposition)
/// Example: é (U+00E9) -> é (U+0065 U+0301)
pub fn normalize_nfd(text: &str) -> String {
    text.nfd().collect()
}

/// Normalize text to NFKC (Compatibility Composition)
/// Example: ﬁ (U+FB01) -> fi (U+0066 U+0069)
pub fn normalize_nfkc(text: &str) -> String {
    text.nfkc().collect()
}

/// Normalize text to NFKD (Compatibility Decomposition)
pub fn normalize_nfkd(text: &str) -> String {
    text.nfkd().collect()
}

/// Strip accents from text
/// Example: "café" -> "cafe", "São Paulo" -> "Sao Paulo"
pub fn strip_accents(text: &str) -> String {
    text.nfd()
        .filter(|c| !is_combining_mark(*c))
        .collect()
}

/// Check if character is a control character
pub fn is_control(c: char) -> bool {
    c.is_control() || c == '\u{FEFF}' // Include zero-width no-break space
}

/// Remove control characters from text
pub fn remove_control_chars(text: &str) -> String {
    text.chars().filter(|c| !is_control(*c)).collect()
}

/// Check if character is whitespace (including Unicode whitespace)
pub fn is_whitespace(c: char) -> bool {
    c.is_whitespace() || matches!(c,
        '\u{00A0}' | // Non-breaking space
        '\u{1680}' | // Ogham space mark
        '\u{2000}'..='\u{200A}' | // Various spaces
        '\u{202F}' | // Narrow no-break space
        '\u{205F}' | // Medium mathematical space
        '\u{3000}'   // Ideographic space
    )
}

/// Normalize whitespace to single space
pub fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Check if character is punctuation
pub fn is_punctuation(c: char) -> bool {
    matches!(c,
        '!' | '"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' |
        '-' | '.' | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\' |
        ']' | '^' | '_' | '`' | '{' | '|' | '}' | '~'
    ) || c.is_ascii_punctuation()
}

/// Check if character is a digit
pub fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Check if character is alphabetic (including accented characters)
pub fn is_alphabetic(c: char) -> bool {
    c.is_alphabetic()
}

/// Convert byte to GPT-2 style Unicode character
/// GPT-2 uses a special mapping for bytes to make them printable
pub fn byte_to_unicode() -> std::collections::HashMap<u8, char> {
    let mut bs: Vec<u8> = Vec::new();

    // Printable ASCII
    bs.extend(b'!'..=b'~');
    bs.extend(b'\xA1'..=b'\xAC');
    bs.extend(b'\xAE'..=b'\xFF');

    let mut cs: Vec<u32> = bs.iter().map(|&b| b as u32).collect();
    let mut n = 0u32;

    for b in 0u8..=255u8 {
        if !bs.contains(&b) {
            bs.push(b);
            cs.push(256 + n);
            n += 1;
        }
    }

    bs.into_iter()
        .zip(cs.into_iter().map(|c| char::from_u32(c).unwrap()))
        .collect()
}

/// Create inverse mapping from unicode character to byte
pub fn unicode_to_byte() -> std::collections::HashMap<char, u8> {
    byte_to_unicode()
        .into_iter()
        .map(|(k, v)| (v, k))
        .collect()
}

/// Check if text is valid UTF-8
pub fn is_valid_utf8(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes).is_ok()
}

/// Convert text to lowercase preserving special characters
pub fn lowercase_preserve_special(text: &str) -> String {
    text.chars()
        .map(|c| {
            if is_alphabetic(c) {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfc_normalization() {
        let nfd = "e\u{0301}"; // e + combining acute accent
        let nfc = normalize_nfc(nfd);
        assert_eq!(nfc, "é");
    }

    #[test]
    fn test_nfkc_normalization() {
        let text = "ﬁ"; // U+FB01 (ligature)
        let normalized = normalize_nfkc(text);
        assert_eq!(normalized, "fi");
    }

    #[test]
    fn test_strip_accents() {
        assert_eq!(strip_accents("café"), "cafe");
        assert_eq!(strip_accents("São Paulo"), "Sao Paulo");
        assert_eq!(strip_accents("naïve"), "naive");
    }

    #[test]
    fn test_remove_control_chars() {
        let text = "Hello\x00World\x1F!";
        let cleaned = remove_control_chars(text);
        assert_eq!(cleaned, "HelloWorld!");
    }

    #[test]
    fn test_normalize_whitespace() {
        let text = "Hello    world\t\n\r   test";
        let normalized = normalize_whitespace(text);
        assert_eq!(normalized, "Hello world test");
    }

    #[test]
    fn test_is_punctuation() {
        assert!(is_punctuation('!'));
        assert!(is_punctuation('.'));
        assert!(is_punctuation(','));
        assert!(!is_punctuation('a'));
        assert!(!is_punctuation('5'));
    }

    #[test]
    fn test_byte_to_unicode() {
        let mapping = byte_to_unicode();
        assert_eq!(mapping.len(), 256);
        assert!(mapping.contains_key(&b'A'));
        assert!(mapping.contains_key(&0));
        assert!(mapping.contains_key(&255));
    }

    #[test]
    fn test_unicode_to_byte() {
        let byte_map = byte_to_unicode();
        let unicode_map = unicode_to_byte();

        assert_eq!(byte_map.len(), unicode_map.len());

        for (byte, unicode_char) in byte_map.iter() {
            assert_eq!(unicode_map.get(unicode_char), Some(byte));
        }
    }

    #[test]
    fn test_lowercase_preserve_special() {
        assert_eq!(lowercase_preserve_special("Hello!"), "hello!");
        assert_eq!(lowercase_preserve_special("Test123"), "test123");
        assert_eq!(lowercase_preserve_special("ÁÉÍ"), "áéí");
    }
}
