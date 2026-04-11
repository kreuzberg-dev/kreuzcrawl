<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: cookies. */
final class CookiesTest extends TestCase
{
    /** Isolates cookies per domain during crawl */
    public function test_cookies_per_domain(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/cookies_per_domain';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'cookies.length' not available on result type
        // skipped: field 'cookies' not available on result type
    }

    /** Maintains cookies across multiple crawl requests */
    public function test_cookies_persistence(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/cookies_persistence';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'cookies' not available on result type
    }

    /** Respects Set-Cookie header from server responses */
    public function test_cookies_set_cookie_response(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $url = getenv('MOCK_SERVER_URL') . '/fixtures/cookies_set_cookie_response';
        $result = Kreuzcrawl::scrape($engine, $url);
        // skipped: field 'cookies' not available on result type
    }
}
