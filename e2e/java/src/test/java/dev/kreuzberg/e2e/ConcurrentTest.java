package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: concurrent. */
class ConcurrentTest {
    @Test
    void testConcurrentBasic() throws Exception {
        // Concurrent crawling fetches all pages with max_concurrent workers
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(6, result.pages().size());
        assertTrue(result.pages().size() >= 6, "expected >= 6");
    }

    @Test
    void testConcurrentDepthTwoFanOut() throws Exception {
        // Concurrent depth=2 crawl correctly fans out and deduplicates across levels
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(4, result.pages().size());
    }

    @Test
    void testConcurrentMaxPagesExact() throws Exception {
        // Concurrent crawling does not exceed max_pages limit even with high concurrency
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.pages().size() <= 3, "expected <= 3");
    }

    @Test
    void testConcurrentPartialErrors() throws Exception {
        // Concurrent crawl handles partial failures gracefully
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.pages().size() >= 2, "expected >= 2");
    }

    @Test
    void testConcurrentRespectsMaxPages() throws Exception {
        // Concurrent crawling respects max_pages limit
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.pages().size() <= 3, "expected <= 3");
    }

}
