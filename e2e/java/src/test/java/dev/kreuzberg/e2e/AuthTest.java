package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: auth. */
class AuthTest {
    @Test
    void testAuthBasicHttp() {
        // Sends HTTP Basic authentication header
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.auth_header_sent());
        assertEquals(200, result.status_code());
    }

    @Test
    void testAuthBearerToken() {
        // Sends Bearer token in Authorization header
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.auth_header_sent());
        assertEquals(200, result.status_code());
    }

    @Test
    void testAuthCustomHeader() {
        // Sends authentication via custom header (X-API-Key)
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.auth_header_sent());
        assertEquals(200, result.status_code());
    }

}
