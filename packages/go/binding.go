package kreuzcrawl

/*
#include "kreuzcrawl.h"
*/
import "C"

import (
    "encoding/json"
    "errors"
    "fmt"
    "unsafe"
)

// lastError retrieves the last error from the FFI layer.
func lastError() error {
    code := int32(C.kcrawl_last_error_code())
    if code == 0 {
        return nil
    }
    ctx := C.kcrawl_last_error_context()
    message := C.GoString(ctx)
    C.kcrawl_free_string(ctx)
    return fmt.Errorf("[%d] %s", code, message)
}

var (
    ErrNotFound = errors.New("not_found")
    ErrUnauthorized = errors.New("unauthorized")
    ErrForbidden = errors.New("forbidden")
    ErrWafBlocked = errors.New("forbidden: waf/blocked")
    ErrTimeout = errors.New("timeout")
    ErrRateLimited = errors.New("rate_limited")
    ErrServerError = errors.New("server_error")
    ErrBadGateway = errors.New("bad_gateway")
    ErrGone = errors.New("gone")
    ErrConnection = errors.New("connection")
    ErrDns = errors.New("dns")
    ErrSsl = errors.New("ssl")
    ErrDataLoss = errors.New("data_loss")
    ErrBrowserError = errors.New("browser")
    ErrBrowserTimeout = errors.New("browser_timeout")
    ErrInvalidConfig = errors.New("invalid_config")
    ErrOther = errors.New("other")
)

// CrawlError is a structured error type.
type CrawlError struct {
    Code    string
    Message string
}

func (e *CrawlError) Error() string { return e.Message }

// When to use the headless browser fallback.
type BrowserMode string

const (
    // Automatically detect when JS rendering is needed and fall back to browser.
    BrowserModeAuto BrowserMode = "auto"
    // Always use the browser for every request.
    BrowserModeAlways BrowserMode = "always"
    // Never use the browser fallback.
    BrowserModeNever BrowserMode = "never"
)


// Wait strategy for browser page rendering.
type BrowserWait string

const (
    // Wait until network activity is idle.
    BrowserWaitNetworkIdle BrowserWait = "network_idle"
    // Wait for a specific CSS selector to appear in the DOM.
    BrowserWaitSelector BrowserWait = "selector"
    // Wait for a fixed duration after navigation.
    BrowserWaitFixed BrowserWait = "fixed"
)


// Authentication configuration.
// Variants: Basic, Bearer, Header
type AuthConfig struct {
    Username *string `json:"username,omitempty"`
    Password *string `json:"password,omitempty"`
    Token *string `json:"token,omitempty"`
    Name *string `json:"name,omitempty"`
    Value *string `json:"value,omitempty"`
}


// The classification of a link.
type LinkType string

const (
    // A link to the same domain.
    LinkTypeInternal LinkType = "internal"
    // A link to a different domain.
    LinkTypeExternal LinkType = "external"
    // A fragment-only link (e.g., `#section`).
    LinkTypeAnchor LinkType = "anchor"
    // A link to a downloadable document (PDF, DOC, etc.).
    LinkTypeDocument LinkType = "document"
)


// The source of an image reference.
type ImageSource string

const (
    // An `<img>` tag.
    ImageSourceImg ImageSource = "img"
    // A `<source>` tag inside `<picture>`.
    ImageSourcePictureSource ImageSource = "picture_source"
    // An `og:image` meta tag.
    ImageSourceOgImage ImageSource = "og_image"
    // A `twitter:image` meta tag.
    ImageSourceTwitterImage ImageSource = "twitter_image"
)


// The type of a feed (RSS, Atom, or JSON Feed).
type FeedType string

const (
    // RSS feed.
    FeedTypeRss FeedType = "rss"
    // Atom feed.
    FeedTypeAtom FeedType = "atom"
    // JSON Feed.
    FeedTypeJsonFeed FeedType = "json_feed"
)


// The category of a downloaded asset.
type AssetCategory string

const (
    // A document file (PDF, DOC, etc.).
    AssetCategoryDocument AssetCategory = "document"
    // An image file.
    AssetCategoryImage AssetCategory = "image"
    // An audio file.
    AssetCategoryAudio AssetCategory = "audio"
    // A video file.
    AssetCategoryVideo AssetCategory = "video"
    // A font file.
    AssetCategoryFont AssetCategory = "font"
    // A CSS stylesheet.
    AssetCategoryStylesheet AssetCategory = "stylesheet"
    // A JavaScript file.
    AssetCategoryScript AssetCategory = "script"
    // An archive file (ZIP, TAR, etc.).
    AssetCategoryArchive AssetCategory = "archive"
    // A data file (JSON, XML, CSV, etc.).
    AssetCategoryData AssetCategory = "data"
    // An unrecognized asset type.
    AssetCategoryOther AssetCategory = "other"
)


// An event emitted during a streaming crawl operation.
// Variants: Page, Error, Complete
type CrawlEvent struct {
    // The URL that failed.
    Url *string `json:"url,omitempty"`
    // The error message.
    Error *string `json:"error,omitempty"`
    // Total number of pages crawled.
    PagesCrawled *uint `json:"pages_crawled,omitempty"`
}


// Metadata about an LLM extraction pass.
type ExtractionMeta struct {
    // Estimated cost of the LLM call in USD.
    Cost *float64 `json:"cost,omitempty"`
    // Number of prompt (input) tokens consumed.
    PromptTokens *uint64 `json:"prompt_tokens,omitempty"`
    // Number of completion (output) tokens generated.
    CompletionTokens *uint64 `json:"completion_tokens,omitempty"`
    // The model identifier used for extraction.
    Model *string `json:"model,omitempty"`
    // Number of content chunks sent to the LLM.
    ChunksProcessed uint `json:"chunks_processed"`
}


// ExtractionMeta option function
type ExtractionMetaOption func(*ExtractionMeta)

// WithCost sets the cost field.
func WithCost(v float64) ExtractionMetaOption {
    return func(c *ExtractionMeta) { c.Cost = v }
}

// WithPromptTokens sets the prompt_tokens field.
func WithPromptTokens(v uint64) ExtractionMetaOption {
    return func(c *ExtractionMeta) { c.PromptTokens = v }
}

// WithCompletionTokens sets the completion_tokens field.
func WithCompletionTokens(v uint64) ExtractionMetaOption {
    return func(c *ExtractionMeta) { c.CompletionTokens = v }
}

// WithModel sets the model field.
func WithModel(v string) ExtractionMetaOption {
    return func(c *ExtractionMeta) { c.Model = v }
}

// WithChunksProcessed sets the chunks_processed field.
func WithChunksProcessed(v uint) ExtractionMetaOption {
    return func(c *ExtractionMeta) { c.ChunksProcessed = v }
}

