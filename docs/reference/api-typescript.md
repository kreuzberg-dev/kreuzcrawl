# TypeScript API Reference

The `kreuzcrawl` npm package provides native bindings to the Rust crawling engine via NAPI-RS.

## Installation

```bash
pnpm add kreuzcrawl
```

## Quick Start

```typescript
import { createEngine } from "kreuzcrawl";

const engine = createEngine();
// Engine is ready for use with scrape/crawl/map operations
```

## Functions

### createEngine

```typescript
function createEngine(config?: JsCrawlConfig): JsCrawlEngineHandle;
```

Create a new crawl engine with optional configuration. Uses default configuration if `config` is omitted.

**Throws:** `Error` if the configuration is invalid.

## Configuration

```typescript
import { createEngine } from "kreuzcrawl";

const engine = createEngine({
  maxDepth: 3,
  maxPages: 100,
  maxConcurrent: 10,
  stayOnDomain: true,
  respectRobotsTxt: true,
  requestTimeout: 30, // seconds
  userAgent: "MyBot/1.0",
});
```

### JsCrawlConfig

All fields are optional. Uses `camelCase` naming per JavaScript conventions.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `maxDepth` | `number` | -- | Maximum crawl depth |
| `maxPages` | `number` | -- | Maximum pages to crawl |
| `maxConcurrent` | `number` | -- | Maximum concurrent requests |
| `respectRobotsTxt` | `boolean` | `false` | Respect robots.txt |
| `userAgent` | `string` | -- | Custom user agent |
| `stayOnDomain` | `boolean` | `false` | Stay on same domain |
| `allowSubdomains` | `boolean` | `false` | Allow subdomains |
| `includePaths` | `string[]` | `[]` | Path include patterns (regex) |
| `excludePaths` | `string[]` | `[]` | Path exclude patterns (regex) |
| `customHeaders` | `Record<string, string>` | `{}` | Custom HTTP headers |
| `requestTimeout` | `number` | -- | Request timeout (seconds) |
| `maxRedirects` | `number` | `10` | Maximum redirects |
| `retryCount` | `number` | `0` | Retry attempts |
| `mainContentOnly` | `boolean` | `false` | Extract main content only |
| `removeTags` | `string[]` | `[]` | CSS selectors to remove |
| `mapLimit` | `number` | -- | Map URL limit |
| `mapSearch` | `string` | -- | Map search filter |
| `downloadAssets` | `boolean` | `false` | Download page assets |
| `assetTypes` | `JsAssetCategory[]` | `[]` | Asset type filter |
| `browser` | `JsBrowserConfig` | -- | Browser configuration |
| `proxy` | `JsProxyConfig` | -- | Proxy configuration |
| `userAgents` | `string[]` | `[]` | UA rotation list |
| `captureScreenshot` | `boolean` | `false` | Capture screenshot |
| `downloadDocuments` | `boolean` | `true` | Download non-HTML docs |

### JsBrowserConfig

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `JsBrowserMode` | `"Auto"` | Browser mode: `"Auto"`, `"Always"`, `"Never"` |
| `endpoint` | `string` | -- | CDP WebSocket endpoint |
| `timeout` | `number` | -- | Timeout in seconds |
| `wait` | `JsBrowserWait` | `"NetworkIdle"` | Wait strategy: `"NetworkIdle"`, `"Selector"`, `"Fixed"` |
| `waitSelector` | `string` | -- | CSS selector to wait for |
| `extraWait` | `number` | -- | Extra wait in seconds |

### JsProxyConfig

| Field | Type | Description |
|-------|------|-------------|
| `url` | `string` | Proxy URL |
| `username` | `string` | Proxy username |
| `password` | `string` | Proxy password |

## Types

Key result types (all use `camelCase` field naming):

| TypeScript Type | Rust Type | Description |
|----------------|-----------|-------------|
| `JsCrawlEngineHandle` | `CrawlEngineHandle` | Opaque engine handle |
| `JsCrawlConfig` | `CrawlConfig` | Configuration |
| `JsScrapeResult` | `ScrapeResult` | Single-page scrape result |
| `JsCrawlResult` | `CrawlResult` | Multi-page crawl result |
| `JsCrawlPageResult` | `CrawlPageResult` | Single page in a crawl |
| `JsMapResult` | `MapResult` | URL discovery result |
| `JsMarkdownResult` | `MarkdownResult` | Markdown conversion |
| `JsPageMetadata` | `PageMetadata` | Page metadata |
| `JsLinkInfo` | `LinkInfo` | Link information |
| `JsImageInfo` | `ImageInfo` | Image information |
| `JsBatchScrapeResult` | `BatchScrapeResult` | Batch scrape item |
| `JsBatchCrawlResult` | `BatchCrawlResult` | Batch crawl item |

## Error Handling

Errors are thrown as JavaScript `Error` objects with a descriptive message. The message includes a prefix tag indicating the error variant:

```typescript
try {
  const engine = createEngine({ maxConcurrent: 0 });
} catch (error) {
  // Error: invalid_config: max_concurrent must be > 0
  console.error(error.message);
}
```

Error variant tags in messages: `[NotFound]`, `[Unauthorized]`, `[Forbidden]`, `[Timeout]`, `[RateLimited]`, `[Connection]`, `[Dns]`, `[Ssl]`, etc.

## Status

The TypeScript bindings currently expose `createEngine()` for engine construction. The async operation functions (`scrape`, `crawl`, `mapUrls`, `batchScrape`, `batchCrawl`) are being generated. Check the package changelog for updates on async function availability.
