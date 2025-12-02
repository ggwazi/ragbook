#!/bin/bash
# Build the Ragbook application

set -e

echo "🔨 Building Ragbook..."

if [ "$1" == "--release" ]; then
    echo "Building release version..."
    cargo build --release
    echo "✅ Release build complete: target/release/ragbook"
else
    echo "Building debug version..."
    cargo build
    echo "✅ Debug build complete: target/debug/ragbook"
fi
