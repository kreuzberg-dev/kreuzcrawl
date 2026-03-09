//! Batch scrape operation for processing multiple URLs.

use crate::error::CrawlError;
use crate::scrape;
use crate::types::{CrawlConfig, ScrapeResult};

/// Scrape multiple URLs and return results for each.
///
/// Each URL is scraped independently using the provided configuration.
/// Results are returned in the same order as the input URLs, paired
/// with the URL string. Failed scrapes return `Err` without stopping
/// other URLs from being processed.
pub async fn batch_scrape(
    urls: &[&str],
    config: &CrawlConfig,
) -> Vec<(String, Result<ScrapeResult, CrawlError>)> {
    let mut results = Vec::with_capacity(urls.len());

    for &url in urls {
        let result = scrape::scrape(url, config).await;
        results.push((url.to_owned(), result));
    }

    results
}
