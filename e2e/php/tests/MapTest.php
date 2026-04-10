<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: map. */
final class MapTest extends TestCase
{
    /** Discovers all URLs on a site without fetching full content */
    public function test_map_discover_urls(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertGreaterThanOrEqual(3, count($result->urls));
    }

    /** Excludes URLs matching patterns from URL map */
    public function test_map_exclude_patterns(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(1, count($result->urls));
    }

    /** Includes subdomain URLs in URL map discovery */
    public function test_map_include_subdomains(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertGreaterThanOrEqual(2, count($result->urls));
        $this->assertStringContainsString("blog.example.com", $result->urls);
    }

    /** Handles large sitemap with 100+ URLs */
    public function test_map_large_sitemap(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertGreaterThanOrEqual(100, count($result->urls));
    }

    /** Limits map result count to specified maximum */
    public function test_map_limit_pagination(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertLessThanOrEqual(5, count($result->urls));
    }

    /** Filters map results by search keyword */
    public function test_map_search_filter(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertGreaterThanOrEqual(2, count($result->urls));
        $this->assertStringContainsString("blog", $result->urls);
    }
}
