<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: strategy. */
final class StrategyTest extends TestCase
{
    /** BestFirst strategy always processes the seed URL first */
    public function test_strategy_best_first_seed(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/strategy_best_first_seed';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.first_page_url_contains' not available on result type
    }

    /** BFS strategy visits pages in breadth-first order */
    public function test_strategy_bfs_default_order(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/strategy_bfs_default_order';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.crawl_order' not available on result type
    }

    /** DFS strategy visits pages in depth-first order */
    public function test_strategy_dfs_depth_first(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/strategy_dfs_depth_first';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'strategy.crawl_order' not available on result type
    }
}
