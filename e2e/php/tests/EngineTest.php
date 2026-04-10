<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: engine. */
final class EngineTest extends TestCase
{
    /** CrawlEngine with defaults batch scrapes like the free function */
    public function test_engine_batch_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(2, $result->batch->completed_count);
        $this->assertEquals(2, $result->batch->total_count);
    }

    /** CrawlEngine with defaults crawls multiple pages like the free function */
    public function test_engine_crawl_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(3, $result->crawl->pages_crawled);
        $this->assertGreaterThanOrEqual(3, $result->crawl->min_pages);
    }

    /** CrawlEngine with defaults discovers URLs like the free function */
    public function test_engine_map_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThanOrEqual(2, $result->map->min_urls);
    }

    /** CrawlEngine with defaults scrapes a page identically to the free function */
    public function test_engine_scrape_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals("text/html", $result->content_type);
        $this->assertEquals("Engine Test", $result->metadata->title);
        $this->assertStringContainsString("Testing the engine", $result->metadata->description_contains);
        $this->assertGreaterThanOrEqual(1, $result->links->min_count);
        $this->assertEquals(1, $result->headings->h1_count);
        $this->assertEquals("Hello Engine", $result->headings->h1_text);
    }

    /** CrawlEngine with defaults streams events like the free function */
    public function test_engine_stream_basic(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(true, $result->stream->has_page_event);
        $this->assertEquals(true, $result->stream->has_complete_event);
        $this->assertGreaterThanOrEqual(3, $result->stream->event_count_min);
    }
}
