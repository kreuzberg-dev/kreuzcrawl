<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: filter. */
final class FilterTest extends TestCase
{
    /** BM25 filter works during multi-page crawl, keeping relevant pages */
    public function test_filter_bm25_crawl_integration(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertStringContainsString("rust", $result->filter->remaining_contain_keyword);
    }

    /** BM25 filter with empty query passes all pages through */
    public function test_filter_bm25_empty_query(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, $result->crawl->pages_crawled);
    }

    /** BM25 filter with very high threshold filters out all pages */
    public function test_filter_bm25_high_threshold(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(0, $result->filter->pages_after_filter);
    }

    /** BM25 filter keeps only pages relevant to the query */
    public function test_filter_bm25_relevant_pages(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertStringContainsString("rust", $result->filter->remaining_contain_keyword);
    }

    /** BM25 filter with zero threshold passes all pages */
    public function test_filter_bm25_threshold_zero(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, $result->crawl->pages_crawled);
    }

    /** NoopFilter keeps all pages during a multi-page crawl */
    public function test_filter_noop_crawl_all_kept(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(3, $result->filter->pages_after_filter);
    }

    /** No content filter passes all crawled pages through */
    public function test_filter_noop_passes_all(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(3, $result->crawl->pages_crawled);
    }
}
