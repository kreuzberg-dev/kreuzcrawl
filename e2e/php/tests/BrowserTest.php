<?php

declare(strict_types=1);

namespace Kreuzberg\E2e;

use PHPUnit\Framework\TestCase;
use Kreuzcrawl\Kreuzcrawl;

/** E2e tests for category: browser. */
final class BrowserTest extends TestCase
{
    /** Browser mode 'auto' without browser feature enabled does not use browser */
    public function test_browser_config_auto_no_feature(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(true, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Browser mode 'never' prevents browser fallback even for SPA shell content */
    public function test_browser_config_never_mode(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(true, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Does NOT flag a short but real content page as needing JS rendering */
    public function test_browser_detect_minimal_page(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(false, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Detects Next.js page with __NEXT_DATA__ but no rendered content as needing JS rendering */
    public function test_browser_detect_next_empty(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(true, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Does NOT flag Next.js page with full SSR content as needing JS rendering */
    public function test_browser_detect_next_rendered(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(true, $result->html_not_empty);
        $this->assertEquals(false, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Does NOT flag a normal server-rendered page as needing JS rendering */
    public function test_browser_detect_normal_page(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(false, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Detects Nuxt SPA shell with empty #__nuxt div as needing JS rendering */
    public function test_browser_detect_nuxt_shell(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(true, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Detects React SPA shell with empty #root div as needing JS rendering */
    public function test_browser_detect_react_shell(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(true, $result->html_not_empty);
        $this->assertEquals(true, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Detects Vue SPA shell with empty #app div as needing JS rendering */
    public function test_browser_detect_vue_shell(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(200, $result->status_code);
        $this->assertEquals(true, $result->browser->js_render_hint);
        $this->assertEquals(false, $result->browser->browser_used);
    }

    /** Browser auto re-fetches SPA shell when JS rendering is detected */
    public function test_browser_fallback_spa_render(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(true, $result->browser->js_render_hint);
        $this->assertEquals(true, $result->browser->browser_used);
    }

    /** Browser fallback triggers when WAF blocks the HTTP request (Cloudflare 403) */
    public function test_browser_fallback_waf_blocked(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(true, $result->browser->browser_used);
    }

    /** Browser mode 'always' uses browser even for normal server-rendered pages */
    public function test_browser_mode_always(): void
    {
        $result = Kreuzcrawl::scrape();
        $this->assertEquals(true, $result->browser->browser_used);
    }
}
