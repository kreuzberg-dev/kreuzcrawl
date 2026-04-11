// E2e tests for category: auth
package e2e_test

import (
	"os"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_AuthBasicHttp(t *testing.T) {
	// Sends HTTP Basic authentication header
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/auth_basic_http"
	result, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.AuthHeaderSent != true {
		t.Errorf("equals mismatch: got %v", result.AuthHeaderSent)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
}

func Test_AuthBearerToken(t *testing.T) {
	// Sends Bearer token in Authorization header
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/auth_bearer_token"
	result, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.AuthHeaderSent != true {
		t.Errorf("equals mismatch: got %v", result.AuthHeaderSent)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
}

func Test_AuthCustomHeader(t *testing.T) {
	// Sends authentication via custom header (X-API-Key)
	engine, createErr := pkg.CreateEngine()
	if createErr != nil {
		t.Fatalf("create handle failed: %v", createErr)
	}
	url := os.Getenv("MOCK_SERVER_URL") + "/fixtures/auth_custom_header"
	result, err := pkg.Scrape(engine, url)
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.AuthHeaderSent != true {
		t.Errorf("equals mismatch: got %v", result.AuthHeaderSent)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %v", result.StatusCode)
	}
}
