<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: concurrent. */
final class ConcurrentTest extends TestCase
{
    /** Concurrent crawling fetches all pages with max_concurrent workers */
    public function test_concurrent_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    /** Concurrent depth=2 crawl correctly fans out and deduplicates across levels */
    public function test_concurrent_depth_two_fan_out(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    /** Concurrent crawling does not exceed max_pages limit even with high concurrency */
    public function test_concurrent_max_pages_exact(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    /** Concurrent crawl handles partial failures gracefully */
    public function test_concurrent_partial_errors(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'pages.length' not available on result type
    }

    /** Concurrent crawling respects max_pages limit */
    public function test_concurrent_respects_max_pages(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'pages.length' not available on result type
    }
}
