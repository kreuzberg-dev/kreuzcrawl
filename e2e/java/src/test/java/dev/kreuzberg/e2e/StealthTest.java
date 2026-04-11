package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: stealth. */
class StealthTest {
    @Test
    void testStealthUaRotationConfig() throws Exception {
        // User-agent rotation config is accepted and crawl succeeds
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/stealth_ua_rotation_config";
        var result = Kreuzcrawl.scrape(engine, url);
        assertEquals(200, result.statusCode());
    }

}
