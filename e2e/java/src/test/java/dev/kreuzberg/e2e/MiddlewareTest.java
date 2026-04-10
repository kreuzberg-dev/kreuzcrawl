package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: middleware. */
class MiddlewareTest {
    @Test
    void testMiddlewareEngineCrawlWithDefaults() throws Exception {
        // Engine crawl with default middleware chain produces correct multi-page results
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(3, result.crawl().pagesCrawled());
        assertTrue(result.crawl().minPages() >= 3, "expected >= 3");
    }

    @Test
    void testMiddlewareNoopNoEffect() throws Exception {
        // Default middleware chain does not affect normal scraping
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("Middleware Test", result.metadata().title().orElse(""));
    }

}
