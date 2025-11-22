//! Example: GPT-2 Tokenization
//!
//! This example demonstrates how to use the GPT-2 tokenizer to encode and decode text.

use avila_tokenizers::models::GPT2Tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== GPT-2 Tokenizer Example ===\n");

    // Load GPT-2 tokenizer
    let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2")?;
    println!("✓ Loaded GPT-2 tokenizer with {} tokens\n", tokenizer.vocab_size());

    // Example 1: Basic encoding
    let text = "Hello, world! How are you today?";
    println!("Input text: \"{}\"", text);

    let ids = tokenizer.encode(text);
    println!("Token IDs: {:?}", ids);
    println!("Token count: {}\n", ids.len());

    // Example 2: Decoding
    let decoded = tokenizer.decode(&ids)?;
    println!("Decoded text: \"{}\"\n", decoded);

    // Example 3: Tokenization (get actual token strings)
    let tokens = tokenizer.tokenize(text);
    println!("Tokens: {:?}\n", tokens);

    // Example 4: Special tokens
    let text_with_special = "This is a test.";
    let ids_with_special = tokenizer.encode_with_special(text_with_special, true);
    println!("Text with special tokens: \"{}\"", text_with_special);
    println!("IDs: {:?}", ids_with_special);
    println!("(includes <|endoftext|> token at the end)\n");

    // Example 5: Batch encoding
    let texts = vec![
        "First sentence.",
        "Second sentence with more words.",
        "Third!",
    ];
    println!("Batch encoding {} texts:", texts.len());
    let batch_ids = tokenizer.encode_batch(&texts);
    for (i, (text, ids)) in texts.iter().zip(batch_ids.iter()).enumerate() {
        println!("  {}: \"{}\" -> {} tokens", i + 1, text, ids.len());
    }
    println!();

    // Example 6: Pair encoding
    let text_a = "Question: What is AI?";
    let text_b = "Answer: Artificial Intelligence.";
    println!("Encoding text pair:");
    println!("  A: \"{}\"", text_a);
    println!("  B: \"{}\"", text_b);
    let pair_ids = tokenizer.encode_pair(text_a, text_b);
    println!("  Combined IDs: {:?} ({} tokens)\n", pair_ids, pair_ids.len());

    // Example 7: Portuguese text
    let portuguese_text = "Olá, como você está? Tudo bem?";
    println!("Portuguese text: \"{}\"", portuguese_text);
    let pt_ids = tokenizer.encode(portuguese_text);
    println!("Token IDs: {:?}", pt_ids);
    println!("Token count: {}\n", pt_ids.len());

    // Example 8: Padding and truncation
    let short_text = "Short";
    let mut ids = tokenizer.encode(short_text);
    println!("Original: \"{}\" -> {} tokens", short_text, ids.len());

    ids = tokenizer.pad(ids, 10, 0);
    println!("Padded to 10: {:?}", ids);

    ids = tokenizer.truncate(ids, 5);
    println!("Truncated to 5: {:?}\n", ids);

    // Example 9: Token lookup
    if let Some(token) = tokenizer.id_to_token(262) {
        println!("Token ID 262: \"{}\"", token);
    }
    if let Some(id) = tokenizer.token_to_id("Hello") {
        println!("Token \"Hello\": ID {}\n", id);
    }

    // Example 10: Count tokens (useful for API limits)
    let long_text = "This is a longer text that we want to count tokens for. It has multiple sentences and punctuation!";
    let token_count = tokenizer.count_tokens(long_text);
    println!("Long text: \"{}\"", long_text);
    println!("Token count: {} (useful for GPT-4 API limits)\n", token_count);

    println!("=== Example Complete ===");
    Ok(())
}
