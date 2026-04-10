// E2e tests for category: error
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_Error401Unauthorized(t *testing.T) {
	// Handles 401 Unauthorized response correctly
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_Error403Forbidden(t *testing.T) {
	// Handles 403 Forbidden response correctly
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_Error404Page(t *testing.T) {
	// Handles 404 response correctly
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_Error408RequestTimeout(t *testing.T) {
	// Handles 408 Request Timeout response correctly
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_Error410Gone(t *testing.T) {
	// Handles 410 Gone response correctly
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_Error500Server(t *testing.T) {
	// Handles 500 server error
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_Error502BadGateway(t *testing.T) {
	// Handles 502 Bad Gateway response correctly
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorConnectionRefused(t *testing.T) {
	// Handles connection refused error gracefully
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorDnsResolution(t *testing.T) {
	// Handles DNS resolution failure gracefully
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorEmptyResponse(t *testing.T) {
	// Handles 200 with completely empty body gracefully
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.HtmlNotEmpty != false {
		t.Errorf("equals mismatch: got %q", result.HtmlNotEmpty)
	}
	if result.Error.IsError != false {
		t.Errorf("equals mismatch: got %q", result.Error.IsError)
	}
}

func Test_ErrorInvalidProxy(t *testing.T) {
	// Proxy pointing to unreachable address causes connection error during scrape
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorPartialResponse(t *testing.T) {
	// Handles incomplete or truncated HTTP response
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorRateLimited(t *testing.T) {
	// Handles 429 rate limiting with Retry-After
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorRetry503(t *testing.T) {
	// Retries request on 503 Service Unavailable response
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorRetryBackoff(t *testing.T) {
	// Implements exponential backoff when retrying failed requests
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorSslInvalidCert(t *testing.T) {
	// Handles SSL certificate validation error
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorTimeout(t *testing.T) {
	// Handles request timeout
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorWafAkamai(t *testing.T) {
	// Akamai WAF detection returns WafBlocked error
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorWafFalse403(t *testing.T) {
	// Detects WAF/bot protection false 403 (Cloudflare challenge page)
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}

func Test_ErrorWafImperva(t *testing.T) {
	// Imperva/Incapsula WAF detection
	_, err := pkg.Scrape()
	if err == nil {
		t.Errorf("expected an error, but call succeeded")
	}
}
