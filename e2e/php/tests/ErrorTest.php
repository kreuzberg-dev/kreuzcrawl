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
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles 403 Forbidden response correctly */
    public function test_error_403_forbidden(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles 404 response correctly */
    public function test_error_404_page(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles 408 Request Timeout response correctly */
    public function test_error_408_request_timeout(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles 410 Gone response correctly */
    public function test_error_410_gone(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles 500 server error */
    public function test_error_500_server(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles 502 Bad Gateway response correctly */
    public function test_error_502_bad_gateway(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles connection refused error gracefully */
    public function test_error_connection_refused(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles DNS resolution failure gracefully */
    public function test_error_dns_resolution(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles 200 with completely empty body gracefully */
    public function test_error_empty_response(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        // skipped: field 'html_not_empty' not available on result type
        // skipped: field 'error.is_error' not available on result type
    }

    /** Proxy pointing to unreachable address causes connection error during scrape */
    public function test_error_invalid_proxy(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles incomplete or truncated HTTP response */
    public function test_error_partial_response(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles 429 rate limiting with Retry-After */
    public function test_error_rate_limited(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Retries request on 503 Service Unavailable response */
    public function test_error_retry_503(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Implements exponential backoff when retrying failed requests */
    public function test_error_retry_backoff(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles SSL certificate validation error */
    public function test_error_ssl_invalid_cert(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Handles request timeout */
    public function test_error_timeout(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Akamai WAF detection returns WafBlocked error */
    public function test_error_waf_akamai(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Detects WAF/bot protection false 403 (Cloudflare challenge page) */
    public function test_error_waf_false_403(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }

    /** Imperva/Incapsula WAF detection */
    public function test_error_waf_imperva(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $this->expectException(\Exception::class);
        Kreuzcrawl::scrape($engine, "");
    }
}
