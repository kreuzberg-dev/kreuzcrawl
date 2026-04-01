//! Link-to-citations conversion for LLM-optimized markdown.
//!
//! Converts inline markdown links `[text](url)` to numbered citations `text[1]`
//! with a reference list appended at the end.

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// Result of citation conversion.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CitationResult {
    /// Markdown with links replaced by numbered citations.
    pub content: String,
    /// Numbered reference list: (index, url, text).
    pub references: Vec<CitationReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CitationReference {
    pub index: usize,
    pub url: String,
    pub text: String,
}

static LINK_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\[([^\]]*)\]\(([^)]+)\)").unwrap());

/// Convert markdown links to numbered citations.
///
/// `[Example](https://example.com)` becomes `Example[1]`
/// with `[1]: https://example.com` in the reference list.
pub fn generate_citations(markdown: &str) -> CitationResult {
    let mut references = Vec::new();
    let mut seen_urls = std::collections::HashMap::new();

    let content = LINK_RE.replace_all(markdown, |caps: &regex::Captures| {
        let text = &caps[1];
        let url = &caps[2];

        let index = if let Some(&idx) = seen_urls.get(url) {
            idx
        } else {
            let idx = references.len() + 1;
            seen_urls.insert(url.to_owned(), idx);
            references.push(CitationReference {
                index: idx,
                url: url.to_owned(),
                text: text.to_owned(),
            });
            idx
        };

        format!("{text}[{index}]")
    });

    CitationResult {
        content: content.into_owned(),
        references,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_citation() {
        let md = "Visit [Example](https://example.com) for more.";
        let result = generate_citations(md);
        assert_eq!(result.content, "Visit Example[1] for more.");
        assert_eq!(result.references.len(), 1);
        assert_eq!(result.references[0].url, "https://example.com");
    }

    #[test]
    fn test_duplicate_urls_same_index() {
        let md = "[A](https://a.com) and [B](https://a.com)";
        let result = generate_citations(md);
        assert!(result.content.contains("A[1]"));
        assert!(result.content.contains("B[1]"));
        assert_eq!(result.references.len(), 1);
    }

    #[test]
    fn test_no_links() {
        let md = "No links here.";
        let result = generate_citations(md);
        assert_eq!(result.content, "No links here.");
        assert!(result.references.is_empty());
    }

    #[test]
    fn test_multiple_different_links() {
        let md = "[A](https://a.com) [B](https://b.com) [C](https://c.com)";
        let result = generate_citations(md);
        assert_eq!(result.references.len(), 3);
        assert!(result.content.contains("[1]"));
        assert!(result.content.contains("[2]"));
        assert!(result.content.contains("[3]"));
    }
}
