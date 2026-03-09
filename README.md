# kreuzcrawl

A Rust crawling engine for turning websites into structured data.

**Status**: Under development

## Features (planned)

- **Scrape** — Fetch a single URL with exhaustive metadata extraction
- **Crawl** — Follow links with BFS traversal, rate limiting, robots.txt compliance
- **Map** — Discover all URLs on a site via sitemap + link crawling
- Anti-bot bypass via TLS fingerprint spoofing (reqwest-impersonate)
- Exhaustive metadata: Open Graph, Twitter Card, Dublin Core, JSON-LD, feeds, images
- Future: html-to-markdown conversion, kreuzberg document extraction, git repo processing

## License

MIT License — see [LICENSE](LICENSE).
