#[cfg(test)]
mod unicode_tests {
    use avila_tokenizers::utils::unicode::{
        normalize_nfc, normalize_nfkc, normalize_nfd, normalize_nfkd,
        strip_accents, is_punctuation, is_whitespace, is_digit,
        byte_to_unicode, unicode_to_byte,
    };

    #[test]
    fn test_nfc_normalization() {
        // Composed form
        let text = "café";
        let normalized = normalize_nfc(text);
        assert_eq!(normalized, "café");
    }

    #[test]
    fn test_nfkc_normalization() {
        let text = "ﬁ"; // ligature
        let normalized = normalize_nfkc(text);
        assert_eq!(normalized, "fi");
    }

    #[test]
    fn test_nfd_normalization() {
        let text = "café";
        let normalized = normalize_nfd(text);
        // Should decompose é into e + combining accent
        assert!(normalized.len() >= text.len());
    }

    #[test]
    fn test_strip_accents() {
        assert_eq!(strip_accents("café"), "cafe");
        assert_eq!(strip_accents("São Paulo"), "Sao Paulo");
        assert_eq!(strip_accents("José"), "Jose");
        assert_eq!(strip_accents("Ação"), "Acao");
    }

    #[test]
    fn test_portuguese_accents() {
        // Test all Portuguese special characters
        assert_eq!(strip_accents("á"), "a");
        assert_eq!(strip_accents("é"), "e");
        assert_eq!(strip_accents("í"), "i");
        assert_eq!(strip_accents("ó"), "o");
        assert_eq!(strip_accents("ú"), "u");
        assert_eq!(strip_accents("ã"), "a");
        assert_eq!(strip_accents("õ"), "o");
        assert_eq!(strip_accents("ç"), "c");
    }

    #[test]
    fn test_is_punctuation() {
        assert!(is_punctuation('.'));
        assert!(is_punctuation(','));
        assert!(is_punctuation('!'));
        assert!(is_punctuation('?'));
        assert!(!is_punctuation('a'));
        assert!(!is_punctuation('1'));
    }

    #[test]
    fn test_is_whitespace() {
        assert!(is_whitespace(' '));
        assert!(is_whitespace('\t'));
        assert!(is_whitespace('\n'));
        assert!(!is_whitespace('a'));
    }

    #[test]
    fn test_is_digit() {
        assert!(is_digit('0'));
        assert!(is_digit('5'));
        assert!(is_digit('9'));
        assert!(!is_digit('a'));
        assert!(!is_digit('.'));
    }

    #[test]
    fn test_byte_to_unicode() {
        let mapping = byte_to_unicode();
        assert_eq!(mapping.len(), 256);

        // Every byte should map to a char
        for byte in 0..=255 {
            assert!(mapping.contains_key(&byte));
        }
    }

    #[test]
    fn test_unicode_to_byte() {
        let mapping = unicode_to_byte();
        assert_eq!(mapping.len(), 256);

        // Should be inverse of byte_to_unicode
        let byte_map = byte_to_unicode();
        for (&byte, &ch) in byte_map.iter() {
            assert_eq!(mapping.get(&ch), Some(&byte));
        }
    }

    #[test]
    fn test_byte_unicode_roundtrip() {
        let byte_to_uni = byte_to_unicode();
        let uni_to_byte = unicode_to_byte();

        for byte in 0..=255 {
            let ch = byte_to_uni[&byte];
            let back = uni_to_byte[&ch];
            assert_eq!(byte, back);
        }
    }

    #[test]
    fn test_mixed_unicode() {
        // Mix of Latin, Portuguese, and other scripts
        let text = "Hello Olá こんにちは 你好";
        let nfc = normalize_nfc(text);
        assert!(!nfc.is_empty());
    }

    #[test]
    fn test_emoji_handling() {
        let text = "Hello 👋 World 🌍";
        let nfc = normalize_nfc(text);
        assert!(nfc.contains("👋"));
        assert!(nfc.contains("🌍"));
    }
}
