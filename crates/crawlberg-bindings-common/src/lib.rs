//! Shared adapter layer for crawlberg language bindings.
//!
//! Re-exports the public binding API from crawlberg.

pub use crawlberg;
pub use crawlberg::{
    BatchCrawlResult, BatchCrawlResults, BatchScrapeResult, BatchScrapeResults, CrawlEngineHandle, batch_crawl,
    batch_scrape, crawl, create_engine, map_urls, scrape,
};
