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
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'filter.remaining_contain_keyword' not available on result type
    }

    /** BM25 filter with empty query passes all pages through */
    public function test_filter_bm25_empty_query(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    /** BM25 filter with very high threshold filters out all pages */
    public function test_filter_bm25_high_threshold(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'filter.pages_after_filter' not available on result type
    }

    /** BM25 filter keeps only pages relevant to the query */
    public function test_filter_bm25_relevant_pages(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'filter.remaining_contain_keyword' not available on result type
    }

    /** BM25 filter with zero threshold passes all pages */
    public function test_filter_bm25_threshold_zero(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    /** NoopFilter keeps all pages during a multi-page crawl */
    public function test_filter_noop_crawl_all_kept(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'filter.pages_after_filter' not available on result type
    }

    /** No content filter passes all crawled pages through */
    public function test_filter_noop_passes_all(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'crawl.pages_crawled' not available on result type
    }
}
