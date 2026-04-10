//! E2e tests for category: strategy

use kreuzcrawl::scrape;

#[test]
fn test_strategy_best_first_seed() {
    // BestFirst strategy always processes the seed URL first
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "3", "equals assertion failed");
    assert!(result.strategy.first_page_url_contains.contains(r#"/"#), "expected to contain: {}", r#"/"#);
}

#[test]
fn test_strategy_bfs_default_order() {
    // BFS strategy visits pages in breadth-first order
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "5", "equals assertion failed");
    assert_eq!(result.strategy.crawl_order, "["/","/a","/b","/a/1","/b/1"]", "equals assertion failed");
}

#[test]
fn test_strategy_dfs_depth_first() {
    // DFS strategy visits pages in depth-first order
    let engine = None;
    let url = None;
    let result = scrape(engine, url).expect("should succeed");
    assert_eq!(result.crawl.pages_crawled, "5", "equals assertion failed");
    assert_eq!(result.strategy.crawl_order, "["/","/b","/b/1","/a","/a/1"]", "equals assertion failed");
}

