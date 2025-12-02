//! Markdown rendering utilities using pulldown-cmark

use pulldown_cmark::{html, Options, Parser};

/// Render markdown content to HTML
///
/// # Arguments
/// * `content` - The markdown content to render
///
/// # Returns
/// The rendered HTML string
pub fn render_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_heading() {
        let markdown = "# Hello World";
        let html = render_markdown(markdown);
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello World"));
    }

    #[test]
    fn test_render_paragraph() {
        let markdown = "This is a paragraph.";
        let html = render_markdown(markdown);
        assert!(html.contains("<p>"));
        assert!(html.contains("This is a paragraph."));
    }

    #[test]
    fn test_render_code_block() {
        let markdown = "```rust\nfn main() {}\n```";
        let html = render_markdown(markdown);
        assert!(html.contains("<code"));
        assert!(html.contains("fn main()"));
    }

    #[test]
    fn test_render_list() {
        let markdown = "- Item 1\n- Item 2\n- Item 3";
        let html = render_markdown(markdown);
        assert!(html.contains("<ul>"));
        assert!(html.contains("<li>"));
    }

    #[test]
    fn test_render_table() {
        let markdown = "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |";
        let html = render_markdown(markdown);
        assert!(html.contains("<table>"));
        assert!(html.contains("<th>"));
    }

    #[test]
    fn test_render_strikethrough() {
        let markdown = "~~strikethrough~~";
        let html = render_markdown(markdown);
        assert!(html.contains("<del>"));
    }

    #[test]
    fn test_render_tasklist() {
        let markdown = "- [x] Task done\n- [ ] Task pending";
        let html = render_markdown(markdown);
        assert!(html.contains("checked"));
    }
}
