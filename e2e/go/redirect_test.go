// E2e tests for category: redirect
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_Redirect301Permanent(t *testing.T) {
	// Follows 301 permanent redirect and returns final page content
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/target`) {
		t.Errorf("expected to contain %s, got %q", `/target`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_Redirect302Found(t *testing.T) {
	// Follows 302 Found redirect correctly
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/found-target`) {
		t.Errorf("expected to contain %s, got %q", `/found-target`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_Redirect303SeeOther(t *testing.T) {
	// Follows 303 See Other redirect (method changes to GET)
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/see-other`) {
		t.Errorf("expected to contain %s, got %q", `/see-other`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_Redirect307Temporary(t *testing.T) {
	// Follows 307 Temporary Redirect (preserves method)
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/temp-target`) {
		t.Errorf("expected to contain %s, got %q", `/temp-target`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_Redirect308Permanent(t *testing.T) {
	// Follows 308 Permanent Redirect (preserves method)
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/perm-target`) {
		t.Errorf("expected to contain %s, got %q", `/perm-target`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_RedirectChain(t *testing.T) {
	// Follows a chain of redirects (301 -> 302 -> 200)
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/step2`) {
		t.Errorf("expected to contain %s, got %q", `/step2`, result.FinalUrl)
	}
	if result.RedirectCount != 2 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_RedirectCrossDomain(t *testing.T) {
	// Reports cross-domain redirect target without following to external domain
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/external-redirect`) {
		t.Errorf("expected to contain %s, got %q", `/external-redirect`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_RedirectLoop(t *testing.T) {
	// Detects redirect loop (A -> B -> A) and returns error
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsError != true {
		t.Errorf("equals mismatch: got %q", result.IsError)
	}
}

func Test_RedirectMaxExceeded(t *testing.T) {
	// Aborts when redirect count exceeds max_redirects limit
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.IsError != true {
		t.Errorf("equals mismatch: got %q", result.IsError)
	}
}

func Test_RedirectMetaRefresh(t *testing.T) {
	// Follows HTML meta-refresh redirect to target page
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/target`) {
		t.Errorf("expected to contain %s, got %q", `/target`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_RedirectRefreshHeader(t *testing.T) {
	// Handles HTTP Refresh header redirect
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/refreshed`) {
		t.Errorf("expected to contain %s, got %q", `/refreshed`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
}

func Test_RedirectTo404(t *testing.T) {
	// Redirect target returns 404 Not Found
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if !strings.Contains(result.FinalUrl, `/gone`) {
		t.Errorf("expected to contain %s, got %q", `/gone`, result.FinalUrl)
	}
	if result.RedirectCount != 1 {
		t.Errorf("equals mismatch: got %q", result.RedirectCount)
	}
	if result.IsError != true {
		t.Errorf("equals mismatch: got %q", result.IsError)
	}
}
