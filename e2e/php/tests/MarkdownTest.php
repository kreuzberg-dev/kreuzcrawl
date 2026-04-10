<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: markdown. */
final class MarkdownTest extends TestCase
{
    /** HTML is always converted to markdown alongside raw HTML */
    public function test_markdown_basic_conversion(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals("Test", $result->metadata->title);
        $this->assertNotEmpty($result->html);
        $this->assertNotEmpty($result->markdown);
        $this->assertStringContainsString("Hello World", $result->markdown);
    }

    /** All crawled pages have markdown field populated */
    public function test_markdown_crawl_all_pages(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(2, $result->crawl->pages_crawled);
    }

    /** Fit markdown removes navigation and boilerplate content */
    public function test_markdown_fit_content(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertNotEmpty($result->markdown);
    }

    /** Markdown conversion preserves heading hierarchy and paragraph text */
    public function test_markdown_headings_and_paragraphs(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertNotEmpty($result->markdown);
        $this->assertStringContainsString("Main Title", $result->markdown);
    }

    /** HTML links are converted to markdown link syntax */
    public function test_markdown_links_converted(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertNotEmpty($result->html);
        $this->assertNotEmpty($result->markdown);
        $this->assertStringContainsString("Example", $result->markdown);
    }

    /** Markdown includes citation conversion with numbered references */
    public function test_markdown_with_citations(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertNotEmpty($result->markdown);
    }
}
