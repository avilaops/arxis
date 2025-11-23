# Security Policy

## Supported Versions

We actively support the following versions with security updates:

| Project          | Version | Supported          |
| ---------------- | ------- | ------------------ |
| avila-clustering | 0.1.x   | :white_check_mark: |
| avila-compress   | 0.3.x   | :white_check_mark: |
| avila-dataframe  | 0.1.x   | :white_check_mark: |
| avila-linalg     | 0.1.x   | :white_check_mark: |
| avila-math       | 0.1.x   | :white_check_mark: |
| avila-ml         | 0.1.x   | :white_check_mark: |
| avila-tokenizer  | 0.1.x   | :white_check_mark: |
| aviladb          | 0.1.x   | :white_check_mark: |
| avx-gateway      | 0.1.x   | :white_check_mark: |
| avx-http         | 0.2.x   | :white_check_mark: |
| avx-image        | 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security vulnerabilities seriously. If you discover a security issue in any of the Arxis projects, please report it responsibly.

### How to Report

**Please do NOT create a public GitHub issue for security vulnerabilities.**

Instead, please report security issues via:

1. **GitHub Security Advisories** (Preferred):
   - Go to the [Security tab](https://github.com/avilaops/arxis/security/advisories)
   - Click "Report a vulnerability"
   - Fill in the details

2. **Email**:
   - Send to: security@avila.inc
   - Subject: `[SECURITY] Arxis - <brief description>`
   - Include:
     - Affected project(s) and version(s)
     - Description of the vulnerability
     - Steps to reproduce
     - Potential impact
     - Any suggested fixes (if available)

### What to Include

To help us address the issue quickly, please include:

- **Type of vulnerability** (e.g., SQL injection, XSS, memory safety)
- **Affected components** (e.g., avila-compress, avx-gateway)
- **Full paths of source file(s)** related to the issue
- **Location** of the affected source code (tag/branch/commit or direct URL)
- **Step-by-step instructions** to reproduce the issue
- **Proof-of-concept or exploit code** (if applicable)
- **Impact assessment** - how an attacker might exploit this
- **Any possible mitigation** you've identified

### Response Timeline

- **Initial Response**: Within 48 hours
- **Status Update**: Within 7 days
- **Fix Timeline**:
  - **Critical**: 1-7 days
  - **High**: 7-14 days
  - **Medium**: 14-30 days
  - **Low**: 30-90 days

### Our Commitment

When you report a security issue:

1. ✅ We will confirm receipt of your report within 48 hours
2. ✅ We will provide regular updates on our progress
3. ✅ We will work with you to understand the issue
4. ✅ We will notify you when the vulnerability is fixed
5. ✅ We will credit you in our security advisories (unless you prefer to remain anonymous)

### Disclosure Policy

- We follow **coordinated disclosure** practices
- We will not publicly disclose the issue until:
  - A fix has been released
  - Affected users have been notified
  - Reasonable time for users to apply the fix has passed (typically 90 days)
- You agree not to publicly disclose the issue until we've had time to address it

### Bug Bounty

While we don't currently have a formal bug bounty program, we recognize and appreciate security researchers:

- **Recognition**: Your name will be added to our Security Hall of Fame
- **Acknowledgment**: Public acknowledgment in release notes and security advisories
- **Rewards**: For significant findings, we may offer rewards at our discretion

## Security Best Practices

### For Users

When using Arxis libraries:

1. **Keep Dependencies Updated**
   ```bash
   cargo update
   cargo audit
   ```

2. **Monitor Security Advisories**
   - Watch the GitHub repository
   - Subscribe to security notifications
   - Check https://rustsec.org/ regularly

3. **Use Latest Stable Versions**
   - Always use the latest stable release
   - Apply security patches promptly

4. **Report Issues**
   - If you find a vulnerability, report it immediately
   - Don't wait for proof-of-concept exploits

### For Contributors

When contributing code:

1. **No Unsafe Code** (without justification)
   - Avoid `unsafe` blocks unless absolutely necessary
   - Document why `unsafe` is required
   - Provide safety guarantees

2. **Input Validation**
   - Validate all user inputs
   - Use type safety to prevent invalid states
   - Handle edge cases explicitly

3. **Dependencies**
   - Only add well-maintained dependencies
   - Check dependency security with `cargo audit`
   - Prefer dependencies with security audits

4. **Testing**
   - Write tests for security-critical code
   - Include fuzzing tests where appropriate
   - Test error handling paths

5. **Code Review**
   - All code must be reviewed before merging
   - Security-critical changes require additional review
   - Document security considerations

## Security Features

Our projects implement several security measures:

### Memory Safety
- ✅ Written in Rust (memory-safe by default)
- ✅ Bounds checking on all array/vector access
- ✅ No undefined behavior in safe code
- ✅ Strong type system prevents common vulnerabilities

### Code Analysis
- ✅ **CodeQL Analysis**: Automated security scanning
- ✅ **Cargo Audit**: Vulnerability scanning for dependencies
- ✅ **Cargo Deny**: License and security policy enforcement
- ✅ **Clippy**: Rust linter for common mistakes
- ✅ **Dependabot**: Automated dependency updates

### Development Practices
- ✅ Required code review for all changes
- ✅ Automated CI/CD with security checks
- ✅ Regular security audits
- ✅ Minimal dependencies philosophy
- ✅ Comprehensive test coverage

## Known Security Considerations

### Performance vs Security

Some projects prioritize performance:

- **avila-compress**: Uses `unsafe` for SIMD optimizations
- **avila-linalg**: Uses `unsafe` for raw pointer operations
- **avx-gateway**: Connection pooling requires careful resource management

All `unsafe` code is:
- ✅ Documented with safety invariants
- ✅ Reviewed by multiple contributors
- ✅ Tested extensively
- ✅ Minimized to critical paths only

### Resource Limits

To prevent DoS attacks, consider:

- **Input size limits**: Set max input sizes for tokenizers, parsers
- **Connection limits**: Configure max connections in gateways
- **Timeout mechanisms**: Use timeouts for all I/O operations
- **Rate limiting**: Implement rate limiting in production

## Security Hall of Fame

We thank the following researchers for responsibly disclosing security issues:

*No vulnerabilities reported yet*

## Contact

- **Security Email**: security@avila.inc
- **General Contact**: nicolas@avila.inc
- **GitHub Issues** (non-security): https://github.com/avilaops/arxis/issues

---

**Last Updated**: November 23, 2025
**Policy Version**: 1.0
