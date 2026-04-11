package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import dev.kreuzberg.kreuzcrawl.Kreuzcrawl;

/** E2e tests for category: validation. */
class ValidationTest {
    @Test
    void testValidationInvalidExcludeRegex() throws Exception {
        // Invalid regex in exclude_paths is rejected
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/validation_invalid_exclude_regex";
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape(engine, url));
    }

    @Test
    void testValidationInvalidIncludeRegex() throws Exception {
        // Invalid regex in include_paths is rejected
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/validation_invalid_include_regex";
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape(engine, url));
    }

    @Test
    void testValidationInvalidRetryCode() throws Exception {
        // Retry code outside 100-599 is rejected
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/validation_invalid_retry_code";
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape(engine, url));
    }

    @Test
    void testValidationMaxPagesZero() throws Exception {
        // max_pages=0 is rejected as invalid config
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/validation_max_pages_zero";
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape(engine, url));
    }

    @Test
    void testValidationMaxRedirectsTooHigh() throws Exception {
        // max_redirects > 100 is rejected as invalid config
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/validation_max_redirects_too_high";
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape(engine, url));
    }

    @Test
    void testValidationTimeoutZero() throws Exception {
        // Zero request timeout is rejected as invalid config
        var engine = Kreuzcrawl.createEngine(null);
        String url = System.getenv("MOCK_SERVER_URL") + "/fixtures/validation_timeout_zero";
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape(engine, url));
    }

}
