<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: auth. */
final class AuthTest extends TestCase
{
    /** Sends HTTP Basic authentication header */
    public function test_auth_basic_http(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(true, $result->auth_header_sent);
        $this->assertEquals(200, $result->status_code);
    }

    /** Sends Bearer token in Authorization header */
    public function test_auth_bearer_token(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(true, $result->auth_header_sent);
        $this->assertEquals(200, $result->status_code);
    }

    /** Sends authentication via custom header (X-API-Key) */
    public function test_auth_custom_header(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(true, $result->auth_header_sent);
        $this->assertEquals(200, $result->status_code);
    }
}
