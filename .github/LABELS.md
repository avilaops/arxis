# GitHub Labels Configuration

This file documents the recommended labels for the Arxis repository.

## How to Apply Labels

You can create these labels via:
1. **GitHub UI**: Settings → Labels → New label
2. **GitHub CLI**: `gh label create "label-name" --color "color" --description "description"`
3. **API/Script**: Use GitHub REST API

## Recommended Labels

### Type Labels (What kind of issue/PR is this?)

```yaml
- name: "bug"
  color: "d73a4a"
  description: "Something isn't working"

- name: "enhancement"
  color: "a2eeef"
  description: "New feature or request"

- name: "documentation"
  color: "0075ca"
  description: "Improvements or additions to documentation"

- name: "performance"
  color: "ff9800"
  description: "Performance related issue or optimization"

- name: "security"
  color: "d93f0b"
  description: "Security vulnerability or concern"

- name: "refactoring"
  color: "fbca04"
  description: "Code refactoring (no functional changes)"

- name: "testing"
  color: "1d76db"
  description: "Related to testing infrastructure or test coverage"
```

### Priority Labels (How urgent is this?)

```yaml
- name: "priority: critical"
  color: "b60205"
  description: "Requires immediate attention"

- name: "priority: high"
  color: "d93f0b"
  description: "Should be addressed soon"

- name: "priority: medium"
  color: "fbca04"
  description: "Normal priority"

- name: "priority: low"
  color: "0e8a16"
  description: "Low priority, nice to have"
```

### Status Labels (What's the current state?)

```yaml
- name: "status: triage"
  color: "ededed"
  description: "Needs initial review and labeling"

- name: "status: accepted"
  color: "0e8a16"
  description: "Issue accepted and will be worked on"

- name: "status: in progress"
  color: "1d76db"
  description: "Currently being worked on"

- name: "status: blocked"
  color: "b60205"
  description: "Blocked by another issue or external factor"

- name: "status: needs info"
  color: "d876e3"
  description: "More information needed from issue author"

- name: "status: stale"
  color: "fef2c0"
  description: "No activity for extended period"

- name: "status: wontfix"
  color: "ffffff"
  description: "This will not be worked on"

- name: "status: duplicate"
  color: "cfd3d7"
  description: "This issue or PR already exists"
```

### Crate Labels (Which part of the codebase?)

```yaml
- name: "crate: arxis"
  color: "006b75"
  description: "Main arxis_quaternions library"

- name: "crate: avila-math"
  color: "006b75"
  description: "Mathematical kernel (quaternions, tensors)"

- name: "crate: avila-telemetry"
  color: "006b75"
  description: "Time series & analytics"

- name: "crate: avila-compress"
  color: "006b75"
  description: "Compression library"

- name: "crate: avila-tokenizers"
  color: "006b75"
  description: "NLP tokenization"

- name: "crate: aviladb"
  color: "006b75"
  description: "Database system"

- name: "crate: avx-platform"
  color: "006b75"
  description: "AVL Platform crates"

- name: "crate: multiple"
  color: "006b75"
  description: "Affects multiple crates"
```

### Area Labels (What domain/feature?)

```yaml
- name: "area: LISA"
  color: "5319e7"
  description: "LISA mission & gravitational waves"

- name: "area: physics"
  color: "5319e7"
  description: "Physics implementations (GW, relativity, cosmology)"

- name: "area: quaternions"
  color: "5319e7"
  description: "Quaternion algebra (3D, dual, SO(4))"

- name: "area: tensors"
  color: "5319e7"
  description: "Tensor operations & ML"

- name: "area: 4D geometry"
  color: "5319e7"
  description: "4D geometry & polytopes"

- name: "area: API"
  color: "5319e7"
  description: "Public API design"

- name: "area: CI/CD"
  color: "5319e7"
  description: "Continuous integration & deployment"

- name: "area: benchmarks"
  color: "5319e7"
  description: "Performance benchmarks"
```

### Difficulty Labels (How hard is this?)

```yaml
- name: "good first issue"
  color: "7057ff"
  description: "Good for newcomers"

- name: "help wanted"
  color: "008672"
  description: "Extra attention is needed"

- name: "difficulty: easy"
  color: "c2e0c6"
  description: "Easy to implement"

- name: "difficulty: medium"
  color: "fbca04"
  description: "Moderate complexity"

- name: "difficulty: hard"
  color: "d93f0b"
  description: "Complex implementation required"
```

### Special Labels

