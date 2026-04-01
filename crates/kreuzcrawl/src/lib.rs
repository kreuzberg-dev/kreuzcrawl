//! kreuzcrawl -- A Rust crawling engine for turning websites into structured data.

mod assets;
mod batch;
#[cfg(feature = "browser")]
mod browser;
mod browser_detect;
#[cfg(feature = "browser")]
mod browser_pool;
pub mod defaults;
pub mod engine;
mod error;
mod helpers;
mod html;
mod http;
mod map;
mod markdown;
mod normalize;
mod robots;
mod scrape;
mod sitemap;
pub mod traits;
mod types;

#[cfg(feature = "browser")]
pub use browser_pool::{BrowserPool, BrowserPoolConfig, PooledPage};
#[cfg(feature = "ai")]
pub use defaults::LlmExtractor;
pub use defaults::{
    AdaptiveStrategy, BestFirstStrategy, BfsStrategy, Bm25Filter, CachingMiddleware, DfsStrategy,
    InMemoryFrontier, NoopEmitter, NoopFilter, NoopMiddleware, NoopRateLimiter, NoopStore,
    PerDomainThrottle, UaRotationMiddleware,
};
pub use engine::{CrawlEngine, CrawlEngineBuilder};
pub use error::CrawlError;
pub use traits::{
    CompleteEvent, ContentFilter, CrawlMiddleware, CrawlStats, CrawlStore, CrawlStrategy,
    ErrorEvent, EventEmitter, Frontier, FrontierEntry, PageEvent, RateLimiter, RequestContext,
    ResponseContext,
};
pub use types::{
    ArticleMetadata, AssetCategory, AuthConfig, BrowserConfig, BrowserMode, BrowserWait,
    CookieInfo, CrawlConfig, CrawlEvent, CrawlPageResult, CrawlResult, DownloadedAsset,
    FaviconInfo, FeedInfo, FeedType, HeadingInfo, HreflangEntry, ImageInfo, ImageSource,
    JsonLdEntry, LinkInfo, LinkType, MapResult, MarkdownResult, PageMetadata, ProxyConfig,
    ResponseMeta, ScrapeResult, SitemapUrl,
};
