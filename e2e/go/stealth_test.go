// E2e tests for category: stealth
package e2e_test

import (
	"encoding/json"
	"os"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_StealthUaRotationConfig(t *testing.T) {
	// User-agent rotation config is accepted and crawl succeeds
	var engineConfig pkg.CrawlConfig
	if err := json.Unmarshal([]byte(`{"user_agents":["Mozilla/5.0 (Windows NT 10.0)","Chrome/120.0.0.0"]}`), &engineConfig); err != nil {
		t.Fatalf("config parse failed: %v", err)
	}
	engine, createErr := pkg.CreateEngine(&engineConfig)
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/stealth_ua_rotation_config"
	result, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
}
