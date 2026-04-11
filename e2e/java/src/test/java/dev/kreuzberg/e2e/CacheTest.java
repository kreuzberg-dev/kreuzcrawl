package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: cache. */
class CacheTest {
    @Test
    void testCacheBasic() throws Exception {
        // Crawling with disk cache enabled succeeds without errors
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/cache_basic";
        var result = Kreuzcrawl.scrape(engine, url);
        assertEquals(200, result.statusCode());
    }

}
