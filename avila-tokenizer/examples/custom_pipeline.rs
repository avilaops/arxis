//! Example: Custom Pipeline
//!
//! This example demonstrates how to build a custom tokenization pipeline
//! by composing normalizers, pre-tokenizers, algorithms, and decoders.

use avila_tokenizers::{
    normalizers::{Normalizer, NFKCNormalizer, LowercaseNormalizer, StripAccents, SequenceNormalizer},
    pre_tokenizers::{PreTokenizer, WhitespaceSplit, PunctuationSplit, DigitsSplit},
    post_processors::{PostProcessor, BertProcessing},
    decoders::{Decoder, WordPieceDecoder, StripDecoder},
};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Custom Pipeline Example ===\n");

    // Example 1: Simple normalization pipeline
    println!("=== Normalization Pipeline ===");
    let text = "Héllö WORLD! Thís ís á tëst.";
    println!("Original: \"{}\"", text);

    // Step 1: NFKC normalization
    let nfkc = NFKCNormalizer;
    let normalized = nfkc.normalize(text);
    println!("After NFKC: \"{}\"", normalized);

    // Step 2: Strip accents
    let strip = StripAccents;
    let stripped = strip.normalize(&normalized);
    println!("After strip accents: \"{}\"", stripped);

    // Step 3: Lowercase
    let lowercase = LowercaseNormalizer;
    let lowercased = lowercase.normalize(&stripped);
    println!("After lowercase: \"{}\"\n", lowercased);

    // Example 2: Chained normalizers using SequenceNormalizer
    println!("=== Chained Normalizers ===");
    let text2 = "Olá MUNDO! Çömö está?";
    println!("Original: \"{}\"", text2);

    let normalizers: Vec<Box<dyn Normalizer>> = vec![
        Box::new(NFKCNormalizer),
        Box::new(StripAccents),
        Box::new(LowercaseNormalizer),
    ];
    let sequence = SequenceNormalizer::new(normalizers);
    let result = sequence.normalize(text2).unwrap_or_else(|_| text2.to_string());
    println!("After sequence: \"{}\"\n", result);

    // Example 3: Pre-tokenization pipeline
    println!("=== Pre-tokenization Pipeline ===");
    let text3 = "Hello, world! How are you? I'm fine.";
    println!("Original: \"{}\"", text3);

    // Step 1: Whitespace split
    let whitespace = WhitespaceSplit;
    let ws_tokens = whitespace.pre_tokenize(text3).unwrap_or_else(|_| vec![text3.to_string()]);
    println!("Whitespace split: {:?}", ws_tokens);

    // Step 2: Punctuation split
    let punct = PunctuationSplit;
    let mut punct_tokens = Vec::new();
    for token in ws_tokens {
        punct_tokens.extend(punct.pre_tokenize(&token).unwrap_or_else(|_| vec![token]));
    }
    println!("Punctuation split: {:?}\n", punct_tokens);

    // Example 4: Full custom pipeline for Portuguese
    println!("=== Portuguese Custom Pipeline ===");
    let pt_text = "Olá! Está TUDO bem? Você está com 25 anos.";
    println!("Original: \"{}\"", pt_text);

    // Normalize (preserve accents for Portuguese, but lowercase)
    let pt_normalizer = LowercaseNormalizer;
    let pt_normalized = pt_normalizer.normalize(pt_text).unwrap_or_else(|_| pt_text.to_string());
    println!("Normalized: \"{}\"", pt_normalized);

    // Pre-tokenize
    let pt_ws = WhitespaceSplit;
    let pt_tokens = pt_ws.pre_tokenize(&pt_normalized).unwrap_or_else(|_| vec![pt_normalized.clone()]);
    println!("Pre-tokenized: {:?}", pt_tokens);

    // Split digits
    let digits = DigitsSplit::new(true);
    let mut pt_final = Vec::new();
    for token in pt_tokens {
        pt_final.extend(digits.pre_tokenize(&token).unwrap_or_else(|_| vec![token]));
    }
    println!("With digits split: {:?}\n", pt_final);

    // Example 5: BERT-style processing
    println!("=== BERT-style Processing ===");
    let bert_text = "First sentence";
    let bert_text2 = "Second sentence";

    println!("Sentence A: \"{}\"", bert_text);
    println!("Sentence B: \"{}\"", bert_text2);

    // Simulate token IDs (in real use, these would come from WordPiece)
    let ids_a = vec![2023, 6251];  // "first sentence"
    let ids_b = vec![2117, 6251];  // "second sentence"

    let bert_processor = BertProcessing::new(
        ("[CLS]".to_string(), 101),
        ("[SEP]".to_string(), 102),
    );

    let processed = bert_processor.process(ids_a.clone(), Some(ids_b.clone()));
    println!("BERT processed IDs: {:?}", processed);
    println!("Format: [CLS] A [SEP] B [SEP]\n");

    // Example 6: Decoder pipeline
    println!("=== Decoder Pipeline ===");

    // WordPiece tokens with ## prefix
    let wp_tokens = vec![
        "hello".to_string(),
        "##world".to_string(),
        "test".to_string(),
        "##ing".to_string(),
    ];
    println!("WordPiece tokens: {:?}", wp_tokens);

    let wp_decoder = WordPieceDecoder;
    let decoded = wp_decoder.decode(&wp_tokens)?;
    println!("After WordPiece decode: \"{}\"", decoded);

    // Strip special tokens
    let special_tokens = vec![
        "[CLS]".to_string(),
        "hello".to_string(),
        "[SEP]".to_string(),
        "world".to_string(),
        "[PAD]".to_string(),
    ];
    println!("\nWith special tokens: {:?}", special_tokens);

    let strip_decoder = StripDecoder::new();
    let stripped_decoded = strip_decoder.decode(&special_tokens)?;
    println!("After stripping: \"{}\"\n", stripped_decoded);

    // Example 7: Complete custom tokenizer pipeline
    println!("=== Complete Custom Tokenizer ===");
    let input = "Olá, MUNDO! Como você está? Estou BEM.";
    println!("Input: \"{}\"", input);

    // 1. Normalize
    let normalizers: Vec<Box<dyn Normalizer>> = vec![
        Box::new(NFKCNormalizer),
        Box::new(LowercaseNormalizer),
    ];
    let normalizer = SequenceNormalizer::new(normalizers);
    let step1 = normalizer.normalize(input).unwrap_or_else(|_| input.to_string());
    println!("\n1. Normalized: \"{}\"", step1);

    // 2. Pre-tokenize
    let step2 = whitespace.pre_tokenize(&step1).unwrap_or_else(|_| vec![step1.clone()]);
    println!("2. Pre-tokenized: {:?}", step2);

    // 3. Apply algorithm (simulated WordPiece)
    let mut step3 = Vec::new();
    for token in step2 {
        // Simulate WordPiece splitting
        if token.len() > 4 {
            let mid = token.len() / 2;
            step3.push(token[..mid].to_string());
            step3.push(format!("##{}", &token[mid..]));
        } else {
            step3.push(token);
        }
    }
    println!("3. After algorithm: {:?}", step3);

    // 4. Post-process (add BERT tokens)
    step3.insert(0, "[CLS]".to_string());
    step3.push("[SEP]".to_string());
    println!("4. Post-processed: {:?}", step3);

    // 5. Decode
    let final_decoded = step3.join(" ").replace(" ##", "");
    println!("5. Final decoded: \"{}\"\n", final_decoded);

    // Example 8: Custom normalizer for URLs and emails
    println!("=== Custom URL/Email Handling ===");
    let text_with_urls = "Check https://example.com and email user@example.com";
    println!("Original: \"{}\"", text_with_urls);

    // Custom preprocessing (would be a custom Normalizer in real code)
    let processed = text_with_urls
        .replace(char::is_alphabetic, |c: char| c.to_lowercase().to_string().chars().next().unwrap());
    println!("Lowercase: \"{}\"", processed);

    let with_placeholders = processed
        .replace("https://", "<URL>")
        .replace("@", " <AT> ");
    println!("With placeholders: \"{}\"\n", with_placeholders);

    // Example 9: Performance comparison
    println!("=== Performance Tips ===");
    println!("1. Use Sequence to chain multiple normalizers efficiently");
    println!("2. Pre-tokenize once, then apply algorithm to each chunk");
    println!("3. Cache frequently used token splits");
    println!("4. Use parallel processing for batch operations");
    println!("5. Profile your pipeline to find bottlenecks\n");

    println!("=== Pipeline Complete ===");
    println!("\nKey takeaways:");
    println!("  • Normalizers: Clean and standardize text");
    println!("  • Pre-tokenizers: Split text into words/subwords");
    println!("  • Algorithms: Apply BPE/WordPiece/Unigram");
    println!("  • Post-processors: Add special tokens");
    println!("  • Decoders: Convert tokens back to text");
    println!("  • Sequence: Chain multiple operations efficiently");

    Ok(())
}
