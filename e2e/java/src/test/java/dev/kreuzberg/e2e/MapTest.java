package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: map. */
class MapTest {
    @Test
    void testMapDiscoverUrls() throws Exception {
        // Discovers all URLs on a site without fetching full content
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.urls().size() >= 3, "expected >= 3");
    }

    @Test
    void testMapExcludePatterns() throws Exception {
        // Excludes URLs matching patterns from URL map
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(1, result.urls().size());
    }

    @Test
    void testMapIncludeSubdomains() throws Exception {
        // Includes subdomain URLs in URL map discovery
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.urls().size() >= 2, "expected >= 2");
        assertTrue(result.urls().contains("blog.example.com"), "expected to contain: " + "blog.example.com");
    }

    @Test
    void testMapLargeSitemap() throws Exception {
        // Handles large sitemap with 100+ URLs
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.urls().size() >= 100, "expected >= 100");
    }

    @Test
    void testMapLimitPagination() throws Exception {
        // Limits map result count to specified maximum
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.urls().size() <= 5, "expected <= 5");
    }

    @Test
    void testMapSearchFilter() throws Exception {
        // Filters map results by search keyword
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertTrue(result.urls().size() >= 2, "expected >= 2");
        assertTrue(result.urls().contains("blog"), "expected to contain: " + "blog");
    }

}
