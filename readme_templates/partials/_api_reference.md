Full API documentation is available at [docs.kreuzcrawl.dev](https://docs.kreuzcrawl.dev).

Key functions:
- `create_engine(config?)` — Create a crawl engine with optional configuration
- `scrape(engine, url)` — Scrape a single URL
- `crawl(engine, url)` — Crawl a website following links
- `map_urls(engine, url)` — Discover all pages on a site
- `batch_scrape(engine, urls)` — Scrape multiple URLs concurrently
- `batch_crawl(engine, urls)` — Crawl multiple seed URLs concurrently
