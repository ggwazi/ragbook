#!/bin/bash
# Run the Ragbook development server

set -e

echo "🚀 Starting Ragbook server..."
echo "   Server will be available at http://localhost:3000"
echo ""

RUST_LOG=${RUST_LOG:-"ragbook=debug,tower_http=debug"} cargo run "$@"
