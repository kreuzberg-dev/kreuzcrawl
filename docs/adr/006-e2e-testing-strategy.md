# ADR-006: Fixture-Driven E2E Testing Strategy

**Status**: Accepted

**Date**: 2026-03-09

## Context

kreuzcrawl needs a comprehensive test suite before implementing engine code (TDD approach). We need to validate scraping, crawling, URL mapping, metadata extraction, robots.txt compliance, sitemap parsing, and error handling — all without making real network calls.

kreuzberg and spikard both use fixture-driven E2E test generation: JSON fixtures as single source of truth, with a Rust CLI tool generating language-native test suites. This pattern scales across language bindings and ensures test parity.

## Decision

- **JSON fixtures** in `fixtures/` define crawl scenarios with mock HTTP responses, configuration, and structured assertions
- **Response body files** (HTML, XML, robots.txt) stored separately in `fixtures/responses/` to keep fixture JSON clean
- **wiremock** serves mock HTTP responses — no real network calls in E2E tests
- **Rust CLI generator** (`tools/e2e-generator/`) reads fixtures and generates complete test projects to `e2e/rust/`
- **Category-based organization**: scrape, crawl, map, metadata, links, images, robots, sitemap, rate_limit, engine, error
- **Generated code policy**: All files in `e2e/` are generated and include DO NOT EDIT headers. Modify fixtures or generator source to change tests.
- **TDD red phase**: Generated tests compile but fail initially — engine implementation makes them pass

## Consequences

### Positive

- Test corpus exists before engine code — drives implementation via TDD
- Single source of truth (fixtures) prevents test divergence across future language bindings
- Mock-based tests are fast, deterministic, and CI-friendly
- Fixture format is extensible for new assertion types and crawl scenarios
- Response body files are reusable across multiple fixtures

### Negative

- Generator must be maintained alongside fixture schema evolution
- Mock responses may not capture all real-world HTTP edge cases
- Additional build step (generate -> compile -> test) adds complexity

## Notes

- Pattern derived from kreuzberg's `tools/e2e-generator/` and spikard's `tools/test-generator/`
- Future: add language generators (Python, TypeScript) when bindings are added
- CI will verify generated code is up-to-date: `task e2e:generate && git diff --exit-code e2e/`