// NewExtractionMeta creates a ExtractionMeta with optional parameters.
func NewExtractionMeta(opts ...ExtractionMetaOption) *ExtractionMeta {
    c := &ExtractionMeta {
        Cost: 0.0,
        PromptTokens: 0,
        CompletionTokens: 0,
        Model: "",
        ChunksProcessed: 0,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Proxy configuration for HTTP requests.
type ProxyConfig struct {
    // Proxy URL (e.g. "http://proxy:8080", "socks5://proxy:1080").
    Url string `json:"url"`
    // Optional username for proxy authentication.
    Username *string `json:"username,omitempty"`
    // Optional password for proxy authentication.
    Password *string `json:"password,omitempty"`
}


// Browser fallback configuration.
type BrowserConfig struct {
    // When to use the headless browser fallback.
    Mode BrowserMode `json:"mode"`
    // CDP WebSocket endpoint for connecting to an external browser instance.
    Endpoint *string `json:"endpoint,omitempty"`
    // Timeout for browser page load and rendering (in milliseconds when serialized).
    Timeout uint64 `json:"timeout"`
    // Wait strategy after browser navigation.
    Wait BrowserWait `json:"wait"`
    // CSS selector to wait for when `wait` is `Selector`.
    WaitSelector *string `json:"wait_selector,omitempty"`
    // Extra time to wait after the wait condition is met.
    ExtraWait *uint64 `json:"extra_wait,omitempty"`
}


// BrowserConfig option function
type BrowserConfigOption func(*BrowserConfig)

// WithMode sets the mode field.
func WithMode(v BrowserMode) BrowserConfigOption {
    return func(c *BrowserConfig) { c.Mode = v }
}

// WithEndpoint sets the endpoint field.
func WithEndpoint(v string) BrowserConfigOption {
    return func(c *BrowserConfig) { c.Endpoint = v }
}

// WithTimeout sets the timeout field.
func WithTimeout(v uint64) BrowserConfigOption {
    return func(c *BrowserConfig) { c.Timeout = v }
}

// WithWait sets the wait field.
func WithWait(v BrowserWait) BrowserConfigOption {
    return func(c *BrowserConfig) { c.Wait = v }
}

// WithWaitSelector sets the wait_selector field.
func WithWaitSelector(v string) BrowserConfigOption {
    return func(c *BrowserConfig) { c.WaitSelector = v }
}

// WithExtraWait sets the extra_wait field.
func WithExtraWait(v uint64) BrowserConfigOption {
    return func(c *BrowserConfig) { c.ExtraWait = v }
}

// NewBrowserConfig creates a BrowserConfig with optional parameters.
func NewBrowserConfig(opts ...BrowserConfigOption) *BrowserConfig {
    c := &BrowserConfig {
        Mode: &BrowserMode{},
        Endpoint: "",
        Timeout: 0,
        Wait: &BrowserWait{},
        WaitSelector: "",
        ExtraWait: 0,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Configuration for crawl, scrape, and map operations.
type CrawlConfig struct {
    // Maximum crawl depth (number of link hops from the start URL).
    MaxDepth *uint `json:"max_depth,omitempty"`
    // Maximum number of pages to crawl.
    MaxPages *uint `json:"max_pages,omitempty"`
    // Maximum number of concurrent requests.
    MaxConcurrent *uint `json:"max_concurrent,omitempty"`
    // Whether to respect robots.txt directives.
    RespectRobotsTxt bool `json:"respect_robots_txt"`
    // Custom user-agent string.
    UserAgent *string `json:"user_agent,omitempty"`
    // Whether to restrict crawling to the same domain.
    StayOnDomain bool `json:"stay_on_domain"`
    // Whether to allow subdomains when `stay_on_domain` is true.
    AllowSubdomains bool `json:"allow_subdomains"`
    // Regex patterns for paths to include during crawling.
    IncludePaths []string `json:"include_paths"`
    // Regex patterns for paths to exclude during crawling.
    ExcludePaths []string `json:"exclude_paths"`
    // Custom HTTP headers to send with each request.
    CustomHeaders map[string]string `json:"custom_headers"`
    // Timeout for individual HTTP requests (in milliseconds when serialized).
    RequestTimeout uint64 `json:"request_timeout"`
    // Maximum number of redirects to follow.
    MaxRedirects uint `json:"max_redirects"`
    // Number of retry attempts for failed requests.
    RetryCount uint `json:"retry_count"`
    // HTTP status codes that should trigger a retry.
    RetryCodes []uint16 `json:"retry_codes"`
    // Whether to enable cookie handling.
    CookiesEnabled bool `json:"cookies_enabled"`
    // Authentication configuration.
    Auth *AuthConfig `json:"auth,omitempty"`
    // Maximum response body size in bytes.
    MaxBodySize *uint `json:"max_body_size,omitempty"`
    // Whether to extract only the main content from HTML pages.
    MainContentOnly bool `json:"main_content_only"`
    // CSS selectors for tags to remove from HTML before processing.
    RemoveTags []string `json:"remove_tags"`
    // Maximum number of URLs to return from a map operation.
    MapLimit *uint `json:"map_limit,omitempty"`
    // Search filter for map results (case-insensitive substring match on URLs).
    MapSearch *string `json:"map_search,omitempty"`
    // Whether to download assets (CSS, JS, images, etc.) from the page.
    DownloadAssets bool `json:"download_assets"`
    // Filter for asset categories to download.
    AssetTypes []AssetCategory `json:"asset_types"`
    // Maximum size in bytes for individual asset downloads.
    MaxAssetSize *uint `json:"max_asset_size,omitempty"`
    // Browser configuration.
    Browser BrowserConfig `json:"browser"`
    // Proxy configuration for HTTP requests.
    Proxy *ProxyConfig `json:"proxy,omitempty"`
    // List of user-agent strings for rotation. If non-empty, overrides `user_agent`.
    UserAgents []string `json:"user_agents"`
    // Whether to capture a screenshot when using the browser.
    CaptureScreenshot bool `json:"capture_screenshot"`
    // Whether to download non-HTML documents (PDF, DOCX, images, code, etc.) instead of skipping them.
    DownloadDocuments bool `json:"download_documents"`
    // Maximum size in bytes for document downloads. Defaults to 50 MB.
    DocumentMaxSize *uint `json:"document_max_size,omitempty"`
    // Allowlist of MIME types to download. If empty, uses built-in defaults.
    DocumentMimeTypes []string `json:"document_mime_types"`
    // Path to write WARC output. If `None`, WARC output is disabled.
    WarcOutput *string `json:"warc_output,omitempty"`
    // Named browser profile for persistent sessions (cookies, localStorage).
    BrowserProfile *string `json:"browser_profile,omitempty"`
    // Whether to save changes back to the browser profile on exit.
    SaveBrowserProfile bool `json:"save_browser_profile"`
}


// CrawlConfig option function
type CrawlConfigOption func(*CrawlConfig)

// WithMaxDepth sets the max_depth field.
func WithMaxDepth(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MaxDepth = v }
}

// WithMaxPages sets the max_pages field.
func WithMaxPages(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MaxPages = v }
}

// WithMaxConcurrent sets the max_concurrent field.
func WithMaxConcurrent(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MaxConcurrent = v }
}

// WithRespectRobotsTxt sets the respect_robots_txt field.
func WithRespectRobotsTxt(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.RespectRobotsTxt = v }
}

// WithUserAgent sets the user_agent field.
func WithUserAgent(v string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.UserAgent = v }
}

// WithStayOnDomain sets the stay_on_domain field.
func WithStayOnDomain(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.StayOnDomain = v }
}

// WithAllowSubdomains sets the allow_subdomains field.
func WithAllowSubdomains(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.AllowSubdomains = v }
}

// WithIncludePaths sets the include_paths field.
func WithIncludePaths(v []string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.IncludePaths = v }
}

// WithExcludePaths sets the exclude_paths field.
func WithExcludePaths(v []string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.ExcludePaths = v }
}

// WithCustomHeaders sets the custom_headers field.
func WithCustomHeaders(v map[string]string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.CustomHeaders = v }
}

// WithRequestTimeout sets the request_timeout field.
func WithRequestTimeout(v uint64) CrawlConfigOption {
    return func(c *CrawlConfig) { c.RequestTimeout = v }
}

// WithMaxRedirects sets the max_redirects field.
func WithMaxRedirects(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MaxRedirects = v }
}

// WithRetryCount sets the retry_count field.
func WithRetryCount(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.RetryCount = v }
}

// WithRetryCodes sets the retry_codes field.
func WithRetryCodes(v []uint16) CrawlConfigOption {
    return func(c *CrawlConfig) { c.RetryCodes = v }
}

// WithCookiesEnabled sets the cookies_enabled field.
func WithCookiesEnabled(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.CookiesEnabled = v }
}

// WithAuth sets the auth field.
func WithAuth(v AuthConfig) CrawlConfigOption {
    return func(c *CrawlConfig) { c.Auth = v }
}

// WithMaxBodySize sets the max_body_size field.
func WithMaxBodySize(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MaxBodySize = v }
}

// WithMainContentOnly sets the main_content_only field.
func WithMainContentOnly(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MainContentOnly = v }
}

// WithRemoveTags sets the remove_tags field.
func WithRemoveTags(v []string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.RemoveTags = v }
}

// WithMapLimit sets the map_limit field.
func WithMapLimit(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MapLimit = v }
}

// WithMapSearch sets the map_search field.
func WithMapSearch(v string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MapSearch = v }
}

// WithDownloadAssets sets the download_assets field.
func WithDownloadAssets(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.DownloadAssets = v }
}

// WithAssetTypes sets the asset_types field.
func WithAssetTypes(v []AssetCategory) CrawlConfigOption {
    return func(c *CrawlConfig) { c.AssetTypes = v }
}

// WithMaxAssetSize sets the max_asset_size field.
func WithMaxAssetSize(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.MaxAssetSize = v }
}

// WithBrowser sets the browser field.
func WithBrowser(v BrowserConfig) CrawlConfigOption {
    return func(c *CrawlConfig) { c.Browser = v }
}

// WithProxy sets the proxy field.
func WithProxy(v ProxyConfig) CrawlConfigOption {
    return func(c *CrawlConfig) { c.Proxy = v }
}

// WithUserAgents sets the user_agents field.
func WithUserAgents(v []string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.UserAgents = v }
}

// WithCaptureScreenshot sets the capture_screenshot field.
func WithCaptureScreenshot(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.CaptureScreenshot = v }
}

// WithDownloadDocuments sets the download_documents field.
func WithDownloadDocuments(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.DownloadDocuments = v }
}

// WithDocumentMaxSize sets the document_max_size field.
func WithDocumentMaxSize(v uint) CrawlConfigOption {
    return func(c *CrawlConfig) { c.DocumentMaxSize = v }
}

// WithDocumentMimeTypes sets the document_mime_types field.
func WithDocumentMimeTypes(v []string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.DocumentMimeTypes = v }
}

// WithWarcOutput sets the warc_output field.
func WithWarcOutput(v string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.WarcOutput = v }
}

// WithBrowserProfile sets the browser_profile field.
func WithBrowserProfile(v string) CrawlConfigOption {
    return func(c *CrawlConfig) { c.BrowserProfile = v }
}

// WithSaveBrowserProfile sets the save_browser_profile field.
func WithSaveBrowserProfile(v bool) CrawlConfigOption {
    return func(c *CrawlConfig) { c.SaveBrowserProfile = v }
}

