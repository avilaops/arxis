use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    /// GPT-2 regex pattern for pre-tokenization
    /// Matches: contractions, letters, numbers, and other characters
    pub static ref GPT2_PATTERN: Regex = Regex::new(
        r"'s|'t|'re|'ve|'m|'ll|'d| ?\p{L}+| ?\p{N}+| ?[^\s\p{L}\p{N}]+|\s+"
    ).expect("Failed to compile GPT2_PATTERN");

    /// BERT-style whitespace pattern
    pub static ref WHITESPACE_PATTERN: Regex = Regex::new(r"\s+").unwrap();

    /// Pattern to match punctuation
    pub static ref PUNCTUATION_PATTERN: Regex = Regex::new(
        r"[!#$%&'()*+,\-./:;<=>?@\[\\\]^_`{|}~]"
    ).unwrap();

    /// Pattern to match digits
    pub static ref DIGIT_PATTERN: Regex = Regex::new(r"\d+").unwrap();

    /// Pattern to match Chinese characters
    pub static ref CHINESE_PATTERN: Regex = Regex::new(
        r"[\u{4E00}-\u{9FFF}\u{3400}-\u{4DBF}\u{20000}-\u{2A6DF}]"
    ).unwrap();

    /// Pattern to match control characters
    pub static ref CONTROL_PATTERN: Regex = Regex::new(r"[\x00-\x1F\x7F]").unwrap();

    /// Pattern to split on whitespace and punctuation
    pub static ref SPLIT_PATTERN: Regex = Regex::new(r"(\s+|[^\w\s]+)").unwrap();

    /// Pattern for SentencePiece metaspace (underscores representing spaces)
    pub static ref METASPACE_PATTERN: Regex = Regex::new(r"▁").unwrap();

    /// Pattern to match word boundaries
    pub static ref WORD_BOUNDARY_PATTERN: Regex = Regex::new(r"\b").unwrap();

    /// Pattern for email addresses
    pub static ref EMAIL_PATTERN: Regex = Regex::new(
        r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b"
    ).unwrap();

    /// Pattern for URLs
    pub static ref URL_PATTERN: Regex = Regex::new(
        r"https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b(?:[-a-zA-Z0-9()@:%_\+.~#?&/=]*)"
    ).unwrap();

    /// Pattern for numbers with separators (e.g., 1,000.50)
    pub static ref NUMBER_PATTERN: Regex = Regex::new(
        r"-?\d{1,3}(?:,\d{3})*(?:\.\d+)?|\d+"
    ).unwrap();
}

/// Split text using GPT-2 pattern
pub fn gpt2_split(text: &str) -> Vec<String> {
    GPT2_PATTERN
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Split text on whitespace
pub fn whitespace_split(text: &str) -> Vec<String> {
    WHITESPACE_PATTERN
        .split(text)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Split text preserving punctuation
pub fn split_with_punctuation(text: &str) -> Vec<String> {
    SPLIT_PATTERN
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .filter(|s| !s.trim().is_empty())
        .collect()
}

/// Remove control characters using regex
pub fn remove_control_characters(text: &str) -> String {
    CONTROL_PATTERN.replace_all(text, "").to_string()
}

/// Check if text contains Chinese characters
pub fn contains_chinese(text: &str) -> bool {
    CHINESE_PATTERN.is_match(text)
}

/// Extract all URLs from text
pub fn extract_urls(text: &str) -> Vec<String> {
    URL_PATTERN
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Extract all email addresses from text
pub fn extract_emails(text: &str) -> Vec<String> {
    EMAIL_PATTERN
        .find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

/// Replace metaspace character with regular space
pub fn replace_metaspace(text: &str) -> String {
    text.replace('▁', " ")
}

/// Add metaspace character instead of regular space
pub fn add_metaspace(text: &str) -> String {
    text.replace(' ', "▁")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpt2_split() {
        let text = "Hello world";
        let tokens = gpt2_split(text);
        assert!(!tokens.is_empty());
        // Basic sanity check - just ensure it processes text
        assert!(tokens.len() > 0);
    }

    #[test]
    fn test_whitespace_split() {
        let text = "Hello   world\t\ntest";
        let tokens = whitespace_split(text);
        assert_eq!(tokens, vec!["Hello", "world", "test"]);
    }

    #[test]
    fn test_split_with_punctuation() {
        let text = "Hello, world!";
        let tokens = split_with_punctuation(text);
        // Should split text into parts
        assert!(!tokens.is_empty());
        // Should contain punctuation or text parts
        assert!(tokens.len() >= 1);
    }

    #[test]
    fn test_contains_chinese() {
        assert!(contains_chinese("你好世界"));
        assert!(contains_chinese("Hello 世界"));
        assert!(!contains_chinese("Hello World"));
    }

    #[test]
    fn test_extract_urls() {
        let text = "Visit https://example.com and http://test.org for more info.";
        let urls = extract_urls(text);
        assert_eq!(urls.len(), 2);
        assert!(urls.contains(&"https://example.com".to_string()));
    }

    #[test]
    fn test_extract_emails() {
        let text = "Contact us at info@example.com or support@test.org";
        let emails = extract_emails(text);
        assert_eq!(emails.len(), 2);
        assert!(emails.contains(&"info@example.com".to_string()));
    }

    #[test]
    fn test_metaspace() {
        let text = "Hello World";
        let with_metaspace = add_metaspace(text);
        assert_eq!(with_metaspace, "Hello▁World");

        let back = replace_metaspace(&with_metaspace);
        assert_eq!(back, text);
    }
}
