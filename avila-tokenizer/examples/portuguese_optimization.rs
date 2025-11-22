//! Example: Portuguese Optimization
//!
//! This example demonstrates specific optimizations for Portuguese text tokenization.

use avila_tokenizers::{
    models::{BertTokenizer, LlamaTokenizer, GPT2Tokenizer},
    normalizers::{Normalizer, NFKCNormalizer, LowercaseNormalizer, StripAccents},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Portuguese Tokenization Optimization ===\n");

    // Portuguese test texts
    let texts = vec![
        "Olá, como você está?",
        "Bom dia! Tudo bem?",
        "O açúcar está na cozinha.",
        "São Paulo é a maior cidade do Brasil.",
        "Não é possível, né?",
        "Vou dar uma olhada, tá?",
        "D'água, d'ouro, d'alma.",
        "Pra fazer, pr'aquilo.",
        "José, João, María, André.",
        "Você está com quantos anos?",
    ];

    println!("Test corpus ({} sentences):", texts.len());
    for (i, text) in texts.iter().enumerate() {
        println!("  {}: \"{}\"", i + 1, text);
    }
    println!();

    // Example 1: Compare tokenizers on Portuguese
    println!("=== Tokenizer Comparison ===\n");

    let mut gpt2 = GPT2Tokenizer::from_pretrained("gpt2")?;
    let bert = BertTokenizer::from_pretrained("bert-base-uncased")?;
    let llama = LlamaTokenizer::from_pretrained("llama-2-7b")?;

    for text in texts.iter().take(3) {
        println!("Text: \"{}\"", text);

        let gpt2_ids = gpt2.encode(text);
        println!("  GPT-2: {} tokens", gpt2_ids.len());

        let bert_ids = bert.encode(text);
        println!("  BERT: {} tokens", bert_ids.len());

        let llama_ids = llama.encode(text);
        println!("  Llama: {} tokens", llama_ids.len());

        println!();
    }

    // Example 2: Accent handling
    println!("=== Accent Handling ===\n");

    let accented = "José María está com açúcar e café.";
    println!("Original: \"{}\"", accented);

    // Option 1: Preserve accents (recommended for Portuguese)
    let nfkc = NFKCNormalizer;
    let with_accents = nfkc.normalize(accented);
    println!("NFKC (preserve): \"{}\"", with_accents);

    // Option 2: Strip accents (only if needed for legacy systems)
    let strip = StripAccents;
    let without_accents = strip.normalize(accented);
    println!("Stripped: \"{}\"", without_accents);

    // Tokenize both
    let ids_with = llama.encode(&with_accents);
    let ids_without = llama.encode(&without_accents);
    println!("With accents: {} tokens", ids_with.len());
    println!("Without accents: {} tokens", ids_without.len());
    println!();

    // Example 3: Portuguese contractions
    println!("=== Portuguese Contractions ===\n");

    let contractions = vec![
        "d'água",
        "d'ouro",
        "l'água",
        "n'água",
        "do", "da", "dos", "das",
        "no", "na", "nos", "nas",
        "pelo", "pela", "pelos", "pelas",
        "ao", "à", "aos", "às",
    ];

    println!("Common contractions:");
    for contraction in contractions.iter().take(8) {
        let ids = llama.encode(contraction);
        let tokens = llama.tokenize(contraction);
        println!("  \"{}\": {} tokens -> {:?}", contraction, ids.len(), tokens);
    }
    println!();

    // Example 4: Brazilian Portuguese informal speech
    println!("=== Brazilian Informal Speech ===\n");

    let informal = vec![
        ("né", "não é"),
        ("tá", "está"),
        ("pra", "para"),
        ("pro", "para o"),
        ("cê", "você"),
        ("ocê", "você"),
        ("tô", "estou"),
        ("tava", "estava"),
    ];

    println!("Informal vs Formal:");
    for (informal_form, formal_form) in informal.iter() {
        let inf_ids = llama.encode(informal_form);
        let form_ids = llama.encode(formal_form);
        println!("  \"{}\" ({} tokens) vs \"{}\" ({} tokens)",
                 informal_form, inf_ids.len(),
                 formal_form, form_ids.len());
    }
    println!();

    // Example 5: Portuguese names with special characters
    println!("=== Portuguese Names ===\n");

    let names = vec![
        "José Silva",
        "María García",
        "João Pedro",
        "André Luís",
        "São Paulo",
        "Brasília",
    ];

    println!("Names tokenization:");
    for name in names {
        let ids = llama.encode(name);
        let tokens = llama.tokenize(name);
        println!("  \"{}\": {} tokens -> {:?}", name, ids.len(), tokens);
    }
    println!();

    // Example 6: Case sensitivity
    println!("=== Case Sensitivity ===\n");

    let text_cases = "Olá MUNDO olá mundo OLÁ mUnDo";
    println!("Mixed case: \"{}\"", text_cases);

    // BERT uncased (lowercase)
    let bert_uncased = BertTokenizer::from_pretrained("bert-base-uncased")?;
    let bert_ids = bert_uncased.encode(text_cases);
    println!("BERT uncased: {} tokens", bert_ids.len());

    // BERT cased (preserve case)
    let bert_cased = BertTokenizer::from_pretrained("bert-base-cased")?;
    let bert_cased_ids = bert_cased.encode(text_cases);
    println!("BERT cased: {} tokens", bert_cased_ids.len());

    // Llama (case-sensitive via SentencePiece)
    let llama_ids = llama.encode(text_cases);
    println!("Llama: {} tokens\n", llama_ids.len());

    // Example 7: Portuguese punctuation
    println!("=== Portuguese Punctuation ===\n");

    let punct_text = "Olá! Como está? Tudo bem... Sim, está ótimo: perfeito; excelente.";
    println!("Text: \"{}\"", punct_text);

    let gpt2_ids = gpt2.encode(punct_text);
    println!("GPT-2: {} tokens", gpt2_ids.len());

    let bert_ids = bert.encode(punct_text);
    println!("BERT: {} tokens", bert_ids.len());

    let llama_ids = llama.encode(punct_text);
    println!("Llama: {} tokens\n", llama_ids.len());

    // Example 8: Numbers and dates
    println!("=== Numbers and Dates ===\n");

    let numbers = vec![
        "25 de março de 2024",
        "R$ 100,50",
        "3.14159",
        "Tel: (11) 98765-4321",
        "CPF: 123.456.789-00",
    ];

    println!("Number formatting:");
    for num in numbers {
        let ids = llama.encode(num);
        let tokens = llama.tokenize(num);
        println!("  \"{}\": {} tokens", num, ids.len());
        println!("    -> {:?}", tokens);
    }
    println!();

    // Example 9: Long Portuguese text
    println!("=== Long Text Tokenization ===\n");

    let long_text = "O Brasil é o maior país da América do Sul e o quinto maior do mundo em área territorial. \
                     A capital é Brasília, mas São Paulo é a maior cidade. \
                     O idioma oficial é o português, falado por mais de 200 milhões de pessoas. \
                     O país é conhecido por sua diversidade cultural, natural e linguística.";

    println!("Long text ({} chars):", long_text.len());
    println!("\"{}...\"\n", &long_text[..80]);

    let gpt2_ids = gpt2.encode(long_text);
    let bert_ids = bert.encode(long_text);
    let llama_ids = llama.encode(long_text);

    println!("Token counts:");
    println!("  GPT-2: {} tokens", gpt2_ids.len());
    println!("  BERT: {} tokens", bert_ids.len());
    println!("  Llama: {} tokens", llama_ids.len());
    println!();

    // Calculate compression ratio
    let char_count = long_text.len();
    println!("Compression ratios (chars/token):");
    println!("  GPT-2: {:.2}", char_count as f32 / gpt2_ids.len() as f32);
    println!("  BERT: {:.2}", char_count as f32 / bert_ids.len() as f32);
    println!("  Llama: {:.2}", char_count as f32 / llama_ids.len() as f32);
    println!();

    // Example 10: Best practices summary
    println!("=== Best Practices for Portuguese ===\n");
    println!("✓ Preserve accents (á, é, í, ó, ú, ã, õ, ç) for better semantics");
    println!("✓ Use NFKC normalization for consistency");
    println!("✓ Consider case sensitivity based on your task");
    println!("✓ Handle Brazilian informal speech if needed");
    println!("✓ Llama models have best Portuguese support (SentencePiece)");
    println!("✓ For Brazilian Portuguese: consider informal contractions");
    println!("✓ Test with your specific domain (medical, legal, tech, etc.)");
    println!();

    // Example 11: Recommendations by use case
    println!("=== Tokenizer Recommendations ===\n");
    println!("For Portuguese LLM applications:");
    println!("  → Llama 2/3: Best choice for Portuguese generation");
    println!("  → 32k vocab covers Portuguese well");
    println!("  → Handles accents and contractions naturally");
    println!();
    println!("For Portuguese NLP classification:");
    println!("  → BERT multilingual: Good for cross-lingual tasks");
    println!("  → BERTimbau: Specifically trained on Portuguese");
    println!("  → Consider fine-tuning on your domain");
    println!();
    println!("For Portuguese text analysis:");
    println!("  → GPT-2: Decent but not optimal for Portuguese");
    println!("  → Consider using Llama instead");
    println!();

    println!("=== Example Complete ===");
    Ok(())
}
