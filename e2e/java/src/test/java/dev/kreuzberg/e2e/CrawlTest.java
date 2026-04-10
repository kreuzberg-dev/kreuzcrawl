package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: crawl. */
class CrawlTest {
    @Test
    void testContentBinarySkip() {
        // Skips image and video content types gracefully
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.content().was_skipped());
    }

    @Test
    void testContentPdfLinkSkip() {
        // Encounters PDF link and skips or marks as document type
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.content().was_skipped());
    }

    @Test
    void testCrawlConcurrentDepth() {
        // Concurrent crawl respects max_depth limit
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.pages().size());
        assertEquals(true, result.stayed_on_domain());
    }

    @Test
    void testCrawlConcurrentLimit() {
        // Respects max concurrent requests limit during crawl
        var result = Kreuzcrawl.scrape();
        assertEquals(5, result.pages().size());
    }

    @Test
    void testCrawlConcurrentMaxPages() {
        // Concurrent crawl respects max_pages budget
        var result = Kreuzcrawl.scrape();
        assertTrue(result.pages().size() <= 3, "expected <= 3");
    }

    @Test
    void testCrawlCustomHeaders() {
        // Sends custom headers on all crawl requests
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.pages().size());
    }

    @Test
    void testCrawlDepthOne() {
        // Follows links one level deep from start page
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.pages().size());
        assertEquals(true, result.stayed_on_domain());
    }

    @Test
    void testCrawlDepthPriority() {
        // Crawls in breadth-first order, processing depth-0 pages before depth-1
        var result = Kreuzcrawl.scrape();
        assertEquals(4, result.pages().size());
    }

    @Test
    void testCrawlDepthTwo() {
        // Crawls 3 levels deep (depth 0, 1, 2)
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.pages().size());
        assertTrue(result.pages().size() >= 3, "expected >= 3");
    }

    @Test
    void testCrawlDepthTwoChain() {
        // Depth=2 crawl follows a chain of links across three levels
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.pages().size());
    }

    @Test
    void testCrawlDoubleSlashNormalization() {
        // Normalizes double slashes in URL paths (//page to /page)
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.unique_urls().size());
    }

    @Test
    void testCrawlEmptyPageNoLinks() {
        // Crawl completes when child page has no outgoing links
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.pages().size());
    }

    @Test
    void testCrawlExcludePathPattern() {
        // Skips URLs matching the exclude path pattern
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.pages().size());
    }

    @Test
    void testCrawlExternalLinksIgnored() {
        // External links are discovered but not followed when stay_on_domain is true
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.pages().size());
        assertEquals(true, result.stayed_on_domain());
    }

    @Test
    void testCrawlFragmentStripping() {
        // Strips #fragment from URLs for deduplication
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.unique_urls().size());
    }

    @Test
    void testCrawlIncludePathPattern() {
        // Only follows URLs matching the include path pattern
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.pages().size());
    }

    @Test
    void testCrawlMaxDepthZero() {
        // max_depth=0 crawls only the seed page with no link following
        var result = Kreuzcrawl.scrape();
        assertEquals(1, result.pages().size());
        assertTrue(result.pages().size() <= 1, "expected <= 1");
    }

    @Test
    void testCrawlMaxPages() {
        // Stops crawling at page budget limit
        var result = Kreuzcrawl.scrape();
        assertTrue(result.pages().size() <= 3, "expected <= 3");
    }

    @Test
    void testCrawlMixedContentTypes() {
        // Crawl handles links to non-HTML content types gracefully
        var result = Kreuzcrawl.scrape();
        assertTrue(result.pages().size() >= 2, "expected >= 2");
    }

    @Test
    void testCrawlMultipleRedirectsInTraversal() {
        // Multiple linked pages with redirects are handled during crawl traversal
        var result = Kreuzcrawl.scrape();
        assertTrue(result.pages().size() >= 1, "expected >= 1");
    }

    @Test
    void testCrawlQueryParamDedup() {
        // Deduplicates URLs with same query params in different order
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.unique_urls().size());
    }

    @Test
    void testCrawlRedirectInTraversal() {
        // Links that redirect are followed during crawl traversal
        var result = Kreuzcrawl.scrape();
        assertTrue(result.pages().size() >= 1, "expected >= 1");
    }

    @Test
    void testCrawlSelfLinkNoLoop() {
        // Page linking to itself does not cause infinite crawl loop
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.pages().size());
    }

    @Test
    void testCrawlSinglePageNoLinks() {
        // Crawling a page with no links returns only the seed page
        var result = Kreuzcrawl.scrape();
        assertEquals(1, result.pages().size());
    }

    @Test
    void testCrawlStayOnDomain() {
        // Does not follow external links when stay_on_domain is true
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.pages().size());
        assertEquals(true, result.stayed_on_domain());
    }

    @Test
    void testCrawlSubdomainExclusion() {
        // Stays on exact domain and skips subdomain links
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.pages().size());
        assertEquals(true, result.stayed_on_domain());
    }

    @Test
    void testCrawlSubdomainInclusion() {
        // Crawls subdomains when allow_subdomains is enabled
        var result = Kreuzcrawl.scrape();
        assertTrue(result.pages().size() >= 2, "expected >= 2");
    }

    @Test
    void testCrawlTrailingSlashDedup() {
        // Deduplicates /page and /page/ as the same URL
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.unique_urls().size());
    }

    @Test
    void testCrawlUrlDeduplication() {
        // Deduplicates URLs that differ only by fragment or query params
        var result = Kreuzcrawl.scrape();
        assertTrue(result.pages().size() <= 2, "expected <= 2");
    }

}
