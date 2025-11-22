#[cfg(test)]
mod bert_tests {
    use avila_tokenizers::models::BertTokenizer;

    #[test]
    fn test_bert_creation() {
        let result = BertTokenizer::from_pretrained("bert-base-uncased");
        assert!(result.is_ok());
    }

    #[test]
    fn test_bert_encode_basic() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let text = "Hello world";
        let ids = tokenizer.encode(text);
        assert!(!ids.is_empty());
    }

    #[test]
    fn test_bert_special_tokens() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let text = "Test";
        let ids = tokenizer.encode_with_special(text);

        // First token should be [CLS] (101)
        assert_eq!(ids[0], 101);

        // Last token should be [SEP] (102)
        assert_eq!(ids[ids.len() - 1], 102);
    }

    #[test]
    fn test_bert_pair_encoding() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let ids = tokenizer.encode_pair("First", "Second");

        // Should have [CLS], [SEP], and [SEP]
        assert!(ids.len() > 3);
        assert_eq!(ids[0], 101); // [CLS]
    }

    #[test]
    fn test_bert_decode() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let text = "Hello";
        let ids = tokenizer.encode(text);
        let decoded = tokenizer.decode(&ids).unwrap();
        assert!(!decoded.is_empty());
    }

    #[test]
    fn test_bert_attention_mask() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let ids = vec![101, 2023, 2003, 0, 0]; // [CLS] this is [PAD] [PAD]
        let mask = tokenizer.create_attention_mask(&ids);
        assert_eq!(mask, vec![1, 1, 1, 0, 0]);
    }

    #[test]
    fn test_bert_token_type_ids() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let ids = tokenizer.encode_pair("A", "B");
        let type_ids = tokenizer.create_token_type_ids(&ids);

        // Should have 0s and 1s
        assert!(type_ids.contains(&0));
        assert!(type_ids.contains(&1));
    }

    #[test]
    fn test_bert_vocab_size() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        assert!(tokenizer.vocab_size() > 100);
    }

    #[test]
    fn test_bert_is_special_token() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        assert!(tokenizer.is_special_token("[CLS]"));
        assert!(tokenizer.is_special_token("[SEP]"));
        assert!(tokenizer.is_special_token("[PAD]"));
        assert!(!tokenizer.is_special_token("hello"));
    }

    #[test]
    fn test_bert_batch_encode() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let texts = vec!["Hello", "World", "Test"];
        let batch = tokenizer.encode_batch(&texts);
        assert_eq!(batch.len(), 3);
    }

    #[test]
    fn test_bert_padding() {
        let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased").unwrap();
        let ids = vec![101, 2023, 102]; // [CLS] this [SEP]
        let padded = tokenizer.pad(ids, 10);
        assert_eq!(padded.len(), 10);
        assert_eq!(padded[padded.len() - 1], 0); // [PAD]
    }
}
