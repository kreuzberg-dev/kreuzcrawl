---
title: "Types Reference"
---

## Types Reference

All types defined by the library, grouped by category. Types are shown using Rust as the canonical representation.

## Result Types

### InteractionResult

Result of executing a sequence of page interaction actions.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `action_results` | `Vec<ActionResult>` | — | Results from each executed action. |
| `final_html` | `String` | — | Final page HTML after all actions completed. |
| `final_url` | `String` | — | Final page URL (may have changed due to navigation). |
| `screenshot` | `Option<Vec<u8>>` | `None` | Screenshot taken after all actions, if requested. |

---

### ActionResult

Result from a single page action execution.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `action_index` | `usize` | — | Zero-based index of the action in the sequence. |
| `action_type` | `str` | — | The type of action that was executed. |
| `success` | `bool` | — | Whether the action completed successfully. |
| `data` | `Option<serde_json::Value>` | `None` | Action-specific return data (screenshot bytes, JS return value, scraped HTML). |
| `error` | `Option<String>` | `None` | Error message if the action failed. |

---

### ScrapeResult

The result of a single-page scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `status_code` | `u16` | — | The HTTP status code of the response. |
| `content_type` | `String` | — | The Content-Type header value. |
| `html` | `String` | — | The HTML body of the response. |
| `body_size` | `usize` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `Vec<LinkInfo>` | — | Links found on the page. |
| `images` | `Vec<ImageInfo>` | — | Images found on the page. |
| `feeds` | `Vec<FeedInfo>` | — | Feed links found on the page. |
| `json_ld` | `Vec<JsonLdEntry>` | — | JSON-LD entries found on the page. |
| `is_allowed` | `bool` | — | Whether the URL is allowed by robots.txt. |
| `crawl_delay` | `Option<u64>` | `None` | The crawl delay from robots.txt, in seconds. |
| `noindex_detected` | `bool` | — | Whether a noindex directive was detected. |
| `nofollow_detected` | `bool` | — | Whether a nofollow directive was detected. |
| `x_robots_tag` | `Option<String>` | `None` | The X-Robots-Tag header value, if present. |
| `is_pdf` | `bool` | — | Whether the content is a PDF. |
| `was_skipped` | `bool` | — | Whether the page was skipped (binary or PDF content). |
| `detected_charset` | `Option<String>` | `None` | The detected character set encoding. |
| `main_content_only` | `bool` | — | Whether main_content_only was active during extraction. |
| `auth_header_sent` | `bool` | — | Whether an authentication header was sent with the request. |
| `response_meta` | `Option<ResponseMeta>` | `None` | Response metadata extracted from HTTP headers. |
| `assets` | `Vec<DownloadedAsset>` | — | Downloaded assets from the page. |
| `js_render_hint` | `bool` | — | Whether the page content suggests JavaScript rendering is needed. |
| `browser_used` | `bool` | — | Whether the browser fallback was used to fetch this page. |
| `markdown` | `Option<MarkdownResult>` | `None` | Markdown conversion of the page content. |
| `extracted_data` | `Option<serde_json::Value>` | `None` | Structured data extracted by LLM. Populated when using LlmExtractor. |
| `extraction_meta` | `Option<ExtractionMeta>` | `None` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `screenshot` | `Option<Vec<u8>>` | `None` | Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled. |
| `downloaded_document` | `Option<DownloadedDocument>` | `None` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |

---

### CrawlPageResult

