# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-12-02

### Added
- `value_as<T>()` method for type-safe value parsing
  ```rust
  let port: u16 = matches.value_as("port").unwrap_or(8080);
  ```
- `any_present()` method to check if any argument from a list is present
  ```rust
  if matches.any_present(&["verbose", "debug"]) { /* ... */ }
  ```
- `all_present()` method to check if all arguments from a list are present
  ```rust
  if matches.all_present(&["user", "password"]) { /* ... */ }
  ```
- `value_or()` method to get value with inline default
  ```rust
  let host = matches.value_or("host", "localhost");
  ```
- `values_count()` method to get number of positional arguments
  ```rust
  println!("Processing {} files", matches.values_count());
  ```
- 5 new unit tests for new features (85%+ coverage)

### Improved
- Enhanced README with professional badges
- Added 8 FAQ entries addressing common questions
- Added 8 common usage patterns with detailed examples
- Added "Real Use Cases" section
- Added visual "How It Works" diagram
- Comprehensive troubleshooting guide
- Better error handling examples

### Documentation
- Expanded README from ~10KB to ~35KB
- Added inline code examples for all new methods
- Improved API documentation with usage examples
- Added comparison with clap, structopt, and argh

## [0.1.0] - 2025-12-01

### Added
- Initial release with core CLI parsing functionality
- Support for short arguments (`-v`) and long arguments (`--verbose`)
- Subcommand support (like `git clone`, `cargo build`)
- Value-taking arguments (`--output file.txt`)
- Boolean flags (`--verbose`)
- Positional arguments
- Automatic help generation (`--help`, `-h`)
- Automatic version display (`--version`, `-V`)
- O(1) argument lookups via HashMap
- Zero dependencies (pure Rust std)
- Memory safe implementation (no unsafe code)

### Documentation
- Comprehensive README with examples
- Basic API documentation
- Installation guide
- Quick start examples

[0.2.0]: https://github.com/avilaops/arxis/releases/tag/avila-cli-v0.2.0
[0.1.0]: https://github.com/avilaops/arxis/releases/tag/avila-cli-v0.1.0
