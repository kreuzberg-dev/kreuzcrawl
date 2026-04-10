# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "batch" do
  it "scrape_batch_basic: Batch scrape of multiple URLs all succeeding" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.batch.completed_count).to eq(3)
    expect(result.batch.failed_count).to eq(0)
    expect(result.batch.total_count).to eq(3)
  end

  it "scrape_batch_partial_failure: Batch scrape with one URL failing returns partial results" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.batch.completed_count).to eq(2)
    expect(result.batch.failed_count).to eq(1)
    expect(result.batch.total_count).to eq(3)
  end

  it "scrape_batch_progress: Batch scrape results include specific URL" do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, "")
    expect(result.batch.total_count).to eq(2)
    expect(result.batch.results).to include("/target")
  end
end
