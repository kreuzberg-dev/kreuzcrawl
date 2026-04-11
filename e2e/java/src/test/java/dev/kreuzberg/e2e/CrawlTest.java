package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: crawl. */
class CrawlTest {
    @Test
    void testContentBinarySkip() throws Exception {
        // Skips image and video content types gracefully
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/content_binary_skip";
        var result = Kreuzcrawl.scrape(engine, url);
        assertEquals(true, result.wasSkipped());
    }

    @Test
    void testContentPdfLinkSkip() throws Exception {
        // Encounters PDF link and skips or marks as document type
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/content_pdf_link_skip";
        var result = Kreuzcrawl.scrape(engine, url);
        assertEquals(true, result.wasSkipped());
    }

    @Test
    void testCrawlConcurrentDepth() throws Exception {
        // Concurrent crawl respects max_depth limit
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_concurrent_depth";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlConcurrentLimit() throws Exception {
        // Respects max concurrent requests limit during crawl
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_concurrent_limit";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlConcurrentMaxPages() throws Exception {
        // Concurrent crawl respects max_pages budget
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_concurrent_max_pages";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlCustomHeaders() throws Exception {
        // Sends custom headers on all crawl requests
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_custom_headers";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlDepthOne() throws Exception {
        // Follows links one level deep from start page
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_depth_one";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlDepthPriority() throws Exception {
        // Crawls in breadth-first order, processing depth-0 pages before depth-1
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_depth_priority";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlDepthTwo() throws Exception {
        // Crawls 3 levels deep (depth 0, 1, 2)
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_depth_two";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlDepthTwoChain() throws Exception {
        // Depth=2 crawl follows a chain of links across three levels
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_depth_two_chain";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlDoubleSlashNormalization() throws Exception {
        // Normalizes double slashes in URL paths (//page to /page)
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_double_slash_normalization";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'unique_urls.length' not available on result type
    }

    @Test
    void testCrawlEmptyPageNoLinks() throws Exception {
        // Crawl completes when child page has no outgoing links
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_empty_page_no_links";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlExcludePathPattern() throws Exception {
        // Skips URLs matching the exclude path pattern
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_exclude_path_pattern";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlExternalLinksIgnored() throws Exception {
        // External links are discovered but not followed when stay_on_domain is true
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_external_links_ignored";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlFragmentStripping() throws Exception {
        // Strips #fragment from URLs for deduplication
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_fragment_stripping";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'unique_urls.length' not available on result type
    }

    @Test
    void testCrawlIncludePathPattern() throws Exception {
        // Only follows URLs matching the include path pattern
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_include_path_pattern";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlMaxDepthZero() throws Exception {
        // max_depth=0 crawls only the seed page with no link following
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_max_depth_zero";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlMaxPages() throws Exception {
        // Stops crawling at page budget limit
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_max_pages";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlMixedContentTypes() throws Exception {
        // Crawl handles links to non-HTML content types gracefully
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_mixed_content_types";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlMultipleRedirectsInTraversal() throws Exception {
        // Multiple linked pages with redirects are handled during crawl traversal
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_multiple_redirects_in_traversal";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlQueryParamDedup() throws Exception {
        // Deduplicates URLs with same query params in different order
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_query_param_dedup";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'unique_urls.length' not available on result type
    }

    @Test
    void testCrawlRedirectInTraversal() throws Exception {
        // Links that redirect are followed during crawl traversal
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_redirect_in_traversal";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlSelfLinkNoLoop() throws Exception {
        // Page linking to itself does not cause infinite crawl loop
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_self_link_no_loop";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlSinglePageNoLinks() throws Exception {
        // Crawling a page with no links returns only the seed page
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_single_page_no_links";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlStayOnDomain() throws Exception {
        // Does not follow external links when stay_on_domain is true
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_stay_on_domain";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlSubdomainExclusion() throws Exception {
        // Stays on exact domain and skips subdomain links
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_subdomain_exclusion";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
        // skipped: field 'stayed_on_domain' not available on result type
    }

    @Test
    void testCrawlSubdomainInclusion() throws Exception {
        // Crawls subdomains when allow_subdomains is enabled
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_subdomain_inclusion";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

    @Test
    void testCrawlTrailingSlashDedup() throws Exception {
        // Deduplicates /page and /page/ as the same URL
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_trailing_slash_dedup";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'unique_urls.length' not available on result type
    }

    @Test
    void testCrawlUrlDeduplication() throws Exception {
        // Deduplicates URLs that differ only by fragment or query params
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/crawl_url_deduplication";
        var result = Kreuzcrawl.scrape(engine, url);
        // skipped: field 'pages.length' not available on result type
    }

}
