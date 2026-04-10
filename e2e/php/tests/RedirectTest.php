<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: redirect. */
final class RedirectTest extends TestCase
{
    /** Follows 301 permanent redirect and returns final page content */
    public function test_redirect_301_permanent(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/target", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
    }

    /** Follows 302 Found redirect correctly */
    public function test_redirect_302_found(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/found-target", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
    }

    /** Follows 303 See Other redirect (method changes to GET) */
    public function test_redirect_303_see_other(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/see-other", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
    }

    /** Follows 307 Temporary Redirect (preserves method) */
    public function test_redirect_307_temporary(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/temp-target", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
    }

    /** Follows 308 Permanent Redirect (preserves method) */
    public function test_redirect_308_permanent(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/perm-target", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
    }

    /** Follows a chain of redirects (301 -> 302 -> 200) */
    public function test_redirect_chain(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/step2", $result->final_url);
        $this->assertEquals(2, $result->redirect_count);
    }

    /** Reports cross-domain redirect target without following to external domain */
    public function test_redirect_cross_domain(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/external-redirect", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
    }

    /** Detects redirect loop (A -> B -> A) and returns error */
    public function test_redirect_loop(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(true, $result->is_error);
    }

    /** Aborts when redirect count exceeds max_redirects limit */
    public function test_redirect_max_exceeded(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertEquals(true, $result->is_error);
    }

    /** Follows HTML meta-refresh redirect to target page */
    public function test_redirect_meta_refresh(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/target", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
    }

    /** Handles HTTP Refresh header redirect */
    public function test_redirect_refresh_header(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/refreshed", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
    }

    /** Redirect target returns 404 Not Found */
    public function test_redirect_to_404(): void
    {
        $engine = Kreuzcrawl::createEngine(null);
        $result = Kreuzcrawl::scrape($engine, "");
        $this->assertStringContainsString("/gone", $result->final_url);
        $this->assertEquals(1, $result->redirect_count);
        $this->assertEquals(true, $result->is_error);
    }
}
