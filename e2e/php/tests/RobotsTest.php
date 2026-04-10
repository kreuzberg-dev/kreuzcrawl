<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: robots. */
final class RobotsTest extends TestCase
{
    /** Permissive robots.txt allows all paths */
    public function test_robots_allow_all(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Allow directive overrides Disallow for specific paths */
    public function test_robots_allow_override(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Correctly parses robots.txt with inline and line comments */
    public function test_robots_comments_handling(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Respects crawl-delay directive from robots.txt */
    public function test_robots_crawl_delay(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.crawl_delay' not available on result type
    }

    /** Robots.txt disallows specific paths */
    public function test_robots_disallow_path(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Detects nofollow meta robots tag and skips link extraction */
    public function test_robots_meta_nofollow(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.nofollow_detected' not available on result type
    }

    /** Detects noindex meta robots tag in HTML page */
    public function test_robots_meta_noindex(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.noindex_detected' not available on result type
    }

    /** Missing robots.txt (404) allows all crawling */
    public function test_robots_missing_404(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Picks the most specific user-agent block from robots.txt */
    public function test_robots_multiple_user_agents(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Parses request-rate directive from robots.txt */
    public function test_robots_request_rate(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.crawl_delay' not available on result type
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Discovers sitemap URL from Sitemap directive in robots.txt */
    public function test_robots_sitemap_directive(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Matches user-agent specific rules in robots.txt */
    public function test_robots_user_agent_specific(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Handles wildcard Disallow patterns in robots.txt */
    public function test_robots_wildcard_paths(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.is_allowed' not available on result type
    }

    /** Respects X-Robots-Tag HTTP header directives */
    public function test_robots_x_robots_tag(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'robots.x_robots_tag' not available on result type
        // skipped: field 'robots.noindex_detected' not available on result type
        // skipped: field 'robots.nofollow_detected' not available on result type
    }
}
