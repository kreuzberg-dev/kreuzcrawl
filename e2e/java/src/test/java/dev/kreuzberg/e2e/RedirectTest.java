package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: redirect. */
class RedirectTest {
    @Test
    void testRedirect301Permanent() throws Exception {
        // Follows 301 permanent redirect and returns final page content
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/target"), "expected to contain: " + "/target");
        assertEquals(1, result.redirectCount());
    }

    @Test
    void testRedirect302Found() throws Exception {
        // Follows 302 Found redirect correctly
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/found-target"), "expected to contain: " + "/found-target");
        assertEquals(1, result.redirectCount());
    }

    @Test
    void testRedirect303SeeOther() throws Exception {
        // Follows 303 See Other redirect (method changes to GET)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/see-other"), "expected to contain: " + "/see-other");
        assertEquals(1, result.redirectCount());
    }

    @Test
    void testRedirect307Temporary() throws Exception {
        // Follows 307 Temporary Redirect (preserves method)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/temp-target"), "expected to contain: " + "/temp-target");
        assertEquals(1, result.redirectCount());
    }

    @Test
    void testRedirect308Permanent() throws Exception {
        // Follows 308 Permanent Redirect (preserves method)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/perm-target"), "expected to contain: " + "/perm-target");
        assertEquals(1, result.redirectCount());
    }

    @Test
    void testRedirectChain() throws Exception {
        // Follows a chain of redirects (301 -> 302 -> 200)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/step2"), "expected to contain: " + "/step2");
        assertEquals(2, result.redirectCount());
    }

    @Test
    void testRedirectCrossDomain() throws Exception {
        // Reports cross-domain redirect target without following to external domain
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/external-redirect"), "expected to contain: " + "/external-redirect");
        assertEquals(1, result.redirectCount());
    }

    @Test
    void testRedirectLoop() throws Exception {
        // Detects redirect loop (A -> B -> A) and returns error
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isError());
    }

    @Test
    void testRedirectMaxExceeded() throws Exception {
        // Aborts when redirect count exceeds max_redirects limit
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.isError());
    }

    @Test
    void testRedirectMetaRefresh() throws Exception {
        // Follows HTML meta-refresh redirect to target page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/target"), "expected to contain: " + "/target");
        assertEquals(1, result.redirectCount());
    }

    @Test
    void testRedirectRefreshHeader() throws Exception {
        // Handles HTTP Refresh header redirect
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/refreshed"), "expected to contain: " + "/refreshed");
        assertEquals(1, result.redirectCount());
    }

    @Test
    void testRedirectTo404() throws Exception {
        // Redirect target returns 404 Not Found
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.finalUrl().contains("/gone"), "expected to contain: " + "/gone");
        assertEquals(1, result.redirectCount());
        assertEquals(true, result.isError());
    }

}
