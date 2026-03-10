# ADR-004: Metadata Extraction — Exhaustive HTML Metadata Model

**Status**: Accepted (updated 2026-03-10)

**Date**: 2026-03-09

## Context

Crawled web pages contain rich metadata beyond the visible text: Open Graph tags for social sharing, Twitter Card markup, Dublin Core for academic/institutional content, JSON-LD for structured data, RSS/Atom feed references, and dozens of other meta tags. Different consumers of crawl data need different subsets of this metadata.

Firecrawl and similar tools extract a curated subset. We want to extract *everything* — let the consumer decide what's relevant. This maximizes the value of each crawl operation and avoids re-crawling when a new metadata field becomes interesting.

## Decision

### Exhaustive Extraction Strategy

Extract all discoverable metadata from every crawled page into a structured `PageMetadata` type. Categories:

1. **Core HTML metadata** — title, description, language, charset, keywords, robots, canonical URL
2. **Open Graph** — og:title, og:description, og:url, og:image, og:audio, og:video, og:type, og:site_name, og:locale (+ alternates)
3. **Twitter Card** — twitter:card, twitter:site, twitter:creator, twitter:title, twitter:description, twitter:image
4. **Article metadata** — article:published_time, article:modified_time, article:section, article:tag, article:author (via `ArticleMetadata` struct)
5. **Dublin Core** — dc.title, dc.creator, dc.subject, dc.description, dc.date, dc.type, dc.format, dc.language, dc.rights
6. **Feed discovery** — RSS, Atom, JSON Feed links from `<link rel="alternate" type="application/rss+xml">`
7. **JSON-LD** — Raw structured data from `<script type="application/ld+json">` tags, preserved as `serde_json::Value`
8. **Hreflang** — `<link rel="alternate" hreflang="...">` entries as `HreflangEntry` structs
9. **Favicons** — `<link rel="icon">` and `<link rel="apple-touch-icon">` as `FaviconInfo` structs
10. **Headings** — H1-H6 extraction as `HeadingInfo` structs with level and text
11. **Word count** — Computed from visible text content

### Shared Extraction Pipeline

Both `scrape()` and `crawl()` use a shared `extract_page_data()` function that extracts all structured data in a single pass over the parsed HTML document:

```rust
pub(crate) fn extract_page_data(
    doc: &Html,
    body: &str,
    base_url: &Url,
    is_html: bool,
    include_extended: bool,  // hreflangs, favicons, headings, word count
) -> HtmlExtraction
```

The `include_extended` flag controls whether expensive extended metadata (hreflangs, favicons, headings, word count) is extracted — enabled for `scrape()`, disabled for `crawl()` to reduce per-page overhead.

### Link Classification

Every link extracted from the page is classified into four types:

- **Internal** — Same registered domain
- **External** — Different domain
- **Anchor** — Fragment-only (`#section`)
- **Document** — PDF, DOCX, XLSX, etc. (detected by URL extension)

Links also track: `text`, `rel` attribute, `nofollow` flag, and `is_protocol_relative`.

### Image Source Tracking

Images are extracted from multiple HTML sources, each tagged with origin via `ImageSource`:

- `Img` — `<img>` tags (src attribute)
- `PictureSource` — `<picture>` / `<source>` elements
- `OgImage` — `<meta property="og:image">`
- `TwitterImage` — `<meta name="twitter:image">`

### HTML Parser Choice

**scraper** (html5ever-based) — Full HTML5 spec-compliant parser with CSS selector support via the `Selector` type. Handles malformed HTML gracefully. Selectors are lazily compiled once via `LazyLock` for performance.

Note: `scraper::Html` is `!Send`, so all document usage is scoped in blocks that complete before any `.await` point to maintain `Send` futures.

### Asset Downloading

When `download_assets` is enabled, the scraper discovers CSS, JS, and image assets from the parsed HTML and downloads them concurrently with a configurable semaphore. Each asset is hashed (SHA-256) for deduplication.

## Consequences

### Positive

- **Maximum data capture**: No re-crawling needed when consumers want new metadata fields
- **Structured output**: Typed fields for common metadata (not just a flat key-value bag)
- **DRY extraction**: Single `extract_page_data` function used by both scrape and crawl paths
- **Classification enables filtering**: Consumers can filter links by type, images by source
- **JSON-LD preserved raw**: No lossy transformation of structured data — consumer decides interpretation
- **Lazy selectors**: CSS selectors compiled once, reused across all pages

### Negative

- **Large output per page**: `PageMetadata` struct with 40+ fields, many `Option<String>`. For crawls of 10K+ pages, output size may be significant
- **Extraction overhead**: Parsing all meta tags adds ~1-2ms per page vs just fetching HTML. Negligible compared to network latency
- **Dublin Core is niche**: Most modern websites don't use Dublin Core. Including it adds type complexity for limited benefit — but extraction cost is near zero
- **`!Send` constraint**: scraper's `Html` type requires careful scoping to avoid blocking async executors

## Notes

Implementation (html/ submodule structure):
- `crates/kreuzcrawl/src/html/extract.rs` — Shared `extract_page_data` pipeline
- `crates/kreuzcrawl/src/html/metadata.rs` — Meta tag extraction, noindex/nofollow detection, meta refresh
- `crates/kreuzcrawl/src/html/links.rs` — Link extraction and classification
- `crates/kreuzcrawl/src/html/images.rs` — Image extraction from multiple sources
- `crates/kreuzcrawl/src/html/feeds.rs` — Feeds, favicons, hreflangs, headings
- `crates/kreuzcrawl/src/html/json_ld.rs` — JSON-LD structured data extraction
- `crates/kreuzcrawl/src/html/content.rs` — Main content extraction, tag removal, word count
- `crates/kreuzcrawl/src/html/charset.rs` — Charset detection from headers and meta tags
- `crates/kreuzcrawl/src/html/detection.rs` — Content type detection (HTML, PDF, binary)
- `crates/kreuzcrawl/src/html/selectors.rs` — Lazy-compiled CSS selectors and regex patterns
- `crates/kreuzcrawl/src/assets.rs` — Asset discovery and concurrent downloading
- `crates/kreuzcrawl/src/types.rs` — PageMetadata, LinkInfo, ImageInfo, FeedInfo, etc.
