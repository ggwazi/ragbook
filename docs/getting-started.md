# Getting Started with Ragbook

Welcome to Ragbook, an Axum-based documentation server with markdown rendering capabilities.

## Features

- **Fast Markdown Rendering**: Uses pulldown-cmark for efficient markdown processing
- **Modern Web Stack**: Built on Axum with async/await support
- **Developer Friendly**: Includes devcontainer and development scripts
- **CI/CD Ready**: GitHub Actions workflows included

## Quick Start

### Prerequisites

- Rust 1.75 or later
- Cargo package manager

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/ggwazi/ragbook.git
   cd ragbook
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the server:
   ```bash
   cargo run
   ```

4. Open your browser and navigate to `http://localhost:3000`

## Project Structure

```
ragbook/
├── src/
│   ├── main.rs        # Application entry point
│   └── markdown.rs    # Markdown rendering utilities
├── docs/              # Documentation files (markdown)
├── templates/         # HTML templates
├── static/            # Static assets
├── scripts/           # Development scripts
└── .devcontainer/     # VS Code devcontainer configuration
```

## Configuration

The server runs on port 3000 by default. You can configure logging levels using the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```

## Next Steps

- [API Reference](api.md)
- [Contributing Guide](contributing.md)
