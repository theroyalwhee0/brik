# Project Coding Standards

## Workflow

### Working with GitHub Issues

- **ALWAYS use `focus-issue <number>` to start work on an issue** - this command:
  - Fetches the issue details and saves them to `.focus/Task.md`
  - Creates or switches to the appropriate issue branch
  - Handles branch management and cleanup automatically
  - Archives previous `.focus/` directories to keep workspace clean
- **DO NOT manually**:
  - Use `gh issue view` - the focus-issue command does this for you
  - Create issue branches - the focus-issue command handles this
  - Manually create `.focus/` directories - always use focus-issue

**Example:**

```bash
# Start working on issue #41
focus-issue 41

# The command automatically:
# - Fetches issue content to .focus/Task.md
# - Creates/switches to branch 41-publish-v090
# - Archives any previous .focus directory
```

## Communication

- **Time estimates**: Don't include time estimates unless explicitly requested
- **Git commits**: Do not commit changes automatically after completing work. Wait for explicit request to commit. The user reviews diffs before committing.
- **Tone**: We're a team of developers. No need to apologize excessively or over-validate. Mistakes happen in development - acknowledge them briefly and move forward with solutions.

## Documentation Standards

### Public API Documentation

- **Comprehensive coverage**: All public items should be documented with `///` doc comments
- Include `# Errors`, `# Panics`, and `# Safety` sections where applicable
- **Documentation Accuracy**: Document only current features; do not document future plans unless imminent (use TODO comments for future work)
- All linting rules are defined in `Cargo.toml`

### Implementation Block Documentation

- **All impl blocks must be documented**, even for standard trait implementations:
  - Standard traits like `Debug`, `Display`, `Clone`, `Eq`, `PartialEq`, `Deref`, etc.
  - Custom trait implementations
  - Inherent impl blocks with methods
- **Format**: Add a `///` doc comment above the impl block explaining its purpose:

  ```rust
  /// Implements Display for NodeData.
  ///
  /// Formats the node data for display purposes, showing the node type
  /// and relevant information based on the variant.
  impl fmt::Display for NodeData {
      // implementation
  }
  ```

- **When to document**:
  - Create documentation when writing new impl blocks
  - Only add documentation where it's missing; do not rewrite existing good documentation
  - If existing documentation is clear and accurate, leave it unchanged

### Test Documentation

- **Every test must have documentation** explaining what it tests and why:

  ```rust
  /// Tests cloning AttrValue instances.
  ///
  /// Verifies that the Clone implementation produces an independent
  /// copy with identical contents.
  #[test]
  fn clone() {
      // test implementation
  }
  ```

- **Content guidelines**:
  - First line: Brief summary starting with "Tests..." or "Verifies..."
  - Second paragraph: Explain what behavior is being validated
  - Mention edge cases, boundary conditions, or important invariants being tested
  - End all sentences with periods
- Document test panic expectations with `#[should_panic]` and explanation
- **When to document**: Create test documentation when writing the test, not as a separate pass

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
- All documentation lints are enabled and enforced (see `[lints.rustdoc]` and `[lints.clippy]` in `Cargo.toml`)
- Examples may use `#![allow(clippy::print_stdout)]` when appropriate
- When an issue is empty please await further explanation.
- Put temp files in .focus/ if it exists.