// NewCrawlConfig creates a CrawlConfig with optional parameters.
func NewCrawlConfig(opts ...CrawlConfigOption) *CrawlConfig {
    c := &CrawlConfig {
        MaxDepth: 0,
        MaxPages: 0,
        MaxConcurrent: 0,
        RespectRobotsTxt: false,
        UserAgent: "",
        StayOnDomain: false,
        AllowSubdomains: false,
        IncludePaths: [][]string,
        ExcludePaths: [][]string,
        CustomHeaders: make(map[string]string),
        RequestTimeout: 0,
        MaxRedirects: 0,
        RetryCount: 0,
        RetryCodes: [][]uint16,
        CookiesEnabled: false,
        Auth: &AuthConfig{},
        MaxBodySize: 0,
        MainContentOnly: false,
        RemoveTags: [][]string,
        MapLimit: 0,
        MapSearch: "",
        DownloadAssets: false,
        AssetTypes: [][]AssetCategory,
        MaxAssetSize: 0,
        Browser: &BrowserConfig{},
        Proxy: &ProxyConfig{},
        UserAgents: [][]string,
        CaptureScreenshot: false,
        DownloadDocuments: false,
        DocumentMaxSize: 0,
        DocumentMimeTypes: [][]string,
        WarcOutput: "",
        BrowserProfile: "",
        SaveBrowserProfile: false,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// A downloaded non-HTML document (PDF, DOCX, image, code file, etc.).
//
// When the crawler encounters non-HTML content and `download_documents` is
// enabled, it downloads the raw bytes and populates this struct instead of
// skipping the resource.
type DownloadedDocument struct {
    // The URL the document was fetched from.
    Url string `json:"url"`
    // The MIME type from the Content-Type header.
    MimeType string `json:"mime_type"`
    // Raw document bytes. Skipped during JSON serialization.
    Content []byte `json:"content"`
    // Size of the document in bytes.
    Size uint `json:"size"`
    // Filename extracted from Content-Disposition or URL path.
    Filename *string `json:"filename,omitempty"`
    // SHA-256 hex digest of the content.
    ContentHash string `json:"content_hash"`
    // Selected response headers.
    Headers map[string]string `json:"headers"`
}


// DownloadedDocument option function
type DownloadedDocumentOption func(*DownloadedDocument)

// WithUrl sets the url field.
func WithUrl(v string) DownloadedDocumentOption {
    return func(c *DownloadedDocument) { c.Url = v }
}

// WithMimeType sets the mime_type field.
func WithMimeType(v string) DownloadedDocumentOption {
    return func(c *DownloadedDocument) { c.MimeType = v }
}

// WithContent sets the content field.
func WithContent(v []byte) DownloadedDocumentOption {
    return func(c *DownloadedDocument) { c.Content = v }
}

// WithSize sets the size field.
func WithSize(v uint) DownloadedDocumentOption {
    return func(c *DownloadedDocument) { c.Size = v }
}

// WithFilename sets the filename field.
func WithFilename(v string) DownloadedDocumentOption {
    return func(c *DownloadedDocument) { c.Filename = v }
}

// WithContentHash sets the content_hash field.
func WithContentHash(v string) DownloadedDocumentOption {
    return func(c *DownloadedDocument) { c.ContentHash = v }
}

// WithHeaders sets the headers field.
func WithHeaders(v map[string]string) DownloadedDocumentOption {
    return func(c *DownloadedDocument) { c.Headers = v }
}

// NewDownloadedDocument creates a DownloadedDocument with optional parameters.
func NewDownloadedDocument(opts ...DownloadedDocumentOption) *DownloadedDocument {
    c := &DownloadedDocument {
        Url: "",
        MimeType: "",
        Content: []byte{},
        Size: 0,
        Filename: "",
        ContentHash: "",
        Headers: make(map[string]string),
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Result of executing a sequence of page interaction actions.
type InteractionResult struct {
    // Results from each executed action.
    ActionResults []ActionResult `json:"action_results"`
    // Final page HTML after all actions completed.
    FinalHtml string `json:"final_html"`
    // Final page URL (may have changed due to navigation).
    FinalUrl string `json:"final_url"`
    // Screenshot taken after all actions, if requested.
    Screenshot *[]byte `json:"screenshot,omitempty"`
}


// InteractionResult option function
type InteractionResultOption func(*InteractionResult)

// WithActionResults sets the action_results field.
func WithActionResults(v []ActionResult) InteractionResultOption {
    return func(c *InteractionResult) { c.ActionResults = v }
}

// WithFinalHtml sets the final_html field.
func WithFinalHtml(v string) InteractionResultOption {
    return func(c *InteractionResult) { c.FinalHtml = v }
}

// WithFinalUrl sets the final_url field.
func WithFinalUrl(v string) InteractionResultOption {
    return func(c *InteractionResult) { c.FinalUrl = v }
}

// WithScreenshot sets the screenshot field.
func WithScreenshot(v []byte) InteractionResultOption {
    return func(c *InteractionResult) { c.Screenshot = v }
}

// NewInteractionResult creates a InteractionResult with optional parameters.
func NewInteractionResult(opts ...InteractionResultOption) *InteractionResult {
    c := &InteractionResult {
        ActionResults: [][]ActionResult,
        FinalHtml: "",
        FinalUrl: "",
        Screenshot: []byte{},
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Result from a single page action execution.
type ActionResult struct {
    // Zero-based index of the action in the sequence.
    ActionIndex uint `json:"action_index"`
    // The type of action that was executed.
    ActionType string `json:"action_type"`
    // Whether the action completed successfully.
    Success bool `json:"success"`
    // Action-specific return data (screenshot bytes, JS return value, scraped HTML).
    Data *map[string]interface{} `json:"data,omitempty"`
    // Error message if the action failed.
    Error *string `json:"error,omitempty"`
}


// The result of a single-page scrape operation.
type ScrapeResult struct {
    // The HTTP status code of the response.
    StatusCode uint16 `json:"status_code"`
    // The Content-Type header value.
    ContentType string `json:"content_type"`
    // The HTML body of the response.
    Html string `json:"html"`
    // The size of the response body in bytes.
    BodySize uint `json:"body_size"`
    // Extracted metadata from the page.
    Metadata PageMetadata `json:"metadata"`
    // Links found on the page.
    Links []LinkInfo `json:"links"`
    // Images found on the page.
    Images []ImageInfo `json:"images"`
    // Feed links found on the page.
    Feeds []FeedInfo `json:"feeds"`
    // JSON-LD entries found on the page.
    JsonLd []JsonLdEntry `json:"json_ld"`
    // Whether the URL is allowed by robots.txt.
    IsAllowed bool `json:"is_allowed"`
    // The crawl delay from robots.txt, in seconds.
    CrawlDelay *uint64 `json:"crawl_delay,omitempty"`
    // Whether a noindex directive was detected.
    NoindexDetected bool `json:"noindex_detected"`
    // Whether a nofollow directive was detected.
    NofollowDetected bool `json:"nofollow_detected"`
    // The X-Robots-Tag header value, if present.
    XRobotsTag *string `json:"x_robots_tag,omitempty"`
    // Whether the content is a PDF.
    IsPdf bool `json:"is_pdf"`
    // Whether the page was skipped (binary or PDF content).
    WasSkipped bool `json:"was_skipped"`
    // The detected character set encoding.
    DetectedCharset *string `json:"detected_charset,omitempty"`
    // Whether main_content_only was active during extraction.
    MainContentOnly bool `json:"main_content_only"`
    // Whether an authentication header was sent with the request.
    AuthHeaderSent bool `json:"auth_header_sent"`
    // Response metadata extracted from HTTP headers.
    ResponseMeta *ResponseMeta `json:"response_meta,omitempty"`
    // Downloaded assets from the page.
    Assets []DownloadedAsset `json:"assets"`
    // Whether the page content suggests JavaScript rendering is needed.
    JsRenderHint bool `json:"js_render_hint"`
    // Whether the browser fallback was used to fetch this page.
    BrowserUsed bool `json:"browser_used"`
    // Markdown conversion of the page content.
    Markdown *MarkdownResult `json:"markdown,omitempty"`
    // Structured data extracted by LLM. Populated when using LlmExtractor.
    ExtractedData *map[string]interface{} `json:"extracted_data,omitempty"`
    // Metadata about the LLM extraction pass (cost, tokens, model).
    ExtractionMeta *ExtractionMeta `json:"extraction_meta,omitempty"`
    // Screenshot of the page as PNG bytes. Populated when browser is used and capture_screenshot is enabled.
    Screenshot *[]byte `json:"screenshot,omitempty"`
    // Downloaded non-HTML document (PDF, DOCX, image, code, etc.).
    DownloadedDocument *DownloadedDocument `json:"downloaded_document,omitempty"`
}


// ScrapeResult option function
type ScrapeResultOption func(*ScrapeResult)

// WithStatusCode sets the status_code field.
func WithStatusCode(v uint16) ScrapeResultOption {
    return func(c *ScrapeResult) { c.StatusCode = v }
}

// WithContentType sets the content_type field.
func WithContentType(v string) ScrapeResultOption {
    return func(c *ScrapeResult) { c.ContentType = v }
}

// WithHtml sets the html field.
func WithHtml(v string) ScrapeResultOption {
    return func(c *ScrapeResult) { c.Html = v }
}

// WithBodySize sets the body_size field.
func WithBodySize(v uint) ScrapeResultOption {
    return func(c *ScrapeResult) { c.BodySize = v }
}

// WithMetadata sets the metadata field.
func WithMetadata(v PageMetadata) ScrapeResultOption {
    return func(c *ScrapeResult) { c.Metadata = v }
}

// WithLinks sets the links field.
func WithLinks(v []LinkInfo) ScrapeResultOption {
    return func(c *ScrapeResult) { c.Links = v }
}

// WithImages sets the images field.
func WithImages(v []ImageInfo) ScrapeResultOption {
    return func(c *ScrapeResult) { c.Images = v }
}

// WithFeeds sets the feeds field.
func WithFeeds(v []FeedInfo) ScrapeResultOption {
    return func(c *ScrapeResult) { c.Feeds = v }
}

// WithJsonLd sets the json_ld field.
func WithJsonLd(v []JsonLdEntry) ScrapeResultOption {
    return func(c *ScrapeResult) { c.JsonLd = v }
}

// WithIsAllowed sets the is_allowed field.
func WithIsAllowed(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.IsAllowed = v }
}

// WithCrawlDelay sets the crawl_delay field.
func WithCrawlDelay(v uint64) ScrapeResultOption {
    return func(c *ScrapeResult) { c.CrawlDelay = v }
}

// WithNoindexDetected sets the noindex_detected field.
func WithNoindexDetected(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.NoindexDetected = v }
}

// WithNofollowDetected sets the nofollow_detected field.
func WithNofollowDetected(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.NofollowDetected = v }
}

// WithXRobotsTag sets the x_robots_tag field.
func WithXRobotsTag(v string) ScrapeResultOption {
    return func(c *ScrapeResult) { c.XRobotsTag = v }
}

// WithIsPdf sets the is_pdf field.
func WithIsPdf(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.IsPdf = v }
}

// WithWasSkipped sets the was_skipped field.
func WithWasSkipped(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.WasSkipped = v }
}

// WithDetectedCharset sets the detected_charset field.
func WithDetectedCharset(v string) ScrapeResultOption {
    return func(c *ScrapeResult) { c.DetectedCharset = v }
}

// WithMainContentOnly sets the main_content_only field.
func WithMainContentOnly(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.MainContentOnly = v }
}

