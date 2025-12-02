# Ragbook 📚

An Axum-based documentation server with markdown rendering capabilities, built in Rust.

## Features

- **Fast Markdown Rendering**: Uses pulldown-cmark for efficient markdown processing
- **Modern Web Stack**: Built on Axum with async/await support
- **Developer Friendly**: Includes devcontainer and development scripts
- **CI/CD Ready**: GitHub Actions workflows included

## Quick Start

### Prerequisites

- Rust 1.75 or later
- Cargo package manager

### Running the Server

```bash
# Build and run in development mode
cargo run

# Or use the convenience script
./scripts/run.sh
```

The server will start at `http://localhost:3000`.

## Development

### Using DevContainer

This project includes a devcontainer configuration for VS Code. Open the project in VS Code and select "Reopen in Container" when prompted.

### Available Scripts

| Script | Description |
|--------|-------------|
| `scripts/build.sh` | Build the project (add `--release` for release build) |
| `scripts/run.sh` | Run the development server |
| `scripts/test.sh` | Run tests (add `--coverage` for coverage report) |
| `scripts/lint.sh` | Run linting checks (fmt + clippy) |

### Project Structure

```
ragbook/
├── src/
│   ├── main.rs        # Application entry point
│   └── markdown.rs    # Markdown rendering utilities
├── docs/              # Documentation files (markdown)
├── templates/         # HTML templates
├── static/            # Static assets
├── scripts/           # Development scripts
├── .devcontainer/     # VS Code devcontainer configuration
└── .github/workflows/ # CI/CD workflows
```

## API Endpoints

| Endpoint | Description |
|----------|-------------|
| `GET /` | Index page with navigation |
| `GET /docs/{path}` | Render markdown documentation |
| `GET /health` | Health check endpoint |
| `GET /static/{path}` | Serve static files |

## License

MIT