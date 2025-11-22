#[cfg(test)]
mod llama_tests {
    use avila_tokenizers::models::LlamaTokenizer;

    #[test]
    fn test_llama2_creation() {
        let result = LlamaTokenizer::from_pretrained("llama-2-7b");
        assert!(result.is_ok());
    }

    #[test]
    fn test_llama3_creation() {
        let result = LlamaTokenizer::from_pretrained("llama-3-8b");
        assert!(result.is_ok());
    }

    #[test]
    fn test_llama_encode_basic() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        let text = "Hello world";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_llama_special_tokens() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        let text = "Test";
        let ids = tokenizer.encode_with_special(text);

        // Should start with <s> (BOS token, id=1)
        assert_eq!(ids[0], 1);
    }

    #[test]
    fn test_llama_decode() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        let text = "Hello";
        let ids = tokenizer.encode(text);
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_llama_vocab_size() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        // Our simplified implementation
        assert!(tokenizer.vocab_size() > 200);
    }

    #[test]
    fn test_llama3_vocab_size() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-3-8b").unwrap();
        // Our simplified implementation
        assert!(tokenizer.vocab_size() > 200);
    }

    #[test]
    fn test_llama_is_special_token() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        assert!(tokenizer.is_special_token("<s>"));
        assert!(tokenizer.is_special_token("</s>"));
        assert!(tokenizer.is_special_token("<unk>"));
        assert!(!tokenizer.is_special_token("hello"));
    }

    #[test]
    fn test_llama_chat_template() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        let messages = vec![
            ("system", "You are helpful"),
            ("user", "Hello"),
        ];
        let formatted = tokenizer.apply_chat_template(&messages);

        assert!(formatted.contains("<<SYS>>"));
        assert!(formatted.contains("[INST]"));
    }

    #[test]
    fn test_llama3_chat_template() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-3-8b").unwrap();
        let messages = vec![
            ("user", "Hello"),
        ];
        let formatted = tokenizer.apply_chat_template_llama3(&messages);

        assert!(formatted.contains("<|start_header_id|>"));
        assert!(formatted.contains("<|end_header_id|>"));
    }

    #[test]
    fn test_llama_batch_encode() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        let texts = vec!["Hello", "World", "Test"];
        let batch = tokenizer.encode_batch(&texts);
        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn test_llama_padding() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        let ids = vec![1, 2, 3];
        let padded = tokenizer.pad(ids, 10);
        assert_eq!(padded.len(), 10);
    }

    #[test]
    fn test_llama_truncation() {
        let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b").unwrap();
        let ids = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let truncated = tokenizer.truncate(ids, 5);
        assert_eq!(truncated.len(), 5);
    }

    #[test]
    fn test_mistral_creation() {
        let result = LlamaTokenizer::from_pretrained("mistral-7b");
        assert!(result.is_ok());
    }

    #[test]
    fn test_code_llama_creation() {
        let result = LlamaTokenizer::from_pretrained("code-llama");
        assert!(result.is_ok());
    }
}
