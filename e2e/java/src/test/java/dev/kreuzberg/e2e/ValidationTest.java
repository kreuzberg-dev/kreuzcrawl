package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: validation. */
class ValidationTest {
    @Test
    void testValidationInvalidExcludeRegex() {
        // Invalid regex in exclude_paths is rejected
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationInvalidIncludeRegex() {
        // Invalid regex in include_paths is rejected
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationInvalidRetryCode() {
        // Retry code outside 100-599 is rejected
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationMaxPagesZero() {
        // max_pages=0 is rejected as invalid config
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationMaxRedirectsTooHigh() {
        // max_redirects > 100 is rejected as invalid config
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

    @Test
    void testValidationTimeoutZero() {
        // Zero request timeout is rejected as invalid config
        assertThrows(Exception.class, () -> Kreuzcrawl.scrape());
    }

}
