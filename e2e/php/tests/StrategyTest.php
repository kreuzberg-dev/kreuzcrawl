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
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(3, $result->crawl->pages_crawled);
        $this->assertStringContainsString("/", $result->strategy->first_page_url_contains);
    }

    /** BFS strategy visits pages in breadth-first order */
    public function test_strategy_bfs_default_order(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(5, $result->crawl->pages_crawled);
        $this->assertEquals(["/", "/a", "/b", "/a/1", "/b/1"], $result->strategy->crawl_order);
    }

    /** DFS strategy visits pages in depth-first order */
    public function test_strategy_dfs_depth_first(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(5, $result->crawl->pages_crawled);
        $this->assertEquals(["/", "/b", "/b/1", "/a", "/a/1"], $result->strategy->crawl_order);
    }
}
