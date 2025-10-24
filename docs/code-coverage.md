# Code Coverage

This project maintains high code coverage standards to ensure code quality and reliability.

## Current Coverage

- **Overall Coverage**: ~95%
- **Minimum Overall Threshold**: 90%
- **Minimum Per-File Threshold**: 70%

## Coverage Enforcement

### GitHub Actions CI

Coverage thresholds are automatically enforced in CI on all pull requests and pushes to `main`.

## Configuring Thresholds

Thresholds can be configured via GitHub repository variables (Settings → Secrets and variables → Actions → Variables):

- **`COVERAGE_OVERALL_THRESHOLD`**: Minimum overall line coverage % (default: 90)
- **`COVERAGE_PER_FILE_THRESHOLD`**: Minimum per-file line coverage % (default: 70)

**Benefits of GitHub Variables:**

- Change thresholds without code changes
- Temporarily lower in emergencies
- Gradually increase over time
- No pull request required

### Example: Temporarily lower threshold during a refactor

1. Go to repository Settings → Actions → Variables
2. Add `COVERAGE_OVERALL_THRESHOLD` = `85`
3. Merge refactoring PR
4. Remove or update variable to restore 90%

## Measuring Coverage Locally

### View Full Coverage Report

```bash
cargo llvm-cov --all-features --all-targets
```

### View Coverage Summary

```bash
cargo llvm-cov --all-features --all-targets --summary-only
```

### Generate HTML Report

```bash
cargo llvm-cov --all-features --all-targets --html
open target/llvm-cov/html/index.html
```

### Generate LCOV Report

```bash
cargo llvm-cov --all-features --all-targets --lcov --output-path lcov.info
```

## Writing Tests

### Test Organization

Tests are organized using Rust's built-in test framework:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature() {
        // Test implementation
    }
}
```

### Coverage Best Practices

1. **Test Public APIs**: Focus on public functions and methods
2. **Test Edge Cases**: Empty inputs, boundary conditions, error cases
3. **Test Happy Paths**: Common usage scenarios
4. **Use Exact Assertions**: Prefer `assert_eq!(value, 3)` over `assert!(value >= 3)`
5. **Document Test Intent**: Use descriptive test names

### Example Test Structure

```rust
#[test]
fn method_name_success_case() {
    let input = create_test_input();
    let result = module::method_name(input);
    assert_eq!(result, expected_value);
}

#[test]
fn method_name_error_case() {
    let invalid_input = create_invalid_input();
    let result = module::method_name(invalid_input);
    assert!(result.is_err());
}

#[test]
fn method_name_edge_case() {
    let empty_input = vec![];
    let result = module::method_name(empty_input);
    assert_eq!(result, expected_empty_result);
}
```

## Troubleshooting

### Coverage Too Low After Changes

1. **Identify uncovered code**:

   ```bash
   cargo llvm-cov --all-features --all-targets --summary-only | grep "brik/src"
   ```

2. **View detailed coverage for specific file**:

   ```bash
   cargo llvm-cov --all-features --all-targets --html
   open target/llvm-cov/html/index.html
   # Navigate to the file to see uncovered lines highlighted
   ```

3. **Add tests for uncovered lines**

4. **Verify coverage improved**:

   ```bash
   cargo llvm-cov --all-features --all-targets --summary-only | grep "your_file.rs"
   ```

### CI Failing Due to Coverage

If CI fails due to coverage:

1. **Check the CI logs** to see which files are below threshold

2. **Run tests with coverage locally**:

   ```bash
   cargo llvm-cov --all-features --all-targets
   ```

3. **Add missing tests** for the files below threshold

4. **Push changes** and CI will re-run

## Tools

### Required

- **cargo-llvm-cov**: Install with `cargo install cargo-llvm-cov`

### Optional

- **cargo-tarpaulin**: Alternative coverage tool
- **codecov.io**: Online coverage reporting (not currently configured)

## Coverage Goals

### Current Status

- ✅ Overall: ~95% (target: 90%)
- ✅ Per-file minimum: 70%

### Future Goals

Consider gradually increasing thresholds:

- Overall: 95% → 96% → 97%
- Per-file: 70% → 75% → 80%

Update thresholds by modifying the GitHub variables after coverage naturally increases through normal development.

## Related Documentation

- [Testing Guide](testing.md) - General testing practices (if exists)
- [Contributing Guide](../CONTRIBUTING.md) - Contribution workflow (if exists)
- [cargo-llvm-cov Documentation](https://github.com/taiki-e/cargo-llvm-cov)
