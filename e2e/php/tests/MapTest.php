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
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/map_discover_urls';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'urls.length' not available on result type
    }

    /** Excludes URLs matching patterns from URL map */
    public function test_map_exclude_patterns(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/map_exclude_patterns';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'urls.length' not available on result type
    }

    /** Includes subdomain URLs in URL map discovery */
    public function test_map_include_subdomains(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/map_include_subdomains';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'urls' not available on result type
    }

    /** Handles large sitemap with 100+ URLs */
    public function test_map_large_sitemap(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/map_large_sitemap';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'urls.length' not available on result type
    }

    /** Limits map result count to specified maximum */
    public function test_map_limit_pagination(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/map_limit_pagination';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'urls.length' not available on result type
    }

    /** Filters map results by search keyword */
    public function test_map_search_filter(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/map_search_filter';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'urls.length' not available on result type
        // skipped: field 'urls' not available on result type
    }
}
