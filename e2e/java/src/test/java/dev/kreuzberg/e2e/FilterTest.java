package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: filter. */
class FilterTest {
    @Test
    void testFilterBm25CrawlIntegration() throws Exception {
        // BM25 filter works during multi-page crawl, keeping relevant pages
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.filter().remainingContainKeyword().contains("rust"), "expected to contain: " + "rust");
    }

    @Test
    void testFilterBm25EmptyQuery() throws Exception {
        // BM25 filter with empty query passes all pages through
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(2, result.crawl().pagesCrawled());
    }

    @Test
    void testFilterBm25HighThreshold() throws Exception {
        // BM25 filter with very high threshold filters out all pages
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(0, result.filter().pagesAfterFilter());
    }

    @Test
    void testFilterBm25RelevantPages() throws Exception {
        // BM25 filter keeps only pages relevant to the query
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.filter().remainingContainKeyword().contains("rust"), "expected to contain: " + "rust");
    }

    @Test
    void testFilterBm25ThresholdZero() throws Exception {
        // BM25 filter with zero threshold passes all pages
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(2, result.crawl().pagesCrawled());
    }

    @Test
    void testFilterNoopCrawlAllKept() throws Exception {
        // NoopFilter keeps all pages during a multi-page crawl
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(3, result.filter().pagesAfterFilter());
    }

    @Test
    void testFilterNoopPassesAll() throws Exception {
        // No content filter passes all crawled pages through
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(3, result.crawl().pagesCrawled());
    }

}
