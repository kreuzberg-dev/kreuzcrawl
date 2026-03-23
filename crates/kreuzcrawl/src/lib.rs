//! kreuzcrawl -- A Rust crawling engine for turning websites into structured data.

mod assets;
mod batch;
#[cfg(feature = "browser")]
mod browser;
mod browser_detect;
#[cfg(feature = "browser")]
mod browser_pool;
mod crawl;
mod error;
mod html;
mod http;
mod map;
mod normalize;
mod robots;
mod scrape;
mod sitemap;
mod stream;
mod types;

pub use batch::batch_scrape;
#[cfg(feature = "browser")]
pub use browser_pool::{BrowserPool, BrowserPoolConfig, PooledPage};
pub use crawl::crawl;
pub use error::CrawlError;
pub use map::map;
pub use scrape::scrape;
pub use stream::crawl_stream;
pub use types::{
    ArticleMetadata, AssetCategory, AuthHeader, BasicAuth, BrowserMode, BrowserWait, CookieInfo,
    CrawlConfig, CrawlEvent, CrawlPageResult, CrawlResult, DownloadedAsset, FaviconInfo, FeedInfo,
    FeedType, HeadingInfo, HreflangEntry, ImageInfo, ImageSource, JsonLdEntry, LinkInfo, LinkType,
    MapResult, PageMetadata, ResponseMeta, ScrapeResult, SitemapUrl,
};
