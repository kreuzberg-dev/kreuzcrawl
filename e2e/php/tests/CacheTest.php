<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: cache. */
final class CacheTest extends TestCase
{
    /** Crawling with disk cache enabled succeeds without errors */
    public function test_cache_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
    }
}
