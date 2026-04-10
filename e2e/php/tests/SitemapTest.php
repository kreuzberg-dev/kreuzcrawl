<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: sitemap. */
final class SitemapTest extends TestCase
{
    /** Parses a standard urlset sitemap */
    public function test_sitemap_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(4, count($result->urls));
        $this->assertEquals(true, $result->has_lastmod);
    }

    /** Parses a gzip-compressed sitemap file */
    public function test_sitemap_compressed_gzip(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(3, count($result->urls));
    }

    /** Handles empty sitemap gracefully */
    public function test_sitemap_empty(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(0, count($result->urls));
    }

    /** Discovers sitemap via robots.txt Sitemap directive */
    public function test_sitemap_from_robots_txt(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(4, count($result->urls));
    }

    /** Follows sitemap index to discover child sitemaps */
    public function test_sitemap_index(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(3, count($result->urls));
    }

    /** Filters sitemap URLs by lastmod date */
    public function test_sitemap_lastmod_filter(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(4, count($result->urls));
        $this->assertEquals(true, $result->has_lastmod);
    }

    /** Uses sitemap URLs exclusively without following page links */
    public function test_sitemap_only_mode(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(4, count($result->urls));
    }

    /** Parses sitemap with XHTML namespace alternate links */
    public function test_sitemap_xhtml_links(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(2, count($result->urls));
        $this->assertEquals(false, $result->has_lastmod);
    }
}
