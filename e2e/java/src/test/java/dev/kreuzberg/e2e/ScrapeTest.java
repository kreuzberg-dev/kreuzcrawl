package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: scrape. */
class ScrapeTest {
    @Test
    void testScrapeAssetDedup() {
        // Same asset linked twice results in one download with one unique hash
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals(2, result.assets().size());
        assertEquals(2, result.assets().unique_hashes());
    }

    @Test
    void testScrapeAssetMaxSize() {
        // Skips assets exceeding max_asset_size limit
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals(2, result.assets().size());
    }

    @Test
    void testScrapeAssetTypeFilter() {
        // Only downloads image assets when asset_types filter is set
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals(1, result.assets().size());
        assertTrue(result.assets().get("").category().contains("image"), "expected to contain: " + "image");
    }

    @Test
    void testScrapeBasicHtmlPage() {
        // Scrapes a simple HTML page and extracts title, description, and links
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals("text/html", result.content_type());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertEquals("Example Domain", result.metadata().title());
        assertTrue(result.metadata().description().contains("illustrative examples"), "expected to contain: " + "illustrative examples");
        assertFalse(result.metadata().canonical_url().isEmpty(), "expected non-empty value");
        assertTrue(result.links().size() > 0, "expected > 0");
        assertTrue(result.links().get("").link_type().contains("external"), "expected to contain: " + "external");
        assertEquals(0, result.images().size());
        assertTrue(result.og().title().isEmpty(), "expected empty value");
    }

    @Test
    void testScrapeComplexLinks() {
        // Classifies links by type: internal, external, anchor, document, image
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertTrue(result.links().size() > 9, "expected > 9");
        assertTrue(result.links().get("").link_type().contains("internal"), "expected to contain: " + "internal");
        assertTrue(result.links().get("").link_type().contains("external"), "expected to contain: " + "external");
        assertTrue(result.links().get("").link_type().contains("anchor"), "expected to contain: " + "anchor");
        assertTrue(result.links().get("").link_type().contains("document"), "expected to contain: " + "document");
    }

    @Test
    void testScrapeDownloadAssets() {
        // Downloads CSS, JS, and image assets from page
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertTrue(result.assets().size() > 2, "expected > 2");
    }

    @Test
    void testScrapeDublinCore() {
        // Extracts Dublin Core metadata from a page
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.dublin_core().title().isEmpty(), "expected non-empty value");
        assertEquals("Effects of Climate Change on Marine Biodiversity", result.dublin_core().title());
        assertEquals("Dr. Jane Smith", result.dublin_core().creator());
    }

    @Test
    void testScrapeEmptyPage() {
        // Handles an empty HTML document without errors
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertTrue(result.links().size() > -1, "expected > -1");
        assertEquals(0, result.images().size());
    }

    @Test
    void testScrapeFeedDiscovery() {
        // Discovers RSS, Atom, and JSON feed links
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals(1, result.feeds().rss().size());
        assertEquals(1, result.feeds().atom().size());
        assertEquals(1, result.feeds().json_feed().size());
    }

    @Test
    void testScrapeImageSources() {
        // Extracts images from img, picture, og:image, twitter:image
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertTrue(result.images().size() > 4, "expected > 4");
        assertEquals("https://example.com/images/og-hero.jpg", result.og().image());
    }

    @Test
    void testScrapeJsHeavySpa() {
        // Handles SPA page with JavaScript-only content (no server-rendered HTML)
        var result = Kreuzcrawl.scrape();
        assertFalse(result.html().isEmpty(), "expected non-empty value");
    }

    @Test
    void testScrapeJsonLd() {
        // Extracts JSON-LD structured data from a page
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.json_ld().isEmpty(), "expected non-empty value");
        assertEquals("Recipe", result.json_ld().type());
        assertEquals("Best Chocolate Cake", result.json_ld().name());
    }

    @Test
    void testScrapeMalformedHtml() {
        // Gracefully handles broken HTML without crashing
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertTrue(result.metadata().description().contains("broken HTML"), "expected to contain: " + "broken HTML");
    }

    @Test
    void testScrapeOgMetadata() {
        // Extracts full Open Graph metadata from a page
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.og().title().isEmpty(), "expected non-empty value");
        assertEquals("Article Title", result.og().title());
        assertEquals("article", result.og().type());
        assertEquals("https://example.com/images/article-hero.jpg", result.og().image());
        assertFalse(result.og().description().isEmpty(), "expected non-empty value");
        assertEquals("Article Title - Example Blog", result.metadata().title());
    }

    @Test
    void testScrapeTwitterCard() {
        // Extracts Twitter Card metadata from a page
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.twitter().card().isEmpty(), "expected non-empty value");
        assertEquals("summary_large_image", result.twitter().card_type());
        assertEquals("New Product Launch", result.twitter().title());
    }

}
