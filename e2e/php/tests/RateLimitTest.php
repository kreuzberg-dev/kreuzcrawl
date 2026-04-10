<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: rate_limit. */
final class RateLimitTest extends TestCase
{
    /** Rate limiter adds delay between requests to the same domain */
    public function test_rate_limit_basic_delay(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(3, $result->crawl->pages_crawled);
        $this->assertGreaterThanOrEqual(150, $result->rate_limit->min_duration_ms);
    }

    /** Rate limiter with zero delay does not slow crawling */
    public function test_rate_limit_zero_no_delay(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(2, $result->crawl->pages_crawled);
    }
}
