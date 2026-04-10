// E2e tests for category: auth
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_AuthBasicHttp(t *testing.T) {
	// Sends HTTP Basic authentication header
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.AuthHeaderSent != true {
		t.Errorf("equals mismatch: got %q", result.AuthHeaderSent)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
}

func Test_AuthBearerToken(t *testing.T) {
	// Sends Bearer token in Authorization header
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.AuthHeaderSent != true {
		t.Errorf("equals mismatch: got %q", result.AuthHeaderSent)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
}

func Test_AuthCustomHeader(t *testing.T) {
	// Sends authentication via custom header (X-API-Key)
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.AuthHeaderSent != true {
		t.Errorf("equals mismatch: got %q", result.AuthHeaderSent)
	}
	if result.StatusCode != 200 {
		t.Errorf("equals mismatch: got %q", result.StatusCode)
	}
}
