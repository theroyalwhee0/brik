# Project Coding Standards

## Communication

- **Time estimates**: Don't include time estimates unless explicitly requested

## Documentation Standards

- **Comprehensive coverage**: All public items should be documented
- Include `# Errors`, `# Panics`, and `# Safety` sections where applicable
- Document test panic expectations
- **Documentation Accuracy**: Document only current features; do not document future plans unless imminent (use TODO comments for future work)
- All linting rules are defined in `Cargo.toml`

## Code Quality

- **Minimize unsafe code**: Use `unsafe` only when necessary and document safety requirements
- **Error handling**: Use `Result` types with descriptive errors
- **Feature gates**: Use `#[cfg(feature = "...")]` for optional functionality (e.g., bloom-filter)
- **Path parameters**: Use `<P: AsRef<Path>>` when functions accept paths
- **Comments**: End comment sentences with periods for readability
- **TODOs**: Format as `// TODO: {description}.` (sentence case, ends with period)

## Dependencies

- Keep dependencies minimal and well-vetted
- Prefer widely-used, maintained crates
- **IMPORTANT**: Never add dependencies without giving me a chance to review it BEFORE you add them.

## Testing

- Write unit tests in `#[cfg(test)]` modules
- Test both with and without feature flags (e.g., `cargo test --features bloom-filter`)
- Use `#[should_panic]` or `Result` types for error case testing

## Linting

- Run `cargo clippy --all-targets` before committing
- Address all clippy warnings
- See `Cargo.toml` for enabled lints (many documentation lints are currently disabled but should be enabled incrementally)
- Examples may use `#![allow(clippy::print_stdout)]` when appropriate
