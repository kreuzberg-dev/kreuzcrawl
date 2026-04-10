package dev.kreuzberg.e2e;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

/** E2e tests for category: robots. */
class RobotsTest {
    @Test
    void testRobotsAllowAll() {
        // Permissive robots.txt allows all paths
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsAllowOverride() {
        // Allow directive overrides Disallow for specific paths
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsCommentsHandling() {
        // Correctly parses robots.txt with inline and line comments
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsCrawlDelay() {
        // Respects crawl-delay directive from robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(2, result.robots().crawl_delay());
    }

    @Test
    void testRobotsDisallowPath() {
        // Robots.txt disallows specific paths
        var result = Kreuzcrawl.scrape();
        assertEquals(false, result.robots().is_allowed());
    }

    @Test
    void testRobotsMetaNofollow() {
        // Detects nofollow meta robots tag and skips link extraction
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().nofollow_detected());
    }

    @Test
    void testRobotsMetaNoindex() {
        // Detects noindex meta robots tag in HTML page
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().noindex_detected());
    }

    @Test
    void testRobotsMissing404() {
        // Missing robots.txt (404) allows all crawling
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsMultipleUserAgents() {
        // Picks the most specific user-agent block from robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsRequestRate() {
        // Parses request-rate directive from robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(5, result.robots().crawl_delay());
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsSitemapDirective() {
        // Discovers sitemap URL from Sitemap directive in robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(true, result.robots().is_allowed());
    }

    @Test
    void testRobotsUserAgentSpecific() {
        // Matches user-agent specific rules in robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(false, result.robots().is_allowed());
    }

    @Test
    void testRobotsWildcardPaths() {
        // Handles wildcard Disallow patterns in robots.txt
        var result = Kreuzcrawl.scrape();
        assertEquals(false, result.robots().is_allowed());
    }

    @Test
    void testRobotsXRobotsTag() {
        // Respects X-Robots-Tag HTTP header directives
        var result = Kreuzcrawl.scrape();
        assertEquals("noindex, nofollow", result.robots().x_robots_tag());
        assertEquals(true, result.robots().noindex_detected());
        assertEquals(true, result.robots().nofollow_detected());
    }

}