// WithAuthHeaderSent sets the auth_header_sent field.
func WithAuthHeaderSent(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.AuthHeaderSent = v }
}

// WithResponseMeta sets the response_meta field.
func WithResponseMeta(v ResponseMeta) ScrapeResultOption {
    return func(c *ScrapeResult) { c.ResponseMeta = v }
}

// WithAssets sets the assets field.
func WithAssets(v []DownloadedAsset) ScrapeResultOption {
    return func(c *ScrapeResult) { c.Assets = v }
}

// WithJsRenderHint sets the js_render_hint field.
func WithJsRenderHint(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.JsRenderHint = v }
}

// WithBrowserUsed sets the browser_used field.
func WithBrowserUsed(v bool) ScrapeResultOption {
    return func(c *ScrapeResult) { c.BrowserUsed = v }
}

// WithMarkdown sets the markdown field.
func WithMarkdown(v MarkdownResult) ScrapeResultOption {
    return func(c *ScrapeResult) { c.Markdown = v }
}

// WithExtractedData sets the extracted_data field.
func WithExtractedData(v map[string]interface{}) ScrapeResultOption {
    return func(c *ScrapeResult) { c.ExtractedData = v }
}

// WithExtractionMeta sets the extraction_meta field.
func WithExtractionMeta(v ExtractionMeta) ScrapeResultOption {
    return func(c *ScrapeResult) { c.ExtractionMeta = v }
}

// WithScreenshot sets the screenshot field.
func WithScreenshot(v []byte) ScrapeResultOption {
    return func(c *ScrapeResult) { c.Screenshot = v }
}

// WithDownloadedDocument sets the downloaded_document field.
func WithDownloadedDocument(v DownloadedDocument) ScrapeResultOption {
    return func(c *ScrapeResult) { c.DownloadedDocument = v }
}

// NewScrapeResult creates a ScrapeResult with optional parameters.
func NewScrapeResult(opts ...ScrapeResultOption) *ScrapeResult {
    c := &ScrapeResult {
        StatusCode: 0,
        ContentType: "",
        Html: "",
        BodySize: 0,
        Metadata: &PageMetadata{},
        Links: [][]LinkInfo,
        Images: [][]ImageInfo,
        Feeds: [][]FeedInfo,
        JsonLd: [][]JsonLdEntry,
        IsAllowed: false,
        CrawlDelay: 0,
        NoindexDetected: false,
        NofollowDetected: false,
        XRobotsTag: "",
        IsPdf: false,
        WasSkipped: false,
        DetectedCharset: "",
        MainContentOnly: false,
        AuthHeaderSent: false,
        ResponseMeta: &ResponseMeta{},
        Assets: [][]DownloadedAsset,
        JsRenderHint: false,
        BrowserUsed: false,
        Markdown: &MarkdownResult{},
        ExtractedData: "",
        ExtractionMeta: &ExtractionMeta{},
        Screenshot: []byte{},
        DownloadedDocument: &DownloadedDocument{},
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// The result of crawling a single page during a crawl operation.
type CrawlPageResult struct {
    // The original URL of the page.
    Url string `json:"url"`
    // The normalized URL of the page.
    NormalizedUrl string `json:"normalized_url"`
    // The HTTP status code of the response.
    StatusCode uint16 `json:"status_code"`
    // The Content-Type header value.
    ContentType string `json:"content_type"`
    // The HTML body of the response.
    Html string `json:"html"`
    // The size of the response body in bytes.
    BodySize uint `json:"body_size"`
    // Extracted metadata from the page.
    Metadata PageMetadata `json:"metadata"`
    // Links found on the page.
    Links []LinkInfo `json:"links"`
    // Images found on the page.
    Images []ImageInfo `json:"images"`
    // Feed links found on the page.
    Feeds []FeedInfo `json:"feeds"`
    // JSON-LD entries found on the page.
    JsonLd []JsonLdEntry `json:"json_ld"`
    // The depth of this page from the start URL.
    Depth uint `json:"depth"`
    // Whether this page is on the same domain as the start URL.
    StayedOnDomain bool `json:"stayed_on_domain"`
    // Whether this page was skipped (binary or PDF content).
    WasSkipped bool `json:"was_skipped"`
    // Whether the content is a PDF.
    IsPdf bool `json:"is_pdf"`
    // The detected character set encoding.
    DetectedCharset *string `json:"detected_charset,omitempty"`
    // Markdown conversion of the page content.
    Markdown *MarkdownResult `json:"markdown,omitempty"`
    // Structured data extracted by LLM. Populated when using LlmExtractor.
    ExtractedData *map[string]interface{} `json:"extracted_data,omitempty"`
    // Metadata about the LLM extraction pass (cost, tokens, model).
    ExtractionMeta *ExtractionMeta `json:"extraction_meta,omitempty"`
    // Downloaded non-HTML document (PDF, DOCX, image, code, etc.).
    DownloadedDocument *DownloadedDocument `json:"downloaded_document,omitempty"`
}


// CrawlPageResult option function
type CrawlPageResultOption func(*CrawlPageResult)

// WithUrl sets the url field.
func WithUrl(v string) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.Url = v }
}

// WithNormalizedUrl sets the normalized_url field.
func WithNormalizedUrl(v string) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.NormalizedUrl = v }
}

// WithStatusCode sets the status_code field.
func WithStatusCode(v uint16) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.StatusCode = v }
}

// WithContentType sets the content_type field.
func WithContentType(v string) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.ContentType = v }
}

// WithHtml sets the html field.
func WithHtml(v string) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.Html = v }
}

// WithBodySize sets the body_size field.
func WithBodySize(v uint) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.BodySize = v }
}

// WithMetadata sets the metadata field.
func WithMetadata(v PageMetadata) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.Metadata = v }
}

// WithLinks sets the links field.
func WithLinks(v []LinkInfo) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.Links = v }
}

// WithImages sets the images field.
func WithImages(v []ImageInfo) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.Images = v }
}

// WithFeeds sets the feeds field.
func WithFeeds(v []FeedInfo) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.Feeds = v }
}

// WithJsonLd sets the json_ld field.
func WithJsonLd(v []JsonLdEntry) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.JsonLd = v }
}

// WithDepth sets the depth field.
func WithDepth(v uint) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.Depth = v }
}

// WithStayedOnDomain sets the stayed_on_domain field.
func WithStayedOnDomain(v bool) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.StayedOnDomain = v }
}

// WithWasSkipped sets the was_skipped field.
func WithWasSkipped(v bool) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.WasSkipped = v }
}

// WithIsPdf sets the is_pdf field.
func WithIsPdf(v bool) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.IsPdf = v }
}

// WithDetectedCharset sets the detected_charset field.
func WithDetectedCharset(v string) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.DetectedCharset = v }
}

// WithMarkdown sets the markdown field.
func WithMarkdown(v MarkdownResult) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.Markdown = v }
}

// WithExtractedData sets the extracted_data field.
func WithExtractedData(v map[string]interface{}) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.ExtractedData = v }
}

// WithExtractionMeta sets the extraction_meta field.
func WithExtractionMeta(v ExtractionMeta) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.ExtractionMeta = v }
}

// WithDownloadedDocument sets the downloaded_document field.
func WithDownloadedDocument(v DownloadedDocument) CrawlPageResultOption {
    return func(c *CrawlPageResult) { c.DownloadedDocument = v }
}

