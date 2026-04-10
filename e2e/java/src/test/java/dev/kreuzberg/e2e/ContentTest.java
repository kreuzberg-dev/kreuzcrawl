package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: content. */
class ContentTest {
    @Test
    void testContent204NoContent() {
        // Handles 204 No Content response gracefully
        var result = Kreuzcrawl.scrape();
        assertEquals(204, result.status_code());
        assertTrue(result.html().isEmpty(), "expected empty value");
    }

    @Test
    void testContentCharsetIso8859() {
        // Handles ISO-8859-1 encoded page correctly
        var result = Kreuzcrawl.scrape();
        assertEquals("iso-8859-1", result.content().detected_charset());
    }

    @Test
    void testContentEmptyBody() {
        // Handles 200 response with empty body gracefully
        var result = Kreuzcrawl.scrape();
        assertEquals(200, result.status_code());
    }

    @Test
    void testContentGzipCompressed() {
        // Handles response with Accept-Encoding gzip negotiation
        var result = Kreuzcrawl.scrape();
        assertFalse(result.html().isEmpty(), "expected non-empty value");
        assertEquals(200, result.status_code());
    }

    @Test
    void testContentLargePageLimit() {
        // Respects max body size limit and truncates or skips oversized pages
        var result = Kreuzcrawl.scrape();
        assertTrue(result.content().body_size() < 1025, "expected < 1025");
    }

    @Test
    void testContentMainOnly() {
        // Extracts only main content area, excluding nav, sidebar, footer
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.content().main_content_only());
    }

    @Test
    void testContentPdfNoExtension() {
        // Detects PDF content by Content-Type header when URL has no .pdf extension
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.content().is_pdf());
    }

    @Test
    void testContentRemoveTags() {
        // Removes specified HTML elements by CSS selector before processing
        var result = Kreuzcrawl.scrape();
        assertFalse(result.html().isEmpty(), "expected non-empty value");
    }

    @Test
    void testContentUtf8Bom() {
        // Handles UTF-8 content with BOM marker correctly
        var result = Kreuzcrawl.scrape();
        assertEquals("utf-8", result.content().detected_charset());
        assertFalse(result.html().isEmpty(), "expected non-empty value");
    }

}
