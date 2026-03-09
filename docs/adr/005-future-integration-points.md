# ADR-005: Future Integration Points — kreuzberg, html-to-markdown, tree-sitter-language-pack

**Status**: Accepted

**Date**: 2026-03-09

## Context

kreuzcrawl is designed as a standalone crawling engine, but it exists within the kreuzberg.dev ecosystem. Three integration points are planned for future phases:

1. **html-to-markdown** — Convert crawled HTML to clean Markdown for LLM consumption
2. **kreuzberg** — Extract text and metadata from non-HTML documents (PDF, DOCX, etc.) encountered during crawling
3. **tree-sitter-language-pack** — Parse source code files when crawling git repositories

These integrations should not be required — kreuzcrawl must remain useful as a pure crawling engine without any of them.

## Decision

### Feature-Gated Optional Dependencies

Each integration is behind a Cargo feature flag:

```toml
[features]
default = []
markdown = ["dep:html-to-markdown"]
extraction = ["dep:kreuzberg"]
code-parsing = ["dep:ts-pack-core"]
full = ["markdown", "extraction", "code-parsing"]
```

### html-to-markdown Integration (feature: `markdown`)

- Adds `OutputFormat::Markdown` to `ScrapeResult`
- Uses the `html-to-markdown` crate (kreuzberg-dev org) for conversion
- Applied as a post-processing step after HTML fetch + metadata extraction
- HTML is always captured regardless — markdown is an additional derived field

### kreuzberg Integration (feature: `extraction`)

- When crawling encounters non-HTML documents (PDF, DOCX, images), delegates to kreuzberg for extraction
- Adds `ExtractionResult` field to `ScrapeResult` for document types
- Enables following document links during crawl (e.g., PDF links on a page)
- Uses kreuzberg's `ExtractionConfig` for document processing settings

### tree-sitter-language-pack Integration (feature: `code-parsing`)

- Enables `CrawlMode::GitRepo` and `CrawlMode::LocalDir`
- Uses `ts-pack-core` to parse source code files with tree-sitter grammars
- Extracts structural information: functions, classes, imports, symbols
- Produces `ExtractedCode` results with language-specific AST data
- Supports 170+ programming languages via ts-pack-core's registry

### Git Repository Processing

When `code-parsing` feature is enabled, kreuzcrawl gains a new crawl mode:

1. Clone repository (via `gix` crate) to temp directory
2. Walk file tree, filtering by extension
3. For each source file: detect language, parse with tree-sitter, extract symbols
4. Produce `RepositoryContent` with per-file `ExtractedCode` results
5. Optionally process non-code files (README, docs) via kreuzberg if `extraction` feature is also enabled

This creates a bridge between kreuzcrawl and kreuzberg: crawl repos → parse code with tree-sitter → extract docs with kreuzberg → unified output.

## Consequences

### Positive

- **Zero-cost when unused**: Feature flags mean no compile-time or runtime cost for disabled integrations
- **Standalone viable**: kreuzcrawl works as a pure HTTP crawler without any kreuzberg ecosystem dependencies
- **Incremental adoption**: Users can enable exactly the integrations they need
- **Ecosystem synergy**: Reuses existing kreuzberg-dev crates rather than reimplementing

### Negative

- **Feature matrix testing**: Each feature combination needs CI coverage (2^3 = 8 combinations, though `full` covers the important one)
- **API surface complexity**: Optional fields on result types add `Option<>` wrapping
- **Version coordination**: Feature-gated dependencies must track compatible versions of kreuzberg, html-to-markdown, ts-pack-core

## Notes

Implementation (future — not part of v0.1):
- Feature flags defined in `crates/kreuzcrawl/Cargo.toml`
- Conditional compilation in relevant modules via `#[cfg(feature = "...")]`
- Integration tests per feature in `tests/` directory
