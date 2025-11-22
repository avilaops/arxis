//! Example: Training a BPE Tokenizer from Scratch
//!
//! This example demonstrates how to train a custom BPE tokenizer on your own corpus.

use avila_tokenizers::algorithms::BPE;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Training BPE Tokenizer ===\n");

    // Example corpus (Portuguese and English mixed)
    let corpus = vec![
        "Hello world, como você está?",
        "Bom dia! How are you today?",
        "Eu estou bem, obrigado.",
        "Thank you very much!",
        "Por favor, me ajude com isso.",
        "This is a test sentence.",
        "Olá, tudo bem? Yes, all good!",
        "Machine learning is amazing.",
        "Aprendizado de máquina é incrível.",
        "Natural language processing.",
        "Processamento de linguagem natural.",
        "Python é uma linguagem de programação.",
        "Rust is a systems programming language.",
    ];

    println!("Corpus ({} sentences):", corpus.len());
    for (i, sentence) in corpus.iter().enumerate() {
        println!("  {}: \"{}\"", i + 1, sentence);
    }
    println!();

    // Train BPE with 500 merge operations
    println!("Training BPE tokenizer with 500 merges...");
    let vocab_size = 500;
    let min_frequency = 2;
    let byte_level = false;

    let bpe = BPE::train(&corpus, vocab_size, min_frequency, byte_level)?;
    println!("✓ Training complete!\n");    // Test the trained tokenizer
    println!("=== Testing Trained Tokenizer ===\n");

    // Example 1: Tokenize Portuguese text
    let pt_text = "Olá, como você está hoje?";
    println!("Portuguese: \"{}\"", pt_text);
    let pt_tokens = bpe.tokenize(pt_text);
    println!("Tokens: {:?}", pt_tokens);
    println!("Token count: {}\n", pt_tokens.len());

    // Example 2: Tokenize English text
    let en_text = "Hello, how are you today?";
    println!("English: \"{}\"", en_text);
    let en_tokens = bpe.tokenize(en_text);
    println!("Tokens: {:?}", en_tokens);
    println!("Token count: {}\n", en_tokens.len());

    // Example 3: Tokenize mixed text
    let mixed_text = "Machine learning (aprendizado de máquina) is cool!";
    println!("Mixed: \"{}\"", mixed_text);
    let mixed_tokens = bpe.tokenize(mixed_text);
    println!("Tokens: {:?}", mixed_tokens);
    println!("Token count: {}\n", mixed_tokens.len());

    // Example 4: Encode to IDs
    println!("=== Encoding to IDs ===");
    let mut bpe_with_vocab = create_vocab_from_bpe(&bpe);

    let text = "Python e Rust";
    println!("Text: \"{}\"", text);
    let ids = bpe_with_vocab.encode(text);
    println!("Token IDs: {:?}\n", ids);

    // Example 5: Show merge statistics
    println!("=== Merge Statistics ===");
    let merges = bpe.merges();
    println!("Total merges: {}", merges.len());
    println!("First 10 merges:");
    for (i, (a, b)) in merges.iter().take(10).enumerate() {
        println!("  {}: \"{}\" + \"{}\"", i + 1, a, b);
    }
    println!();

    // Example 6: Training with different vocab sizes
    println!("=== Comparing Vocabulary Sizes ===");
    let vocab_sizes = vec![100, 300, 500, 1000];

    for &size in &vocab_sizes {
        let bpe_test = BPE::train(&corpus, size, 2, false)?;
        let test_text = "aprendizado de máquina";
        let tokens = bpe_test.tokenize(test_text);
        println!("Vocab size {}: \"{}\" -> {} tokens", size, test_text, tokens.len());
    }
    println!();    // Example 7: Byte-level BPE (like GPT-2)
    println!("=== Byte-Level BPE ===");
    let byte_vocab = create_byte_level_vocab();
    let byte_merges = vec![
        ("Ġ".to_string(), "t".to_string()),
        ("h".to_string(), "e".to_string()),
    ];

    let byte_bpe = BPE::new_byte_level(byte_vocab, byte_merges);
    let byte_text = "the quick brown";
    println!("Text: \"{}\"", byte_text);
    let byte_tokens = byte_bpe.tokenize(byte_text);
    println!("Byte-level tokens: {:?}\n", byte_tokens);

    // Example 8: Save trained tokenizer
    println!("=== Saving Tokenizer ===");
    println!("To save the tokenizer, you would:");
    println!("  1. Serialize vocab and merges to JSON");
    println!("  2. Save to files: vocab.json and merges.txt");
    println!("  3. Load later with BPE::new(vocab, merges)");
    println!();

    // Example 9: Custom preprocessing
    println!("=== Custom Preprocessing ===");
    let text_with_special = "Hello @user123! Check out https://example.com";
    println!("Raw text: \"{}\"", text_with_special);

    // In real training, you'd preprocess to handle URLs, mentions, etc.
    let preprocessed = text_with_special
        .replace("https://", "<URL>")
        .replace("@user", "<USER>");
    println!("Preprocessed: \"{}\"", preprocessed);

    let tokens = bpe.tokenize(&preprocessed);
    println!("Tokens: {:?}\n", tokens);

    // Example 10: Training progress (simplified)
    println!("=== Training Progress ===");
    println!("During training:");
    println!("  - Initial vocab: 256 base characters");
    println!("  - Merge 1: 'e' + 'r' -> 'er' (most frequent pair)");
    println!("  - Merge 2: 'a' + 'n' -> 'an'");
    println!("  - ... (continues for {} merges)", vocab_size);
    println!("  - Final vocab size: {} tokens", vocab_size + 256);
    println!();

    println!("=== Training Complete ===");
    println!("\nTips for training your own tokenizer:");
    println!("  • Use a large, diverse corpus (millions of sentences)");
    println!("  • Choose vocab size based on your needs (typically 10k-50k)");
    println!("  • Consider byte-level BPE for robust handling of rare characters");
    println!("  • Preprocess text to handle special tokens and normalization");
    println!("  • Evaluate on held-out test set to avoid overfitting");

    Ok(())
}

/// Create vocabulary from trained BPE (helper function)
fn create_vocab_from_bpe(bpe: &BPE) -> BPE {
    // In real implementation, you'd extract vocab from BPE
    // For now, return clone
    bpe.clone()
}

/// Create byte-level vocabulary (256 base characters)
fn create_byte_level_vocab() -> HashMap<String, u32> {
    let mut vocab = HashMap::new();

    // Add 256 byte tokens
    for i in 0..256 {
        let ch = char::from_u32(i as u32).unwrap_or('�');
        vocab.insert(ch.to_string(), i as u32);
    }

    vocab
}