The result of crawling a single page during a crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The original URL of the page. |
| `normalized_url` | `String` | — | The normalized URL of the page. |
| `status_code` | `u16` | — | The HTTP status code of the response. |
| `content_type` | `String` | — | The Content-Type header value. |
| `html` | `String` | — | The HTML body of the response. |
| `body_size` | `usize` | — | The size of the response body in bytes. |
| `metadata` | `PageMetadata` | — | Extracted metadata from the page. |
| `links` | `Vec<LinkInfo>` | — | Links found on the page. |
| `images` | `Vec<ImageInfo>` | — | Images found on the page. |
| `feeds` | `Vec<FeedInfo>` | — | Feed links found on the page. |
| `json_ld` | `Vec<JsonLdEntry>` | — | JSON-LD entries found on the page. |
| `depth` | `usize` | — | The depth of this page from the start URL. |
| `stayed_on_domain` | `bool` | — | Whether this page is on the same domain as the start URL. |
| `was_skipped` | `bool` | — | Whether this page was skipped (binary or PDF content). |
| `is_pdf` | `bool` | — | Whether the content is a PDF. |
| `detected_charset` | `Option<String>` | `None` | The detected character set encoding. |
| `markdown` | `Option<MarkdownResult>` | `None` | Markdown conversion of the page content. |
| `extracted_data` | `Option<serde_json::Value>` | `None` | Structured data extracted by LLM. Populated when using LlmExtractor. |
| `extraction_meta` | `Option<ExtractionMeta>` | `None` | Metadata about the LLM extraction pass (cost, tokens, model). |
| `downloaded_document` | `Option<DownloadedDocument>` | `None` | Downloaded non-HTML document (PDF, DOCX, image, code, etc.). |

---

### CrawlResult

The result of a multi-page crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `pages` | `Vec<CrawlPageResult>` | — | The list of crawled pages. |
| `final_url` | `String` | — | The final URL after following redirects. |
| `redirect_count` | `usize` | — | The number of redirects followed. |
| `was_skipped` | `bool` | — | Whether any page was skipped during crawling. |
| `error` | `Option<String>` | `None` | An error message, if the crawl encountered an issue. |
| `cookies` | `Vec<CookieInfo>` | — | Cookies collected during the crawl. |
| `normalized_urls` | `Vec<String>` | — | Normalized URLs encountered during crawling (for deduplication counting). |

---

### MapResult

The result of a map operation, containing discovered URLs.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `urls` | `Vec<SitemapUrl>` | — | The list of discovered URLs. |

---

### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | Converted markdown text. |
| `document_structure` | `Option<serde_json::Value>` | `None` | Structured document tree with semantic nodes. |
| `tables` | `Vec<serde_json::Value>` | — | Extracted tables with structured cell data. |
| `warnings` | `Vec<String>` | — | Non-fatal processing warnings. |
| `citations` | `Option<CitationResult>` | `None` | Content with links replaced by numbered citations. |
| `fit_content` | `Option<String>` | `None` | Content-filtered markdown optimized for LLM consumption. |

---

### CitationResult

Result of citation conversion.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `content` | `String` | — | Markdown with links replaced by numbered citations. |
| `references` | `Vec<CitationReference>` | — | Numbered reference list: (index, url, text). |

---

### BatchScrapeResult

Result from a single URL in a batch scrape operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL that was scraped. |
| `result` | `Option<ScrapeResult>` | `None` | The scrape result, if successful. |
| `error` | `Option<String>` | `None` | The error message, if the scrape failed. |

---

### BatchCrawlResult

Result from a single URL in a batch crawl operation.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The seed URL that was crawled. |
| `result` | `Option<CrawlResult>` | `None` | The crawl result, if successful. |
| `error` | `Option<String>` | `None` | The error message, if the crawl failed. |

---

## Configuration Types

See [Configuration Reference](configuration.md) for detailed defaults and language-specific representations.

### ProxyConfig

Proxy configuration for HTTP requests.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Proxy URL (e.g. "<http://proxy:8080>", "socks5://proxy:1080"). |
| `username` | `Option<String>` | `None` | Optional username for proxy authentication. |
| `password` | `Option<String>` | `None` | Optional password for proxy authentication. |

---

### BrowserConfig

Browser fallback configuration.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `mode` | `BrowserMode` | — | When to use the headless browser fallback. |
| `endpoint` | `Option<String>` | `None` | CDP WebSocket endpoint for connecting to an external browser instance. |
| `timeout` | `Duration` | — | Timeout for browser page load and rendering (in milliseconds when serialized). |
| `wait` | `BrowserWait` | — | Wait strategy after browser navigation. |
| `wait_selector` | `Option<String>` | `None` | CSS selector to wait for when `wait` is `Selector`. |
| `extra_wait` | `Option<Duration>` | `None` | Extra time to wait after the wait condition is met. |

---

### CrawlConfig

