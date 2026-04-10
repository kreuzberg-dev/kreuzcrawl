package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: cookies. */
class CookiesTest {
    @Test
    void testCookiesPerDomain() throws Exception {
        // Isolates cookies per domain during crawl
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(1, result.cookies().size());
        assertTrue(result.cookies().contains("domain_cookie"), "expected to contain: " + "domain_cookie");
    }

    @Test
    void testCookiesPersistence() throws Exception {
        // Maintains cookies across multiple crawl requests
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.cookies().contains("session"), "expected to contain: " + "session");
    }

    @Test
    void testCookiesSetCookieResponse() throws Exception {
        // Respects Set-Cookie header from server responses
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.cookies().contains("tracking"), "expected to contain: " + "tracking");
    }

}
