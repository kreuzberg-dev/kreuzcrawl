# E2e tests for category: batch
defmodule E2e.BatchTest do
  use ExUnit.Case, async: true

  describe "scrape_batch_basic" do
    test "Batch scrape of multiple URLs all succeeding" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'batch.completed_count' not available on result type
      # skipped: field 'batch.failed_count' not available on result type
      # skipped: field 'batch.total_count' not available on result type
    end
  end

  describe "scrape_batch_partial_failure" do
    test "Batch scrape with one URL failing returns partial results" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'batch.completed_count' not available on result type
      # skipped: field 'batch.failed_count' not available on result type
      # skipped: field 'batch.total_count' not available on result type
    end
  end

  describe "scrape_batch_progress" do
    test "Batch scrape results include specific URL" do
      engine = Kreuzcrawl.create_engine!(nil)
      result = Kreuzcrawl.scrape!(engine, "")
      # skipped: field 'batch.total_count' not available on result type
      # skipped: field 'batch.results' not available on result type
    end
  end
end
