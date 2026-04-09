# Python API Reference

The `kreuzcrawl` Python package provides native bindings to the Rust crawling engine via PyO3. All async functions return awaitables.

## Installation

```bash
pip install kreuzcrawl
```

## Quick Start

```python
import asyncio
import kreuzcrawl

async def main():
    engine = kreuzcrawl.create_engine()
    result = await kreuzcrawl.scrape(engine, "https://example.com")
    print(result.metadata.title)

asyncio.run(main())
```

## Functions

### create_engine

```python
def create_engine(config: CrawlConfig | None = None) -> CrawlEngineHandle
```

Create a new crawl engine with optional configuration. Uses default configuration if `config` is `None`.

**Raises:** `RuntimeError` if the configuration is invalid.

### scrape

```python
async def scrape(engine: CrawlEngineHandle, url: str) -> ScrapeResult
```

Scrape a single URL and return extracted page data.

**Raises:** `RuntimeError` on network or extraction errors.

### crawl

```python
async def crawl(engine: CrawlEngineHandle, url: str) -> CrawlResult
```

Crawl a website starting from `url`, following links up to the configured depth.

**Raises:** `RuntimeError` on crawl errors.

### map_urls

```python
async def map_urls(engine: CrawlEngineHandle, url: str) -> MapResult
```

Discover all pages on a website by following links and sitemaps.

**Raises:** `RuntimeError` on errors.

### batch_scrape

```python
async def batch_scrape(engine: CrawlEngineHandle, urls: list[str]) -> list[BatchScrapeResult]
```

Scrape multiple URLs concurrently. Each result contains either a `ScrapeResult` or an error message.

### batch_crawl

```python
async def batch_crawl(engine: CrawlEngineHandle, urls: list[str]) -> list[BatchCrawlResult]
```

Crawl multiple seed URLs concurrently.

## Configuration

```python
config = kreuzcrawl.CrawlConfig(
    max_depth=3,
    max_pages=100,
    max_concurrent=10,
    stay_on_domain=True,
    respect_robots_txt=True,
    request_timeout=30000,  # milliseconds
    user_agent="MyBot/1.0",
)
engine = kreuzcrawl.create_engine(config)
```

All `CrawlConfig` fields are optional keyword arguments with sensible defaults. See [Configuration Reference](configuration.md) for the full field list.

### Sub-configurations

```python
# Proxy
proxy = kreuzcrawl.ProxyConfig(
    url="http://proxy:8080",
    username="user",
    password="pass",
)

# Browser
browser = kreuzcrawl.BrowserConfig(
    mode=kreuzcrawl.BrowserMode.Auto,
    timeout=30000,
    wait=kreuzcrawl.BrowserWait.NetworkIdle,
)
```

## Types

All Rust types are exposed as frozen Python classes with read-only properties. Key types:

| Python Class | Rust Type | Description |
|-------------|-----------|-------------|
| `CrawlEngineHandle` | `CrawlEngineHandle` | Opaque engine handle |
| `CrawlConfig` | `CrawlConfig` | Configuration |
| `ScrapeResult` | `ScrapeResult` | Single-page scrape result |
| `CrawlResult` | `CrawlResult` | Multi-page crawl result |
| `CrawlPageResult` | `CrawlPageResult` | Single page in a crawl |
| `MapResult` | `MapResult` | URL discovery result |
| `MarkdownResult` | `MarkdownResult` | Markdown conversion |
| `PageMetadata` | `PageMetadata` | Page metadata |
| `LinkInfo` | `LinkInfo` | Link information |
| `ImageInfo` | `ImageInfo` | Image information |
| `BatchScrapeResult` | `BatchScrapeResult` | Batch scrape item |
| `BatchCrawlResult` | `BatchCrawlResult` | Batch crawl item |

## Error Handling

All errors from the Rust engine are raised as `RuntimeError` with a descriptive message. The error message contains the error variant prefix (e.g. `"not_found:"`, `"timeout:"`, `"dns:"`) for programmatic handling.

```python
try:
    result = await kreuzcrawl.scrape(engine, "https://example.com")
except RuntimeError as e:
    msg = str(e)
    if "not_found" in msg:
        print("Page not found")
    elif "timeout" in msg:
        print("Request timed out")
    else:
        print(f"Error: {msg}")
```

## Complete Example

```python
import asyncio
import kreuzcrawl

async def main():
    config = kreuzcrawl.CrawlConfig(
        max_depth=2,
        max_pages=50,
        stay_on_domain=True,
    )
    engine = kreuzcrawl.create_engine(config)

    # Scrape a single page
    result = await kreuzcrawl.scrape(engine, "https://example.com")
    print(f"Title: {result.metadata.title}")
    print(f"Links: {len(result.links)}")
    if result.markdown:
        print(f"Markdown length: {len(result.markdown.content)}")

    # Crawl a site
    crawl_result = await kreuzcrawl.crawl(engine, "https://example.com")
    for page in crawl_result.pages:
        print(f"  [{page.depth}] {page.url}")

    # Map a site
    map_result = await kreuzcrawl.map_urls(engine, "https://example.com")
    for entry in map_result.urls:
        print(f"  {entry.url}")

asyncio.run(main())
```
