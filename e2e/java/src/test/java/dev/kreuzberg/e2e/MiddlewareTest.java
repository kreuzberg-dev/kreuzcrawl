package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: middleware. */
class MiddlewareTest {
    @Test
    void testMiddlewareEngineCrawlWithDefaults() throws Exception {
        // Engine crawl with default middleware chain produces correct multi-page results
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/middleware_engine_crawl_with_defaults";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
        // skipped: field 'crawl.min_pages' not available on result type
    }

    @Test
    void testMiddlewareNoopNoEffect() throws Exception {
        // Default middleware chain does not affect normal scraping
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/middleware_noop_no_effect";
        var result = Kreuzcrawl.scrape(engine, url);
        assertEquals(200, result.statusCode());
        assertEquals("Middleware Test", result.metadata().title().orElse(""));
    }

}
