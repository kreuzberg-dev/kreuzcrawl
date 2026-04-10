package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: sitemap. */
class SitemapTest {
    @Test
    void testSitemapBasic() {
        // Parses a standard urlset sitemap
        var result = Kreuzcrawl.scrape();
        assertEquals(4, result.urls().size());
        assertEquals(true, result.has_lastmod());
    }

    @Test
    void testSitemapCompressedGzip() {
        // Parses a gzip-compressed sitemap file
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.urls().size());
    }

    @Test
    void testSitemapEmpty() {
        // Handles empty sitemap gracefully
        var result = Kreuzcrawl.scrape();
        assertEquals(0, result.urls().size());
    }

    @Test
    void testSitemapFromRobotsTxt() {
        // Discovers sitemap via robots.txt Sitemap directive
        var result = Kreuzcrawl.scrape();
        assertEquals(4, result.urls().size());
    }

    @Test
    void testSitemapIndex() {
        // Follows sitemap index to discover child sitemaps
        var result = Kreuzcrawl.scrape();
        assertEquals(3, result.urls().size());
    }

    @Test
    void testSitemapLastmodFilter() {
        // Filters sitemap URLs by lastmod date
        var result = Kreuzcrawl.scrape();
        assertEquals(4, result.urls().size());
        assertEquals(true, result.has_lastmod());
    }

    @Test
    void testSitemapOnlyMode() {
        // Uses sitemap URLs exclusively without following page links
        var result = Kreuzcrawl.scrape();
        assertEquals(4, result.urls().size());
    }

    @Test
    void testSitemapXhtmlLinks() {
        // Parses sitemap with XHTML namespace alternate links
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.urls().size());
        assertEquals(false, result.has_lastmod());
    }

}