// NewCrawlPageResult creates a CrawlPageResult with optional parameters.
func NewCrawlPageResult(opts ...CrawlPageResultOption) *CrawlPageResult {
    c := &CrawlPageResult {
        Url: "",
        NormalizedUrl: "",
        StatusCode: 0,
        ContentType: "",
        Html: "",
        BodySize: 0,
        Metadata: &PageMetadata{},
        Links: [][]LinkInfo,
        Images: [][]ImageInfo,
        Feeds: [][]FeedInfo,
        JsonLd: [][]JsonLdEntry,
        Depth: 0,
        StayedOnDomain: false,
        WasSkipped: false,
        IsPdf: false,
        DetectedCharset: "",
        Markdown: &MarkdownResult{},
        ExtractedData: "",
        ExtractionMeta: &ExtractionMeta{},
        DownloadedDocument: &DownloadedDocument{},
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// The result of a multi-page crawl operation.
type CrawlResult struct {
    // The list of crawled pages.
    Pages []CrawlPageResult `json:"pages"`
    // The final URL after following redirects.
    FinalUrl string `json:"final_url"`
    // The number of redirects followed.
    RedirectCount uint `json:"redirect_count"`
    // Whether any page was skipped during crawling.
    WasSkipped bool `json:"was_skipped"`
    // An error message, if the crawl encountered an issue.
    Error *string `json:"error,omitempty"`
    // Cookies collected during the crawl.
    Cookies []CookieInfo `json:"cookies"`
}


// CrawlResult option function
type CrawlResultOption func(*CrawlResult)

// WithPages sets the pages field.
func WithPages(v []CrawlPageResult) CrawlResultOption {
    return func(c *CrawlResult) { c.Pages = v }
}

// WithFinalUrl sets the final_url field.
func WithFinalUrl(v string) CrawlResultOption {
    return func(c *CrawlResult) { c.FinalUrl = v }
}

// WithRedirectCount sets the redirect_count field.
func WithRedirectCount(v uint) CrawlResultOption {
    return func(c *CrawlResult) { c.RedirectCount = v }
}

// WithWasSkipped sets the was_skipped field.
func WithWasSkipped(v bool) CrawlResultOption {
    return func(c *CrawlResult) { c.WasSkipped = v }
}

// WithError sets the error field.
func WithError(v string) CrawlResultOption {
    return func(c *CrawlResult) { c.Error = v }
}

// WithCookies sets the cookies field.
func WithCookies(v []CookieInfo) CrawlResultOption {
    return func(c *CrawlResult) { c.Cookies = v }
}

// NewCrawlResult creates a CrawlResult with optional parameters.
func NewCrawlResult(opts ...CrawlResultOption) *CrawlResult {
    c := &CrawlResult {
        Pages: [][]CrawlPageResult,
        FinalUrl: "",
        RedirectCount: 0,
        WasSkipped: false,
        Error: "",
        Cookies: [][]CookieInfo,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// A URL entry from a sitemap.
type SitemapUrl struct {
    // The URL.
    Url string `json:"url"`
    // The last modification date, if present.
    Lastmod *string `json:"lastmod,omitempty"`
    // The change frequency, if present.
    Changefreq *string `json:"changefreq,omitempty"`
    // The priority, if present.
    Priority *string `json:"priority,omitempty"`
}


// SitemapUrl option function
type SitemapUrlOption func(*SitemapUrl)

// WithUrl sets the url field.
func WithUrl(v string) SitemapUrlOption {
    return func(c *SitemapUrl) { c.Url = v }
}

// WithLastmod sets the lastmod field.
func WithLastmod(v string) SitemapUrlOption {
    return func(c *SitemapUrl) { c.Lastmod = v }
}

// WithChangefreq sets the changefreq field.
func WithChangefreq(v string) SitemapUrlOption {
    return func(c *SitemapUrl) { c.Changefreq = v }
}

// WithPriority sets the priority field.
func WithPriority(v string) SitemapUrlOption {
    return func(c *SitemapUrl) { c.Priority = v }
}

// NewSitemapUrl creates a SitemapUrl with optional parameters.
func NewSitemapUrl(opts ...SitemapUrlOption) *SitemapUrl {
    c := &SitemapUrl {
        Url: "",
        Lastmod: "",
        Changefreq: "",
        Priority: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// The result of a map operation, containing discovered URLs.
type MapResult struct {
    // The list of discovered URLs.
    Urls []SitemapUrl `json:"urls"`
}


// MapResult option function
type MapResultOption func(*MapResult)

// WithUrls sets the urls field.
func WithUrls(v []SitemapUrl) MapResultOption {
    return func(c *MapResult) { c.Urls = v }
}

// NewMapResult creates a MapResult with optional parameters.
func NewMapResult(opts ...MapResultOption) *MapResult {
    c := &MapResult {
        Urls: [][]SitemapUrl,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Rich markdown conversion result from HTML processing.
type MarkdownResult struct {
    // Converted markdown text.
    Content string `json:"content"`
    // Structured document tree with semantic nodes.
    DocumentStructure *map[string]interface{} `json:"document_structure,omitempty"`
    // Extracted tables with structured cell data.
    Tables []map[string]interface{} `json:"tables"`
    // Non-fatal processing warnings.
    Warnings []string `json:"warnings"`
    // Content with links replaced by numbered citations.
    Citations *CitationResult `json:"citations,omitempty"`
    // Content-filtered markdown optimized for LLM consumption.
    FitContent *string `json:"fit_content,omitempty"`
}


// MarkdownResult option function
type MarkdownResultOption func(*MarkdownResult)

// WithContent sets the content field.
func WithContent(v string) MarkdownResultOption {
    return func(c *MarkdownResult) { c.Content = v }
}

// WithDocumentStructure sets the document_structure field.
func WithDocumentStructure(v map[string]interface{}) MarkdownResultOption {
    return func(c *MarkdownResult) { c.DocumentStructure = v }
}

// WithTables sets the tables field.
func WithTables(v []map[string]interface{}) MarkdownResultOption {
    return func(c *MarkdownResult) { c.Tables = v }
}

// WithWarnings sets the warnings field.
func WithWarnings(v []string) MarkdownResultOption {
    return func(c *MarkdownResult) { c.Warnings = v }
}

// WithCitations sets the citations field.
func WithCitations(v CitationResult) MarkdownResultOption {
    return func(c *MarkdownResult) { c.Citations = v }
}

// WithFitContent sets the fit_content field.
func WithFitContent(v string) MarkdownResultOption {
    return func(c *MarkdownResult) { c.FitContent = v }
}

// NewMarkdownResult creates a MarkdownResult with optional parameters.
func NewMarkdownResult(opts ...MarkdownResultOption) *MarkdownResult {
    c := &MarkdownResult {
        Content: "",
        DocumentStructure: "",
        Tables: [][]map[string]interface{},
        Warnings: [][]string,
        Citations: &CitationResult{},
        FitContent: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Cached page data for HTTP response caching.
type CachedPage struct {
    Url string `json:"url"`
    StatusCode uint16 `json:"status_code"`
    ContentType string `json:"content_type"`
    Body string `json:"body"`
    Etag *string `json:"etag,omitempty"`
    LastModified *string `json:"last_modified,omitempty"`
    CachedAt uint64 `json:"cached_at"`
}


// Information about a link found on a page.
type LinkInfo struct {
    // The resolved URL of the link.
    Url string `json:"url"`
    // The visible text of the link.
    Text string `json:"text"`
    // The classification of the link.
    LinkType LinkType `json:"link_type"`
    // The `rel` attribute value, if present.
    Rel *string `json:"rel,omitempty"`
    // Whether the link has `rel="nofollow"`.
    Nofollow bool `json:"nofollow"`
}


// LinkInfo option function
type LinkInfoOption func(*LinkInfo)

// WithUrl sets the url field.
func WithUrl(v string) LinkInfoOption {
    return func(c *LinkInfo) { c.Url = v }
}

// WithText sets the text field.
func WithText(v string) LinkInfoOption {
    return func(c *LinkInfo) { c.Text = v }
}

// WithLinkType sets the link_type field.
func WithLinkType(v LinkType) LinkInfoOption {
    return func(c *LinkInfo) { c.LinkType = v }
}

// WithRel sets the rel field.
func WithRel(v string) LinkInfoOption {
    return func(c *LinkInfo) { c.Rel = v }
}

// WithNofollow sets the nofollow field.
func WithNofollow(v bool) LinkInfoOption {
    return func(c *LinkInfo) { c.Nofollow = v }
}

// NewLinkInfo creates a LinkInfo with optional parameters.
func NewLinkInfo(opts ...LinkInfoOption) *LinkInfo {
    c := &LinkInfo {
        Url: "",
        Text: "",
        LinkType: &LinkType{},
        Rel: "",
        Nofollow: false,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Information about an image found on a page.
type ImageInfo struct {
    // The image URL.
    Url string `json:"url"`
    // The alt text, if present.
    Alt *string `json:"alt,omitempty"`
    // The width attribute, if present and parseable.
    Width *uint32 `json:"width,omitempty"`
    // The height attribute, if present and parseable.
    Height *uint32 `json:"height,omitempty"`
    // The source of the image reference.
    Source ImageSource `json:"source"`
}


// ImageInfo option function
type ImageInfoOption func(*ImageInfo)

// WithUrl sets the url field.
func WithUrl(v string) ImageInfoOption {
    return func(c *ImageInfo) { c.Url = v }
}

// WithAlt sets the alt field.
func WithAlt(v string) ImageInfoOption {
    return func(c *ImageInfo) { c.Alt = v }
}

// WithWidth sets the width field.
func WithWidth(v uint32) ImageInfoOption {
    return func(c *ImageInfo) { c.Width = v }
}

// WithHeight sets the height field.
func WithHeight(v uint32) ImageInfoOption {
    return func(c *ImageInfo) { c.Height = v }
}

// WithSource sets the source field.
func WithSource(v ImageSource) ImageInfoOption {
    return func(c *ImageInfo) { c.Source = v }
}

// NewImageInfo creates a ImageInfo with optional parameters.
func NewImageInfo(opts ...ImageInfoOption) *ImageInfo {
    c := &ImageInfo {
        Url: "",
        Alt: "",
        Width: 0,
        Height: 0,
        Source: &ImageSource{},
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Information about a feed link found on a page.
type FeedInfo struct {
    // The feed URL.
    Url string `json:"url"`
    // The feed title, if present.
    Title *string `json:"title,omitempty"`
    // The type of feed.
    FeedType FeedType `json:"feed_type"`
}


// FeedInfo option function
type FeedInfoOption func(*FeedInfo)

// WithUrl sets the url field.
func WithUrl(v string) FeedInfoOption {
    return func(c *FeedInfo) { c.Url = v }
}

// WithTitle sets the title field.
func WithTitle(v string) FeedInfoOption {
    return func(c *FeedInfo) { c.Title = v }
}

// WithFeedType sets the feed_type field.
func WithFeedType(v FeedType) FeedInfoOption {
    return func(c *FeedInfo) { c.FeedType = v }
}

// NewFeedInfo creates a FeedInfo with optional parameters.
func NewFeedInfo(opts ...FeedInfoOption) *FeedInfo {
    c := &FeedInfo {
        Url: "",
        Title: "",
        FeedType: &FeedType{},
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// A JSON-LD structured data entry found on a page.
type JsonLdEntry struct {
    // The `@type` value from the JSON-LD object.
    SchemaType string `json:"schema_type"`
    // The `name` value, if present.
    Name *string `json:"name,omitempty"`
    // The raw JSON-LD string.
    Raw string `json:"raw"`
}


// JsonLdEntry option function
type JsonLdEntryOption func(*JsonLdEntry)

// WithSchemaType sets the schema_type field.
func WithSchemaType(v string) JsonLdEntryOption {
    return func(c *JsonLdEntry) { c.SchemaType = v }
}

// WithName sets the name field.
func WithName(v string) JsonLdEntryOption {
    return func(c *JsonLdEntry) { c.Name = v }
}

// WithRaw sets the raw field.
func WithRaw(v string) JsonLdEntryOption {
    return func(c *JsonLdEntry) { c.Raw = v }
}

// NewJsonLdEntry creates a JsonLdEntry with optional parameters.
func NewJsonLdEntry(opts ...JsonLdEntryOption) *JsonLdEntry {
    c := &JsonLdEntry {
        SchemaType: "",
        Name: "",
        Raw: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Information about an HTTP cookie received from a response.
type CookieInfo struct {
    // The cookie name.
    Name string `json:"name"`
    // The cookie value.
    Value string `json:"value"`
    // The cookie domain, if specified.
    Domain *string `json:"domain,omitempty"`
    // The cookie path, if specified.
    Path *string `json:"path,omitempty"`
}


// CookieInfo option function
type CookieInfoOption func(*CookieInfo)

// WithName sets the name field.
func WithName(v string) CookieInfoOption {
    return func(c *CookieInfo) { c.Name = v }
}

// WithValue sets the value field.
func WithValue(v string) CookieInfoOption {
    return func(c *CookieInfo) { c.Value = v }
}

// WithDomain sets the domain field.
func WithDomain(v string) CookieInfoOption {
    return func(c *CookieInfo) { c.Domain = v }
}

// WithPath sets the path field.
func WithPath(v string) CookieInfoOption {
    return func(c *CookieInfo) { c.Path = v }
}

// NewCookieInfo creates a CookieInfo with optional parameters.
func NewCookieInfo(opts ...CookieInfoOption) *CookieInfo {
    c := &CookieInfo {
        Name: "",
        Value: "",
        Domain: "",
        Path: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// A downloaded asset from a page.
type DownloadedAsset struct {
    // The original URL of the asset.
    Url string `json:"url"`
    // The SHA-256 content hash of the asset.
    ContentHash string `json:"content_hash"`
    // The MIME type from the Content-Type header.
    MimeType *string `json:"mime_type,omitempty"`
    // The size of the asset in bytes.
    Size uint `json:"size"`
    // The category of the asset.
    AssetCategory AssetCategory `json:"asset_category"`
    // The HTML tag that referenced this asset (e.g., "link", "script", "img").
    HtmlTag *string `json:"html_tag,omitempty"`
}


// DownloadedAsset option function
type DownloadedAssetOption func(*DownloadedAsset)

// WithUrl sets the url field.
func WithUrl(v string) DownloadedAssetOption {
    return func(c *DownloadedAsset) { c.Url = v }
}

// WithContentHash sets the content_hash field.
func WithContentHash(v string) DownloadedAssetOption {
    return func(c *DownloadedAsset) { c.ContentHash = v }
}

// WithMimeType sets the mime_type field.
func WithMimeType(v string) DownloadedAssetOption {
    return func(c *DownloadedAsset) { c.MimeType = v }
}

// WithSize sets the size field.
func WithSize(v uint) DownloadedAssetOption {
    return func(c *DownloadedAsset) { c.Size = v }
}

// WithAssetCategory sets the asset_category field.
func WithAssetCategory(v AssetCategory) DownloadedAssetOption {
    return func(c *DownloadedAsset) { c.AssetCategory = v }
}

// WithHtmlTag sets the html_tag field.
func WithHtmlTag(v string) DownloadedAssetOption {
    return func(c *DownloadedAsset) { c.HtmlTag = v }
}

// NewDownloadedAsset creates a DownloadedAsset with optional parameters.
func NewDownloadedAsset(opts ...DownloadedAssetOption) *DownloadedAsset {
    c := &DownloadedAsset {
        Url: "",
        ContentHash: "",
        MimeType: "",
        Size: 0,
        AssetCategory: &AssetCategory{},
        HtmlTag: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Article metadata extracted from `article:*` Open Graph tags.
type ArticleMetadata struct {
    // The article publication time.
    PublishedTime *string `json:"published_time,omitempty"`
    // The article modification time.
    ModifiedTime *string `json:"modified_time,omitempty"`
    // The article author.
    Author *string `json:"author,omitempty"`
    // The article section.
    Section *string `json:"section,omitempty"`
    // The article tags.
    Tags []string `json:"tags"`
}


// ArticleMetadata option function
type ArticleMetadataOption func(*ArticleMetadata)

// WithPublishedTime sets the published_time field.
func WithPublishedTime(v string) ArticleMetadataOption {
    return func(c *ArticleMetadata) { c.PublishedTime = v }
}

// WithModifiedTime sets the modified_time field.
func WithModifiedTime(v string) ArticleMetadataOption {
    return func(c *ArticleMetadata) { c.ModifiedTime = v }
}

// WithAuthor sets the author field.
func WithAuthor(v string) ArticleMetadataOption {
    return func(c *ArticleMetadata) { c.Author = v }
}

// WithSection sets the section field.
func WithSection(v string) ArticleMetadataOption {
    return func(c *ArticleMetadata) { c.Section = v }
}

// WithTags sets the tags field.
func WithTags(v []string) ArticleMetadataOption {
    return func(c *ArticleMetadata) { c.Tags = v }
}

// NewArticleMetadata creates a ArticleMetadata with optional parameters.
func NewArticleMetadata(opts ...ArticleMetadataOption) *ArticleMetadata {
    c := &ArticleMetadata {
        PublishedTime: "",
        ModifiedTime: "",
        Author: "",
        Section: "",
        Tags: [][]string,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// An hreflang alternate link entry.
type HreflangEntry struct {
    // The language code (e.g., "en", "fr", "x-default").
    Lang string `json:"lang"`
    // The URL for this language variant.
    Url string `json:"url"`
}


// HreflangEntry option function
type HreflangEntryOption func(*HreflangEntry)

// WithLang sets the lang field.
func WithLang(v string) HreflangEntryOption {
    return func(c *HreflangEntry) { c.Lang = v }
}

// WithUrl sets the url field.
func WithUrl(v string) HreflangEntryOption {
    return func(c *HreflangEntry) { c.Url = v }
}

// NewHreflangEntry creates a HreflangEntry with optional parameters.
func NewHreflangEntry(opts ...HreflangEntryOption) *HreflangEntry {
    c := &HreflangEntry {
        Lang: "",
        Url: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Information about a favicon or icon link.
type FaviconInfo struct {
    // The icon URL.
    Url string `json:"url"`
    // The `rel` attribute (e.g., "icon", "apple-touch-icon").
    Rel string `json:"rel"`
    // The `sizes` attribute, if present.
    Sizes *string `json:"sizes,omitempty"`
    // The MIME type, if present.
    MimeType *string `json:"mime_type,omitempty"`
}


// FaviconInfo option function
type FaviconInfoOption func(*FaviconInfo)

// WithUrl sets the url field.
func WithUrl(v string) FaviconInfoOption {
    return func(c *FaviconInfo) { c.Url = v }
}

// WithRel sets the rel field.
func WithRel(v string) FaviconInfoOption {
    return func(c *FaviconInfo) { c.Rel = v }
}

// WithSizes sets the sizes field.
func WithSizes(v string) FaviconInfoOption {
    return func(c *FaviconInfo) { c.Sizes = v }
}

// WithMimeType sets the mime_type field.
func WithMimeType(v string) FaviconInfoOption {
    return func(c *FaviconInfo) { c.MimeType = v }
}

// NewFaviconInfo creates a FaviconInfo with optional parameters.
func NewFaviconInfo(opts ...FaviconInfoOption) *FaviconInfo {
    c := &FaviconInfo {
        Url: "",
        Rel: "",
        Sizes: "",
        MimeType: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// A heading element extracted from the page.
type HeadingInfo struct {
    // The heading level (1-6).
    Level uint8 `json:"level"`
    // The heading text content.
    Text string `json:"text"`
}


// HeadingInfo option function
type HeadingInfoOption func(*HeadingInfo)

// WithLevel sets the level field.
func WithLevel(v uint8) HeadingInfoOption {
    return func(c *HeadingInfo) { c.Level = v }
}

// WithText sets the text field.
func WithText(v string) HeadingInfoOption {
    return func(c *HeadingInfo) { c.Text = v }
}

// NewHeadingInfo creates a HeadingInfo with optional parameters.
func NewHeadingInfo(opts ...HeadingInfoOption) *HeadingInfo {
    c := &HeadingInfo {
        Level: 0,
        Text: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Response metadata extracted from HTTP headers.
type ResponseMeta struct {
    // The ETag header value.
    Etag *string `json:"etag,omitempty"`
    // The Last-Modified header value.
    LastModified *string `json:"last_modified,omitempty"`
    // The Cache-Control header value.
    CacheControl *string `json:"cache_control,omitempty"`
    // The Server header value.
    Server *string `json:"server,omitempty"`
    // The X-Powered-By header value.
    XPoweredBy *string `json:"x_powered_by,omitempty"`
    // The Content-Language header value.
    ContentLanguage *string `json:"content_language,omitempty"`
    // The Content-Encoding header value.
    ContentEncoding *string `json:"content_encoding,omitempty"`
}


// ResponseMeta option function
type ResponseMetaOption func(*ResponseMeta)

// WithEtag sets the etag field.
func WithEtag(v string) ResponseMetaOption {
    return func(c *ResponseMeta) { c.Etag = v }
}

// WithLastModified sets the last_modified field.
func WithLastModified(v string) ResponseMetaOption {
    return func(c *ResponseMeta) { c.LastModified = v }
}

// WithCacheControl sets the cache_control field.
func WithCacheControl(v string) ResponseMetaOption {
    return func(c *ResponseMeta) { c.CacheControl = v }
}

// WithServer sets the server field.
func WithServer(v string) ResponseMetaOption {
    return func(c *ResponseMeta) { c.Server = v }
}

// WithXPoweredBy sets the x_powered_by field.
func WithXPoweredBy(v string) ResponseMetaOption {
    return func(c *ResponseMeta) { c.XPoweredBy = v }
}

// WithContentLanguage sets the content_language field.
func WithContentLanguage(v string) ResponseMetaOption {
    return func(c *ResponseMeta) { c.ContentLanguage = v }
}

// WithContentEncoding sets the content_encoding field.
func WithContentEncoding(v string) ResponseMetaOption {
    return func(c *ResponseMeta) { c.ContentEncoding = v }
}

// NewResponseMeta creates a ResponseMeta with optional parameters.
func NewResponseMeta(opts ...ResponseMetaOption) *ResponseMeta {
    c := &ResponseMeta {
        Etag: "",
        LastModified: "",
        CacheControl: "",
        Server: "",
        XPoweredBy: "",
        ContentLanguage: "",
        ContentEncoding: "",
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Metadata extracted from an HTML page's `<meta>` tags and `<title>` element.
type PageMetadata struct {
    // The page title from the `<title>` element.
    Title *string `json:"title,omitempty"`
    // The meta description.
    Description *string `json:"description,omitempty"`
    // The canonical URL from `<link rel="canonical">`.
    CanonicalUrl *string `json:"canonical_url,omitempty"`
    // Keywords from `<meta name="keywords">`.
    Keywords *string `json:"keywords,omitempty"`
    // Author from `<meta name="author">`.
    Author *string `json:"author,omitempty"`
    // Viewport content from `<meta name="viewport">`.
    Viewport *string `json:"viewport,omitempty"`
    // Theme color from `<meta name="theme-color">`.
    ThemeColor *string `json:"theme_color,omitempty"`
    // Generator from `<meta name="generator">`.
    Generator *string `json:"generator,omitempty"`
    // Robots content from `<meta name="robots">`.
    Robots *string `json:"robots,omitempty"`
    // The `lang` attribute from the `<html>` element.
    HtmlLang *string `json:"html_lang,omitempty"`
    // The `dir` attribute from the `<html>` element.
    HtmlDir *string `json:"html_dir,omitempty"`
    // Open Graph title.
    OgTitle *string `json:"og_title,omitempty"`
    // Open Graph type.
    OgType *string `json:"og_type,omitempty"`
    // Open Graph image URL.
    OgImage *string `json:"og_image,omitempty"`
    // Open Graph description.
    OgDescription *string `json:"og_description,omitempty"`
    // Open Graph URL.
    OgUrl *string `json:"og_url,omitempty"`
    // Open Graph site name.
    OgSiteName *string `json:"og_site_name,omitempty"`
    // Open Graph locale.
    OgLocale *string `json:"og_locale,omitempty"`
    // Open Graph video URL.
    OgVideo *string `json:"og_video,omitempty"`
    // Open Graph audio URL.
    OgAudio *string `json:"og_audio,omitempty"`
    // Open Graph locale alternates.
    OgLocaleAlternates *[]string `json:"og_locale_alternates,omitempty"`
    // Twitter card type.
    TwitterCard *string `json:"twitter_card,omitempty"`
    // Twitter title.
    TwitterTitle *string `json:"twitter_title,omitempty"`
    // Twitter description.
    TwitterDescription *string `json:"twitter_description,omitempty"`
    // Twitter image URL.
    TwitterImage *string `json:"twitter_image,omitempty"`
    // Twitter site handle.
    TwitterSite *string `json:"twitter_site,omitempty"`
    // Twitter creator handle.
    TwitterCreator *string `json:"twitter_creator,omitempty"`
    // Dublin Core title.
    DcTitle *string `json:"dc_title,omitempty"`
    // Dublin Core creator.
    DcCreator *string `json:"dc_creator,omitempty"`
    // Dublin Core subject.
    DcSubject *string `json:"dc_subject,omitempty"`
    // Dublin Core description.
    DcDescription *string `json:"dc_description,omitempty"`
    // Dublin Core publisher.
    DcPublisher *string `json:"dc_publisher,omitempty"`
    // Dublin Core date.
    DcDate *string `json:"dc_date,omitempty"`
    // Dublin Core type.
    DcType *string `json:"dc_type,omitempty"`
    // Dublin Core format.
    DcFormat *string `json:"dc_format,omitempty"`
    // Dublin Core identifier.
    DcIdentifier *string `json:"dc_identifier,omitempty"`
    // Dublin Core language.
    DcLanguage *string `json:"dc_language,omitempty"`
    // Dublin Core rights.
    DcRights *string `json:"dc_rights,omitempty"`
    // Article metadata from `article:*` Open Graph tags.
    Article *ArticleMetadata `json:"article,omitempty"`
    // Hreflang alternate links.
    Hreflangs *[]HreflangEntry `json:"hreflangs,omitempty"`
    // Favicon and icon links.
    Favicons *[]FaviconInfo `json:"favicons,omitempty"`
    // Heading elements (h1-h6).
    Headings *[]HeadingInfo `json:"headings,omitempty"`
    // Computed word count of the page body text.
    WordCount *uint `json:"word_count,omitempty"`
}


// PageMetadata option function
type PageMetadataOption func(*PageMetadata)

// WithTitle sets the title field.
func WithTitle(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.Title = v }
}

// WithDescription sets the description field.
func WithDescription(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.Description = v }
}

// WithCanonicalUrl sets the canonical_url field.
func WithCanonicalUrl(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.CanonicalUrl = v }
}

// WithKeywords sets the keywords field.
func WithKeywords(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.Keywords = v }
}

// WithAuthor sets the author field.
func WithAuthor(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.Author = v }
}

// WithViewport sets the viewport field.
func WithViewport(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.Viewport = v }
}

// WithThemeColor sets the theme_color field.
func WithThemeColor(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.ThemeColor = v }
}

// WithGenerator sets the generator field.
func WithGenerator(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.Generator = v }
}

// WithRobots sets the robots field.
func WithRobots(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.Robots = v }
}

// WithHtmlLang sets the html_lang field.
func WithHtmlLang(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.HtmlLang = v }
}

// WithHtmlDir sets the html_dir field.
func WithHtmlDir(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.HtmlDir = v }
}

// WithOgTitle sets the og_title field.
func WithOgTitle(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgTitle = v }
}

// WithOgType sets the og_type field.
func WithOgType(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgType = v }
}

// WithOgImage sets the og_image field.
func WithOgImage(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgImage = v }
}

// WithOgDescription sets the og_description field.
func WithOgDescription(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgDescription = v }
}

// WithOgUrl sets the og_url field.
func WithOgUrl(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgUrl = v }
}

// WithOgSiteName sets the og_site_name field.
func WithOgSiteName(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgSiteName = v }
}

// WithOgLocale sets the og_locale field.
func WithOgLocale(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgLocale = v }
}

// WithOgVideo sets the og_video field.
func WithOgVideo(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgVideo = v }
}

// WithOgAudio sets the og_audio field.
func WithOgAudio(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgAudio = v }
}

// WithOgLocaleAlternates sets the og_locale_alternates field.
func WithOgLocaleAlternates(v []string) PageMetadataOption {
    return func(c *PageMetadata) { c.OgLocaleAlternates = v }
}

// WithTwitterCard sets the twitter_card field.
func WithTwitterCard(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.TwitterCard = v }
}

// WithTwitterTitle sets the twitter_title field.
func WithTwitterTitle(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.TwitterTitle = v }
}

// WithTwitterDescription sets the twitter_description field.
func WithTwitterDescription(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.TwitterDescription = v }
}

// WithTwitterImage sets the twitter_image field.
func WithTwitterImage(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.TwitterImage = v }
}

// WithTwitterSite sets the twitter_site field.
func WithTwitterSite(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.TwitterSite = v }
}

// WithTwitterCreator sets the twitter_creator field.
func WithTwitterCreator(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.TwitterCreator = v }
}

// WithDcTitle sets the dc_title field.
func WithDcTitle(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcTitle = v }
}

// WithDcCreator sets the dc_creator field.
func WithDcCreator(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcCreator = v }
}

// WithDcSubject sets the dc_subject field.
func WithDcSubject(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcSubject = v }
}

// WithDcDescription sets the dc_description field.
func WithDcDescription(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcDescription = v }
}

// WithDcPublisher sets the dc_publisher field.
func WithDcPublisher(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcPublisher = v }
}

// WithDcDate sets the dc_date field.
func WithDcDate(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcDate = v }
}

