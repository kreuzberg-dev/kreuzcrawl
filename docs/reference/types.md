# Types Reference

All public types in the `kreuzcrawl` crate. These types are used across the Rust core library, REST API responses, and language binding interfaces.

## Result Types

### ScrapeResult

Result of a single-page scrape operation.

| Field | Type | Description |
|-------|------|-------------|
| `status_code` | `u16` | HTTP status code of the response |
| `content_type` | `String` | Content-Type header value |
| `html` | `String` | HTML body of the response |
| `body_size` | `usize` | Size of the response body in bytes |
| `metadata` | `PageMetadata` | Extracted metadata from the page |
| `links` | `Vec<LinkInfo>` | Links found on the page |
| `images` | `Vec<ImageInfo>` | Images found on the page |
| `feeds` | `Vec<FeedInfo>` | Feed links found on the page |
| `json_ld` | `Vec<JsonLdEntry>` | JSON-LD structured data entries |
| `is_allowed` | `bool` | Whether the URL is allowed by robots.txt |
| `crawl_delay` | `Option<u64>` | Crawl delay from robots.txt (seconds) |
| `noindex_detected` | `bool` | Whether a noindex directive was detected |
| `nofollow_detected` | `bool` | Whether a nofollow directive was detected |
| `x_robots_tag` | `Option<String>` | X-Robots-Tag header value, if present |
| `is_pdf` | `bool` | Whether the content is a PDF |
| `was_skipped` | `bool` | Whether the page was skipped (binary or PDF content) |
| `detected_charset` | `Option<String>` | Detected character set encoding |
| `main_content_only` | `bool` | Whether main_content_only was active |
| `auth_header_sent` | `bool` | Whether an auth header was sent with the request |
| `response_meta` | `Option<ResponseMeta>` | Response metadata from HTTP headers |
| `assets` | `Vec<DownloadedAsset>` | Downloaded assets from the page |
| `js_render_hint` | `bool` | Whether the content suggests JS rendering is needed |
| `browser_used` | `bool` | Whether the browser fallback was used |
| `markdown` | `Option<MarkdownResult>` | Markdown conversion of the page content |
| `extracted_data` | `Option<Value>` | Structured data extracted by LLM (with `LlmExtractor`) |
| `extraction_meta` | `Option<ExtractionMeta>` | Metadata about the LLM extraction pass |
| `screenshot` | `Option<Vec<u8>>` | Screenshot PNG bytes (skipped in JSON serialization) |
| `downloaded_document` | `Option<DownloadedDocument>` | Downloaded non-HTML document |

---

### CrawlResult

Result of a multi-page crawl operation.

| Field | Type | Description |
|-------|------|-------------|
| `pages` | `Vec<CrawlPageResult>` | List of crawled pages |
| `final_url` | `String` | Final URL after redirects |
| `redirect_count` | `usize` | Number of redirects followed |
| `was_skipped` | `bool` | Whether any page was skipped |
| `error` | `Option<String>` | Error message, if the crawl encountered an issue |
| `cookies` | `Vec<CookieInfo>` | Cookies collected during the crawl |

**Methods:**

- `unique_normalized_urls() -> usize` -- Returns the count of unique normalized URLs encountered.

---

### CrawlPageResult

Result of crawling a single page during a crawl operation.

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | Original URL of the page |
| `normalized_url` | `String` | Normalized URL |
| `status_code` | `u16` | HTTP status code |
| `content_type` | `String` | Content-Type header value |
| `html` | `String` | HTML body |
| `body_size` | `usize` | Response body size in bytes |
| `metadata` | `PageMetadata` | Extracted page metadata |
| `links` | `Vec<LinkInfo>` | Links found on the page |
| `images` | `Vec<ImageInfo>` | Images found on the page |
| `feeds` | `Vec<FeedInfo>` | Feed links found on the page |
| `json_ld` | `Vec<JsonLdEntry>` | JSON-LD entries |
| `depth` | `usize` | Depth of this page from the start URL |
| `stayed_on_domain` | `bool` | Whether this page is on the same domain |
| `was_skipped` | `bool` | Whether this page was skipped |
| `is_pdf` | `bool` | Whether the content is a PDF |
| `detected_charset` | `Option<String>` | Detected charset |
| `markdown` | `Option<MarkdownResult>` | Markdown conversion |
| `extracted_data` | `Option<Value>` | LLM-extracted structured data |
| `extraction_meta` | `Option<ExtractionMeta>` | LLM extraction metadata |
| `downloaded_document` | `Option<DownloadedDocument>` | Downloaded non-HTML document |

