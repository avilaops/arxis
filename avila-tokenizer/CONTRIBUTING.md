# Contributing to avila-tokenizers

First off, thank you for considering contributing to avila-tokenizers! 🎉

The following is a set of guidelines for contributing to this project. These are mostly guidelines, not rules. Use your best judgment, and feel free to propose changes to this document in a pull request.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title** for the issue
* **Describe the exact steps which reproduce the problem**
* **Provide specific examples to demonstrate the steps**
* **Describe the behavior you observed after following the steps**
* **Explain which behavior you expected to see instead and why**
* **Include code samples and test cases** if possible

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a step-by-step description of the suggested enhancement**
* **Provide specific examples to demonstrate the steps**
* **Describe the current behavior and expected behavior**
* **Explain why this enhancement would be useful**

### Pull Requests

* Fill in the required template
* Do not include issue numbers in the PR title
* Follow the Rust style guide
* Include thoughtfully-worded, well-structured tests
* Document new code
* End all files with a newline

## Development Process

### Setup

```bash
# Clone the repository
git clone https://github.com/avilaops/arxis.git
cd arxis/avila-tokenizer

# Build the project
cargo build

# Run tests
cargo test

# Run benchmarks
cargo bench
```

### Coding Style

* Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/)
* Run `cargo fmt` before committing
* Run `cargo clippy` and fix all warnings
* Write meaningful commit messages

### Testing

* Write unit tests for new functionality
* Ensure all tests pass: `cargo test`
* Add integration tests when appropriate
* Maintain test coverage above 80%

### Documentation

* Update README.md if you change functionality
* Add doc comments for public APIs
* Include examples in doc comments
* Update CHANGELOG.md

### Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line

Example:
```
Add support for custom tokenizers

- Implement TokenizerTrait interface
- Add documentation and examples
- Include unit tests

Fixes #123
```

### Branch Naming

* `feature/description` - New features
* `fix/description` - Bug fixes
* `docs/description` - Documentation changes
* `refactor/description` - Code refactoring
* `test/description` - Adding tests

## Project Structure

```
avila-tokenizer/
├── src/
│   ├── algorithms/     # Tokenization algorithms (BPE, WordPiece, Unigram)
│   ├── models/         # Pre-trained models (GPT-2, BERT, Llama)
│   ├── normalizers/    # Text normalization
│   ├── pre_tokenizers/ # Pre-tokenization
│   ├── post_processors/# Post-processing
│   ├── decoders/       # Decoding
│   └── utils/          # Utilities
├── tests/              # Integration tests
├── examples/           # Usage examples
└── benches/            # Benchmarks
```

## Adding New Models

To add a new model:

1. Create a new file in `src/models/`
2. Implement the tokenizer using existing algorithms
3. Add tests in `tests/`
4. Add example in `examples/`
5. Update documentation

## Adding New Algorithms

To add a new algorithm:

1. Create a new file in `src/algorithms/`
2. Implement the `Tokenizer` trait
3. Add comprehensive tests
4. Add benchmarks
5. Document the algorithm

## Release Process

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite: `cargo test --all-features`
4. Create a git tag: `git tag -a v0.x.x -m "Release v0.x.x"`
5. Push tag: `git push origin v0.x.x`
6. Publish to crates.io: `cargo publish`

## Getting Help

* Open an issue for questions
* Join our Discord (if available)
* Check the documentation

## Recognition

Contributors will be recognized in:
* README.md Contributors section
* Release notes
* Project documentation

Thank you for contributing! 🚀

---

**Maintained by**: Avila Operations (avilaops)
**License**: MIT OR Apache-2.0
