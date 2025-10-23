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
- **Path parameters**: Use `<P: AsRef<Path>>` when functions accept paths
- **Comments**: End comment sentences with periods for readability
- **TODOs**: Format as `// TODO: {description}.` (sentence case, ends with period)

## File Organization (One Item Per File)

- **Guideline**: Place one public item (struct, enum, or trait) per file as a general rule.
  - File names should match the item name (e.g., `ElementData` struct goes in `element_data.rs`)
  - Each file contains the item and all its implementations (Display, Default, methods, etc.)
  - Type aliases are exempt from this rule and can be grouped logically
  - When violating this guideline, include a comment explaining why:

    ```rust
    // Multiple node types grouped together for cohesion.
    pub struct NodeRef { ... }
    pub struct Node { ... }
    ```

- **Benefits of this pattern**:
  - Clear file-to-type mapping for navigation
  - Focused context when editing specific types
  - Precise git history (changes to specific types only touch their files)
  - Reduced merge conflicts when working on different types
  - Tests can be colocated with their specific type

## Module Organization (mod.rs as Table of Contents)

- **Rule**: `mod.rs` files should **only** contain module declarations and re-exports.
  - All implementation code (structs, enums, functions, impls, tests) must be in separate files
  - `mod.rs` serves as the module's table of contents
  - Each public item gets its own file following the "one item per file" convention
- **Example structure**:

  ```rust
  // tree/mod.rs - GOOD: Only declarations and re-exports
  pub mod element_data;
  pub mod node;
  pub mod node_ref;
  pub mod node_data;

  pub use element_data::ElementData;
  pub use node::Node;
  pub use node_ref::NodeRef;
  pub use node_data::NodeData;
  ```

- **Benefits**:
  - `mod.rs` becomes a clear module index
  - Easier navigation (know exactly where each type lives)
  - Better git blame/history (changes to types don't affect module structure)
  - Consistent with "one item per file" convention
  - Clearer separation of concerns

## Dependencies

- Keep dependencies minimal and well-vetted
- Prefer widely-used, maintained crates
- **IMPORTANT**: Never add dependencies without giving me a chance to review it BEFORE you add them.

## Testing

- Write unit tests in `#[cfg(test)]` modules
- Use `#[should_panic]` or `Result` types for error case testing

## Linting

- Run `cargo clippy --all-targets` before committing
- Address all clippy warnings
- See `Cargo.toml` for enabled lints (many documentation lints are currently disabled but should be enabled incrementally)
- Examples may use `#![allow(clippy::print_stdout)]` when appropriate
