# GitHub Secrets Configuration

This document explains how to configure GitHub secrets required for the avila-tokenizer automation workflows.

## Required Secrets

### 1. CARGO_TOKEN (Required for Release)

**Purpose**: Allows GitHub Actions to publish to crates.io automatically.

**Setup Instructions**:

1. **Get your crates.io API token**:
   - Go to https://crates.io/settings/tokens
   - Log in with your GitHub account if needed
   - Click "New Token"
   - Give it a name: "GitHub Actions - avila-tokenizers"
   - Select scope: "publish-update" (or use default)
   - Click "Generate"
   - **IMPORTANT**: Copy the token immediately - you can't see it again!

2. **Add secret to GitHub**:
   - Go to https://github.com/avilaops/arxis/settings/secrets/actions
   - Click "New repository secret"
   - Name: `CARGO_TOKEN`
   - Value: Paste the token you copied
   - Click "Add secret"

3. **Verify**:
   - The release workflow will now be able to publish to crates.io
   - Test by creating a tag: `git tag v0.1.0 && git push origin v0.1.0`

---

### 2. CODECOV_TOKEN (Optional)

**Purpose**: Allows uploading code coverage reports to Codecov.

**Note**: For **public repositories**, this token is **optional**. Codecov can authenticate using the automatic `GITHUB_TOKEN`.

**Setup Instructions** (if needed):

1. **Get Codecov token**:
   - Go to https://codecov.io
   - Log in with GitHub
   - Navigate to the avilaops/arxis repository
   - Go to Settings > General
   - Copy the "Repository Upload Token"

2. **Add secret to GitHub**:
   - Go to https://github.com/avilaops/arxis/settings/secrets/actions
   - Click "New repository secret"
   - Name: `CODECOV_TOKEN`
   - Value: Paste the Codecov token
   - Click "Add secret"

3. **Update workflow** (if using token):
   ```yaml
   - name: Upload coverage to Codecov
     uses: codecov/codecov-action@v3
     with:
         files: ./avila-tokenizer/cobertura.xml
         token: ${{ secrets.CODECOV_TOKEN }}
         fail_ci_if_error: false
   ```

---

## Automatic Secrets

### GITHUB_TOKEN (Automatic)

**Purpose**: Allows GitHub Actions to interact with your repository.

**Note**: This is **automatically provided** by GitHub Actions. No setup required.

**What it's used for**:
- Creating releases
- Uploading release assets
- Commenting on PRs
- Updating statuses

**Permissions** (already configured in workflows):
```yaml
permissions:
  contents: write
  pull-requests: write
```

---

## Verification

### Check if secrets are set

1. Go to https://github.com/avilaops/arxis/settings/secrets/actions
2. You should see:
   - `CARGO_TOKEN` (set by you)
   - `CODECOV_TOKEN` (optional)
   - `GITHUB_TOKEN` is automatic, won't appear in the list

### Test the secrets

**Test CARGO_TOKEN**:
```bash
# Create a test tag
git tag v0.1.0-test
git push origin v0.1.0-test

# Watch the workflow
# https://github.com/avilaops/arxis/actions

# Clean up
git tag -d v0.1.0-test
git push origin :refs/tags/v0.1.0-test
```

**Test CODECOV_TOKEN**:
```bash
# Push a commit
git commit --allow-empty -m "test: trigger coverage"
git push origin main

# Check workflow logs
# Should see "Upload coverage to Codecov" step succeed
```

---

## Security Best Practices

### 1. Token Rotation
- Rotate `CARGO_TOKEN` every 6-12 months
- If compromised, revoke immediately at crates.io

### 2. Minimal Scope
- Use minimal required scopes for tokens
- Don't use personal access tokens with admin rights

### 3. Environment Secrets
Consider using environment-specific secrets for:
- Production releases (main branch)
- Staging releases (develop branch)

```yaml
environment:
  name: production
  url: https://crates.io/crates/avila-tokenizers
```

### 4. Audit Access
- Regularly review who has access to secrets
- Remove tokens from accounts that no longer need them

---

## Troubleshooting

### "CARGO_TOKEN not found"

**Problem**: Workflow fails with "error: no token found"

**Solutions**:
1. Verify secret name is exactly `CARGO_TOKEN` (case-sensitive)
2. Check secret is set at repository level (not organization)
3. Re-generate and re-add the token

### "Invalid token"

**Problem**: "error: failed to parse response"

**Solutions**:
1. Token may have expired - generate new one
2. Token may have wrong scope - regenerate with "publish-update"
3. Crates.io may be down - check https://status.crates.io

### "Permission denied"

**Problem**: Can't publish to crates.io

**Solutions**:
1. Ensure you're an owner of the crate on crates.io
2. Add co-owners: `cargo owner --add github-user`
3. Verify token has correct permissions

---

## Alternative: Manual Publishing

If automated publishing fails, you can always publish manually:

```bash
cd avila-tokenizer

# Login to crates.io
cargo login

# Publish
cargo publish
```

---

## Documentation

- [GitHub Encrypted Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [Crates.io API Tokens](https://doc.rust-lang.org/cargo/reference/config.html#registriestokentoken)
- [Codecov Token Documentation](https://docs.codecov.com/docs/about-codecov-tokens)

---

## Contact

For issues with secrets setup:
- Open an issue: https://github.com/avilaops/arxis/issues
- Email: nicolas@avila.inc
