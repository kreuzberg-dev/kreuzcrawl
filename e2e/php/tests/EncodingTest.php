<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: encoding. */
final class EncodingTest extends TestCase
{
    /** Handles double-encoded URL characters (%25C3%25B6) */
    public function test_encoding_double_encoded(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertNotEmpty($result->html);
        $this->assertGreaterThanOrEqual(1, count($result->links));
    }

    /** Handles charset mismatch between HTTP header and HTML meta tag */
    public function test_encoding_mixed_charset_page(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertNotEmpty($result->html);
    }

    /** Handles percent-encoded spaces and characters in URL paths */
    public function test_encoding_percent_encoded_path(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertNotEmpty($result->html);
        $this->assertGreaterThanOrEqual(2, count($result->links));
    }

    /** Handles Unicode characters in URLs (Hebrew, Japanese, Cyrillic) */
    public function test_encoding_unicode_url(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertNotEmpty($result->html);
    }
}
