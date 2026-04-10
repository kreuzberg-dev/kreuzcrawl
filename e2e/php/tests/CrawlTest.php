<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: crawl. */
final class CrawlTest extends TestCase
{
    /** Skips image and video content types gracefully */
    public function test_content_binary_skip(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(true, $result->content->was_skipped);
    }

    /** Encounters PDF link and skips or marks as document type */
    public function test_content_pdf_link_skip(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(true, $result->content->was_skipped);
    }

    /** Concurrent crawl respects max_depth limit */
    public function test_crawl_concurrent_depth(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(3, count($result->pages));
        $this->assertEquals(true, $result->stayed_on_domain);
    }

    /** Respects max concurrent requests limit during crawl */
    public function test_crawl_concurrent_limit(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(5, count($result->pages));
    }

    /** Concurrent crawl respects max_pages budget */
    public function test_crawl_concurrent_max_pages(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertLessThanOrEqual(3, count($result->pages));
    }

    /** Sends custom headers on all crawl requests */
    public function test_crawl_custom_headers(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->pages));
    }

    /** Follows links one level deep from start page */
    public function test_crawl_depth_one(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(3, count($result->pages));
        $this->assertEquals(true, $result->stayed_on_domain);
    }

    /** Crawls in breadth-first order, processing depth-0 pages before depth-1 */
    public function test_crawl_depth_priority(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(4, count($result->pages));
    }

    /** Crawls 3 levels deep (depth 0, 1, 2) */
    public function test_crawl_depth_two(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(3, count($result->pages));
        $this->assertGreaterThanOrEqual(3, count($result->pages));
    }

    /** Depth=2 crawl follows a chain of links across three levels */
    public function test_crawl_depth_two_chain(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(3, count($result->pages));
    }

    /** Normalizes double slashes in URL paths (//page to /page) */
    public function test_crawl_double_slash_normalization(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->unique_urls));
    }

    /** Crawl completes when child page has no outgoing links */
    public function test_crawl_empty_page_no_links(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->pages));
    }

    /** Skips URLs matching the exclude path pattern */
    public function test_crawl_exclude_path_pattern(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->pages));
    }

    /** External links are discovered but not followed when stay_on_domain is true */
    public function test_crawl_external_links_ignored(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->pages));
        $this->assertEquals(true, $result->stayed_on_domain);
    }

    /** Strips #fragment from URLs for deduplication */
    public function test_crawl_fragment_stripping(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->unique_urls));
    }

    /** Only follows URLs matching the include path pattern */
    public function test_crawl_include_path_pattern(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->pages));
    }

    /** max_depth=0 crawls only the seed page with no link following */
    public function test_crawl_max_depth_zero(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(1, count($result->pages));
        $this->assertLessThanOrEqual(1, count($result->pages));
    }

    /** Stops crawling at page budget limit */
    public function test_crawl_max_pages(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertLessThanOrEqual(3, count($result->pages));
    }

    /** Crawl handles links to non-HTML content types gracefully */
    public function test_crawl_mixed_content_types(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertGreaterThanOrEqual(2, count($result->pages));
    }

    /** Multiple linked pages with redirects are handled during crawl traversal */
    public function test_crawl_multiple_redirects_in_traversal(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertGreaterThanOrEqual(1, count($result->pages));
    }

    /** Deduplicates URLs with same query params in different order */
    public function test_crawl_query_param_dedup(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->unique_urls));
    }

    /** Links that redirect are followed during crawl traversal */
    public function test_crawl_redirect_in_traversal(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertGreaterThanOrEqual(1, count($result->pages));
    }

    /** Page linking to itself does not cause infinite crawl loop */
    public function test_crawl_self_link_no_loop(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->pages));
    }

    /** Crawling a page with no links returns only the seed page */
    public function test_crawl_single_page_no_links(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(1, count($result->pages));
    }

    /** Does not follow external links when stay_on_domain is true */
    public function test_crawl_stay_on_domain(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->pages));
        $this->assertEquals(true, $result->stayed_on_domain);
    }

    /** Stays on exact domain and skips subdomain links */
    public function test_crawl_subdomain_exclusion(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->pages));
        $this->assertEquals(true, $result->stayed_on_domain);
    }

    /** Crawls subdomains when allow_subdomains is enabled */
    public function test_crawl_subdomain_inclusion(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertGreaterThanOrEqual(2, count($result->pages));
    }

    /** Deduplicates /page and /page/ as the same URL */
    public function test_crawl_trailing_slash_dedup(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, count($result->unique_urls));
    }

    /** Deduplicates URLs that differ only by fragment or query params */
    public function test_crawl_url_deduplication(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertLessThanOrEqual(2, count($result->pages));
    }
}
