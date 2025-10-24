# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

> **Note**: This changelog documents changes to the `brik` fork starting from October 2025.
> For the historical changelog from the upstream `kuchikiki` project (v0.8.2 and earlier),
> see [docs/historical-changelog.md](docs/historical-changelog.md).

## [Unreleased]

### Added

- Namespace support for XML and SVG documents behind `namespaces` feature flag (#15, #21, #23, #29)
  - Namespace-aware attribute methods with prefix and URI handling
  - Namespace iterator and filtering capabilities
  - Namespace manipulation and batch removal operations
  - CSS selectors with namespace support (e.g., `svg|rect`, `*|div`)
  - Example demonstrating HTML and SVG namespace handling
- Feature flag `safe` to eliminate all unsafe code blocks (#17)
  - Default mode uses unsafe code for performance
- CI/CD workflow with GitHub Actions (#14, #16, #18, #25)
  - Automated testing for both default and safe modes
  - Clippy linting and security audits
- Comprehensive API documentation with examples (#35)
  - All public items documented
  - Panic and error documentation
  - Examples for namespace usage and template processing (#26)
- Colocated tests with implementation code (#20, #31)
  - Tests moved from separate files to inline modules
  - Improved discoverability and maintainability

### Changed

- Forked from `kuchikiki` 0.8.2 and rebranded to `brik` (2025-10-21)
- Renamed internal types from `Kuchiki*` to `Brik*` (#24, #25)
  - Public API remains compatible
- Restructured codebase following one-pub-per-file convention (#30, #32)
  - Each public type in its own file
  - `mod.rs` files serve as module table of contents
  - Improved navigation and git history tracking
- Updated dependencies (#1)

## [0.8.2] - 2023-05-15

Last release before `brik` fork from the `kuchikiki` project by Brave Browser maintainers.
See [docs/historical-changelog.md](docs/historical-changelog.md) for details.

---

**Historical Note**: This project was originally created by Simon Sapin as `kuchiki`,
maintained by Brave Browser as `kuchikiki`, and is now maintained as `brik`.

[unreleased]: https://github.com/theroyalwhee0/brik/compare/v0.8.2...HEAD
[0.8.2]: https://github.com/theroyalwhee0/brik/releases/tag/v0.8.2
