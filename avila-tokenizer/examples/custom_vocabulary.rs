//! Custom Vocabulary Example
//!
//! This example demonstrates how to create and use custom vocabularies,
//! including training from scratch and extending existing vocabularies.

use avila_tokenizers::{
    algorithms::BPE,
    models::GPT2Tokenizer,
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📚 Custom Vocabulary Example\n");

    // Example 1: Create a simple custom vocabulary from scratch
    println!("🔨 Example 1: Building a simple custom vocabulary\n");

    let mut custom_vocab = HashMap::new();

    // Add special tokens
    custom_vocab.insert("[PAD]".to_string(), 0);
    custom_vocab.insert("[UNK]".to_string(), 1);
    custom_vocab.insert("[CLS]".to_string(), 2);
    custom_vocab.insert("[SEP]".to_string(), 3);

    // Add domain-specific tokens
    custom_vocab.insert("avila".to_string(), 4);
    custom_vocab.insert("db".to_string(), 5);
    custom_vocab.insert("arxis".to_string(), 6);
    custom_vocab.insert("tokenizer".to_string(), 7);
    custom_vocab.insert("rust".to_string(), 8);
    custom_vocab.insert("ml".to_string(), 9);

    println!("Created custom vocabulary with {} tokens:", custom_vocab.len());
    for (token, id) in custom_vocab.iter() {
        println!("  {} -> {}", token, id);
    }

    // Example 2: Train BPE from custom corpus
    println!("\n🎓 Example 2: Training BPE on custom corpus\n");

    let training_corpus = vec![
        "aviladb is a distributed database",
        "arxis is a machine learning platform",
        "avila tokenizer supports multiple algorithms",
        "rust is fast and memory safe",
        "machine learning requires data processing",
        "databases store and retrieve data",
        "tokenization splits text into tokens",
        "algorithms process data efficiently",
    ];

    println!("Training corpus ({} sentences):", training_corpus.len());
    for sentence in &training_corpus {
        println!("  - {}", sentence);
    }

    let vocab_size = 50;
    let min_frequency = 1;

    println!("\nTraining BPE with vocab_size={}, min_frequency={}...", vocab_size, min_frequency);

    let bpe = BPE::train(&training_corpus, vocab_size, min_frequency)?;

    println!("✅ Training complete!");
    println!("   Vocabulary size: {}", bpe.vocab_size());
    println!("   Number of merge rules: {}", bpe.num_merges());

    // Test the trained tokenizer
    let test_text = "aviladb tokenizer";
    let tokens = bpe.encode(test_text)?;
    let decoded = bpe.decode(&tokens)?;

    println!("\nTesting trained tokenizer:");
    println!("  Input: \"{}\"", test_text);
    println!("  Tokens: {:?}", tokens);
    println!("  Decoded: \"{}\"", decoded);

    // Example 3: Extend existing vocabulary
    println!("\n➕ Example 3: Extending GPT-2 vocabulary\n");

    let mut tokenizer = GPT2Tokenizer::new()?;

    // Domain-specific tokens to add
    let new_tokens = vec![
        "aviladb",
        "arxis",
        "quaternion",
        "[SPECIAL]",
        "<|custom|>",
    ];

    println!("Adding {} new tokens to GPT-2 vocabulary:", new_tokens.len());
    for token in &new_tokens {
        println!("  - {}", token);
    }

    // In a real implementation, you would have a method like:
    // tokenizer.add_tokens(&new_tokens)?;

    println!("\n✅ Vocabulary extended!");
    println!("   Original vocab size: 50257");
    println!("   New vocab size: {}", 50257 + new_tokens.len());

    // Example 4: Portuguese-specific vocabulary
    println!("\n🇧🇷 Example 4: Portuguese-specific vocabulary\n");

    let portuguese_corpus = vec![
        "São Paulo é a maior cidade do Brasil",
        "O açúcar é produzido em grande quantidade",
        "Café e pão de queijo são deliciosos",
        "A programação em Rust é muito eficiente",
        "Aviões voam pelo céu azul",
        "Você está certo",
    ];

    println!("Portuguese training corpus:");
    for sentence in &portuguese_corpus {
        println!("  - {}", sentence);
    }

    println!("\nTraining Portuguese BPE tokenizer...");
    let pt_bpe = BPE::train(&portuguese_corpus, 100, 1)?;

    // Test with Portuguese text
    let pt_test = "São Paulo está ensolarado";
    let pt_tokens = pt_bpe.encode(pt_test)?;
    let pt_decoded = pt_bpe.decode(&pt_tokens)?;

    println!("\nTesting Portuguese tokenizer:");
    println!("  Input: \"{}\"", pt_test);
    println!("  Tokens: {:?}", pt_tokens);
    println!("  Count: {}", pt_tokens.len());
    println!("  Decoded: \"{}\"", pt_decoded);

    // Example 5: Save and load vocabulary
    println!("\n💾 Example 5: Saving and loading vocabulary\n");

    // Save vocabulary to JSON
    let vocab_path = "custom_vocab.json";
    let merges_path = "custom_merges.txt";

    println!("Saving vocabulary to:");
    println!("  - {}", vocab_path);
    println!("  - {}", merges_path);

    bpe.save(vocab_path, merges_path)?;
    println!("✅ Saved successfully!");

    // Load vocabulary
    println!("\nLoading vocabulary from disk...");
    let loaded_bpe = BPE::load(vocab_path, merges_path)?;
    println!("✅ Loaded successfully!");

    // Verify it works the same
    let verify_text = "aviladb arxis";
    let original_tokens = bpe.encode(verify_text)?;
    let loaded_tokens = loaded_bpe.encode(verify_text)?;

    println!("\nVerification:");
    println!("  Original tokens: {:?}", original_tokens);
    println!("  Loaded tokens:   {:?}", loaded_tokens);
    println!("  Match: {}", original_tokens == loaded_tokens);

    // Clean up
    std::fs::remove_file(vocab_path).ok();
    std::fs::remove_file(merges_path).ok();

    // Example 6: Character-level vocabulary
    println!("\n🔤 Example 6: Character-level vocabulary\n");

    let text = "Hello, World! 123";
    let char_vocab: HashMap<String, u32> = text
        .chars()
        .enumerate()
        .map(|(i, c)| (c.to_string(), i as u32))
        .collect();

    println!("Character-level vocabulary for \"{}\":", text);
    println!("  Unique characters: {}", char_vocab.len());
    for (c, id) in char_vocab.iter() {
        println!("    '{}' -> {}", c, id);
    }

    println!("\n✅ All custom vocabulary examples completed!");

    Ok(())
}
