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
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'rate_limit.min_duration_ms' not available on result type
    }

    /** Rate limiter with zero delay does not slow crawling */
    public function test_rate_limit_zero_no_delay(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }
}