---

### MapResult

Result of a map operation (URL discovery).

| Field | Type | Description |
|-------|------|-------------|
| `urls` | `Vec<SitemapUrl>` | Discovered URLs |

---

### MarkdownResult

Rich markdown conversion result from HTML processing.

| Field | Type | Description |
|-------|------|-------------|
| `content` | `String` | Converted markdown text |
| `document_structure` | `Option<Value>` | Structured document tree with semantic nodes |
| `tables` | `Vec<Value>` | Extracted tables with structured cell data |
| `warnings` | `Vec<String>` | Non-fatal processing warnings |
| `citations` | `Option<CitationResult>` | Content with links replaced by numbered citations |
| `fit_content` | `Option<String>` | Content-filtered markdown optimized for LLM consumption |

---

### DownloadedDocument

A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | URL the document was fetched from |
| `mime_type` | `String` | MIME type from Content-Type header |
| `content` | `Vec<u8>` | Raw document bytes (skipped in JSON serialization) |
| `size` | `usize` | Size of the document in bytes |
| `filename` | `Option<String>` | Filename from Content-Disposition or URL path |
| `content_hash` | `String` | SHA-256 hex digest of the content |
| `headers` | `HashMap<String, String>` | Selected response headers |

---

### InteractionResult

Result of executing a sequence of page interaction actions.

| Field | Type | Description |
|-------|------|-------------|
| `action_results` | `Vec<ActionResult>` | Results from each executed action |
| `final_html` | `String` | Final page HTML after all actions |
| `final_url` | `String` | Final page URL (may have changed) |
| `screenshot` | `Option<Vec<u8>>` | Screenshot after all actions (skipped in JSON) |

---

### ActionResult

Result from a single page action execution.

| Field | Type | Description |
|-------|------|-------------|
| `action_index` | `usize` | Zero-based index of the action |
| `action_type` | `String` | Type of action executed |
| `success` | `bool` | Whether the action completed successfully |
| `data` | `Option<Value>` | Action-specific return data |
| `error` | `Option<String>` | Error message if the action failed |

---

## Metadata Types

### PageMetadata

Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.

**Standard fields:**

| Field | Type | Description |
|-------|------|-------------|
| `title` | `Option<String>` | Page title from `<title>` |
| `description` | `Option<String>` | Meta description |
| `canonical_url` | `Option<String>` | Canonical URL from `<link rel="canonical">` |
| `keywords` | `Option<String>` | Keywords from `<meta name="keywords">` |
| `author` | `Option<String>` | Author from `<meta name="author">` |
| `viewport` | `Option<String>` | Viewport content |
| `theme_color` | `Option<String>` | Theme color |
| `generator` | `Option<String>` | Generator |
| `robots` | `Option<String>` | Robots content |
| `html_lang` | `Option<String>` | `lang` attribute from `<html>` |
| `html_dir` | `Option<String>` | `dir` attribute from `<html>` |
| `word_count` | `Option<usize>` | Computed word count of body text |

**Open Graph fields:**

