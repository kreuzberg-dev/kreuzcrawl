package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;
import com.fasterxml.jackson.databind.ObjectMapper;
import dev.kreuzberg.kreuzcrawl.CrawlConfig;

/** E2e tests for category: stream. */
class StreamTest {

    private static final ObjectMapper MAPPER = new ObjectMapper();
    @Test
    void testCrawlStreamEvents() throws Exception {
        // Crawl stream produces page and complete events
        var engineConfig = MAPPER.readValue("{\"max_depth\":1,\"respect_robots_txt\":false}", CrawlConfig.class);
        var engine = Kreuzcrawl.createEngine(engineConfig);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_stream_events";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'stream.event_count_min' not available on result type
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
    }

    @Test
    void testStreamDepthCrawl() throws Exception {
        // Stream produces events for multi-depth crawl with link following
        var engineConfig = MAPPER.readValue("{\"max_concurrent\":1,\"max_depth\":2}", CrawlConfig.class);
        var engine = Kreuzcrawl.createEngine(engineConfig);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/stream_depth_crawl";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'stream.event_count_min' not available on result type
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
    }

    @Test
    void testStreamWithErrorEvent() throws Exception {
        // Stream emits page and complete events even when some pages fail
        var engineConfig = MAPPER.readValue("{\"max_concurrent\":1,\"max_depth\":1}", CrawlConfig.class);
        var engine = Kreuzcrawl.createEngine(engineConfig);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/stream_with_error_event";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'stream.has_page_event' not available on result type
        // skipped: field 'stream.has_complete_event' not available on result type
        // skipped: field 'stream.event_count_min' not available on result type
    }

}