Configuration for crawl, scrape, and map operations.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `max_depth` | `Option<usize>` | `None` | Maximum crawl depth (number of link hops from the start URL). |
| `max_pages` | `Option<usize>` | `None` | Maximum number of pages to crawl. |
| `max_concurrent` | `Option<usize>` | `None` | Maximum number of concurrent requests. |
| `respect_robots_txt` | `bool` | — | Whether to respect robots.txt directives. |
| `user_agent` | `Option<String>` | `None` | Custom user-agent string. |
| `stay_on_domain` | `bool` | — | Whether to restrict crawling to the same domain. |
| `allow_subdomains` | `bool` | — | Whether to allow subdomains when `stay_on_domain` is true. |
| `include_paths` | `Vec<String>` | — | Regex patterns for paths to include during crawling. |
| `exclude_paths` | `Vec<String>` | — | Regex patterns for paths to exclude during crawling. |
| `custom_headers` | `HashMap<String, String>` | — | Custom HTTP headers to send with each request. |
| `request_timeout` | `Duration` | — | Timeout for individual HTTP requests (in milliseconds when serialized). |
| `max_redirects` | `usize` | — | Maximum number of redirects to follow. |
| `retry_count` | `usize` | — | Number of retry attempts for failed requests. |
| `retry_codes` | `Vec<u16>` | — | HTTP status codes that should trigger a retry. |
| `cookies_enabled` | `bool` | — | Whether to enable cookie handling. |
| `auth` | `Option<AuthConfig>` | `None` | Authentication configuration. |
| `max_body_size` | `Option<usize>` | `None` | Maximum response body size in bytes. |
| `main_content_only` | `bool` | — | Whether to extract only the main content from HTML pages. |
| `remove_tags` | `Vec<String>` | — | CSS selectors for tags to remove from HTML before processing. |
| `map_limit` | `Option<usize>` | `None` | Maximum number of URLs to return from a map operation. |
| `map_search` | `Option<String>` | `None` | Search filter for map results (case-insensitive substring match on URLs). |
| `download_assets` | `bool` | — | Whether to download assets (CSS, JS, images, etc.) from the page. |
| `asset_types` | `Vec<AssetCategory>` | — | Filter for asset categories to download. |
| `max_asset_size` | `Option<usize>` | `None` | Maximum size in bytes for individual asset downloads. |
| `browser` | `BrowserConfig` | — | Browser configuration. |
| `proxy` | `Option<ProxyConfig>` | `None` | Proxy configuration for HTTP requests. |
| `user_agents` | `Vec<String>` | — | List of user-agent strings for rotation. If non-empty, overrides `user_agent`. |
| `capture_screenshot` | `bool` | — | Whether to capture a screenshot when using the browser. |
| `download_documents` | `bool` | — | Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them. |
| `document_max_size` | `Option<usize>` | `None` | Maximum size in bytes for document downloads. Defaults to 50 MB. |
| `document_mime_types` | `Vec<String>` | — | Allowlist of MIME types to download. If empty, uses built-in defaults. |
| `warc_output` | `Option<PathBuf>` | `None` | Path to write WARC output. If `None`, WARC output is disabled. |
| `browser_profile` | `Option<String>` | `None` | Named browser profile for persistent sessions (cookies, localStorage). |
| `save_browser_profile` | `bool` | — | Whether to save changes back to the browser profile on exit. |

---

### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

When the crawler encounters non-HTML content and `download_documents` is
enabled, it downloads the raw bytes and populates this struct instead of
skipping the resource.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL the document was fetched from. |
| `mime_type` | `str` | — | The MIME type from the Content-Type header. |
| `content` | `Vec<u8>` | — | Raw document bytes. Skipped during JSON serialization. |
| `size` | `usize` | — | Size of the document in bytes. |
| `filename` | `Option<str>` | `None` | Filename extracted from Content-Disposition or URL path. |
| `content_hash` | `str` | — | SHA-256 hex digest of the content. |
| `headers` | `HashMap<str, str>` | — | Selected response headers. |

---

### SitemapUrl

A URL entry from a sitemap.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The URL. |
| `lastmod` | `Option<String>` | `None` | The last modification date, if present. |
| `changefreq` | `Option<String>` | `None` | The change frequency, if present. |
| `priority` | `Option<String>` | `None` | The priority, if present. |

---

### CachedPage

