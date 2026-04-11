<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: validation. */
final class ValidationTest extends TestCase
{
    /** Invalid regex in exclude_paths is rejected */
    public function test_validation_invalid_exclude_regex(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/validation_invalid_exclude_regex';
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, $url);
    }

    /** Invalid regex in include_paths is rejected */
    public function test_validation_invalid_include_regex(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/validation_invalid_include_regex';
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, $url);
    }

    /** Retry code outside 100-599 is rejected */
    public function test_validation_invalid_retry_code(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/validation_invalid_retry_code';
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, $url);
    }

    /** max_pages=0 is rejected as invalid config */
    public function test_validation_max_pages_zero(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/validation_max_pages_zero';
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, $url);
    }

    /** max_redirects > 100 is rejected as invalid config */
    public function test_validation_max_redirects_too_high(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/validation_max_redirects_too_high';
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, $url);
    }

    /** Zero request timeout is rejected as invalid config */
    public function test_validation_timeout_zero(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/validation_timeout_zero';
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, $url);
    }
}
