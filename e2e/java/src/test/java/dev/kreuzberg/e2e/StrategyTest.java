package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: strategy. */
class StrategyTest {
    @Test
    void testStrategyBestFirstSeed() throws Exception {
        // BestFirst strategy always processes the seed URL first
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(3, result.crawl().pagesCrawled());
        assertTrue(result.strategy().firstPageUrlContains().contains("/"), "expected to contain: " + "/");
    }

    @Test
    void testStrategyBfsDefaultOrder() throws Exception {
        // BFS strategy visits pages in breadth-first order
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(5, result.crawl().pagesCrawled());
        assertEquals(java.util.List.of("/", "/a", "/b", "/a/1", "/b/1"), result.strategy().crawlOrder());
    }

    @Test
    void testStrategyDfsDepthFirst() throws Exception {
        // DFS strategy visits pages in depth-first order
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(5, result.crawl().pagesCrawled());
        assertEquals(java.util.List.of("/", "/b", "/b/1", "/a", "/a/1"), result.strategy().crawlOrder());
    }

}
