//! Batch Processing Example
//!
//! This example demonstrates efficient batch tokenization for processing
//! multiple texts in parallel using Rayon.

use avila_tokenizers::models::GPT2Tokenizer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Batch Processing Example\n");

    // Initialize tokenizer
    let tokenizer = GPT2Tokenizer::new()?;

    // Sample batch of texts (could be from a file, database, etc.)
    let texts = vec![
        "Hello, world! This is the first text.",
        "Batch processing is very efficient.",
        "avila-tokenizers supports parallel processing with Rayon.",
        "This example shows how to tokenize multiple texts at once.",
        "Performance scales linearly with number of CPU cores.",
        "Ideal for preprocessing large datasets for training.",
        "Each text is tokenized independently.",
        "Results are collected in the same order as input.",
    ];

    println!("📝 Processing {} texts...\n", texts.len());

    // Single-threaded batch processing
    let start = std::time::Instant::now();
    let encoded_single: Vec<Vec<u32>> = texts
        .iter()
        .map(|text| tokenizer.encode(text))
        .collect::<Result<Vec<_>, _>>()?;
    let single_duration = start.elapsed();

    println!("⏱️  Single-threaded: {:?}", single_duration);
    println!("   Total tokens: {}", encoded_single.iter().map(|e| e.len()).sum::<usize>());

    // Parallel batch processing with Rayon
    use rayon::prelude::*;

    let start = std::time::Instant::now();
    let encoded_parallel: Vec<Vec<u32>> = texts
        .par_iter()
        .map(|text| tokenizer.encode(text))
        .collect::<Result<Vec<_>, _>>()?;
    let parallel_duration = start.elapsed();

    println!("⚡ Parallel (Rayon): {:?}", parallel_duration);
    println!("   Total tokens: {}", encoded_parallel.iter().map(|e| e.len()).sum::<usize>());
    println!("   Speedup: {:.2}x", single_duration.as_secs_f64() / parallel_duration.as_secs_f64());

    // Display first few results
    println!("\n📊 Sample Results:");
    for (i, (text, tokens)) in texts.iter().zip(encoded_parallel.iter()).take(3).enumerate() {
        println!("\n  Text {}: \"{}\"", i + 1, text);
        println!("  Tokens: {:?}", tokens);
        println!("  Count: {} tokens", tokens.len());
    }

    // Batch decoding
    println!("\n🔄 Batch Decoding:");
    let start = std::time::Instant::now();
    let decoded: Vec<String> = encoded_parallel
        .par_iter()
        .map(|tokens| tokenizer.decode(tokens))
        .collect::<Result<Vec<_>, _>>()?;
    let decode_duration = start.elapsed();

    println!("⏱️  Decoded {} texts in {:?}", decoded.len(), decode_duration);

    // Verify round-trip
    let all_match = texts.iter().zip(decoded.iter()).all(|(orig, dec)| orig == dec);
    println!("✅ Round-trip test: {}", if all_match { "PASS" } else { "FAIL" });

    // Memory-efficient streaming for very large datasets
    println!("\n📦 Streaming Large Dataset:");

    // Simulate reading from a large file in chunks
    let large_dataset = vec!["text"; 10000]; // 10k texts
    let chunk_size = 1000;

    let start = std::time::Instant::now();
    let mut total_tokens = 0;

    for chunk in large_dataset.chunks(chunk_size) {
        let encoded_chunk: Vec<Vec<u32>> = chunk
            .par_iter()
            .map(|text| tokenizer.encode(text))
            .collect::<Result<Vec<_>, _>>()?;

        total_tokens += encoded_chunk.iter().map(|e| e.len()).sum::<usize>();

        // Process chunk (e.g., save to disk, send to training pipeline)
    }

    let stream_duration = start.elapsed();
    println!("   Processed {} texts in {} chunks", large_dataset.len(), large_dataset.len() / chunk_size);
    println!("   Total tokens: {}", total_tokens);
    println!("   Duration: {:?}", stream_duration);
    println!("   Throughput: {:.0} texts/sec", large_dataset.len() as f64 / stream_duration.as_secs_f64());

    Ok(())
}
