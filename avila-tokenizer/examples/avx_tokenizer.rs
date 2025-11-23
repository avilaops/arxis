//! Example: Avx Tokenizer
//!
//! Demonstrates the new Avx (Avila eXtended) tokenizer with multilingual support

use avila_tokenizers::models::AvxTokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Avx Tokenizer Example ===\n");

    // Example 1: Avx Base (64K tokens)
    println!("--- Avx Base (64K tokens) ---");
    let mut tokenizer_base = AvxTokenizer::from_pretrained("avx-base")?;
    println!("✓ Loaded Avx Base with {} tokens\n", tokenizer_base.vocab_size());

    let text = "Hello, world! How are you today?";
    println!("Input: \"{}\"", text);
    let ids = tokenizer_base.encode(text);
    println!("Token IDs: {:?}", ids);
    println!("Token count: {}\n", ids.len());

    let decoded = tokenizer_base.decode(&ids)?;
    println!("Decoded: \"{}\"\n", decoded);

    // Example 2: Avx Portuguese (48K tokens - optimized for PT-BR)
    println!("--- Avx Portuguese (48K tokens) ---");
    let mut tokenizer_pt = AvxTokenizer::from_pretrained("avx-pt-br")?;
    println!("✓ Loaded Avx PT-BR with {} tokens\n", tokenizer_pt.vocab_size());

    let pt_texts = vec![
        "Olá, como você está?",
        "Tudo bem? Vou dar uma olhada.",
        "São Paulo é incrível!",
        "Você é brasileiro, né?",
        "Beleza! Tá combinado então.",
    ];

    println!("Portuguese texts:");
    for text in &pt_texts {
        let ids = tokenizer_pt.encode(text);
        let decoded = tokenizer_pt.decode(&ids)?;
        println!("  Original: \"{}\"", text);
        println!("  Tokens: {} | Decoded: \"{}\"", ids.len(), decoded);
    }
    println!();

    // Example 3: Avx with Special Tokens
    println!("--- Special Tokens ---");
    let text = "This is a test.";
    println!("Input: \"{}\"", text);

    let ids_with_special = tokenizer_base.encode_with_special(text);
    println!("With special tokens: {:?}", ids_with_special);
    println!("First token (1) = <|begin|>\n");

    // Example 4: Chat Template
    println!("--- Chat Template (Avx Format) ---");
    let messages = vec![
        ("system", "You are a helpful AI assistant fluent in Portuguese."),
        ("user", "Olá! Pode me ajudar com algo?"),
        ("assistant", "Claro! Como posso ajudá-lo hoje?"),
        ("user", "Explique o que é inteligência artificial."),
    ];

    let formatted = tokenizer_base.apply_chat_template(&messages);
    println!("Formatted chat:\n{}", formatted);

    let chat_ids = tokenizer_base.encode_with_special(&formatted);
    println!("Chat tokens: {} tokens\n", chat_ids.len());

    // Example 5: Avx Multilingual
    println!("--- Avx Multilingual (96K tokens) ---");
    let mut tokenizer_multi = AvxTokenizer::from_pretrained("avx-multilingual")?;
    println!("✓ Loaded Avx Multilingual with {} tokens\n", tokenizer_multi.vocab_size());

    let multilingual_texts = vec![
        ("English", "Hello, world!"),
        ("Portuguese", "Olá, mundo!"),
        ("Spanish", "¡Hola, mundo!"),
        ("French", "Bonjour, monde!"),
    ];

    println!("Multilingual support:");
    for (lang, text) in &multilingual_texts {
        let ids = tokenizer_multi.encode(text);
        println!("  {}: \"{}\" -> {} tokens", lang, text, ids.len());
    }
    println!();

    // Example 6: Avx Large (128K tokens - with hybrid mode)
    println!("--- Avx Large (128K tokens - Hybrid BPE+Unigram) ---");
    let mut tokenizer_large = AvxTokenizer::from_pretrained("avx-large")?;
    println!("✓ Loaded Avx Large with {} tokens", tokenizer_large.vocab_size());
    println!("✓ Hybrid mode: BPE for common tokens, Unigram for rare\n");

    let complex_text = "The avila-tokenizers library supports GPT-2, BERT, Llama, and now Avx!";
    println!("Complex text: \"{}\"", complex_text);
    let ids = tokenizer_large.encode(complex_text);
    println!("Tokens: {}\n", ids.len());

    // Example 7: Batch Processing
    println!("--- Batch Processing ---");
    let batch_texts = vec![
        "First sentence.",
        "Segunda frase em português.",
        "Third sentence with more words.",
    ];

    println!("Batch encoding {} texts:", batch_texts.len());
    let batch_ids = tokenizer_base.encode_batch(&batch_texts);
    for (i, (text, ids)) in batch_texts.iter().zip(batch_ids.iter()).enumerate() {
        println!("  {}: \"{}\" -> {} tokens", i + 1, text, ids.len());
    }
    println!();

    // Example 8: Special Token Lookup
    println!("--- Special Tokens ---");
    let special_tokens = tokenizer_base.get_special_tokens();
    println!("Avx special tokens:");
    for (token, id) in special_tokens.iter() {
        println!("  {}: {}", token, id);
    }
    println!();

    // Example 9: Padding and Truncation
    println!("--- Padding & Truncation ---");
    let short_text = "Short";
    let mut ids = tokenizer_base.encode(short_text);
    println!("Original: \"{}\" -> {} tokens", short_text, ids.len());

    ids = tokenizer_base.pad(ids, 10);
    println!("Padded to 10: {:?}", ids);

    ids = tokenizer_base.truncate(ids, 5);
    println!("Truncated to 5: {:?}\n", ids);

    // Example 10: Comparing Models
    println!("=== Model Comparison ===");
    println!("| Model              | Vocab Size | Optimization       |");
    println!("|--------------------|-----------|--------------------|");
    println!("| Avx Base           | 64K       | Balanced           |");
    println!("| Avx PT-BR          | 48K       | Portuguese-focused |");
    println!("| Avx Multilingual   | 96K       | 100+ languages     |");
    println!("| Avx Large          | 128K      | Hybrid BPE+Unigram |");
    println!();

    println!("=== Example Complete ===");
    Ok(())
}