// WithDcType sets the dc_type field.
func WithDcType(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcType = v }
}

// WithDcFormat sets the dc_format field.
func WithDcFormat(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcFormat = v }
}

// WithDcIdentifier sets the dc_identifier field.
func WithDcIdentifier(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcIdentifier = v }
}

// WithDcLanguage sets the dc_language field.
func WithDcLanguage(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcLanguage = v }
}

// WithDcRights sets the dc_rights field.
func WithDcRights(v string) PageMetadataOption {
    return func(c *PageMetadata) { c.DcRights = v }
}

// WithArticle sets the article field.
func WithArticle(v ArticleMetadata) PageMetadataOption {
    return func(c *PageMetadata) { c.Article = v }
}

// WithHreflangs sets the hreflangs field.
func WithHreflangs(v []HreflangEntry) PageMetadataOption {
    return func(c *PageMetadata) { c.Hreflangs = v }
}

// WithFavicons sets the favicons field.
func WithFavicons(v []FaviconInfo) PageMetadataOption {
    return func(c *PageMetadata) { c.Favicons = v }
}

// WithHeadings sets the headings field.
func WithHeadings(v []HeadingInfo) PageMetadataOption {
    return func(c *PageMetadata) { c.Headings = v }
}

// WithWordCount sets the word_count field.
func WithWordCount(v uint) PageMetadataOption {
    return func(c *PageMetadata) { c.WordCount = v }
}

