//! Shared adapter layer for kreuzcrawl language bindings.
//!
//! Re-exports the public binding API from kreuzcrawl.

pub use kreuzcrawl;
pub use kreuzcrawl::{
    BatchCrawlResult, BatchCrawlResults, BatchScrapeResult, BatchScrapeResults, CrawlEngineHandle, batch_crawl,
    batch_scrape, crawl, create_engine, map_urls, scrape,
};
