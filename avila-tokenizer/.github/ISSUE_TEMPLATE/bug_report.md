---
name: Bug report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Initialize tokenizer with '...'
2. Call method '....'
3. Pass input '....'
4. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Code example**
```rust
use avila_tokenizers::models::GPT2Tokenizer;

fn main() {
    let mut tokenizer = GPT2Tokenizer::from_pretrained("gpt2").unwrap();
    // Your code here that demonstrates the bug
}
```

**Error message**
```
Paste the full error message here
```

**Environment:**
 - OS: [e.g. Ubuntu 22.04, Windows 11, macOS 14]
 - Rust version: [e.g. 1.75.0]
 - avila-tokenizers version: [e.g. 0.1.0]
 - Installation method: [e.g. cargo add, git dependency]

**Additional context**
Add any other context about the problem here.

**Possible Solution**
If you have ideas on how to fix this, please share them here.
