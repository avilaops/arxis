#[cfg(test)]
mod gpt2_tests {
    use avila_tokenizers::models::GPT2Tokenizer;

    #[test]
    fn test_gpt2_creation() {
        let result = GPT2Tokenizer::from_pretrained("gpt2");
        assert!(result.is_ok());
    }

    #[test]
    fn test_gpt2_encode_basic() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let text = "Hello world";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
        assert!(ids.len() > 0);
    }

    #[test]
    fn test_gpt2_encode_decode() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let text = "Hello";
        let ids = tokenizer.encode(text);
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_gpt2_empty_text() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let ids = tokenizer.encode("");
        assert_eq!(ids.len(), 0);
    }

    #[test]
    fn test_gpt2_special_tokens() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let text = "Test";
        let ids = tokenizer.encode_with_special(text, true);
        assert!(ids.len() > 0);
    }

    #[test]
    fn test_gpt2_batch_encode() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let texts = vec!["Hello", "World", "Test"];
        let batch = tokenizer.encode_batch(&texts);
        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn test_gpt2_vocab_size() {
        let tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        // Our implementation uses a simplified vocab for demonstration
        assert!(tokenizer.vocab_size() > 200);
        assert!(tokenizer.vocab_size() <= 300);
    }

    #[test]
    fn test_gpt2_padding() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let ids = tokenizer.encode("Test");
        let padded = tokenizer.pad(ids, 10, 0);
        assert_eq!(padded.len(), 10);
    }

    #[test]
    fn test_gpt2_truncation() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let ids = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let truncated = tokenizer.truncate(ids, 5);
        assert_eq!(truncated.len(), 5);
    }

    #[test]
    fn test_gpt2_pair_encoding() {
        let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
        let ids = tokenizer.encode_pair("First", "Second");
        assert!(ids.len() > 0);
    }
}