// NewPageMetadata creates a PageMetadata with optional parameters.
func NewPageMetadata(opts ...PageMetadataOption) *PageMetadata {
    c := &PageMetadata {
        Title: "",
        Description: "",
        CanonicalUrl: "",
        Keywords: "",
        Author: "",
        Viewport: "",
        ThemeColor: "",
        Generator: "",
        Robots: "",
        HtmlLang: "",
        HtmlDir: "",
        OgTitle: "",
        OgType: "",
        OgImage: "",
        OgDescription: "",
        OgUrl: "",
        OgSiteName: "",
        OgLocale: "",
        OgVideo: "",
        OgAudio: "",
        OgLocaleAlternates: [][]string,
        TwitterCard: "",
        TwitterTitle: "",
        TwitterDescription: "",
        TwitterImage: "",
        TwitterSite: "",
        TwitterCreator: "",
        DcTitle: "",
        DcCreator: "",
        DcSubject: "",
        DcDescription: "",
        DcPublisher: "",
        DcDate: "",
        DcType: "",
        DcFormat: "",
        DcIdentifier: "",
        DcLanguage: "",
        DcRights: "",
        Article: &ArticleMetadata{},
        Hreflangs: [][]HreflangEntry,
        Favicons: [][]FaviconInfo,
        Headings: [][]HeadingInfo,
        WordCount: 0,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// Result of citation conversion.
type CitationResult struct {
    // Markdown with links replaced by numbered citations.
    Content string `json:"content"`
    // Numbered reference list: (index, url, text).
    References []CitationReference `json:"references"`
}


// CitationResult option function
type CitationResultOption func(*CitationResult)

// WithContent sets the content field.
func WithContent(v string) CitationResultOption {
    return func(c *CitationResult) { c.Content = v }
}

// WithReferences sets the references field.
func WithReferences(v []CitationReference) CitationResultOption {
    return func(c *CitationResult) { c.References = v }
}

// NewCitationResult creates a CitationResult with optional parameters.
func NewCitationResult(opts ...CitationResultOption) *CitationResult {
    c := &CitationResult {
        Content: "",
        References: [][]CitationReference,
    }
    for _, opt := range opts {
        opt(c)
    }
    return c
}


// CitationReference is a type.
type CitationReference struct {
    Index uint `json:"index"`
    Url string `json:"url"`
    Text string `json:"text"`
}


// Opaque handle to a configured crawl engine.
//
// Constructed via [`create_engine`] with an optional [`CrawlConfig`].
// All default trait implementations (BFS strategy, in-memory frontier,
// per-domain throttle, etc.) are used internally.
type CrawlEngineHandle struct {
}


// Result from a single URL in a batch scrape operation.
type BatchScrapeResult struct {
    // The URL that was scraped.
    Url string `json:"url"`
    // The scrape result, if successful.
    Result *ScrapeResult `json:"result,omitempty"`
    // The error message, if the scrape failed.
    Error *string `json:"error,omitempty"`
}


// Result from a single URL in a batch crawl operation.
type BatchCrawlResult struct {
    // The seed URL that was crawled.
    Url string `json:"url"`
    // The crawl result, if successful.
    Result *CrawlResult `json:"result,omitempty"`
    // The error message, if the crawl failed.
    Error *string `json:"error,omitempty"`
}


// Create a new crawl engine with the given configuration.
//
// If `config` is `None`, uses [`CrawlConfig::default()`].
// Returns an error if the configuration is invalid.
func CreateEngine(config *CrawlConfig) (*CrawlEngineHandle, error) {
    jsonBytes, err := json.Marshal(config)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cConfig := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cConfig))

    ptr := C.kcrawl_create_engine(cConfig)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalCrawlEngineHandle(ptr), nil
}