| Field | Type |
|-------|------|
| `og_title` | `Option<String>` |
| `og_type` | `Option<String>` |
| `og_image` | `Option<String>` |
| `og_description` | `Option<String>` |
| `og_url` | `Option<String>` |
| `og_site_name` | `Option<String>` |
| `og_locale` | `Option<String>` |
| `og_video` | `Option<String>` |
| `og_audio` | `Option<String>` |
| `og_locale_alternates` | `Option<Vec<String>>` |

**Twitter Card fields:**

| Field | Type |
|-------|------|
| `twitter_card` | `Option<String>` |
| `twitter_title` | `Option<String>` |
| `twitter_description` | `Option<String>` |
| `twitter_image` | `Option<String>` |
| `twitter_site` | `Option<String>` |
| `twitter_creator` | `Option<String>` |

**Dublin Core fields:**

| Field | Type |
|-------|------|
| `dc_title` | `Option<String>` |
| `dc_creator` | `Option<String>` |
| `dc_subject` | `Option<String>` |
| `dc_description` | `Option<String>` |
| `dc_publisher` | `Option<String>` |
| `dc_date` | `Option<String>` |
| `dc_type` | `Option<String>` |
| `dc_format` | `Option<String>` |
| `dc_identifier` | `Option<String>` |
| `dc_language` | `Option<String>` |
| `dc_rights` | `Option<String>` |

**Structured sub-fields:**

| Field | Type | Description |
|-------|------|-------------|
| `article` | `Option<ArticleMetadata>` | Article metadata from `article:*` OG tags |
| `hreflangs` | `Option<Vec<HreflangEntry>>` | Hreflang alternate links |
| `favicons` | `Option<Vec<FaviconInfo>>` | Favicon and icon links |
| `headings` | `Option<Vec<HeadingInfo>>` | Heading elements (h1--h6) |

---

### ArticleMetadata

| Field | Type | Description |
|-------|------|-------------|
| `published_time` | `Option<String>` | Publication time |
| `modified_time` | `Option<String>` | Modification time |
| `author` | `Option<String>` | Article author |
| `section` | `Option<String>` | Article section |
| `tags` | `Vec<String>` | Article tags |

### HreflangEntry

| Field | Type | Description |
|-------|------|-------------|
| `lang` | `String` | Language code (e.g. `"en"`, `"fr"`, `"x-default"`) |
| `url` | `String` | URL for this language variant |

### FaviconInfo

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | Icon URL |
| `rel` | `String` | `rel` attribute (e.g. `"icon"`, `"apple-touch-icon"`) |
| `sizes` | `Option<String>` | `sizes` attribute |
| `mime_type` | `Option<String>` | MIME type |

### HeadingInfo

| Field | Type | Description |
|-------|------|-------------|
| `level` | `u8` | Heading level (1--6) |
| `text` | `String` | Heading text content |

### ResponseMeta

Response metadata extracted from HTTP headers.

| Field | Type | Description |
|-------|------|-------------|
| `etag` | `Option<String>` | ETag header |
| `last_modified` | `Option<String>` | Last-Modified header |
| `cache_control` | `Option<String>` | Cache-Control header |
| `server` | `Option<String>` | Server header |
| `x_powered_by` | `Option<String>` | X-Powered-By header |
| `content_language` | `Option<String>` | Content-Language header |
| `content_encoding` | `Option<String>` | Content-Encoding header |

### ExtractionMeta

Metadata about an LLM extraction pass.

| Field | Type | Description |
|-------|------|-------------|
| `cost` | `Option<f64>` | Estimated cost of the LLM call in USD |
| `prompt_tokens` | `Option<u64>` | Number of prompt (input) tokens consumed |
| `completion_tokens` | `Option<u64>` | Number of completion (output) tokens generated |
| `model` | `Option<String>` | Model identifier used for extraction |
| `chunks_processed` | `usize` | Number of content chunks sent to the LLM |

---

## Discovery Types

### LinkInfo

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | Resolved URL of the link |
| `text` | `String` | Visible text of the link |
| `link_type` | `LinkType` | Classification of the link |
| `rel` | `Option<String>` | `rel` attribute value |
| `nofollow` | `bool` | Whether the link has `rel="nofollow"` |

