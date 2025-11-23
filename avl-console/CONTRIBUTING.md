# Contributing to AVL Console

First off, thank you for considering contributing to AVL Console! It's people like you that make AVL Console such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples**
- **Describe the behavior you observed**
- **Explain which behavior you expected to see**
- **Include screenshots if relevant**
- **Include your environment details** (OS, Rust version, etc.)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- **Use a clear and descriptive title**
- **Provide a step-by-step description of the suggested enhancement**
- **Provide specific examples to demonstrate the steps**
- **Describe the current behavior** and **explain the expected behavior**
- **Explain why this enhancement would be useful**

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints
6. Issue that pull request!

## Development Setup

### Prerequisites

- Rust 1.75 or later
- Docker and Docker Compose (for integration tests)
- Node.js 18+ (for load testing)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/avilaops/arxis.git
cd arxis/avl-console

# Build the project
cargo build

# Run tests
cargo test --all-features

# Run clippy
cargo clippy --all-features -- -D warnings

# Format code
cargo fmt --all

# Run benchmarks
cargo bench
```

### Project Structure

```
avl-console/
├── src/              # Source code
│   ├── lib.rs        # Library entry point
│   ├── api.rs        # REST API endpoints
│   ├── ai_assistant.rs  # AI Assistant module
│   ├── vector_persistence.rs  # Vector storage
│   └── ...
├── benches/          # Performance benchmarks
├── tests/            # Integration tests
├── examples/         # Example code
├── k8s/              # Kubernetes manifests
├── load-tests/       # Load testing scripts
└── docker-compose.yml
```

## Coding Guidelines

### Rust Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write documentation for public APIs
- Add tests for new functionality

### Commit Messages

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- `feat`: A new feature
- `fix`: A bug fix
- `docs`: Documentation only changes
- `style`: Changes that don't affect the meaning of the code
- `refactor`: A code change that neither fixes a bug nor adds a feature
- `perf`: A code change that improves performance
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to the build process or auxiliary tools

Examples:
```
feat(ai): add support for Anthropic Claude
fix(auth): correct JWT validation logic
docs(readme): update deployment instructions
perf(vector): optimize similarity search algorithm
```

### Documentation

- Document all public APIs with `///` doc comments
- Include examples in documentation
- Update README.md for user-facing changes
- Add inline comments for complex logic

### Testing

- Write unit tests for all new functions
- Add integration tests for new features
- Maintain >80% code coverage
- Use `proptest` for property-based testing where appropriate

### Performance

- Profile code before optimizing
- Use `criterion` for performance benchmarks
- Consider memory usage and allocations
- Use `cargo flamegraph` for profiling

## Pull Request Process

1. **Create an issue** first to discuss the change
2. **Fork the repository** and create a branch
3. **Write code** following our guidelines
4. **Add tests** for new functionality
5. **Update documentation** as needed
6. **Run the test suite**: `cargo test --all-features`
7. **Run clippy**: `cargo clippy --all-features -- -D warnings`
8. **Format code**: `cargo fmt --all`
9. **Commit your changes** with conventional commits
10. **Push to your fork** and submit a pull request

### PR Checklist

- [ ] Tests pass locally
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated
- [ ] Commit messages follow conventions
- [ ] PR description clearly describes changes
- [ ] Breaking changes are documented
- [ ] Benchmarks run (if performance-related)

## Review Process

1. **Automated checks** - CI/CD pipeline runs automatically
2. **Code review** - At least one maintainer reviews the PR
3. **Feedback** - Address any comments or suggestions
4. **Approval** - Maintainer approves the PR
5. **Merge** - Maintainer merges the PR

## Release Process

Releases follow [Semantic Versioning](https://semver.org/):

- **Major version** (1.0.0): Breaking changes
- **Minor version** (0.1.0): New features, backwards compatible
- **Patch version** (0.0.1): Bug fixes, backwards compatible

## Community

- **Discord**: https://discord.gg/avilacloud
- **Discussions**: https://github.com/avilaops/arxis/discussions
- **Email**: dev@avila.cloud

## Recognition

Contributors are recognized in:
- CHANGELOG.md
- README.md contributors section
- GitHub contributors graph
- Release notes

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (MIT OR Apache-2.0).

---

Thank you for contributing to AVL Console! 🚀
