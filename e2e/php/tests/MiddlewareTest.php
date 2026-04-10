<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: middleware. */
final class MiddlewareTest extends TestCase
{
    /** Engine crawl with default middleware chain produces correct multi-page results */
    public function test_middleware_engine_crawl_with_defaults(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(3, $result->crawl->pages_crawled);
        $this->assertGreaterThanOrEqual(3, $result->crawl->min_pages);
    }

    /** Default middleware chain does not affect normal scraping */
    public function test_middleware_noop_no_effect(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals("Middleware Test", $result->metadata->title);
    }
}