### LinkType

| Value | Description |
|-------|-------------|
| `internal` | Link to the same domain |
| `external` | Link to a different domain |
| `anchor` | Fragment-only link (e.g. `#section`) |
| `document` | Link to a downloadable document (PDF, DOC, etc.) |

### ImageInfo

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | Image URL |
| `alt` | `Option<String>` | Alt text |
| `width` | `Option<u32>` | Width attribute (if parseable) |
| `height` | `Option<u32>` | Height attribute (if parseable) |
| `source` | `ImageSource` | Source of the image reference |

### ImageSource

| Value | Description |
|-------|-------------|
| `img` | An `<img>` tag |
| `picture_source` | A `<source>` tag inside `<picture>` |
| `og:image` | An `og:image` meta tag |
| `twitter:image` | A `twitter:image` meta tag |

### FeedInfo

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | Feed URL |
| `title` | `Option<String>` | Feed title |
| `feed_type` | `FeedType` | Type of feed |

### FeedType

| Value | Description |
|-------|-------------|
| `rss` | RSS feed |
| `atom` | Atom feed |
| `json_feed` | JSON Feed |

### JsonLdEntry

| Field | Type | Description |
|-------|------|-------------|
| `schema_type` | `String` | `@type` value from the JSON-LD object |
| `name` | `Option<String>` | `name` value, if present |
| `raw` | `String` | Raw JSON-LD string |

### CookieInfo

| Field | Type | Description |
|-------|------|-------------|
| `name` | `String` | Cookie name |
| `value` | `String` | Cookie value |
| `domain` | `Option<String>` | Cookie domain |
| `path` | `Option<String>` | Cookie path |

---

## Asset Types

### DownloadedAsset

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | Original URL of the asset |
| `content_hash` | `String` | SHA-256 content hash |
| `mime_type` | `Option<String>` | MIME type from Content-Type header |
| `size` | `usize` | Size in bytes |
| `asset_category` | `AssetCategory` | Category of the asset |
| `html_tag` | `Option<String>` | HTML tag that referenced this asset (e.g. `"link"`, `"script"`, `"img"`) |

### AssetCategory

| Value | Description |
|-------|-------------|
| `document` | Document file (PDF, DOC, etc.) |
| `image` | Image file |
| `audio` | Audio file |
| `video` | Video file |
| `font` | Font file |
| `stylesheet` | CSS stylesheet |
| `script` | JavaScript file |
| `archive` | Archive file (ZIP, TAR, etc.) |
| `data` | Data file (JSON, XML, CSV, etc.) |
| `other` | Unrecognized asset type |

---

## Sitemap Types

### SitemapUrl

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | The URL |
| `lastmod` | `Option<String>` | Last modification date |
| `changefreq` | `Option<String>` | Change frequency |
| `priority` | `Option<String>` | Priority |

---

## Streaming Types

### CrawlEvent

Event emitted during a streaming crawl operation (tagged enum).

| Variant | Fields | Description |
|---------|--------|-------------|
| `Page` | `CrawlPageResult` | A single page has been crawled |
| `Error` | `url: String`, `error: String` | An error occurred crawling a URL |
| `Complete` | `pages_crawled: usize` | The crawl has completed |

---

## Binding Types

### CrawlEngineHandle

Opaque handle to a configured crawl engine, used for polyglot bindings. Constructed via `create_engine()`.

### BatchScrapeResult

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | URL that was scraped |
| `result` | `Option<ScrapeResult>` | Scrape result (if successful) |
| `error` | `Option<String>` | Error message (if failed) |

### BatchCrawlResult

| Field | Type | Description |
|-------|------|-------------|
| `url` | `String` | Seed URL that was crawled |
| `result` | `Option<CrawlResult>` | Crawl result (if successful) |
| `error` | `Option<String>` | Error message (if failed) |
