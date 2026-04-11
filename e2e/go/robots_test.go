// E2e tests for category: robots
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_RobotsAllowAll(t *testing.T) {
	// Permissive robots.txt allows all paths
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != true {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsAllowOverride(t *testing.T) {
	// Allow directive overrides Disallow for specific paths
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != true {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsCommentsHandling(t *testing.T) {
	// Correctly parses robots.txt with inline and line comments
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != true {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsCrawlDelay(t *testing.T) {
	// Respects crawl-delay directive from robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.CrawlDelay != nil && *result.CrawlDelay != 2 {
		t.Errorf("equals mismatch: got %v", result.CrawlDelay)
	}
}

func Test_RobotsDisallowPath(t *testing.T) {
	// Robots.txt disallows specific paths
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != false {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsMetaNofollow(t *testing.T) {
	// Detects nofollow meta robots tag and skips link extraction
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.NofollowDetected != true {
		t.Errorf("equals mismatch: got %v", result.NofollowDetected)
	}
}

func Test_RobotsMetaNoindex(t *testing.T) {
	// Detects noindex meta robots tag in HTML page
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.NoindexDetected != true {
		t.Errorf("equals mismatch: got %v", result.NoindexDetected)
	}
}

func Test_RobotsMissing404(t *testing.T) {
	// Missing robots.txt (404) allows all crawling
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != true {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsMultipleUserAgents(t *testing.T) {
	// Picks the most specific user-agent block from robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != true {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsRequestRate(t *testing.T) {
	// Parses request-rate directive from robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.CrawlDelay != nil && *result.CrawlDelay != 5 {
		t.Errorf("equals mismatch: got %v", result.CrawlDelay)
	}
	if result.IsAllowed != true {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsSitemapDirective(t *testing.T) {
	// Discovers sitemap URL from Sitemap directive in robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != true {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsUserAgentSpecific(t *testing.T) {
	// Matches user-agent specific rules in robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != false {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsWildcardPaths(t *testing.T) {
	// Handles wildcard Disallow patterns in robots.txt
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsAllowed != false {
		t.Errorf("equals mismatch: got %v", result.IsAllowed)
	}
}

func Test_RobotsXRobotsTag(t *testing.T) {
	// Respects X-Robots-Tag HTTP header directives
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	result, err := pkg.Scrape(engine, "")
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	var xRobotsTag string
	if result.XRobotsTag != nil {
		xRobotsTag = *result.XRobotsTag
	}
	if strings.TrimSpace(xRobotsTag) != `noindex, nofollow` {
		t.Errorf("equals mismatch: got %v", xRobotsTag)
	}
	if result.NoindexDetected != true {
		t.Errorf("equals mismatch: got %v", result.NoindexDetected)
	}
	if result.NofollowDetected != true {
		t.Errorf("equals mismatch: got %v", result.NofollowDetected)
	}
}
