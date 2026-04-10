package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: metadata. */
class MetadataTest {
    @Test
    void testMetadataArticleTimes() throws Exception {
        // Extracts article:published_time, modified_time, author, section, and tags
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("2024-01-15T10:00:00Z", result.article().publishedTime());
        assertEquals("2024-06-20T14:30:00Z", result.article().modifiedTime());
        assertEquals("Jane Developer", result.article().author());
        assertEquals("Technology", result.article().section());
        assertEquals(3, result.article().tags().size());
    }

    @Test
    void testMetadataFavicons() throws Exception {
        // Extracts favicon link tags including apple-touch-icon
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals(5, result.favicons().size());
        assertFalse(result.favicons().get("").appleTouch().isEmpty(), "expected non-empty value");
    }

    @Test
    void testMetadataHeadings() throws Exception {
        // Extracts heading hierarchy (h1-h6) from HTML page
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals(1, result.headings().h1().size());
        assertEquals("Primary Heading", result.headings().h1().get("0").text());
        assertEquals(8, result.headings().size());
    }

    @Test
    void testMetadataHreflang() throws Exception {
        // Extracts hreflang alternate link tags
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals(4, result.hreflang().size());
        assertTrue(result.hreflang().get("").lang().contains("en"), "expected to contain: " + "en");
    }

    @Test
    void testMetadataKeywordsAuthor() throws Exception {
        // Extracts keywords, author, viewport, generator, theme-color, robots, lang, dir metadata
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("Comprehensive Metadata Test Page", result.metadata().title().orElse(""));
        assertFalse(result.metadata().canonicalUrl().orElse("").isEmpty(), "expected non-empty value");
        assertFalse(result.metadata().keywords().isEmpty(), "expected non-empty value");
        assertTrue(result.metadata().keywords().contains("rust"), "expected to contain: " + "rust");
        assertEquals("Jane Developer", result.metadata().author());
        assertFalse(result.metadata().viewport().isEmpty(), "expected non-empty value");
        assertEquals("kreuzcrawl/1.0", result.metadata().generator());
        assertEquals("#ff6600", result.metadata().themeColor());
        assertEquals("index, follow", result.metadata().robots());
        assertEquals("en", result.metadata().lang());
        assertEquals("ltr", result.metadata().dir());
    }

    @Test
    void testMetadataOgVideoAudio() throws Exception {
        // Extracts og:video, og:audio, and og:locale:alternate metadata
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertEquals("https://example.com/video.mp4", result.og().video());
        assertEquals("https://example.com/audio.mp3", result.og().audio());
        assertEquals(2, result.og().localeAlternate().size());
    }

    @Test
    void testMetadataResponseHeaders() throws Exception {
        // Extracts response metadata from HTTP headers (etag, server, content-language)
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertFalse(result.responseHeaders().etag().isEmpty(), "expected non-empty value");
        assertFalse(result.responseHeaders().lastModified().isEmpty(), "expected non-empty value");
        assertTrue(result.responseHeaders().server().contains("nginx"), "expected to contain: " + "nginx");
        assertEquals("en-US", result.responseHeaders().contentLanguage());
    }

    @Test
    void testMetadataWordCount() throws Exception {
        // Computes word count from visible page text
        var engine = Kreuzcrawl.createEngine(null);
        var result = Kreuzcrawl.scrape(engine, "");
        assertEquals(200, result.statusCode());
        assertTrue(result.computed().wordCount() > 99, "expected > 99");
        assertTrue(result.computed().wordCount() < 301, "expected < 301");
    }

}
