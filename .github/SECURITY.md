# Security Policy

## Supported Versions

We release patches for security vulnerabilities for the following versions:

| Version | Supported |
| ------- | --------- |
| 0.2.x   | ✅ Yes     |
| 0.1.x   | ❌ No      |
| < 0.1   | ❌ No      |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to:
- **nicolas@avila.inc**
- **dev@avila.inc** (CC for team awareness)

Include as much information as possible:

- Type of vulnerability
- Full paths of source file(s) related to the manifestation
- Location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### What to Expect

- **Acknowledgment**: Within 48 hours
- **Initial assessment**: Within 7 days
- **Regular updates**: Every 7-14 days
- **Fix timeline**: Depends on severity
  - Critical: 1-7 days
  - High: 7-30 days
  - Medium: 30-90 days
  - Low: Best effort

## Security Update Process

1. Security issue is received and assigned a primary handler
2. Problem is confirmed and affected versions determined
3. Code is audited to find similar problems
4. Fixes are prepared for all supported versions
5. Fixes are released as quickly as possible
6. Security advisory is published

## Security Best Practices

When using Arxis:

### Input Validation

Always validate inputs, especially when:
- Loading external data (LISA data files, etc.)
- Processing user-provided parameters
- Parsing configuration files

```rust
use arxis_quaternions::physics::*;

// Good: Validate before use
fn process_mass(mass: f64) -> Result<f64, String> {
    if mass <= 0.0 {
        return Err("Mass must be positive".to_string());
    }
    if mass > 1e10 {
        return Err("Mass too large".to_string());
    }
    Ok(mass)
}
```

### Safe Numerics

Be aware of numerical instabilities:
- Division by zero
- Overflow/underflow
- NaN propagation

```rust
// Good: Check for numerical issues
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b.abs() < 1e-10 {
        return None;
    }
    Some(a / b)
}
```

### Memory Safety

Rust provides memory safety by default, but be careful with:
- Unsafe code blocks (we avoid these)
- Large array allocations
- Infinite loops in numerical solvers

### Dependencies

We regularly audit dependencies using:
- `cargo audit` for known vulnerabilities
- Dependabot for automated updates
- Manual review of dependency changes

## Disclosure Policy

- Security issues are disclosed publicly after a fix is available
- Credit is given to reporters (unless they prefer anonymity)
- CVE IDs are requested for significant vulnerabilities

## Comments on this Policy

If you have suggestions on how this process could be improved, please submit a pull request.

## Contact

- **Security Emails**:
  - nicolas@avila.inc (Primary)
  - dev@avila.inc (Team CC)
- **PGP Key**: Available upon request
- **Response Time**: Within 48 hours

## Hall of Fame

We recognize security researchers who help us maintain a secure codebase:

<!-- This section will be updated as we receive reports -->

---

**Thank you for helping keep Arxis and our users safe!**
