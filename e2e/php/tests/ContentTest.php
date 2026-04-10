<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: content. */
final class ContentTest extends TestCase
{
    /** Handles 204 No Content response gracefully */
    public function test_content_204_no_content(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(204, $result->status_code);
        $this->assertEmpty($result->html);
    }

    /** Handles ISO-8859-1 encoded page correctly */
    public function test_content_charset_iso8859(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals("iso-8859-1", $result->content->detected_charset);
    }

    /** Handles 200 response with empty body gracefully */
    public function test_content_empty_body(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
    }

    /** Handles response with Accept-Encoding gzip negotiation */
    public function test_content_gzip_compressed(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertNotEmpty($result->html);
        $this->assertEquals(200, $result->status_code);
    }

    /** Respects max body size limit and truncates or skips oversized pages */
    public function test_content_large_page_limit(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertLessThan(1025, $result->content->body_size);
    }

    /** Extracts only main content area, excluding nav, sidebar, footer */
    public function test_content_main_only(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(true, $result->content->main_content_only);
    }

    /** Detects PDF content by Content-Type header when URL has no .pdf extension */
    public function test_content_pdf_no_extension(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(true, $result->content->is_pdf);
    }

    /** Removes specified HTML elements by CSS selector before processing */
    public function test_content_remove_tags(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertNotEmpty($result->html);
    }

    /** Handles UTF-8 content with BOM marker correctly */
    public function test_content_utf8_bom(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals("utf-8", $result->content->detected_charset);
        $this->assertNotEmpty($result->html);
    }
}
