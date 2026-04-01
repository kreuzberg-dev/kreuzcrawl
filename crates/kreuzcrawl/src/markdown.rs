//! HTML-to-Markdown conversion — always active.

/// Convert an HTML string to Markdown.
///
/// Delegates to `html-to-markdown-rs` in a blocking task so the
/// conversion does not block the async runtime.
pub(crate) async fn convert_to_markdown(html: &str) -> Option<String> {
    let html = html.to_owned();
    tokio::task::spawn_blocking(move || {
        html_to_markdown_rs::convert(&html, None)
            .ok()
            .and_then(|r| r.content)
    })
    .await
    .ok()
    .flatten()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn converts_heading() {
        let md = convert_to_markdown("<h1>Hello</h1>").await;
        let md = md.expect("should produce markdown");
        assert!(
            md.contains("# Hello"),
            "expected '# Hello' in markdown, got: {md}"
        );
    }

    #[tokio::test]
    async fn converts_paragraph() {
        let md = convert_to_markdown("<p>Some text.</p>").await;
        let md = md.expect("should produce markdown");
        assert!(
            md.contains("Some text."),
            "expected 'Some text.' in markdown, got: {md}"
        );
    }

    #[tokio::test]
    async fn converts_link() {
        let md = convert_to_markdown(r#"<a href="https://example.com">Click</a>"#).await;
        let md = md.expect("should produce markdown");
        assert!(
            md.contains("[Click](https://example.com)"),
            "expected markdown link, got: {md}"
        );
    }

    #[tokio::test]
    async fn converts_full_page() {
        let html = r#"<html><head><title>Test</title></head><body>
            <h1>Hello World</h1>
            <p>This is a paragraph.</p>
            <a href="/link">Click here</a>
        </body></html>"#;
        let md = convert_to_markdown(html).await;
        let md = md.expect("should produce markdown");
        assert!(md.contains("# Hello World"), "missing heading: {md}");
        assert!(
            md.contains("This is a paragraph."),
            "missing paragraph: {md}"
        );
        assert!(md.contains("[Click here]"), "missing link text: {md}");
    }

    #[tokio::test]
    async fn empty_html_returns_some() {
        let md = convert_to_markdown("").await;
        // Even empty HTML should return Some (possibly empty string)
        assert!(md.is_some(), "empty html should still return Some");
    }
}
