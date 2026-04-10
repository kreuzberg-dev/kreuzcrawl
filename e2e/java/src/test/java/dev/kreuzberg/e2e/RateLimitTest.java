package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: rate_limit. */
class RateLimitTest {
    @Test
    void testRateLimitBasicDelay() throws Exception {
        // Rate limiter adds delay between requests to the same domain
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(3, result.crawl().pagesCrawled());
        assertTrue(result.rateLimit().minDurationMs() >= 150, "expected >= 150");
    }

    @Test
    void testRateLimitZeroNoDelay() throws Exception {
        // Rate limiter with zero delay does not slow crawling
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(2, result.crawl().pagesCrawled());
    }

}
