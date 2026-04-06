# ADR-006: Fixture-Driven E2E Testing Strategy

**Status**: Accepted (updated 2026-03-10)

**Date**: 2026-03-09

## Context

kreuzcrawl needs a comprehensive test suite before implementing engine code (TDD approach). We need to validate scraping, crawling, URL mapping, metadata extraction, robots.txt compliance, sitemap parsing, and error handling — all without making real network calls.

kreuzberg and spikard both use fixture-driven E2E test generation: JSON fixtures as single source of truth, with a Rust CLI tool generating language-native test suites. This pattern scales across language bindings and ensures test parity.

## Decision

- **JSON fixtures** in `fixtures/` define crawl scenarios with mock HTTP responses, configuration, and structured assertions
- **Response body files** (HTML, XML, robots.txt, PDF, assets) stored separately in `fixtures/responses/` to keep fixture JSON clean
- **wiremock** serves mock HTTP responses — no real network calls in E2E tests
- **Rust CLI generator** (`tools/e2e-generator/`) reads fixtures and generates complete test projects to `e2e/rust/`
- **Category-based organization** with 15 categories: scrape, metadata, links, crawl, robots, sitemap, error, redirect, content, cookies, auth, map, batch, stream, encoding
- **Generated code policy**: All files in `e2e/` are generated, gitignored, and include DO NOT EDIT headers. Modify fixtures or generator source to change tests.
- **Post-generation quality**: Generated code is run through `cargo fmt` and must pass `cargo clippy -D warnings`

### Category-to-API Routing

Fixtures are routed to the appropriate API based on category:

| API | Categories |
|-----|-----------|
| `kreuzcrawl::scrape()` | scrape, metadata, links, robots, content, auth, error, encoding |
| `kreuzcrawl::crawl()` | crawl, cookies, redirect |
| `kreuzcrawl::map()` | sitemap, map |
| `kreuzcrawl::batch_scrape()` | batch |
| `kreuzcrawl::crawl_stream()` | stream |

Fixtures can override their category with an explicit `"category"` field in the JSON when a fixture lives in one directory but should route to a different API.

### Assertion Types (30)

The schema supports 30 structured assertion types:

| Type | Validates |
|------|-----------|
| status_code, content_type, html_not_empty | Basic response properties |
| metadata | Title, description, canonical URL |
| links | Count bounds, type presence, URL matching |
| images | Count validation |
| og, twitter, dublin_core | Social/academic metadata |
| json_ld | Structured data type and name |
| feeds | RSS/Atom/JSON Feed counts |
| robots | Allow/disallow, crawl delay, noindex/nofollow |
| sitemap | URL count, lastmod presence |
| crawl | Pages crawled, domain scope, normalized URLs |
| error | Error type, WAF detection |
| redirect | Final URL, redirect count |
| content | Charset, body size, PDF, binary skip |
| cookies | Count, name presence |
| auth | Header sent, status code |
| map | URL count bounds, URL matching |
| extended_metadata | Keywords, author, viewport, generator, lang, dir |
| article | Published/modified time, author, section, tags |
| extended_og | Video, audio, locale alternates |
| hreflang | Count, language presence |
| favicons | Count, apple-touch-icon presence |
| headings | H1 count/text, total heading count |
| computed | Word count bounds |
| response_meta | ETag, Last-Modified, Server, Content-Language |
| assets | Count, category, unique hash count |
| stream | Event count, page/complete event presence |
| batch | Completed/failed/total count, URL result match |

### Fixture Schema

Each fixture JSON contains:

- `id` — Unique identifier (must match filename)
- `description` — Human-readable purpose
- `tags` — Filtering and grouping labels
- `category` — Optional override (inferred from directory by default)
- `mock_response` — Single-route HTTP response (status, headers, body)
- `mock_responses` — Multi-route responses for crawl scenarios (array)
- `config` — CrawlConfig overrides (depth, auth, headers, etc.)
- `assertions` — Structured expectations (30 assertion types)
- `skip` — Conditional skip directives

The full schema is defined in `fixtures/schema.json` (JSON Schema draft-07).

### Taskfile Integration

```text
task e2e:generate    # Generate + format + lint
task e2e:format      # cargo fmt on generated code
task e2e:lint        # cargo clippy on generated code
task e2e:test        # Run generated tests
task e2e:list        # List all fixtures
task e2e:verify      # Full pipeline + git diff check
task e2e:verify:ci   # CI: generate + fmt --check + clippy + diff
```

## Consequences

### Positive

- Test corpus exists before engine code — drives implementation via TDD
- Single source of truth (fixtures) prevents test divergence across future language bindings
- Mock-based tests are fast, deterministic, and CI-friendly
- Fixture format is extensible for new assertion types and crawl scenarios
- Response body files are reusable across multiple fixtures
- Generated code quality enforced by fmt + clippy
- Generator validates fixtures at load time (auth exclusivity, response source, ID matching)

### Negative

- Generator must be maintained alongside fixture schema evolution
- Mock responses may not capture all real-world HTTP edge cases
- Additional build step (generate -> compile -> test) adds complexity

## Notes

- Pattern derived from kreuzberg's `tools/e2e-generator/` and spikard's `tools/test-generator/`
- Future: add language generators (Python, TypeScript) when bindings are added
- CI verifies generated code is up-to-date: `task e2e:verify:ci`
