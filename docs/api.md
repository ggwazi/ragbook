# API Reference

This document describes the API endpoints available in Ragbook.

## Endpoints

### GET /

Returns the main index page with navigation to documentation.

**Response**: HTML page

### GET /health

Health check endpoint to verify server status.

**Response**: `200 OK` with body "OK"

### GET /docs/{path}

Renders a markdown file from the `docs/` directory as HTML.

**Parameters**:
- `path` - Path to the markdown file (e.g., `getting-started.md`)

**Response**: HTML page with rendered markdown content

**Example**:
```bash
curl http://localhost:3000/docs/getting-started.md
```

### GET /static/{path}

Serves static files from the `static/` directory.

**Parameters**:
- `path` - Path to the static file

## Markdown Features

The markdown renderer supports:

| Feature | Syntax | Example |
|---------|--------|---------|
| Headers | `# Header` | # Header |
| Bold | `**bold**` | **bold** |
| Italic | `*italic*` | *italic* |
| Links | `[text](url)` | [text](url) |
| Code | `` `code` `` | `code` |
| Tables | See above | Table support |
| Task lists | `- [x] Task` | Checkboxes |
| Strikethrough | `~~text~~` | ~~text~~ |

## Error Handling

### 404 Not Found

Returned when a requested document doesn't exist.

```html
<h1>404 - Document Not Found</h1>
```

## Examples

### Using curl

```bash
# Check server health
curl http://localhost:3000/health

# Get documentation
curl http://localhost:3000/docs/getting-started.md

# Get index page
curl http://localhost:3000/
```

### Using httpie

```bash
http GET localhost:3000/health
http GET localhost:3000/docs/api.md
```
