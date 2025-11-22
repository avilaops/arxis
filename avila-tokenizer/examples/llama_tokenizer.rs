//! Example: Llama Tokenization
//!
//! This example demonstrates how to use the Llama tokenizer for LLM applications.

use avila_tokenizers::models::LlamaTokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Llama Tokenizer Example ===\n");

    // Load Llama 2 tokenizer
    let tokenizer = LlamaTokenizer::from_pretrained("llama-2-7b")?;
    println!("✓ Loaded Llama 2 tokenizer with {} tokens\n", tokenizer.vocab_size());

    // Example 1: Basic encoding
    let text = "Hello, world! How are you today?";
    println!("Input text: \"{}\"", text);

    let ids = tokenizer.encode(text);
    println!("Token IDs: {:?}", ids);
    println!("Token count: {}\n", ids.len());

    // Example 2: Encoding with special tokens <s> and </s>
    let text_with_special = "This is a test.";
    let ids_with_special = tokenizer.encode_with_special(text_with_special);
    println!("Text with special tokens: \"{}\"", text_with_special);
    println!("IDs: {:?}", ids_with_special);
    println!("First token (1) = <s> (beginning of sequence)\n");

    // Example 3: Decoding
    let decoded = tokenizer.decode(&ids)?;
    println!("Decoded text: \"{}\"\n", decoded);

    // Example 4: Tokenization (get actual token strings)
    let tokens = tokenizer.tokenize(text);
    println!("Tokens: {:?}", tokens);
    println!("(Note: ▁ represents space in SentencePiece)\n");

    // Example 5: Portuguese text
    let portuguese_text = "Olá, como você está? Tudo bem com você?";
    println!("Portuguese text: \"{}\"", portuguese_text);
    let pt_ids = tokenizer.encode(portuguese_text);
    println!("Token IDs: {:?}", pt_ids);
    println!("Token count: {}", pt_ids.len());
    let pt_tokens = tokenizer.tokenize(portuguese_text);
    println!("Tokens: {:?}\n", pt_tokens);

    // Example 6: Chat template (Llama 2 style)
    println!("=== Chat Template Example (Llama 2) ===");
    let messages = vec![
        ("system", "You are a helpful AI assistant that speaks Portuguese."),
        ("user", "Olá! Como você está?"),
        ("assistant", "Olá! Estou bem, obrigado. Como posso ajudar você hoje?"),
        ("user", "Me conte sobre o Brasil."),
    ];

    let formatted = tokenizer.apply_chat_template(&messages);
    println!("Formatted chat:\n{}\n", formatted);

    let chat_ids = tokenizer.encode_with_special(&formatted);
    println!("Chat token count: {}\n", chat_ids.len());

    // Example 7: Llama 3 chat template (different format)
    let tokenizer_llama3 = LlamaTokenizer::from_pretrained("llama-3-8b")?;
    println!("=== Chat Template Example (Llama 3) ===");
    let formatted_llama3 = tokenizer_llama3.apply_chat_template_llama3(&messages);
    println!("Formatted chat (Llama 3 style):\n{}\n", formatted_llama3);

    // Example 8: Batch encoding
    let texts = vec![
        "First sentence in Portuguese: Olá!",
        "Second sentence: Como vai?",
        "Third: Tudo bem?",
    ];
    println!("Batch encoding {} texts:", texts.len());
    let batch_ids = tokenizer.encode_batch(&texts);
    for (i, (text, ids)) in texts.iter().zip(batch_ids.iter()).enumerate() {
        println!("  {}: \"{}\" -> {} tokens", i + 1, text, ids.len());
    }
    println!();

    // Example 9: Special token lookup
    let special_tokens = tokenizer.get_special_tokens();
    println!("Special tokens:");
    for (token, id) in special_tokens.iter() {
        println!("  {}: {}", token, id);
    }
    println!();

    // Example 10: Token scores (log probabilities)
    println!("Token scores:");
    if let Some(score) = tokenizer.get_score("▁the") {
        println!("  '▁the': {:.2}", score);
    }
    if let Some(score) = tokenizer.get_score("▁Hello") {
        println!("  '▁Hello': {:.2}", score);
    }
    println!();

    // Example 11: Padding and truncation
    let short_text = "Short";
    let mut ids = tokenizer.encode_with_special(short_text);
    println!("Original: \"{}\" -> {} tokens", short_text, ids.len());

    ids = tokenizer.pad(ids, 15);
    println!("Padded to 15: {:?}", ids);

    ids = tokenizer.truncate(ids, 8);
    println!("Truncated to 8: {:?}\n", ids);

    // Example 12: Instruction-following format
    let instruction = "Translate the following text to Portuguese:";
    let input_text = "Hello, how are you?";
    let prompt = format!("{}\n\nInput: {}\n\nOutput:", instruction, input_text);

    println!("Instruction-following prompt:");
    println!("{}", prompt);
    let prompt_ids = tokenizer.encode_with_special(&prompt);
    println!("Token count: {}\n", prompt_ids.len());

    // Example 13: Code generation (Code Llama)
    let code_llama = LlamaTokenizer::from_pretrained("code-llama")?;
    println!("=== Code Llama Example ===");
    let code = r#"
def hello_world():
    print("Hello, World!")
    return True
"#;
    println!("Code snippet:");
    println!("{}", code);
    let code_ids = code_llama.encode(code);
    println!("Token count: {}\n", code_ids.len());

    // Example 14: Multi-turn conversation
    println!("=== Multi-turn Conversation ===");
    let conversation = vec![
        ("user", "What is the capital of Brazil?"),
        ("assistant", "The capital of Brazil is Brasília."),
        ("user", "Tell me more about it."),
    ];
    let conv_formatted = tokenizer.apply_chat_template(&conversation);
    println!("{}", conv_formatted);

    println!("=== Example Complete ===");
    Ok(())
}
