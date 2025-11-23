//! Streaming Tokenization Example
//!
//! This example demonstrates how to tokenize text as it arrives in chunks,
//! useful for processing streaming data from APIs, files, or network sources.

use avila_tokenizers::models::GPT2Tokenizer;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌊 Streaming Tokenization Example\n");

    let tokenizer = GPT2Tokenizer::new()?;

    // Example 1: Simulate streaming from a file
    println!("📄 Example 1: Line-by-line streaming\n");

    let sample_text = "This is line one.\nThis is line two.\nThis is line three.\n";
    let reader = BufReader::new(sample_text.as_bytes());

    let mut line_num = 0;
    let mut total_tokens = 0;

    for line in reader.lines() {
        let line = line?;
        line_num += 1;

        // Tokenize each line as it arrives
        let tokens = tokenizer.encode(&line)?;
        total_tokens += tokens.len();

        println!("Line {}: \"{}\"", line_num, line);
        println!("  Tokens: {:?}", tokens);
        println!("  Count: {}\n", tokens.len());
    }

    println!("Total lines: {}", line_num);
    println!("Total tokens: {}\n", total_tokens);

    // Example 2: Word-by-word streaming with buffering
    println!("📝 Example 2: Word-by-word streaming with context\n");

    let sentence = "The quick brown fox jumps over the lazy dog";
    let words: Vec<&str> = sentence.split_whitespace().collect();

    let mut buffer = String::new();
    let window_size = 3; // Process every 3 words

    for (i, word) in words.iter().enumerate() {
        buffer.push_str(word);
        buffer.push(' ');

        if (i + 1) % window_size == 0 || i == words.len() - 1 {
            // Process buffer
            let tokens = tokenizer.encode(&buffer)?;
            println!("Window [{}..{}]: \"{}\"", i + 1 - buffer.split_whitespace().count() + 1, i + 1, buffer.trim());
            println!("  Tokens: {:?}", tokens);
            println!("  Count: {}\n", tokens.len());

            buffer.clear();
        }
    }

    // Example 3: Streaming with overlap (sliding window)
    println!("🔄 Example 3: Sliding window tokenization\n");

    let long_text = "Sliding window tokenization is useful for maintaining context across chunks. This technique ensures that tokens at chunk boundaries are not lost. It is commonly used in document processing.";

    let chunk_size = 50; // characters
    let overlap = 10;    // characters

    let mut start = 0;
    let mut chunk_num = 0;

    while start < long_text.len() {
        let end = (start + chunk_size).min(long_text.len());
        let chunk = &long_text[start..end];
        chunk_num += 1;

        let tokens = tokenizer.encode(chunk)?;

        println!("Chunk {} [{}..{}]:", chunk_num, start, end);
        println!("  Text: \"{}...\"", &chunk[..chunk.len().min(40)]);
        println!("  Tokens: {} tokens\n", tokens.len());

        // Move window with overlap
        start += chunk_size - overlap;
        if start >= long_text.len() {
            break;
        }
    }

    // Example 4: Simulated real-time streaming
    println!("⏱️  Example 4: Real-time streaming simulation\n");

    let streaming_text = vec![
        "Hello, ",
        "this is ",
        "a streaming ",
        "message ",
        "arriving ",
        "in chunks. ",
        "Each chunk ",
        "is processed ",
        "immediately."
    ];

    let mut accumulated = String::new();
    let mut total_chunks = 0;

    for (i, chunk) in streaming_text.iter().enumerate() {
        accumulated.push_str(chunk);
        total_chunks += 1;

        // Tokenize the accumulated text so far
        let tokens = tokenizer.encode(&accumulated)?;

        println!("Chunk {} received: \"{}\"", i + 1, chunk);
        println!("  Accumulated: \"{}\"", accumulated);
        println!("  Total tokens so far: {}\n", tokens.len());

        // Simulate network delay
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Streaming complete!");
    println!("Total chunks received: {}", total_chunks);

    // Example 5: Memory-efficient streaming for large files
    println!("\n💾 Example 5: Memory-efficient large file processing\n");

    let simulated_large_file = "Lorem ipsum dolor sit amet. ".repeat(100);
    let reader = BufReader::new(simulated_large_file.as_bytes());

    let mut total_lines = 0;
    let mut total_tokens = 0;

    for line in reader.lines() {
        let line = line?;
        let tokens = tokenizer.encode(&line)?;

        total_lines += 1;
        total_tokens += tokens.len();

        // Only print summary for large files
        if total_lines % 20 == 0 {
            println!("Processed {} lines, {} tokens...", total_lines, total_tokens);
        }
    }

    println!("\nFinal statistics:");
    println!("  Total lines: {}", total_lines);
    println!("  Total tokens: {}", total_tokens);
    println!("  Average tokens/line: {:.2}", total_tokens as f64 / total_lines as f64);

    Ok(())
}
