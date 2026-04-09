# Configuration Reference

All operations in kreuzcrawl are controlled by `CrawlConfig`. Configuration can be provided programmatically, via the CLI, or through the REST API and MCP tool parameters.

## CrawlConfig

The main configuration struct. All fields have sensible defaults. Serialized with `serde` (JSON-compatible).

### Crawl Control

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_depth` | `Option<usize>` | `None` (unlimited) | Maximum crawl depth (number of link hops from the start URL) |
| `max_pages` | `Option<usize>` | `None` (unlimited) | Maximum number of pages to crawl. Must be > 0 if set. |
| `max_concurrent` | `Option<usize>` | `None` (engine default) | Maximum number of concurrent requests. Must be > 0 if set. |
| `stay_on_domain` | `bool` | `false` | Restrict crawling to the same domain as the start URL |
| `allow_subdomains` | `bool` | `false` | Allow subdomains when `stay_on_domain` is `true` |
| `include_paths` | `Vec<String>` | `[]` | Regex patterns for paths to include during crawling |
| `exclude_paths` | `Vec<String>` | `[]` | Regex patterns for paths to exclude during crawling |

### HTTP Settings

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `request_timeout` | `Duration` | 30 seconds | Timeout for individual HTTP requests. Serialized as milliseconds in JSON. Must be > 0. |
| `max_redirects` | `usize` | `10` | Maximum number of redirects to follow. Must be <= 100. |
| `retry_count` | `usize` | `0` | Number of retry attempts for failed requests |
| `retry_codes` | `Vec<u16>` | `[]` | HTTP status codes that trigger a retry (must be 100--599) |
| `max_body_size` | `Option<usize>` | `None` (unlimited) | Maximum response body size in bytes |
| `custom_headers` | `HashMap<String, String>` | `{}` | Custom HTTP headers sent with each request |
| `cookies_enabled` | `bool` | `false` | Whether to enable cookie handling |
| `user_agent` | `Option<String>` | `None` (default UA) | Custom user-agent string |
| `user_agents` | `Vec<String>` | `[]` | User-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `respect_robots_txt` | `bool` | `false` | Whether to respect robots.txt directives |

### Content Extraction

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `main_content_only` | `bool` | `false` | Extract only the main content from HTML pages (removes nav, footer, etc.) |
| `remove_tags` | `Vec<String>` | `[]` | CSS selectors for tags to remove from HTML before processing |

### Map Operation

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `map_limit` | `Option<usize>` | `None` (unlimited) | Maximum number of URLs to return from a map operation |
| `map_search` | `Option<String>` | `None` | Case-insensitive substring filter for map results |

### Asset Downloads

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `download_assets` | `bool` | `false` | Download assets (CSS, JS, images, etc.) from the page |
| `asset_types` | `Vec<AssetCategory>` | `[]` (all types) | Filter for asset categories to download |
| `max_asset_size` | `Option<usize>` | `None` (unlimited) | Maximum size in bytes for individual asset downloads |

### Document Downloads

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `download_documents` | `bool` | `true` | Download non-HTML documents (PDF, DOCX, images, code files) instead of skipping them |
| `document_max_size` | `Option<usize>` | 50 MB | Maximum size in bytes for document downloads |
| `document_mime_types` | `Vec<String>` | `[]` (built-in defaults) | Allowlist of MIME types to download. Empty uses defaults. |

### Browser Configuration

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `browser` | `BrowserConfig` | See below | Browser fallback configuration |
| `capture_screenshot` | `bool` | `false` | Capture a screenshot when using the browser |
| `browser_profile` | `Option<String>` | `None` | Named browser profile for persistent sessions (cookies, localStorage) |
| `save_browser_profile` | `bool` | `false` | Save changes back to the browser profile on exit |

### Network

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `proxy` | `Option<ProxyConfig>` | `None` | Proxy configuration for HTTP requests |
| `auth` | `Option<AuthConfig>` | `None` | Authentication configuration |

### Output

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `warc_output` | `Option<PathBuf>` | `None` | Path to write WARC output. `None` disables WARC. |

---

## BrowserConfig

Controls the headless browser fallback for JavaScript-rendered pages.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `BrowserMode` | `Auto` | When to use the headless browser |
| `endpoint` | `Option<String>` | `None` | CDP WebSocket endpoint for an external browser instance |
| `timeout` | `Duration` | 30 seconds | Timeout for browser page load and rendering (serialized as ms) |
| `wait` | `BrowserWait` | `NetworkIdle` | Wait strategy after browser navigation |
| `wait_selector` | `Option<String>` | `None` | CSS selector to wait for (required when `wait` is `Selector`) |
| `extra_wait` | `Option<Duration>` | `None` | Extra time to wait after the wait condition is met (serialized as ms) |

### BrowserMode

| Value | Description |
|-------|-------------|
| `auto` | Automatically detect when JS rendering is needed and fall back to browser |
| `always` | Always use the browser for every request |
| `never` | Never use the browser fallback |

### BrowserWait

| Value | Description |
|-------|-------------|
| `network_idle` | Wait until network activity is idle |
| `selector` | Wait for a specific CSS selector to appear in the DOM |
| `fixed` | Wait for a fixed duration after navigation |

---

## ProxyConfig

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | `string` | Yes | Proxy URL (e.g. `"http://proxy:8080"`, `"socks5://proxy:1080"`) |
| `username` | `Option<String>` | No | Username for proxy authentication |
| `password` | `Option<String>` | No | Password for proxy authentication |

---

## AuthConfig

Tagged union (discriminated by `type` field in JSON).

### Basic Authentication

```json
{
  "type": "basic",
  "username": "user",
  "password": "pass"
}
```

### Bearer Token

```json
{
  "type": "bearer",
  "token": "eyJhbGci..."
}
```

### Custom Header

```json
{
  "type": "header",
  "name": "X-API-Key",
  "value": "my-api-key"
}
```

---

## Validation Rules

`CrawlConfig::validate()` enforces these constraints:

| Rule | Error |
|------|-------|
| `max_concurrent` must be > 0 (if set) | `InvalidConfig` |
| `max_pages` must be > 0 (if set) | `InvalidConfig` |
| `max_redirects` must be <= 100 | `InvalidConfig` |
| `request_timeout` must be > 0 | `InvalidConfig` |
| `retry_codes` must be valid HTTP status codes (100--599) | `InvalidConfig` |
| `include_paths` entries must be valid regex | `InvalidConfig` |
| `exclude_paths` entries must be valid regex | `InvalidConfig` |
| `browser.wait_selector` required when `browser.wait` is `Selector` | `InvalidConfig` |

---

## JSON Example

```json
{
  "max_depth": 3,
  "max_pages": 100,
  "max_concurrent": 10,
  "respect_robots_txt": true,
  "stay_on_domain": true,
  "allow_subdomains": false,
  "include_paths": ["/docs/.*"],
  "exclude_paths": ["/blog/.*"],
  "request_timeout": 30000,
  "max_redirects": 10,
  "retry_count": 2,
  "retry_codes": [429, 502, 503],
  "main_content_only": true,
  "remove_tags": ["nav", "footer", ".ads"],
  "download_documents": true,
  "document_max_size": 52428800,
  "browser": {
    "mode": "auto",
    "timeout": 30000,
    "wait": "network_idle"
  },
  "proxy": {
    "url": "http://proxy:8080",
    "username": "user",
    "password": "pass"
  },
  "auth": {
    "type": "bearer",
    "token": "eyJhbGci..."
  }
}
```