// Scrape a single URL, returning extracted page data.
func Scrape(engine CrawlEngineHandle, url string) (*ScrapeResult, error) {
    jsonBytes, err := json.Marshal(engine)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cEngine := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cEngine))

    cUrl := C.CString(url)
    defer C.free(unsafe.Pointer(cUrl))

    ptr := C.kcrawl_scrape(cEngine, cUrl)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalScrapeResult(ptr), nil
}


// Crawl a website starting from `url`, following links up to the configured depth.
func Crawl(engine CrawlEngineHandle, url string) (*CrawlResult, error) {
    jsonBytes, err := json.Marshal(engine)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cEngine := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cEngine))

    cUrl := C.CString(url)
    defer C.free(unsafe.Pointer(cUrl))

    ptr := C.kcrawl_crawl(cEngine, cUrl)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalCrawlResult(ptr), nil
}


// Discover all pages on a website by following links and sitemaps.
func MapUrls(engine CrawlEngineHandle, url string) (*MapResult, error) {
    jsonBytes, err := json.Marshal(engine)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cEngine := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cEngine))

    cUrl := C.CString(url)
    defer C.free(unsafe.Pointer(cUrl))

    ptr := C.kcrawl_map_urls(cEngine, cUrl)
    if err := lastError(); err != nil {
        return nil, err
    }
    return unmarshalMapResult(ptr), nil
}


// Scrape multiple URLs concurrently.
func BatchScrape(engine CrawlEngineHandle, urls []string) *[]BatchScrapeResult {
    jsonBytes, err := json.Marshal(engine)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cEngine := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cEngine))

    ptr := C.kcrawl_batch_scrape(cEngine, cUrls)
    return unmarshalListBatchScrapeResult(ptr)
}


// Crawl multiple seed URLs concurrently, each following links to configured depth.
func BatchCrawl(engine CrawlEngineHandle, urls []string) *[]BatchCrawlResult {
    jsonBytes, err := json.Marshal(engine)
    if err != nil {
        return fmt.Errorf("failed to marshal: %w", err)
    }
    cEngine := C.CString(string(jsonBytes))
    defer C.free(unsafe.Pointer(cEngine))

    ptr := C.kcrawl_batch_crawl(cEngine, cUrls)
    return unmarshalListBatchCrawlResult(ptr)
}


// Default is a method.
func (r *BrowserConfig) Default() *BrowserConfig {
    ptr := C.kcrawl_browser_config_default (unsafe.Pointer(r), )
    return unmarshalBrowserConfig(ptr)
}


// Default is a method.
func (r *CrawlConfig) Default() *CrawlConfig {
    ptr := C.kcrawl_crawl_config_default (unsafe.Pointer(r), )
    return unmarshalCrawlConfig(ptr)
}
