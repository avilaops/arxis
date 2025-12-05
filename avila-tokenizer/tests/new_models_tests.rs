#[cfg(test)]
mod new_models_tests {
    use avila_tokenizers::models::{
        claude::ClaudeTokenizer,
        falcon::FalconTokenizer,
        gemini::GeminiTokenizer,
        mistral::MistralTokenizer,
    };

    // Claude Tests
    #[test]
    fn test_claude_models_load() {
        assert!(ClaudeTokenizer::from_pretrained("claude-1").is_ok());
        assert!(ClaudeTokenizer::from_pretrained("claude-2").is_ok());
        assert!(ClaudeTokenizer::from_pretrained("claude-3-sonnet").is_ok());
        assert!(ClaudeTokenizer::from_pretrained("claude-3.5-sonnet").is_ok());
    }

    #[test]
    fn test_claude_encode_decode() {
        let mut tokenizer = ClaudeTokenizer::from_pretrained("claude-3-sonnet").unwrap();
        let text = "Hello, I'm Claude, an AI assistant created by Anthropic.";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
        
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_claude_batch_encoding() {
        let mut tokenizer = ClaudeTokenizer::from_pretrained("claude-3-sonnet").unwrap();
        let texts = vec![
            "First text",
            "Second text",
            "Third text with more content",
        ];
        let batch = tokenizer.encode_batch(&texts);
        assert_eq!(batch.len(), 3);
        assert!(batch[2].len() > batch[0].len()); // Longer text = more tokens
    }

    #[test]
    fn test_claude_portuguese() {
        let mut tokenizer = ClaudeTokenizer::from_pretrained("claude-3-sonnet").unwrap();
        let text = "Olá, como você está hoje? Espero que esteja bem!";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_claude_empty_text() {
        let mut tokenizer = ClaudeTokenizer::from_pretrained("claude-2").unwrap();
        let ids = tokenizer.encode("");
        assert!(ids.is_empty());
    }

    // Falcon Tests
    #[test]
    fn test_falcon_models_load() {
        assert!(FalconTokenizer::from_pretrained("falcon-7b").is_ok());
        assert!(FalconTokenizer::from_pretrained("falcon-40b").is_ok());
        assert!(FalconTokenizer::from_pretrained("falcon-180b").is_ok());
    }

    #[test]
    fn test_falcon_encode_decode() {
        let mut tokenizer = FalconTokenizer::from_pretrained("falcon-7b").unwrap();
        let text = "The Falcon is a large language model from TII.";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
        
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_falcon_technical_text() {
        let mut tokenizer = FalconTokenizer::from_pretrained("falcon-7b").unwrap();
        let text = "Machine learning and data science research on computer algorithms.";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_falcon_180b_multilingual() {
        let mut tokenizer = FalconTokenizer::from_pretrained("falcon-180b").unwrap();
        let texts = vec![
            "Hello in English",
            "Olá em português",
            "Hola en español",
        ];
        for text in texts {
            let ids = tokenizer.encode(text);
            assert!(!ids.is_empty(), "Failed on: {}", text);
        }
    }

    #[test]
    fn test_falcon_vocab_size() {
        let tokenizer = FalconTokenizer::from_pretrained("falcon-7b").unwrap();
        assert!(tokenizer.vocab_size() > 0);
    }

    // Gemini Tests
    #[test]
    fn test_gemini_models_load() {
        assert!(GeminiTokenizer::from_pretrained("gemini-pro").is_ok());
        assert!(GeminiTokenizer::from_pretrained("gemini-ultra").is_ok());
        assert!(GeminiTokenizer::from_pretrained("gemini-flash").is_ok());
        assert!(GeminiTokenizer::from_pretrained("gemini-1.5-pro").is_ok());
    }

    #[test]
    fn test_gemini_encode_decode() {
        let tokenizer = GeminiTokenizer::from_pretrained("gemini-pro").unwrap();
        let text = "Google's Gemini is a highly capable multimodal AI.";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
        
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_gemini_multilingual_extensive() {
        let tokenizer = GeminiTokenizer::from_pretrained("gemini-pro").unwrap();
        let texts = vec![
            "English text",
            "Texto em português",
            "Texto en español",
            "Texte en français",
            "Text auf Deutsch",
            "日本語のテキスト",
            "中文文本",
            "한국어 텍스트",
        ];
        
        for text in texts {
            let ids = tokenizer.encode(text);
            assert!(!ids.is_empty(), "Failed on: {}", text);
        }
    }

    #[test]
    fn test_gemini_portuguese_brasil() {
        let tokenizer = GeminiTokenizer::from_pretrained("gemini-pro").unwrap();
        let text = "O Brasil é um país maravilhoso com cultura rica!";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_gemini_batch_encoding() {
        let tokenizer = GeminiTokenizer::from_pretrained("gemini-flash").unwrap();
        let texts = vec!["Short", "Medium length text", "A much longer piece of text here"];
        let batch = tokenizer.encode_batch(&texts);
        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn test_gemini_vocab_size() {
        let tokenizer = GeminiTokenizer::from_pretrained("gemini-pro").unwrap();
        // Gemini has a large multilingual vocabulary
        assert!(tokenizer.vocab_size() > 100); // More than base tokens
    }

    // Mistral Tests
    #[test]
    fn test_mistral_load() {
        assert!(MistralTokenizer::from_pretrained("mistral-7b").is_ok());
    }

    #[test]
    fn test_mistral_encode_decode() {
        let tokenizer = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        let text = "Mistral AI creates powerful open models.";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
        
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_mistral_batch() {
        let tokenizer = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        let texts = vec!["First", "Second", "Third"];
        let batch = tokenizer.encode_batch(&texts);
        assert_eq!(batch.len(), 3);
    }

    // Cross-model compatibility tests
    #[test]
    fn test_all_models_handle_same_text() {
        let text = "The quick brown fox jumps over the lazy dog.";
        
        let mut claude = ClaudeTokenizer::from_pretrained("claude-3-sonnet").unwrap();
        let mut falcon = FalconTokenizer::from_pretrained("falcon-7b").unwrap();
        let gemini = GeminiTokenizer::from_pretrained("gemini-pro").unwrap();
        let mistral = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        
        let claude_ids = claude.encode(text);
        let falcon_ids = falcon.encode(text);
        let gemini_ids = gemini.encode(text);
        let mistral_ids = mistral.encode(text);
        
        // All should produce tokens
        assert!(!claude_ids.is_empty());
        assert!(!falcon_ids.is_empty());
        assert!(!gemini_ids.is_empty());
        assert!(!mistral_ids.is_empty());
    }

    #[test]
    fn test_all_models_portuguese() {
        let text = "Olá, como você está?";
        
        let mut claude = ClaudeTokenizer::from_pretrained("claude-3-sonnet").unwrap();
        let mut falcon = FalconTokenizer::from_pretrained("falcon-180b").unwrap();
        let gemini = GeminiTokenizer::from_pretrained("gemini-pro").unwrap();
        let mistral = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        
        assert!(!claude.encode(text).is_empty());
        assert!(!falcon.encode(text).is_empty());
        assert!(!gemini.encode(text).is_empty());
        assert!(!mistral.encode(text).is_empty());
    }

    #[test]
    fn test_all_models_empty_input() {
        let mut claude = ClaudeTokenizer::from_pretrained("claude-2").unwrap();
        let mut falcon = FalconTokenizer::from_pretrained("falcon-7b").unwrap();
        let gemini = GeminiTokenizer::from_pretrained("gemini-pro").unwrap();
        let mistral = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        
        assert!(claude.encode("").is_empty());
        assert!(falcon.encode("").is_empty());
        assert!(gemini.encode("").is_empty());
        assert!(mistral.encode("").is_empty());
    }

    #[test]
    fn test_all_models_unicode() {
        let text = "Hello 世界 🌍 emoji test";
        
        let mut claude = ClaudeTokenizer::from_pretrained("claude-3-sonnet").unwrap();
        let mut falcon = FalconTokenizer::from_pretrained("falcon-7b").unwrap();
        let gemini = GeminiTokenizer::from_pretrained("gemini-pro").unwrap();
        let mistral = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        
        assert!(!claude.encode(text).is_empty());
        assert!(!falcon.encode(text).is_empty());
        assert!(!gemini.encode(text).is_empty());
        assert!(!mistral.encode(text).is_empty());
    }

    #[test]
    fn test_all_models_long_text() {
        let text = "Lorem ipsum dolor sit amet. ".repeat(50);
        
        let mut claude = ClaudeTokenizer::from_pretrained("claude-2").unwrap();
        let mut falcon = FalconTokenizer::from_pretrained("falcon-7b").unwrap();
        let gemini = GeminiTokenizer::from_pretrained("gemini-flash").unwrap();
        let mistral = MistralTokenizer::from_pretrained("mistral-7b").unwrap();
        
        let claude_ids = claude.encode(&text);
        let falcon_ids = falcon.encode(&text);
        let gemini_ids = gemini.encode(&text);
        let mistral_ids = mistral.encode(&text);
        
        assert!(claude_ids.len() > 50);
        assert!(falcon_ids.len() > 50);
        assert!(gemini_ids.len() > 50);
        assert!(mistral_ids.len() > 50);
    }
}
