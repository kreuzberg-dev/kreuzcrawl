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
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(1, count($result->cookies));
        $this->assertStringContainsString("domain_cookie", $result->cookies);
    }

    /** Maintains cookies across multiple crawl requests */
    public function test_cookies_persistence(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertStringContainsString("session", $result->cookies);
    }

    /** Respects Set-Cookie header from server responses */
    public function test_cookies_set_cookie_response(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertStringContainsString("tracking", $result->cookies);
    }
}
