package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: markdown. */
class MarkdownTest {
    @Test
    void testMarkdownBasicConversion() {
        // HTML is always converted to markdown alongside raw HTML
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertEquals("Test", result.metadata().title());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertFalse(result.markdown().isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().contains("Hello World"), "expected to contain: " + "Hello World");
    }

    @Test
    void testMarkdownCrawlAllPages() {
        // All crawled pages have markdown field populated
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.crawl().pages_crawled());
    }

    @Test
    void testMarkdownFitContent() {
        // Fit markdown removes navigation and boilerplate content
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.markdown().isEmpty(), "expected non-empty value");
    }

    @Test
    void testMarkdownHeadingsAndParagraphs() {
        // Markdown conversion preserves heading hierarchy and paragraph text
        var result = Kreuzcrawl.scrape();
        assertFalse(result.markdown().isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().contains("Main Title"), "expected to contain: " + "Main Title");
    }

    @Test
    void testMarkdownLinksConverted() {
        // HTML links are converted to markdown link syntax
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertFalse(result.markdown().isEmpty(), "expected non-empty value");
        assertTrue(result.markdown().contains("Example"), "expected to contain: " + "Example");
    }

    @Test
    void testMarkdownWithCitations() {
        // Markdown includes citation conversion with numbered references
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
        assertFalse(result.markdown().isEmpty(), "expected non-empty value");
    }

}
