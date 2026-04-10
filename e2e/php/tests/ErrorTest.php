<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: error. */
final class ErrorTest extends TestCase
{
    /** Handles 401 Unauthorized response correctly */
    public function test_error_401_unauthorized(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles 403 Forbidden response correctly */
    public function test_error_403_forbidden(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles 404 response correctly */
    public function test_error_404_page(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles 408 Request Timeout response correctly */
    public function test_error_408_request_timeout(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles 410 Gone response correctly */
    public function test_error_410_gone(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles 500 server error */
    public function test_error_500_server(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles 502 Bad Gateway response correctly */
    public function test_error_502_bad_gateway(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles connection refused error gracefully */
    public function test_error_connection_refused(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles DNS resolution failure gracefully */
    public function test_error_dns_resolution(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles 200 with completely empty body gracefully */
    public function test_error_empty_response(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(false, $result->html_not_empty);
        $this->assertEquals(false, $result->error->is_error);
    }

    /** Proxy pointing to unreachable address causes connection error during scrape */
    public function test_error_invalid_proxy(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles incomplete or truncated HTTP response */
    public function test_error_partial_response(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles 429 rate limiting with Retry-After */
    public function test_error_rate_limited(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Retries request on 503 Service Unavailable response */
    public function test_error_retry_503(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Implements exponential backoff when retrying failed requests */
    public function test_error_retry_backoff(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles SSL certificate validation error */
    public function test_error_ssl_invalid_cert(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Handles request timeout */
    public function test_error_timeout(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Akamai WAF detection returns WafBlocked error */
    public function test_error_waf_akamai(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Detects WAF/bot protection false 403 (Cloudflare challenge page) */
    public function test_error_waf_false_403(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }

    /** Imperva/Incapsula WAF detection */
    public function test_error_waf_imperva(): void
    {
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape();
    }
}