Cached page data for HTTP response caching.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | Url |
| `status_code` | `u16` | — | Status code |
| `content_type` | `String` | — | Content type |
| `body` | `String` | — | Body |
| `etag` | `Option<String>` | `None` | Etag |
| `last_modified` | `Option<String>` | `None` | Last modified |
| `cached_at` | `u64` | — | Cached at |

---

### LinkInfo

Information about a link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The resolved URL of the link. |
| `text` | `String` | — | The visible text of the link. |
| `link_type` | `LinkType` | — | The classification of the link. |
| `rel` | `Option<String>` | `None` | The `rel` attribute value, if present. |
| `nofollow` | `bool` | — | Whether the link has `rel="nofollow"`. |

---

### ImageInfo

Information about an image found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The image URL. |
| `alt` | `Option<String>` | `None` | The alt text, if present. |
| `width` | `Option<u32>` | `None` | The width attribute, if present and parseable. |
| `height` | `Option<u32>` | `None` | The height attribute, if present and parseable. |
| `source` | `ImageSource` | — | The source of the image reference. |

---

### FeedInfo

Information about a feed link found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The feed URL. |
| `title` | `Option<String>` | `None` | The feed title, if present. |
| `feed_type` | `FeedType` | — | The type of feed. |

---

### JsonLdEntry

A JSON-LD structured data entry found on a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `schema_type` | `String` | — | The `@type` value from the JSON-LD object. |
| `name` | `Option<String>` | `None` | The `name` value, if present. |
| `raw` | `String` | — | The raw JSON-LD string. |

---

### CookieInfo

Information about an HTTP cookie received from a response.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | — | The cookie name. |
| `value` | `String` | — | The cookie value. |
| `domain` | `Option<String>` | `None` | The cookie domain, if specified. |
| `path` | `Option<String>` | `None` | The cookie path, if specified. |

---

### DownloadedAsset

A downloaded asset from a page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The original URL of the asset. |
| `content_hash` | `String` | — | The SHA-256 content hash of the asset. |
| `mime_type` | `Option<String>` | `None` | The MIME type from the Content-Type header. |
| `size` | `usize` | — | The size of the asset in bytes. |
| `asset_category` | `AssetCategory` | — | The category of the asset. |
| `html_tag` | `Option<String>` | `None` | The HTML tag that referenced this asset (e.g., "link", "script", "img"). |

---

### HreflangEntry

An hreflang alternate link entry.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `lang` | `String` | — | The language code (e.g., "en", "fr", "x-default"). |
| `url` | `String` | — | The URL for this language variant. |

---

### FaviconInfo

Information about a favicon or icon link.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `url` | `String` | — | The icon URL. |
| `rel` | `String` | — | The `rel` attribute (e.g., "icon", "apple-touch-icon"). |
| `sizes` | `Option<String>` | `None` | The `sizes` attribute, if present. |
| `mime_type` | `Option<String>` | `None` | The MIME type, if present. |

---

### HeadingInfo

A heading element extracted from the page.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `level` | `u8` | — | The heading level (1-6). |
| `text` | `String` | — | The heading text content. |

---

### CitationReference

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `index` | `usize` | — | Index |
| `url` | `String` | — | Url |
| `text` | `String` | — | Text |

---

## Metadata Types

### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `cost` | `Option<f64>` | `None` | Estimated cost of the LLM call in USD. |
| `prompt_tokens` | `Option<u64>` | `None` | Number of prompt (input) tokens consumed. |
| `completion_tokens` | `Option<u64>` | `None` | Number of completion (output) tokens generated. |
| `model` | `Option<String>` | `None` | The model identifier used for extraction. |
| `chunks_processed` | `usize` | — | Number of content chunks sent to the LLM. |

---

### ArticleMetadata

Article metadata extracted from `article:*` Open Graph tags.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `published_time` | `Option<String>` | `None` | The article publication time. |
| `modified_time` | `Option<String>` | `None` | The article modification time. |
| `author` | `Option<String>` | `None` | The article author. |
| `section` | `Option<String>` | `None` | The article section. |
| `tags` | `Vec<String>` | — | The article tags. |

---

### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `etag` | `Option<String>` | `None` | The ETag header value. |
| `last_modified` | `Option<String>` | `None` | The Last-Modified header value. |
| `cache_control` | `Option<String>` | `None` | The Cache-Control header value. |
| `server` | `Option<String>` | `None` | The Server header value. |
| `x_powered_by` | `Option<String>` | `None` | The X-Powered-By header value. |
| `content_language` | `Option<String>` | `None` | The Content-Language header value. |
| `content_encoding` | `Option<String>` | `None` | The Content-Encoding header value. |

---

### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `Option<String>` | `None` | The page title from the `<title>` element. |
| `description` | `Option<String>` | `None` | The meta description. |
| `canonical_url` | `Option<String>` | `None` | The canonical URL from `<link rel="canonical">`. |
| `keywords` | `Option<String>` | `None` | Keywords from `<meta name="keywords">`. |
| `author` | `Option<String>` | `None` | Author from `<meta name="author">`. |
| `viewport` | `Option<String>` | `None` | Viewport content from `<meta name="viewport">`. |
| `theme_color` | `Option<String>` | `None` | Theme color from `<meta name="theme-color">`. |
| `generator` | `Option<String>` | `None` | Generator from `<meta name="generator">`. |
| `robots` | `Option<String>` | `None` | Robots content from `<meta name="robots">`. |
| `html_lang` | `Option<String>` | `None` | The `lang` attribute from the `<html>` element. |
| `html_dir` | `Option<String>` | `None` | The `dir` attribute from the `<html>` element. |
| `og_title` | `Option<String>` | `None` | Open Graph title. |
| `og_type` | `Option<String>` | `None` | Open Graph type. |
| `og_image` | `Option<String>` | `None` | Open Graph image URL. |
| `og_description` | `Option<String>` | `None` | Open Graph description. |
| `og_url` | `Option<String>` | `None` | Open Graph URL. |
| `og_site_name` | `Option<String>` | `None` | Open Graph site name. |
| `og_locale` | `Option<String>` | `None` | Open Graph locale. |
| `og_video` | `Option<String>` | `None` | Open Graph video URL. |
| `og_audio` | `Option<String>` | `None` | Open Graph audio URL. |
| `og_locale_alternates` | `Vec<String>` | `None` | Open Graph locale alternates. |
| `twitter_card` | `Option<String>` | `None` | Twitter card type. |
| `twitter_title` | `Option<String>` | `None` | Twitter title. |
| `twitter_description` | `Option<String>` | `None` | Twitter description. |
| `twitter_image` | `Option<String>` | `None` | Twitter image URL. |
| `twitter_site` | `Option<String>` | `None` | Twitter site handle. |
| `twitter_creator` | `Option<String>` | `None` | Twitter creator handle. |
| `dc_title` | `Option<String>` | `None` | Dublin Core title. |
| `dc_creator` | `Option<String>` | `None` | Dublin Core creator. |
| `dc_subject` | `Option<String>` | `None` | Dublin Core subject. |
| `dc_description` | `Option<String>` | `None` | Dublin Core description. |
| `dc_publisher` | `Option<String>` | `None` | Dublin Core publisher. |
| `dc_date` | `Option<String>` | `None` | Dublin Core date. |
| `dc_type` | `Option<String>` | `None` | Dublin Core type. |
| `dc_format` | `Option<String>` | `None` | Dublin Core format. |
| `dc_identifier` | `Option<String>` | `None` | Dublin Core identifier. |
| `dc_language` | `Option<String>` | `None` | Dublin Core language. |
| `dc_rights` | `Option<String>` | `None` | Dublin Core rights. |
| `article` | `Option<ArticleMetadata>` | `None` | Article metadata from `article:*` Open Graph tags. |
| `hreflangs` | `Vec<HreflangEntry>` | `None` | Hreflang alternate links. |
| `favicons` | `Vec<FaviconInfo>` | `None` | Favicon and icon links. |
| `headings` | `Vec<HeadingInfo>` | `None` | Heading elements (h1-h6). |
| `word_count` | `Option<usize>` | `None` | Computed word count of the page body text. |

---

## Other Types

### CrawlEngineHandle

Opaque handle to a configured crawl engine.

Constructed via `create_engine` with an optional `CrawlConfig`.
All default trait implementations (BFS strategy, in-memory frontier,
per-domain throttle, etc.) are used internally.

*Opaque type — fields are not directly accessible.*

---
