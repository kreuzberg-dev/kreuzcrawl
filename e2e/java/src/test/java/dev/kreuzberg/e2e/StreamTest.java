package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: stream. */
class StreamTest {
    @Test
    void testCrawlStreamEvents() throws Exception {
        // Crawl stream produces page and complete events
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.stream().eventCountMin() >= 4, "expected >= 4");
        assertEquals(true, result.stream().hasPageEvent());
        assertEquals(true, result.stream().hasCompleteEvent());
    }

    @Test
    void testStreamDepthCrawl() throws Exception {
        // Stream produces events for multi-depth crawl with link following
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.stream().eventCountMin() >= 5, "expected >= 5");
        assertEquals(true, result.stream().hasPageEvent());
        assertEquals(true, result.stream().hasCompleteEvent());
    }

    @Test
    void testStreamWithErrorEvent() throws Exception {
        // Stream emits page and complete events even when some pages fail
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(true, result.stream().hasPageEvent());
        assertEquals(true, result.stream().hasCompleteEvent());
        assertTrue(result.stream().eventCountMin() >= 2, "expected >= 2");
    }

}
