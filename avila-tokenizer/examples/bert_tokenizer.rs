//! Example: BERT Tokenization
//!
//! This example demonstrates how to use the BERT tokenizer for NLP tasks.

use avila_tokenizers::models::BertTokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== BERT Tokenizer Example ===\n");

    // Load BERT tokenizer
    let tokenizer = BertTokenizer::from_pretrained("bert-base-uncased")?;
    println!("✓ Loaded BERT tokenizer with {} tokens\n", tokenizer.vocab_size());

    // Example 1: Basic encoding
    let text = "Hello, world! How are you today?";
    println!("Input text: \"{}\"", text);

    let ids = tokenizer.encode(text);
    println!("Token IDs: {:?}", ids);
    println!("Token count: {}\n", ids.len());

    // Example 2: Encoding with special tokens [CLS] and [SEP]
    let text_with_special = "This is a test.";
    let ids_with_special = tokenizer.encode_with_special(text_with_special);
    println!("Text with special tokens: \"{}\"", text_with_special);
    println!("IDs: {:?}", ids_with_special);
    println!("First token (101) = [CLS], Last token (102) = [SEP]\n");

    // Example 3: Decoding
    let decoded = tokenizer.decode(&ids)?;
    println!("Decoded text: \"{}\"\n", decoded);

    // Example 4: Tokenization (get actual token strings)
    let tokens = tokenizer.tokenize(text);
    println!("Tokens: {:?}", tokens);
    println!("(Note: subwords have ## prefix)\n");

    // Example 5: Sentence pair encoding (for BERT sentence classification)
    let sentence_a = "The cat sat on the mat.";
    let sentence_b = "The dog stood on the rug.";
    println!("Encoding sentence pair:");
    println!("  A: \"{}\"", sentence_a);
    println!("  B: \"{}\"", sentence_b);
    let pair_ids = tokenizer.encode_pair(sentence_a, sentence_b);
    println!("  Combined IDs: {:?}", pair_ids);
    println!("  Format: [CLS] sentence_a [SEP] sentence_b [SEP]\n");

    // Example 6: Create attention mask
    let text_with_padding = "Short text.";
    let mut ids = tokenizer.encode_with_special(text_with_padding);
    ids = tokenizer.pad(ids, 10); // Pad to length 10

    println!("Padded IDs: {:?}", ids);
    let attention_mask = tokenizer.create_attention_mask(&ids);
    println!("Attention mask: {:?}", attention_mask);
    println!("(1 = real token, 0 = padding)\n");

    // Example 7: Create token type IDs
    let pair_text = tokenizer.encode_pair("First", "Second");
    let token_type_ids = tokenizer.create_token_type_ids(&pair_text);
    println!("Token type IDs: {:?}", token_type_ids);
    println!("(0 = first sequence, 1 = second sequence)\n");

    // Example 8: Batch encoding
    let texts = vec![
        "First sentence.",
        "Second sentence with more words.",
        "Third!",
    ];
    println!("Batch encoding {} texts:", texts.len());
    let batch_ids = tokenizer.encode_batch_with_special(&texts);
    for (i, (text, ids)) in texts.iter().zip(batch_ids.iter()).enumerate() {
        println!("  {}: \"{}\" -> {} tokens", i + 1, text, ids.len());
    }
    println!();

    // Example 9: Portuguese text (uncased)
    let portuguese_text = "Olá, como você está? Tudo bem?";
    println!("Portuguese text: \"{}\"", portuguese_text);
    let pt_ids = tokenizer.encode(portuguese_text);
    println!("Token IDs: {:?}", pt_ids);
    println!("Token count: {}", pt_ids.len());
    let pt_tokens = tokenizer.tokenize(portuguese_text);
    println!("Tokens: {:?}\n", pt_tokens);

    // Example 10: Special token lookup
    let special_tokens = tokenizer.get_special_tokens();
    println!("Special tokens:");
    for (token, id) in special_tokens.iter() {
        println!("  {}: {}", token, id);
    }
    println!();

    // Example 11: Padding and truncation
    let long_text = "This is a very long sentence that needs to be truncated to fit within the model's maximum sequence length.";
    println!("Original text: \"{}\"", long_text);
    let ids = tokenizer.encode_with_special(long_text);
    println!("Original length: {} tokens", ids.len());

    let padded_truncated = tokenizer.pad_and_truncate(ids, 20);
    println!("After pad_and_truncate(20): {:?}\n", padded_truncated);

    // Example 12: Check if token is special
    println!("Token checks:");
    println!("  '[CLS]' is special: {}", tokenizer.is_special_token("[CLS]"));
    println!("  'hello' is special: {}", tokenizer.is_special_token("hello"));
    println!();

    // Example 13: Masked Language Modeling (MLM) example
    let mlm_text = "The capital of France is [MASK].";
    println!("MLM example: \"{}\"", mlm_text);
    let mlm_ids = tokenizer.encode_with_special(mlm_text);
    println!("IDs with [MASK] token (103): {:?}\n", mlm_ids);

    println!("=== Example Complete ===");
    Ok(())
}
