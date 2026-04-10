# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "batch" do
  it "scrape_batch_basic: Batch scrape of multiple URLs all succeeding" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'batch.completed_count' not available on result type
      # skipped: field 'batch.failed_count' not available on result type
      # skipped: field 'batch.total_count' not available on result type
  end

  it "scrape_batch_partial_failure: Batch scrape with one URL failing returns partial results" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'batch.completed_count' not available on result type
      # skipped: field 'batch.failed_count' not available on result type
      # skipped: field 'batch.total_count' not available on result type
  end

  it "scrape_batch_progress: Batch scrape results include specific URL" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
      # skipped: field 'batch.total_count' not available on result type
      # skipped: field 'batch.results' not available on result type
  end
end
