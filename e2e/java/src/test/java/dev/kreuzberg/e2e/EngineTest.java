package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: engine. */
class EngineTest {
    @Test
    void testEngineBatchBasic() throws Exception {
        // CrawlEngine with defaults batch scrapes like the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(2, result.batch().completedCount());
        assertEquals(2, result.batch().totalCount());
    }

    @Test
    void testEngineCrawlBasic() throws Exception {
        // CrawlEngine with defaults crawls multiple pages like the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(3, result.crawl().pagesCrawled());
        assertTrue(result.crawl().minPages() >= 3, "expected >= 3");
    }

    @Test
    void testEngineMapBasic() throws Exception {
        // CrawlEngine with defaults discovers URLs like the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.map().minUrls() >= 2, "expected >= 2");
    }

    @Test
    void testEngineScrapeBasic() throws Exception {
        // CrawlEngine with defaults scrapes a page identically to the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("text/html", result.contentType());
        assertEquals("Engine Test", result.metadata().title().orElse(""));
        assertTrue(result.metadata().descriptionContains().contains("Testing the engine"), "expected to contain: " + "Testing the engine");
        assertTrue(result.links().minCount() >= 1, "expected >= 1");
        assertEquals(1, result.headings().h1Count());
        assertEquals("Hello Engine", result.headings().h1Text());
    }

    @Test
    void testEngineStreamBasic() throws Exception {
        // CrawlEngine with defaults streams events like the free function
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.stream().hasPageEvent());
        assertEquals(true, result.stream().hasCompleteEvent());
        assertTrue(result.stream().eventCountMin() >= 3, "expected >= 3");
    }

}
