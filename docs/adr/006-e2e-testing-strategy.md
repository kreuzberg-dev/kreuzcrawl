# ADR-006: Fixture-Driven E2E Testing Strategy

**Status**: Accepted (updated 2026-03-09)

**Date**: 2026-03-09

## Context

kreuzcrawl needs a comprehensive test suite before implementing engine code (TDD approach). We need to validate scraping, crawling, URL mapping, metadata extraction, robots.txt compliance, sitemap parsing, and error handling — all without making real network calls.

kreuzberg and spikard both use fixture-driven E2E test generation: JSON fixtures as single source of truth, with a Rust CLI tool generating language-native test suites. This pattern scales across language bindings and ensures test parity.

## Decision

- **JSON fixtures** in `fixtures/` define crawl scenarios with mock HTTP responses, configuration, and structured assertions
- **Response body files** (HTML, XML, robots.txt, PDF) stored separately in `fixtures/responses/` to keep fixture JSON clean
- **wiremock** serves mock HTTP responses — no real network calls in E2E tests
- **Rust CLI generator** (`tools/e2e-generator/`) reads fixtures and generates complete test projects to `e2e/rust/`
- **Category-based organization** with 12 categories: scrape, metadata, links, crawl, robots, sitemap, error, redirect, content, cookies, auth, map
- **Generated code policy**: All files in `e2e/` are generated, gitignored, and include DO NOT EDIT headers. Modify fixtures or generator source to change tests.
- **Post-generation quality**: Generated code is run through `cargo fmt` and must pass `cargo clippy -D warnings`
- **TDD red phase**: Generated tests compile but fail initially — engine implementation makes them pass

### Category-to-API Routing

Fixtures are routed to the appropriate API based on category:

| API | Categories |
|-----|-----------|
| `kreuzcrawl::scrape()` | scrape, metadata, links, robots, content, auth, error |
| `kreuzcrawl::crawl()` | crawl, cookies, redirect |
| `kreuzcrawl::map()` | sitemap, map |

Fixtures can override their category with an explicit `"category"` field in the JSON when a fixture lives in one directory but should route to a different API.

### Fixture Schema

Each fixture JSON contains:

- `id` — Unique identifier (must match filename)
- `description` — Human-readable purpose
- `tags` — Filtering and grouping labels
- `category` — Optional override (inferred from directory by default)
- `mock_response` — Single-route HTTP response (status, headers, body)
- `mock_responses` — Multi-route responses for crawl scenarios (array)
- `config` — CrawlConfig overrides (depth, auth, headers, etc.)
- `assertions` — Structured expectations (status, metadata, links, crawl results, etc.)
- `skip` — Conditional skip directives

The full schema is defined in `fixtures/schema.json` (JSON Schema draft-07).

### Taskfile Integration

```
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

### Negative

- Generator must be maintained alongside fixture schema evolution
- Mock responses may not capture all real-world HTTP edge cases
- Additional build step (generate -> compile -> test) adds complexity

## Notes

- Pattern derived from kreuzberg's `tools/e2e-generator/` and spikard's `tools/test-generator/`
- Future: add language generators (Python, TypeScript) when bindings are added
- CI verifies generated code is up-to-date: `task e2e:verify:ci`
