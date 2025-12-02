#!/bin/bash
# Run tests for Ragbook

set -e

echo "🧪 Running Ragbook tests..."

if [ "$1" == "--coverage" ]; then
    echo "Running tests with coverage..."
    cargo install cargo-tarpaulin --locked 2>/dev/null || true
    cargo tarpaulin --out Html
    echo "✅ Coverage report generated: tarpaulin-report.html"
else
    cargo test "$@"
    echo "✅ All tests passed!"
fi
