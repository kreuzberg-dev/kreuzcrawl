// E2e tests for category: cookies
package e2e_test

import (
	"encoding/json"
	"os"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_CookiesPerDomain(t *testing.T) {
	// Isolates cookies per domain during crawl
	var engineConfig pkg.CrawlConfig
	if err := json.Unmarshal([]byte(`{"cookies_enabled":true,"max_depth":1,"respect_robots_txt":false}`), &engineConfig); err != nil {
		t.Fatalf("config parse failed: %v", err)
	}
	engine, createErr := pkg.CreateEngine(&engineConfig)
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/cookies_per_domain"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'cookies.length' not available on result type
	// skipped: field 'cookies' not available on result type
}

func Test_CookiesPersistence(t *testing.T) {
	// Maintains cookies across multiple crawl requests
	var engineConfig pkg.CrawlConfig
	if err := json.Unmarshal([]byte(`{"cookies_enabled":true,"max_depth":1,"respect_robots_txt":false}`), &engineConfig); err != nil {
		t.Fatalf("config parse failed: %v", err)
	}
	engine, createErr := pkg.CreateEngine(&engineConfig)
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/cookies_persistence"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'cookies' not available on result type
}

func Test_CookiesSetCookieResponse(t *testing.T) {
	// Respects Set-Cookie header from server responses
	var engineConfig pkg.CrawlConfig
	if err := json.Unmarshal([]byte(`{"cookies_enabled":true,"max_depth":1,"respect_robots_txt":false}`), &engineConfig); err != nil {
		t.Fatalf("config parse failed: %v", err)
	}
	engine, createErr := pkg.CreateEngine(&engineConfig)
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/cookies_set_cookie_response"
	_, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	// skipped: field 'cookies' not available on result type
}
