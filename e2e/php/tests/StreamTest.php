<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: stream. */
final class StreamTest extends TestCase
{
    /** Crawl stream produces page and complete events */
    public function test_crawl_stream_events(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThanOrEqual(4, $result->stream->event_count_min);
        $this->assertEquals(true, $result->stream->has_page_event);
        $this->assertEquals(true, $result->stream->has_complete_event);
    }

    /** Stream produces events for multi-depth crawl with link following */
    public function test_stream_depth_crawl(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertGreaterThanOrEqual(5, $result->stream->event_count_min);
        $this->assertEquals(true, $result->stream->has_page_event);
        $this->assertEquals(true, $result->stream->has_complete_event);
    }

    /** Stream emits page and complete events even when some pages fail */
    public function test_stream_with_error_event(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(true, $result->stream->has_page_event);
        $this->assertEquals(true, $result->stream->has_complete_event);
        $this->assertGreaterThanOrEqual(2, $result->stream->event_count_min);
    }
}
