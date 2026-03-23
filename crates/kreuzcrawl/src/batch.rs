//! Batch scrape operation for processing multiple URLs.

use std::sync::Arc;

use tokio::sync::Semaphore;

use crate::error::CrawlError;
use crate::scrape;
use crate::types::{CrawlConfig, ScrapeResult};

/// Default concurrency limit when `max_concurrent` is not set.
const DEFAULT_CONCURRENCY: usize = 10;

/// Scrape multiple URLs concurrently and return results for each.
///
/// Each URL is scraped independently using the provided configuration.
/// Results are returned in the same order as the input URLs, paired
/// with the URL string. Failed scrapes return `Err` without stopping
/// other URLs from being processed.
///
/// Concurrency is bounded by `config.max_concurrent` (default: 10).
/// Each URL is processed in a separate async task. If a task panics,
/// it is caught and reported as an error without crashing the batch.
pub async fn batch_scrape(
    urls: &[&str],
    config: &CrawlConfig,
) -> Vec<(String, Result<ScrapeResult, CrawlError>)> {
    let concurrency = config.max_concurrent.unwrap_or(DEFAULT_CONCURRENCY);
    let sem = Arc::new(Semaphore::new(concurrency));
    let config = config.clone();

    type UrlResult = (String, Result<ScrapeResult, CrawlError>);
    let mut handles: Vec<(String, tokio::task::JoinHandle<UrlResult>)> =
        Vec::with_capacity(urls.len());

    for &url in urls {
        let url_for_error = url.to_owned();
        let url_owned = url.to_owned();
        let sem = Arc::clone(&sem);
        let config = config.clone();

        let task = tokio::spawn(async move {
            let _permit = match sem.acquire().await {
                Ok(p) => p,
                Err(_) => return (url_owned, Err(CrawlError::Other("semaphore closed".into()))),
            };
            let result = scrape::scrape(&url_owned, &config).await;
            (url_owned, result)
        });

        handles.push((url_for_error, task));
    }

    let mut results = Vec::with_capacity(handles.len());
    for (url, task_handle) in handles {
        match task_handle.await {
            Ok(pair) => results.push(pair),
            Err(e) => results.push((url, Err(CrawlError::Other(e.to_string())))),
        }
    }

    results
}
