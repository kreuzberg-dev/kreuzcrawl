// E2e tests for category: batch
package e2e_test

import (
	"strings"
	"testing"

	pkg "github.com/kreuzberg-dev/kreuzcrawl"
)

func Test_ScrapeBatchBasic(t *testing.T) {
	// Batch scrape of multiple URLs all succeeding
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Batch.CompletedCount != 3 {
		t.Errorf("equals mismatch: got %q", result.Batch.CompletedCount)
	}
	if result.Batch.FailedCount != 0 {
		t.Errorf("equals mismatch: got %q", result.Batch.FailedCount)
	}
	if result.Batch.TotalCount != 3 {
		t.Errorf("equals mismatch: got %q", result.Batch.TotalCount)
	}
}

func Test_ScrapeBatchPartialFailure(t *testing.T) {
	// Batch scrape with one URL failing returns partial results
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Batch.CompletedCount != 2 {
		t.Errorf("equals mismatch: got %q", result.Batch.CompletedCount)
	}
	if result.Batch.FailedCount != 1 {
		t.Errorf("equals mismatch: got %q", result.Batch.FailedCount)
	}
	if result.Batch.TotalCount != 3 {
		t.Errorf("equals mismatch: got %q", result.Batch.TotalCount)
	}
}

func Test_ScrapeBatchProgress(t *testing.T) {
	// Batch scrape results include specific URL
	result, err := pkg.Scrape()
	if err != nil {
		t.Fatalf("call failed: %v", err)
	}
	if result.Batch.TotalCount != 2 {
		t.Errorf("equals mismatch: got %q", result.Batch.TotalCount)
	}
	if !strings.Contains(result.Batch.Results, `/target`) {
		t.Errorf("expected to contain %s, got %q", `/target`, result.Batch.Results)
	}
}
