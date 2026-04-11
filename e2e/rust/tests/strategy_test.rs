//! E2e tests for category: strategy

use kreuzcrawl::scrape;
use kreuzcrawl::create_engine;

#[tokio::test]
async fn test_strategy_best_first_seed() {
    // BestFirst strategy always processes the seed URL first
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = format!("{}/fixtures/{}", std::env::var("MOCK_SERVER_URL").expect("MOCK_SERVER_URL not set"), "strategy_best_first_seed");
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'strategy.first_page_url_contains' not available on result type
}

#[tokio::test]
async fn test_strategy_bfs_default_order() {
    // BFS strategy visits pages in breadth-first order
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = format!("{}/fixtures/{}", std::env::var("MOCK_SERVER_URL").expect("MOCK_SERVER_URL not set"), "strategy_bfs_default_order");
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'strategy.crawl_order' not available on result type
}

#[tokio::test]
async fn test_strategy_dfs_depth_first() {
    // DFS strategy visits pages in depth-first order
    let engine = kreuzcrawl::create_engine(None).expect("handle creation should succeed");
    let url = format!("{}/fixtures/{}", std::env::var("MOCK_SERVER_URL").expect("MOCK_SERVER_URL not set"), "strategy_dfs_depth_first");
    let _ = scrape(&engine, &url).await.expect("should succeed");
    // skipped: field 'crawl.pages_crawled' not available on result type
    // skipped: field 'strategy.crawl_order' not available on result type
}
