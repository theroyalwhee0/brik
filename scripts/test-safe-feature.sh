#!/usr/bin/env bash

# Test that the safe feature successfully eliminates all unsafe code

set -e

echo "Testing safe feature with unsafe code forbidden..."
echo ""

# Test 1: Build with safe feature should succeed even with -F unsafe-code
echo "Test 1: Building with safe feature and unsafe forbidden..."
if ! RUSTFLAGS="-F unsafe-code" cargo build --features safe --quiet; then
    echo "❌ FAILED: Safe mode build failed with -F unsafe-code"
    exit 1
fi
echo "✓ Passed: Safe mode builds successfully"
echo ""

# Test 2: Test with safe feature should succeed
echo "Test 2: Testing with safe feature and unsafe forbidden..."
if ! RUSTFLAGS="-F unsafe-code" cargo test --features safe --quiet; then
    echo "❌ FAILED: Safe mode tests failed with -F unsafe-code"
    exit 1
fi
echo "✓ Passed: Safe mode tests pass"
echo ""

# Test 3: Default build should fail with -F unsafe-code (has unsafe blocks)
echo "Test 3: Verifying default mode has unsafe code..."
if RUSTFLAGS="-F unsafe-code" cargo build --quiet 2>/dev/null; then
    echo "❌ FAILED: Default build should fail with -F unsafe-code (no unsafe blocks found)"
    exit 1
fi
echo "✓ Passed: Default mode correctly uses unsafe code"
echo ""

echo "✅ All safe feature tests passed!"
echo "   1. Safe mode builds successfully with -F unsafe-code"
echo "   2. Safe mode tests pass with -F unsafe-code"
echo "   3. Default mode correctly uses unsafe code"
