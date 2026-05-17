/* Re-export wasm-bindgen classes with Wasm prefix for alef e2e test compatibility.
 *
 * alef's wasm e2e test generator hardcodes `strip_prefix("Wasm")` to handle
 * class names, assuming all wasm types are prefixed with "Wasm". When
 * [crates.wasm] type_prefix = "" is set in alef.toml, the binding exports
 * classes without the prefix (e.g., `CrawlConfig` instead of `WasmCrawlConfig`).
 *
 * This wrapper re-exports with the Wasm prefix so the generated tests can import
 * the classes they expect.
 *
 * TODO: fix upstream in alef/crates/alef-e2e/src/codegen/typescript/test_file.rs
 * to use configured type_prefix instead of hardcoding "Wasm" strip.
 */

const binding = require("./kreuzcrawl_wasm.js");

// Re-export all exports from the binding
module.exports = binding;

// Also export with Wasm prefix for e2e test compatibility
module.exports.WasmArticleMetadata = binding.ArticleMetadata;
module.exports.WasmAssetCategory = binding.AssetCategory;
module.exports.WasmAuthConfig = binding.AuthConfig;
module.exports.WasmBatchCrawlResult = binding.BatchCrawlResult;
module.exports.WasmBatchScrapeResult = binding.BatchScrapeResult;
module.exports.WasmBrowserConfig = binding.BrowserConfig;
module.exports.WasmBrowserMode = binding.BrowserMode;
module.exports.WasmBrowserWait = binding.BrowserWait;
module.exports.WasmCitationReference = binding.CitationReference;
module.exports.WasmCitationResult = binding.CitationResult;
module.exports.WasmContentConfig = binding.ContentConfig;
module.exports.WasmCookieInfo = binding.CookieInfo;
module.exports.WasmCrawlConfig = binding.CrawlConfig;
module.exports.WasmCrawlEngineHandle = binding.CrawlEngineHandle;
module.exports.WasmCrawlPageResult = binding.CrawlPageResult;
module.exports.WasmCrawlResult = binding.CrawlResult;
module.exports.WasmDownloadedAsset = binding.DownloadedAsset;
module.exports.WasmDownloadedDocument = binding.DownloadedDocument;
module.exports.WasmExtractionMeta = binding.ExtractionMeta;
module.exports.WasmFaviconInfo = binding.FaviconInfo;
module.exports.WasmFeedInfo = binding.FeedInfo;
module.exports.WasmFeedType = binding.FeedType;
module.exports.WasmHeadingInfo = binding.HeadingInfo;
module.exports.WasmHreflangEntry = binding.HreflangEntry;
module.exports.WasmImageInfo = binding.ImageInfo;
module.exports.WasmImageSource = binding.ImageSource;
module.exports.WasmJsonLdEntry = binding.JsonLdEntry;
module.exports.WasmLinkInfo = binding.LinkInfo;
module.exports.WasmLinkType = binding.LinkType;
module.exports.WasmMapResult = binding.MapResult;
module.exports.WasmMarkdownResult = binding.MarkdownResult;
module.exports.WasmPageMetadata = binding.PageMetadata;
module.exports.WasmProxyConfig = binding.ProxyConfig;
module.exports.WasmResponseMeta = binding.ResponseMeta;
module.exports.WasmScrapeResult = binding.ScrapeResult;
module.exports.WasmSitemapUrl = binding.SitemapUrl;
