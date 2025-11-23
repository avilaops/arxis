use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::error::{Result, TokenizerError};
use super::VocabHashMap;

/// Vocabulary file format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabFile {
    pub version: String,
    pub model_type: String,
    pub vocab: HashMap<String, u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merges: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_tokens: Option<HashMap<String, u32>>,
}

/// Load vocabulary from JSON file
pub fn load_vocab_json(path: impl AsRef<Path>) -> Result<HashMap<String, u32>> {
    let file = File::open(path.as_ref())
        .map_err(|e| TokenizerError::IoError(format!("Failed to open vocab file: {}", e)))?;

    let vocab: HashMap<String, u32> = serde_json::from_reader(file)?;

    Ok(vocab)
}

/// Load vocabulary from text file (one token per line)
pub fn load_vocab_txt(path: impl AsRef<Path>) -> Result<HashMap<String, u32>> {
    let file = File::open(path.as_ref())
        .map_err(|e| TokenizerError::IoError(format!("Failed to open vocab file: {}", e)))?;

    let reader = BufReader::new(file);
    let mut vocab = HashMap::new();

    for (id, line) in reader.lines().enumerate() {
        let token = line?;
        vocab.insert(token, id as u32);
    }

    Ok(vocab)
}

/// Load vocabulary from structured JSON file
pub fn load_vocab_file(path: impl AsRef<Path>) -> Result<VocabFile> {
    let file = File::open(path.as_ref())
        .map_err(|e| TokenizerError::IoError(format!("Failed to open vocab file: {}", e)))?;

    let vocab_file: VocabFile = serde_json::from_reader(file)?;

    Ok(vocab_file)
}

/// Load merges from text file (BPE format)
/// Format: "token1 token2" (one merge per line)
pub fn load_merges_txt(path: impl AsRef<Path>) -> Result<Vec<(String, String)>> {
    let file = File::open(path.as_ref())
        .map_err(|e| TokenizerError::IoError(format!("Failed to open merges file: {}", e)))?;

    let reader = BufReader::new(file);
    let mut merges = Vec::new();

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        // Skip header line if present
        if i == 0 && line.starts_with("#version") {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            return Err(TokenizerError::InvalidMergeFile(
                format!("Invalid merge format at line {}: expected 2 tokens, got {}", i + 1, parts.len())
            ));
        }

        merges.push((parts[0].to_string(), parts[1].to_string()));
    }

    Ok(merges)
}

/// Save vocabulary to JSON file
pub fn save_vocab_json(vocab: &HashMap<String, u32>, path: impl AsRef<Path>) -> Result<()> {
    let file = File::create(path.as_ref())?;
    serde_json::to_writer_pretty(file, vocab)?;
    Ok(())
}

/// Save vocabulary to text file
pub fn save_vocab_txt(vocab: &HashMap<String, u32>, path: impl AsRef<Path>) -> Result<()> {
    use std::io::Write;

    let mut file = File::create(path.as_ref())?;

    // Sort by ID
    let mut items: Vec<_> = vocab.iter().collect();
    items.sort_by_key(|(_, &id)| id);

    for (token, _) in items {
        writeln!(file, "{}", token)?;
    }

    Ok(())
}

/// Save merges to text file
pub fn save_merges_txt(merges: &[(String, String)], path: impl AsRef<Path>) -> Result<()> {
    use std::io::Write;

    let mut file = File::create(path.as_ref())?;

    writeln!(file, "#version: 0.2")?;

    for (token1, token2) in merges {
        writeln!(file, "{} {}", token1, token2)?;
    }

    Ok(())
}

/// Save vocabulary file (structured format)
pub fn save_vocab_file(vocab_file: &VocabFile, path: impl AsRef<Path>) -> Result<()> {
    let file = File::create(path.as_ref())?;
    serde_json::to_writer_pretty(file, vocab_file)?;
    Ok(())
}

type VocabWithMerges = (HashMap<String, u32>, Vec<(String, String)>);

/// Load GPT-2 style vocabulary (vocab.json + merges.txt)
pub fn load_gpt2_vocab(
    vocab_path: impl AsRef<Path>,
    merges_path: impl AsRef<Path>,
) -> Result<VocabWithMerges> {
    let vocab = load_vocab_json(vocab_path)?;
    let merges = load_merges_txt(merges_path)?;

    Ok((vocab, merges))
}

/// Load BERT style vocabulary (vocab.txt)
pub fn load_bert_vocab(path: impl AsRef<Path>) -> Result<VocabHashMap> {
    let vocab = load_vocab_txt(path)?;
    let mut vocab_map = VocabHashMap::from_hashmap(vocab);

    // Set standard BERT special tokens
    if let Some(&unk_id) = vocab_map.token_to_id_map().get("[UNK]") {
        vocab_map.set_unk_token("[UNK]".to_string(), unk_id);
    }

    Ok(vocab_map)
}

