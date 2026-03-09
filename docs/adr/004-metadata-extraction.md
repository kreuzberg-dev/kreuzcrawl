# ADR-004: Metadata Extraction — Exhaustive HTML Metadata Model

**Status**: Accepted

**Date**: 2026-03-09

## Context

Crawled web pages contain rich metadata beyond the visible text: Open Graph tags for social sharing, Twitter Card markup, Dublin Core for academic/institutional content, JSON-LD for structured data, RSS/Atom feed references, and dozens of other meta tags. Different consumers of crawl data need different subsets of this metadata.

Firecrawl and similar tools extract a curated subset. We want to extract *everything* — let the consumer decide what's relevant. This maximizes the value of each crawl operation and avoids re-crawling when a new metadata field becomes interesting.

## Decision

### Exhaustive Extraction Strategy

Extract all discoverable metadata from every crawled page into a structured `PageMetadata` type. Categories:

1. **Core HTML metadata** — title, description, language, charset, keywords, robots, canonical URL, favicon
2. **Open Graph** — og:title, og:description, og:url, og:image, og:audio, og:video, og:type, og:site_name, og:locale (+ alternates), og:determiner
3. **Twitter Card** — twitter:card, twitter:site, twitter:creator, twitter:title, twitter:description, twitter:image
4. **Article metadata** — article:published_time, article:modified_time, article:section, article:tag, article:author
5. **Dublin Core** — dc.title, dc.creator, dc.subject, dc.description, dc.date, dc.type, dc.format, dc.language, dc.rights, dcterms.created, dcterms.modified, dcterms.audience, dcterms.keywords
6. **Feed discovery** — RSS, Atom, JSON Feed links from `<link rel="alternate" type="application/rss+xml">`
7. **JSON-LD** — Raw structured data from `<script type="application/ld+json">` tags, preserved as `serde_json::Value`
8. **Custom meta** — All remaining `<meta>` tags captured in `HashMap<String, String>`

### Link Classification

Every link extracted from the page is classified:

- **Internal** — Same registered domain
- **External** — Different domain
- **Anchor** — Fragment-only (`#section`)
- **Document** — PDF, DOCX, XLSX, etc. (detected by URL extension)
- **Image** — Image file links
- **Media** — Video/audio file links
- **Feed** — RSS/Atom/JSON Feed URLs
- **Stylesheet** — CSS references
- **Script** — JavaScript references
- **Other** — Unclassified

### Image Source Tracking

Images are extracted from multiple HTML sources, each tagged with origin:

- `<img>` tags (src, srcset)
- `<picture>` / `<source>` elements
- `<meta property="og:image">`
- `<meta name="twitter:image">`
- `<link rel="icon">` / favicon
- CSS `background-image` (from inline styles only — no stylesheet parsing in v0.1)

### HTML Parser Choice

**tl (tree of links)** — Zero-copy HTML parser that's ~10x faster than scraper/html5ever for our use case (metadata extraction from well-structured `<head>` and link enumeration). Falls back gracefully on malformed HTML.

## Consequences

### Positive

- **Maximum data capture**: No re-crawling needed when consumers want new metadata fields
- **Structured output**: Typed fields for common metadata (not just a flat key-value bag)
- **Extensible**: `custom_meta: HashMap<String, String>` catches anything not in named fields
- **Classification enables filtering**: Consumers can filter links by type, images by source
- **JSON-LD preserved raw**: No lossy transformation of structured data — consumer decides interpretation

### Negative

- **Large output per page**: `PageMetadata` struct with 40+ fields, many `Option<String>`. For crawls of 10K+ pages, output size may be significant
- **Extraction overhead**: Parsing all meta tags adds ~1-2ms per page vs just fetching HTML. Negligible compared to network latency
- **Dublin Core is niche**: Most modern websites don't use Dublin Core. Including it adds type complexity for limited benefit — but extraction cost is near zero
- **CSS background-image limitations**: Only inline styles parsed in v0.1. External stylesheet `background-image` requires CSS parsing, deferred

## Notes

Implementation:
- `crates/kreuzcrawl/src/extract/mod.rs` — Extraction orchestration
- `crates/kreuzcrawl/src/extract/metadata.rs` — HTML meta tag extraction
- `crates/kreuzcrawl/src/extract/links.rs` — Link extraction + classification
- `crates/kreuzcrawl/src/extract/images.rs` — Image extraction from all sources
- `crates/kreuzcrawl/src/extract/json_ld.rs` — JSON-LD structured data extraction
- `crates/kreuzcrawl/src/types.rs` — PageMetadata, PageLink, PageImage, ScrapeResult structs
