# Arxis Project Governance

## Project Leadership

Arxis is currently maintained by **Nicolas Ávila** as the founding developer and project lead.

### Maintainer
- **Name**: Nicolas Ávila
- **Email**: nicolas@avila.inc
- **GitHub**: [@avilaops](https://github.com/avilaops)
- **WhatsApp**: +55 17 99781-1471
- **Role**: Project founder, lead developer, and maintainer

## Decision Making Process

### Current Phase: Benevolent Dictator For Life (BDFL)

As Arxis is in active development with a single primary maintainer, decisions are made by Nicolas Ávila with community input through:

1. **GitHub Discussions** - Feature discussions and design decisions
2. **GitHub Issues** - Bug reports, feature requests, and proposals
3. **Pull Requests** - Code contributions with review process
4. **Direct Communication** - Email or WhatsApp for urgent matters

### Future Governance Model

As the project grows, we plan to transition to a **Meritocratic Governance** model:

1. **Core Team**: Active contributors with commit access
2. **Maintainers**: Responsible for specific crates or subsystems
3. **Contributors**: Community members who submit PRs and participate in discussions
4. **Steering Committee**: (Future) For major architectural decisions

## Contribution Process

### 1. Proposal Stage
- Open a **GitHub Discussion** or **Issue** for significant changes
- Describe the problem, proposed solution, and alternatives
- Gather community feedback (minimum 7 days for major changes)

### 2. Review Stage
- Submit a **Pull Request** following the PR template
- Automated CI checks must pass
- Code review by maintainers
- Address feedback and requested changes

### 3. Acceptance Stage
- PR is approved by a maintainer
- Merged to main branch
- Contributor is acknowledged in CHANGELOG
- Changes included in next release

## Types of Changes

### Minor Changes (Direct PR)
- Bug fixes
- Documentation improvements
- Code cleanup/refactoring
- Test additions
- Performance optimizations (non-breaking)

**Process**: Submit PR directly, no prior discussion needed

### Major Changes (Discussion First)
- New crates or major modules
- Breaking API changes
- Significant architectural changes
- New dependencies
- Performance changes that may break compatibility

**Process**: Open Discussion or Issue first, then PR after consensus

## Crate Ownership

### Current Ownership (All: Nicolas Ávila)
- `arxis_quaternions` - Main library
- `avila-math` - Mathematical kernel
- `avila-telemetry` - Time series & analytics
- `avila-compress` - Compression library
- `avila-tokenizers` - NLP tokenization
- `aviladb` - Database system
- `avx-*` - AVL Platform crates

### Future Ownership Model
As the project grows, we'll establish **crate maintainers** for each major component.

## Release Process

### Version Numbering
We follow **Semantic Versioning 2.0.0**:
- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes (backward compatible)

### Release Cycle
- **Patch releases**: As needed for critical bugs
- **Minor releases**: Monthly or when features are ready
- **Major releases**: When breaking changes accumulate

### Release Checklist
1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run full test suite (`cargo test --workspace`)
4. Run `cargo clippy` and `cargo fmt`
5. Create git tag (e.g., `v0.3.0`)
6. Publish to crates.io
7. Create GitHub Release with notes
8. Update documentation site

## Code of Conduct

All contributors must follow our [Code of Conduct](CODE_OF_CONDUCT.md), which is based on the Contributor Covenant 2.1.

### Enforcement
- **First violation**: Private warning
- **Second violation**: Public warning
- **Third violation**: Temporary ban (30-90 days)
- **Severe/repeated violations**: Permanent ban

Reports should be sent to nicolas@avila.inc and will be handled confidentially.

## Intellectual Property

### Licensing
Arxis is **dual-licensed** under:
- **MIT License** - [LICENSE-MIT](LICENSE-MIT)
- **Apache License 2.0** - [LICENSE-APACHE](LICENSE-APACHE)

### Contributor License Agreement (CLA)
By submitting a pull request, you agree that:
1. You have the right to license your contribution
2. Your contribution is licensed under MIT and Apache-2.0
3. You grant the project maintainers the right to use your contribution

No formal CLA document is required - submission of a PR constitutes agreement.

## Communication Channels

### Official Channels
1. **GitHub Issues** - Bug reports and feature requests
2. **GitHub Discussions** - Design discussions and Q&A
3. **GitHub Pull Requests** - Code contributions
4. **Email** (nicolas@avila.inc) - Security issues, governance questions

### Response Times (Best Effort)
- **Security issues**: Within 24 hours
- **Bug reports**: Within 7 days
- **Feature requests**: Within 14 days
- **Pull requests**: Within 7 days (initial review)

### Language
- Primary: **English** (for code, documentation, issues, PRs)
- Secondary: **Portuguese** (README sections, discussions)

## Conflict Resolution

### Process
1. **Discussion**: Try to resolve disagreement in the PR/Issue
2. **Mediation**: Maintainer mediates if discussion stalls
3. **Decision**: Maintainer makes final decision with explanation
4. **Appeal**: Can be appealed to the project lead (currently Nicolas)

### Philosophy
- **Technical merit** over personal preference
- **Data and benchmarks** over opinions
- **Community benefit** over individual convenience
- **Long-term maintainability** over short-term convenience

## Funding and Sponsorship

Currently, Arxis is developed as **open-source** without direct funding.

### Future Sponsorship
We may accept sponsorship through:
- GitHub Sponsors
- Open Collective
- Corporate sponsors

### Transparency
- All funding will be disclosed publicly
- Sponsorship does not grant decision-making power
- Funds will be used for: hosting, CI resources, development time

## Recognition

### Contributor Recognition
- **CONTRIBUTORS.md** - List of all contributors
- **CHANGELOG.md** - Credit for each contribution
- **GitHub Releases** - Acknowledgment of contributors

### Roles
- **Maintainer**: Commit access, release authority
- **Contributor**: Merged PRs, active participation
- **Community Member**: Discussions, issue reports, testing

## Amendment Process

This governance document can be amended by:
1. Opening a **Discussion** proposing changes
2. Community feedback period (minimum 14 days)
3. Final decision by project lead
4. PR to update this document

## Roadmap and Planning

### Public Roadmap
- Major features tracked in **GitHub Projects**
- Milestones for releases in **GitHub Milestones**
- Long-term vision in **MANIFESTO.md**

### Community Input
- Feature voting through 👍 reactions on issues
- Design discussions in GitHub Discussions
- User feedback from real-world usage

## Scientific Integrity

Arxis is designed for **scientific computing** and **astrophysics research**. We are committed to:

### Correctness
- Rigorous testing (101+ tests)
- Validation against known results (GW150914, PSR B1913+16)
- Peer review of physics implementations
- Clear documentation of assumptions and limitations

### Reproducibility
- All algorithms documented
- Example code for all features
- Benchmarks for performance claims
- Open access to code and data

### References
- Citations to scientific papers
- Links to NASA/ESA mission documentation
- Attribution to prior art and related work

## Related Projects

### AVL Cloud Platform
Arxis is part of the larger **AVL (Avila Cloud Platform)** ecosystem:
- **Shared governance** for common components
- **Independent releases** for each crate
- **Coordinated roadmap** across platform

## Contact

For governance questions, concerns, or proposals:

**Nicolas Ávila**
Email: nicolas@avila.inc
GitHub: [@avilaops](https://github.com/avilaops)
WhatsApp: +55 17 99781-1471

---

**Last Updated**: November 23, 2025
**Version**: 1.0
**Status**: Active

*This governance model will evolve as the project and community grow.*