/// Load SentencePiece model vocabulary
pub fn load_sentencepiece_vocab(path: impl AsRef<Path>) -> Result<Vec<(String, f64)>> {
    let file = File::open(path.as_ref())
        .map_err(|e| TokenizerError::IoError(format!("Failed to open vocab file: {}", e)))?;

    let reader = BufReader::new(file);
    let mut pieces = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() >= 2 {
            let token = parts[0].to_string();
            let score: f64 = parts[1].parse()
                .map_err(|_| TokenizerError::InvalidVocabulary(
                    format!("Invalid score for token: {}", token)
                ))?;

            pieces.push((token, score));
        }
    }

    Ok(pieces)
}

/// Build vocabulary from text corpus
pub fn build_vocab_from_corpus(
    corpus: &[&str],
    vocab_size: usize,
    min_frequency: usize,
) -> Result<HashMap<String, u32>> {
    use std::collections::HashMap;

    // Count token frequencies
    let mut freq: HashMap<String, usize> = HashMap::new();

    for text in corpus {
        for token in text.split_whitespace() {
            *freq.entry(token.to_string()).or_insert(0) += 1;
        }
    }

    // Filter by minimum frequency
    let mut tokens: Vec<_> = freq.into_iter()
        .filter(|(_, count)| *count >= min_frequency)
        .collect();

    // Sort by frequency (descending)
    tokens.sort_by(|a, b| b.1.cmp(&a.1));

    // Take top vocab_size tokens
    let tokens: Vec<_> = tokens.into_iter()
        .take(vocab_size)
        .map(|(token, _)| token)
        .collect();

    // Assign IDs
    let vocab: HashMap<String, u32> = tokens.into_iter()
        .enumerate()
        .map(|(id, token)| (token, id as u32))
        .collect();

    Ok(vocab)
}

/// Validate vocabulary consistency
pub fn validate_vocab(vocab: &HashMap<String, u32>) -> Result<()> {
    // Check for duplicate IDs
    let mut seen_ids = std::collections::HashSet::new();

    for (token, &id) in vocab.iter() {
        if !seen_ids.insert(id) {
            return Err(TokenizerError::InvalidVocabulary(
                format!("Duplicate ID {} for token {}", id, token)
            ));
        }
    }

    // Check for empty tokens
    for token in vocab.keys() {
        if token.is_empty() {
            return Err(TokenizerError::InvalidVocabulary(
                "Vocabulary contains empty token".to_string()
            ));
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_build_vocab_from_corpus() {
        let corpus = vec![
            "hello world",
            "hello rust",
            "world programming",
            "hello hello hello",
        ];

        let vocab = build_vocab_from_corpus(&corpus, 10, 1).unwrap();

        assert!(vocab.contains_key("hello"));
        assert!(vocab.contains_key("world"));
        assert!(vocab.contains_key("rust"));
    }

    #[test]
    fn test_validate_vocab() {
        let mut vocab = HashMap::new();
        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);

        assert!(validate_vocab(&vocab).is_ok());

        // Test duplicate IDs
        vocab.insert("test".to_string(), 1); // Duplicate ID
        assert!(validate_vocab(&vocab).is_err());
    }

    #[test]
    fn test_save_and_load_json() {
        let mut vocab = HashMap::new();
        vocab.insert("hello".to_string(), 1);
        vocab.insert("world".to_string(), 2);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        save_vocab_json(&vocab, path).unwrap();
        let loaded_vocab = load_vocab_json(path).unwrap();

        assert_eq!(vocab, loaded_vocab);
    }

    #[test]
    fn test_save_and_load_txt() {
        let mut vocab = HashMap::new();
        vocab.insert("hello".to_string(), 0);
        vocab.insert("world".to_string(), 1);

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        save_vocab_txt(&vocab, path).unwrap();
        let loaded_vocab = load_vocab_txt(path).unwrap();

        assert_eq!(loaded_vocab.get("hello"), Some(&0));
        assert_eq!(loaded_vocab.get("world"), Some(&1));
    }

    #[test]
    fn test_save_and_load_merges() {
        let merges = vec![
            ("h".to_string(), "e".to_string()),
            ("he".to_string(), "l".to_string()),
        ];

        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        save_merges_txt(&merges, path).unwrap();
        let loaded_merges = load_merges_txt(path).unwrap();

        assert_eq!(merges, loaded_merges);
    }
}
