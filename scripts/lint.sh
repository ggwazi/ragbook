#!/bin/bash
# Lint the Ragbook codebase

set -e

echo "🔍 Linting Ragbook..."

echo "Running cargo fmt check..."
cargo fmt -- --check

echo "Running clippy..."
cargo clippy -- -D warnings

echo "✅ Linting complete - no issues found!"
