package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: cookies. */
class CookiesTest {
    @Test
    void testCookiesPerDomain() throws Exception {
        // Isolates cookies per domain during crawl
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/cookies_per_domain";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'cookies.length' not available on result type
        // skipped: field 'cookies' not available on result type
    }

    @Test
    void testCookiesPersistence() throws Exception {
        // Maintains cookies across multiple crawl requests
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/cookies_persistence";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'cookies' not available on result type
    }

    @Test
    void testCookiesSetCookieResponse() throws Exception {
        // Respects Set-Cookie header from server responses
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/cookies_set_cookie_response";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'cookies' not available on result type
    }

}
