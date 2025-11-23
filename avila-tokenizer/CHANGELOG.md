# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial implementation of core tokenizer algorithms
- BPE (Byte-Pair Encoding) support for GPT-2/3/4 style models
- WordPiece support for BERT family models
- Unigram support for SentencePiece models
- Character-level tokenization for ByT5
- Pre-configured tokenizers: GPT-2, BERT, Llama
- Unicode normalization (NFC, NFKC)
- Multiple pre-tokenizers (whitespace, byte-level, metaspace, punctuation, digits)
- Post-processing for BERT and RoBERTa
- Comprehensive decoder implementations
- Portuguese-optimized tokenization
- Batch processing support
- Extensive test suite
- Benchmarking infrastructure

### Performance
- Target: 3x faster than HuggingFace Tokenizers for GPT-2
- Target: 4x faster than HuggingFace Tokenizers for BERT
- Memory footprint: < 100MB

## [0.1.0] - 2025-11-22

### Added
- Initial release of avila-tokenizers
- Complete implementation of BPE algorithm with caching
- Complete implementation of WordPiece algorithm
- Complete implementation of Unigram algorithm
- Support for GPT-2, BERT, and Llama models
- Comprehensive normalizers (lowercase, strip accents, NFC, NFKC)
- Multiple pre-tokenization strategies
- Post-processing pipeline
- Decoder implementations
- Training capabilities for custom vocabularies
- Example code for common use cases
- Compatibility tests with OpenAI tiktoken and HuggingFace
- CI/CD pipeline with GitHub Actions
- Security audit integration
- Code coverage reporting
- Documentation and examples

[Unreleased]: https://github.com/avilaops/arxis/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/avilaops/arxis/releases/tag/v0.1.0
