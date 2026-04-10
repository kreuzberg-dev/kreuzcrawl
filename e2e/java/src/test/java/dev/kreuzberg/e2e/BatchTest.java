package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: batch. */
class BatchTest {
    @Test
    void testScrapeBatchBasic() throws Exception {
        // Batch scrape of multiple URLs all succeeding
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(3, result.batch().completedCount());
        assertEquals(0, result.batch().failedCount());
        assertEquals(3, result.batch().totalCount());
    }

    @Test
    void testScrapeBatchPartialFailure() throws Exception {
        // Batch scrape with one URL failing returns partial results
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(2, result.batch().completedCount());
        assertEquals(1, result.batch().failedCount());
        assertEquals(3, result.batch().totalCount());
    }

    @Test
    void testScrapeBatchProgress() throws Exception {
        // Batch scrape results include specific URL
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(2, result.batch().totalCount());
        assertTrue(result.batch().results().contains("/target"), "expected to contain: " + "/target");
    }

}
