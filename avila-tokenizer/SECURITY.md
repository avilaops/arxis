# Security Policy

## Supported Versions

We release patches for security vulnerabilities. Currently supported versions:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

The avila-tokenizers team takes security bugs seriously. We appreciate your efforts to responsibly disclose your findings, and will make every effort to acknowledge your contributions.

### Where to Report

**Do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:

**security@avila.cloud**

### What to Include

Please include the following information in your report:

* Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
* Full paths of source file(s) related to the manifestation of the issue
* The location of the affected source code (tag/branch/commit or direct URL)
* Any special configuration required to reproduce the issue
* Step-by-step instructions to reproduce the issue
* Proof-of-concept or exploit code (if possible)
* Impact of the issue, including how an attacker might exploit it

### Response Timeline

* **Initial Response**: Within 48 hours
* **Status Update**: Within 7 days
* **Fix Timeline**: Varies by severity
  - Critical: 1-7 days
  - High: 7-14 days
  - Medium: 14-30 days
  - Low: 30-90 days

### What to Expect

After you submit a report, we will:

1. Confirm receipt of your vulnerability report
2. Send you regular updates about our progress
3. Work with you to understand the issue
4. Notify you when the vulnerability is fixed

### Disclosure Policy

* Security issues are given the highest priority
* We follow coordinated disclosure practices
* We will not publicly disclose the issue until:
  - A fix has been released
  - Affected users have been notified
  - Reasonable time for users to apply the fix has passed (typically 90 days)

### Recognition

We believe in recognizing security researchers who help us keep our users safe. If you report a valid security issue:

* We will publicly acknowledge your contribution (unless you prefer to remain anonymous)
* Your name will be added to our Security Hall of Fame
* For significant findings, we may offer a reward (at our discretion)

## Security Best Practices

When using avila-tokenizers:

### Input Validation

* Always validate and sanitize user input before tokenizing
* Be aware of potential injection attacks through text input
* Set reasonable limits on input size to prevent DoS

### Dependencies

* Keep dependencies up to date: `cargo update`
* Regularly audit dependencies: `cargo audit`
* Use `cargo-deny` to check for known vulnerabilities

### Safe Usage

```rust
// Good: Validate input size
if text.len() > MAX_INPUT_SIZE {
    return Err(TokenizerError::InputTooLarge);
}
let tokens = tokenizer.encode(text);

// Good: Handle errors properly
match tokenizer.encode(text) {
    Ok(tokens) => process_tokens(tokens),
    Err(e) => log_error_and_return(e),
}

// Avoid: Unbounded input
let tokens = tokenizer.encode(user_input); // Risky!
```

### Resource Limits

* Set maximum token limits
* Implement timeout mechanisms for long-running operations
* Monitor memory usage in production

### Data Privacy

* Be careful with sensitive data in tokenized form
* Consider implementing PII detection/redaction
* Follow data protection regulations (GDPR, LGPD, etc.)

## Security Features

### Memory Safety

* Written in Rust - memory-safe by default
* No unsafe code in core algorithms
* Bounds checking on all array/vector access

### Type Safety

* Strong type system prevents common vulnerabilities
* No undefined behavior
* Compile-time guarantees

### Zero External Dependencies (Core)

* Minimal attack surface
* All algorithms implemented internally
* No network calls in core functionality

## Known Security Considerations

### Performance DoS

* Very large inputs can cause high CPU/memory usage
* Recommendation: Implement input size limits
* Example: Limit to 1MB of text per request

### Unicode Handling

* Complex Unicode can cause processing delays
* Built-in normalization helps mitigate this
* Consider pre-filtering unusual Unicode ranges

## Security Updates

Security updates will be:

1. Released as patch versions (0.1.x)
2. Documented in CHANGELOG.md
3. Announced via:
   - GitHub Security Advisories
   - Crates.io advisory database
   - Project README
   - Email to security@avila.cloud subscribers

## Vulnerability Disclosure History

No security vulnerabilities have been reported or disclosed for this project to date.

---

**Last Updated**: November 22, 2025
**Security Contact**: security@avila.cloud
**PGP Key**: Available upon request