```yaml
- name: "breaking change"
  color: "b60205"
  description: "Changes that break backward compatibility"

- name: "dependencies"
  color: "0366d6"
  description: "Pull requests that update a dependency file"

- name: "pinned"
  color: "d4c5f9"
  description: "Important issue that should not be closed by stale bot"

- name: "work in progress"
  color: "fbca04"
  description: "PR is still being worked on"

- name: "needs review"
  color: "d876e3"
  description: "Waiting for code review"

- name: "question"
  color: "d876e3"
  description: "Further information is requested"

- name: "roadmap"
  color: "0e8a16"
  description: "Part of the project roadmap"

- name: "research"
  color: "5319e7"
  description: "Requires research or experimentation"

- name: "NASA mission"
  color: "000080"
  description: "Related to NASA/ESA missions (LISA, LIGO)"
```

## Label Usage Examples

### Issue Examples

**Bug Report**:
- `bug` (required)
- `priority: high` (if critical)
- `crate: avila-math` (affected crate)
- `area: quaternions` (specific area)

**Feature Request**:
- `enhancement` (required)
- `priority: medium`
- `crate: arxis`
- `area: LISA`
- `help wanted` (if looking for contributors)

**Documentation Issue**:
- `documentation` (required)
- `good first issue` (if easy to fix)
- `crate: multiple`

### PR Examples

**Bug Fix PR**:
- `bug`
- `crate: avila-telemetry`
- `priority: high`

**New Feature PR**:
- `enhancement`
- `breaking change` (if API changes)
- `crate: arxis`
- `area: physics`

**Documentation PR**:
- `documentation`
- `crate: multiple`

## Automation

Labels are automatically applied by:
1. **auto-label.yml** - Based on title prefixes
2. **stale.yml** - Adds `stale` label to inactive issues/PRs
3. **dependabot.yml** - Adds `dependencies` label

## Label Management Tips

1. **Consistency**: Always use labels consistently
2. **Multiple Labels**: Issues can have multiple labels
3. **Update Labels**: Update as issue status changes
4. **Search by Label**: Use labels for filtering and searching
5. **Label Descriptions**: Always include clear descriptions

## Creating Labels via GitHub CLI

```bash
# Type labels
gh label create "bug" --color "d73a4a" --description "Something isn't working"
gh label create "enhancement" --color "a2eeef" --description "New feature or request"
gh label create "documentation" --color "0075ca" --description "Improvements or additions to documentation"
gh label create "performance" --color "ff9800" --description "Performance related issue"
gh label create "security" --color "d93f0b" --description "Security vulnerability or concern"

# Priority labels
gh label create "priority: critical" --color "b60205" --description "Requires immediate attention"
gh label create "priority: high" --color "d93f0b" --description "Should be addressed soon"
gh label create "priority: medium" --color "fbca04" --description "Normal priority"
gh label create "priority: low" --color "0e8a16" --description "Low priority"

# Status labels
gh label create "status: triage" --color "ededed" --description "Needs initial review"
gh label create "status: accepted" --color "0e8a16" --description "Accepted and will be worked on"
gh label create "status: in progress" --color "1d76db" --description "Currently being worked on"
gh label create "status: blocked" --color "b60205" --description "Blocked by another issue"

# Crate labels
gh label create "crate: arxis" --color "006b75" --description "Main arxis_quaternions library"
gh label create "crate: avila-math" --color "006b75" --description "Mathematical kernel"
gh label create "crate: avila-telemetry" --color "006b75" --description "Time series & analytics"

# Area labels
gh label create "area: LISA" --color "5319e7" --description "LISA mission & gravitational waves"
gh label create "area: physics" --color "5319e7" --description "Physics implementations"
gh label create "area: quaternions" --color "5319e7" --description "Quaternion algebra"

# Difficulty labels
gh label create "good first issue" --color "7057ff" --description "Good for newcomers"
gh label create "help wanted" --color "008672" --description "Extra attention needed"

# Special labels
gh label create "breaking change" --color "b60205" --description "Breaks backward compatibility"
gh label create "pinned" --color "d4c5f9" --description "Should not be closed by stale bot"
gh label create "NASA mission" --color "000080" --description "Related to NASA/ESA missions"
```

## References

- [GitHub Labels Documentation](https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/managing-labels)
- [Standard Labels](https://github.com/github/docs/blob/main/content/issues/using-labels-and-milestones-to-track-work/managing-labels.md)
- [Label Colors](https://github.com/github/docs/blob/main/content/issues/using-labels-and-milestones-to-track-work/managing-labels.md#choosing-label-colors)

---

*Labels configuration for Arxis - The Mathematical Citadel 🏛️*
