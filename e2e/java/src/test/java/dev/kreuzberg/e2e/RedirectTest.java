package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: redirect. */
class RedirectTest {
    @Test
    void testRedirect301Permanent() throws Exception {
        // Follows 301 permanent redirect and returns final page content
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_301_permanent";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirect302Found() throws Exception {
        // Follows 302 Found redirect correctly
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_302_found";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirect303SeeOther() throws Exception {
        // Follows 303 See Other redirect (method changes to GET)
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_303_see_other";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirect307Temporary() throws Exception {
        // Follows 307 Temporary Redirect (preserves method)
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_307_temporary";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirect308Permanent() throws Exception {
        // Follows 308 Permanent Redirect (preserves method)
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_308_permanent";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirectChain() throws Exception {
        // Follows a chain of redirects (301 -> 302 -> 200)
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_chain";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirectCrossDomain() throws Exception {
        // Reports cross-domain redirect target without following to external domain
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_cross_domain";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirectLoop() throws Exception {
        // Detects redirect loop (A -> B -> A) and returns error
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_loop";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'is_error' not available on result type
    }

    @Test
    void testRedirectMaxExceeded() throws Exception {
        // Aborts when redirect count exceeds max_redirects limit
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_max_exceeded";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'is_error' not available on result type
    }

    @Test
    void testRedirectMetaRefresh() throws Exception {
        // Follows HTML meta-refresh redirect to target page
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_meta_refresh";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirectRefreshHeader() throws Exception {
        // Handles HTTP Refresh header redirect
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_refresh_header";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
    }

    @Test
    void testRedirectTo404() throws Exception {
        // Redirect target returns 404 Not Found
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/redirect_to_404";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'final_url' not available on result type
        // skipped: field 'redirect_count' not available on result type
        // skipped: field 'is_error' not available on result type
    }

}
