# Contributing

Thank you for your interest in contributing to kuchikiki! This document provides guidelines for contributing to the project.

## Code of Conduct

This project adheres to the Contributor Covenant [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

If you find a bug, please create an issue with:

- A clear, descriptive title
- Steps to reproduce the problem
- Expected vs actual behavior
- Your environment (OS, Rust version, kuchikiki version)
- Any relevant error messages or logs

### Suggesting Enhancements

Enhancement suggestions are welcome! Please create an issue describing:

- The motivation for the enhancement
- A clear description of the proposed functionality
- Any potential implementation considerations

### Pull Requests

1. **Fork and Clone**: Fork the repository and clone it locally
2. **Create a Branch**: Create a feature branch from `main`
3. **Make Changes**: Follow the project's coding standards (see below)
4. **Test**: Ensure all tests pass with `cargo test`
5. **Lint**: Run `cargo clippy` and address any warnings
6. **Commit**: Write clear, descriptive commit messages
7. **Push**: Push your branch to your fork
8. **Open a PR**: Submit a pull request to the `main` branch

## Development Setup

```bash
# Clone your fork
git clone https://github.com/YOUR-USERNAME/kuchikiki.git
cd kuchikiki

# Build the project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy

# Run with bloom-filter feature
cargo test --features bloom-filter
```

## Coding Standards

This project maintains strict code quality standards:

### Required Practices

- **Comprehensive documentation**: All public items should be documented
  - Include `# Errors`, `# Panics`, and `# Safety` sections where applicable
- **Follow project conventions**: Maintain consistency with existing code style
- **Clear naming**: Use descriptive names for functions, variables, and types
- **Error handling**: Use `Result` types with descriptive error variants

### Code Organization

- **Module structure**: Follow the existing organizational patterns
- **Error handling**: Use appropriate error types and provide helpful error messages
- **Testing**: Add tests for new functionality in `#[cfg(test)]` modules or separate test files

### Version Control

When adding new files, ensure they are properly tracked:

- Follow the project's `.gitignore` patterns
- Don't commit generated files or build artifacts
- Review file permissions before committing

## Adding New Features

When adding new features:

1. Discuss significant changes in an issue first
2. Follow the project's existing architecture patterns
3. Update documentation (README.md, rustdoc comments, etc.)
4. Add comprehensive tests for new functionality
5. Ensure all existing tests pass

## Testing

- Write unit tests for new functionality
- Consider integration tests for complex features
- Ensure tests pass with `cargo test`
- Test with feature flags: `cargo test --features bloom-filter`
- Use `#[should_panic]` or `Result` types for error case testing

## Documentation

- Update README.md for user-facing changes
- Add rustdoc comments for all public APIs
- Include examples in documentation where helpful
- Run `cargo doc --open` to preview documentation

## Dependencies

- Keep dependencies minimal and well-vetted
- Prefer widely-used, maintained crates
- Discuss new dependencies before adding them in an issue
- Consider using feature flags for optional dependencies

## Questions?

If you have questions about contributing, feel free to:

- Open an issue for discussion
- Check existing issues and pull requests for context

Thank you for contributing!
