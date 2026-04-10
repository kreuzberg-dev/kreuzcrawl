//! Feed, favicon, hreflang, and heading extraction from HTML documents.

use tl::VDom;

use crate::types::{FaviconInfo, FeedInfo, FeedType, HeadingInfo, HreflangEntry};

use super::get_attr;
use super::selectors::{SEL_FAVICON, SEL_FEED_ALTERNATE, SEL_HEADINGS, SEL_HREFLANG};

/// Extract feed links (RSS, Atom, JSON Feed) from a parsed HTML document.
pub(crate) fn extract_feeds(dom: &VDom<'_>) -> Vec<FeedInfo> {
    let parser = dom.parser();
    let mut feeds = Vec::new();

    if let Some(iter) = dom.query_selector(SEL_FEED_ALTERNATE) {
        for handle in iter {
            let Some(tag) = handle.get(parser).and_then(|n| n.as_tag()) else {
                continue;
            };
            let link_type = get_attr(tag, "type").unwrap_or("");
            let href = get_attr(tag, "href").unwrap_or("").to_owned();
            let title = get_attr(tag, "title").map(String::from);

            let feed_type = match link_type {
                "application/rss+xml" => Some(FeedType::Rss),
                "application/atom+xml" => Some(FeedType::Atom),
                "application/json" | "application/feed+json" => Some(FeedType::JsonFeed),
                _ => None,
            };

            if let Some(ft) = feed_type {
                feeds.push(FeedInfo {
                    url: href,
                    title,
                    feed_type: ft,
                });
            }
        }
    }
    feeds
}

/// Extract hreflang alternate links from a parsed HTML document.
pub(crate) fn extract_hreflangs(dom: &VDom<'_>) -> Vec<HreflangEntry> {
    let parser = dom.parser();
    let mut entries = Vec::new();
    if let Some(iter) = dom.query_selector(SEL_HREFLANG) {
        for handle in iter {
            let Some(tag) = handle.get(parser).and_then(|n| n.as_tag()) else {
                continue;
            };
            let lang = get_attr(tag, "hreflang").unwrap_or("").to_owned();
            let url = get_attr(tag, "href").unwrap_or("").to_owned();
            if !lang.is_empty() && !url.is_empty() {
                entries.push(HreflangEntry { lang, url });
            }
        }
    }
    entries
}

/// Extract favicon and icon links from a parsed HTML document.
pub(crate) fn extract_favicons(dom: &VDom<'_>) -> Vec<FaviconInfo> {
    let parser = dom.parser();
    let mut favicons = Vec::new();
    if let Some(iter) = dom.query_selector(SEL_FAVICON) {
        for handle in iter {
            let Some(tag) = handle.get(parser).and_then(|n| n.as_tag()) else {
                continue;
            };
            let url = get_attr(tag, "href").unwrap_or("").to_owned();
            if url.is_empty() {
                continue;
            }
            let rel = get_attr(tag, "rel").unwrap_or("").to_owned();
            let sizes = get_attr(tag, "sizes").map(String::from);
            let mime_type = get_attr(tag, "type").map(String::from);
            favicons.push(FaviconInfo {
                url,
                rel,
                sizes,
                mime_type,
            });
        }
    }
    favicons
}

/// Extract heading elements (h1-h6) from a parsed HTML document.
pub(crate) fn extract_headings(dom: &VDom<'_>) -> Vec<HeadingInfo> {
    let parser = dom.parser();
    let mut headings = Vec::new();
    if let Some(iter) = dom.query_selector(SEL_HEADINGS) {
        for handle in iter {
            let Some(tag) = handle.get(parser).and_then(|n| n.as_tag()) else {
                continue;
            };
            let tag_name = tag.name().as_utf8_str();
            let level = match tag_name.as_ref() {
                "h1" => 1,
                "h2" => 2,
                "h3" => 3,
                "h4" => 4,
                "h5" => 5,
                "h6" => 6,
                _ => continue,
            };
            let text = tag.inner_text(parser).trim().to_owned();
            headings.push(HeadingInfo { level, text });
        }
    }
    headings
}
