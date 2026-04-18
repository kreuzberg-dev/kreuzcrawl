//! Content processing: tag removal, word count, main content extraction.

use tl::{ParserOptions, VDom};

use super::selectors::SEL_MAIN_CONTENT;

/// Remove elements matching the given CSS selectors from the HTML string.
pub(crate) fn apply_remove_tags(html: &str, tags: &[String]) -> String {
    let Ok(dom) = tl::parse(html, ParserOptions::default()) else {
        return html.to_owned();
    };
    let parser = dom.parser();
    let serialized = dom.outer_html();
    let mut ranges: Vec<(usize, usize)> = Vec::new();
    for tag in tags {
        if let Some(iter) = dom.query_selector(tag) {
            for handle in iter {
                if let Some(node) = handle.get(parser) {
                    let fragment = node.outer_html(parser).to_string();
                    // Find the exact position in the serialized output
                    if let Some(pos) = serialized.find(&fragment) {
                        ranges.push((pos, pos + fragment.len()));
                    }
                }
            }
        }
    }
    if ranges.is_empty() {
        return html.to_owned();
    }
    // Sort by start position descending so we remove from end first,
    // preserving earlier offsets.
    ranges.sort_by_key(|b| std::cmp::Reverse(b.0));
    // Deduplicate overlapping ranges
    ranges.dedup_by(|a, b| a.0 >= b.0 && a.0 < b.1);
    let mut output = serialized;
    for (start, end) in &ranges {
        output.replace_range(*start..*end, "");
    }
    output
}

/// Compute the word count of visible text in the HTML body.
pub(crate) fn compute_word_count(dom: &VDom<'_>) -> usize {
    let parser = dom.parser();
    let body_text = dom
        .query_selector("body")
        .and_then(|mut iter| {
            iter.next()
                .and_then(|h| h.get(parser))
                .map(|node| node.inner_text(parser).to_string())
        })
        .unwrap_or_default();
    body_text.split_whitespace().count()
}

/// Extract the main content from an HTML page.
pub(crate) fn extract_main_content(html: &str) -> String {
    let Ok(dom) = tl::parse(html, ParserOptions::default()) else {
        return html.to_owned();
    };
    let parser = dom.parser();
    dom.query_selector(SEL_MAIN_CONTENT)
        .and_then(|mut iter| {
            iter.next()
                .and_then(|h| h.get(parser))
                .and_then(|node| node.as_tag())
                .map(|tag| tag.outer_html(parser))
        })
        .unwrap_or_else(|| html.to_owned())
}
