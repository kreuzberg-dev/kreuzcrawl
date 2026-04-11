package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: filter. */
class FilterTest {
    @Test
    void testFilterBm25CrawlIntegration() throws Exception {
        // BM25 filter works during multi-page crawl, keeping relevant pages
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/filter_bm25_crawl_integration";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'filter.remaining_contain_keyword' not available on result type
    }

    @Test
    void testFilterBm25EmptyQuery() throws Exception {
        // BM25 filter with empty query passes all pages through
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/filter_bm25_empty_query";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    @Test
    void testFilterBm25HighThreshold() throws Exception {
        // BM25 filter with very high threshold filters out all pages
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/filter_bm25_high_threshold";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'filter.pages_after_filter' not available on result type
    }

    @Test
    void testFilterBm25RelevantPages() throws Exception {
        // BM25 filter keeps only pages relevant to the query
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/filter_bm25_relevant_pages";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'filter.remaining_contain_keyword' not available on result type
    }

    @Test
    void testFilterBm25ThresholdZero() throws Exception {
        // BM25 filter with zero threshold passes all pages
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/filter_bm25_threshold_zero";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

    @Test
    void testFilterNoopCrawlAllKept() throws Exception {
        // NoopFilter keeps all pages during a multi-page crawl
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/filter_noop_crawl_all_kept";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'filter.pages_after_filter' not available on result type
    }

    @Test
    void testFilterNoopPassesAll() throws Exception {
        // No content filter passes all crawled pages through
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/filter_noop_passes_all";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'crawl.pages_crawled' not available on result type
    }

}
