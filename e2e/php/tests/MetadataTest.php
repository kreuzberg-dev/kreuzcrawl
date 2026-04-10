<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: metadata. */
final class MetadataTest extends TestCase
{
    /** Extracts article:published_time, modified_time, author, section, and tags */
    public function test_metadata_article_times(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals("2024-01-15T10:00:00Z", $result->article->published_time);
        $this->assertEquals("2024-06-20T14:30:00Z", $result->article->modified_time);
        $this->assertEquals("Jane Developer", $result->article->author);
        $this->assertEquals("Technology", $result->article->section);
        $this->assertEquals(3, count($result->article->tags));
    }

    /** Extracts favicon link tags including apple-touch-icon */
    public function test_metadata_favicons(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(5, count($result->favicons));
        $this->assertNotEmpty($result->favicons[""]->apple_touch);
    }

    /** Extracts heading hierarchy (h1-h6) from HTML page */
    public function test_metadata_headings(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(1, count($result->headings->h1));
        $this->assertEquals("Primary Heading", $result->headings->h1["0"]->text);
        $this->assertEquals(8, count($result->headings));
    }

    /** Extracts hreflang alternate link tags */
    public function test_metadata_hreflang(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(4, count($result->hreflang));
        $this->assertStringContainsString("en", $result->hreflang[""]->lang);
    }

    /** Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata */
    public function test_metadata_keywords_author(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals("Comprehensive Metadata Test Page", $result->metadata->title);
        $this->assertNotEmpty($result->metadata->canonical_url);
        $this->assertNotEmpty($result->metadata->keywords);
        $this->assertStringContainsString("rust", $result->metadata->keywords);
        $this->assertEquals("Jane Developer", $result->metadata->author);
        $this->assertNotEmpty($result->metadata->viewport);
        $this->assertEquals("kreuzcrawl/1.0", $result->metadata->generator);
        $this->assertEquals("#ff6600", $result->metadata->theme_color);
        $this->assertEquals("index, follow", $result->metadata->robots);
        $this->assertEquals("en", $result->metadata->lang);
        $this->assertEquals("ltr", $result->metadata->dir);
    }

    /** Extracts og:video, og:audio, and og:locale:alternate metadata */
    public function test_metadata_og_video_audio(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals("https://example.com/video.mp4", $result->og->video);
        $this->assertEquals("https://example.com/audio.mp3", $result->og->audio);
        $this->assertEquals(2, count($result->og->locale_alternate));
    }

    /** Extracts response metadata from HTTP headers (etag, server, content-language) */
    public function test_metadata_response_headers(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertNotEmpty($result->response_headers->etag);
        $this->assertNotEmpty($result->response_headers->last_modified);
        $this->assertStringContainsString("nginx", $result->response_headers->server);
        $this->assertEquals("en-US", $result->response_headers->content_language);
    }

    /** Computes word count from visible page text */
    public function test_metadata_word_count(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(200, $result->status_code);
        $this->assertGreaterThan(99, $result->computed->word_count);
        $this->assertLessThan(301, $result->computed->word_count);
    }
}
