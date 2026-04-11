# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'engine' do
  it 'engine_scrape_basic: CrawlEngine with defaults scrapes a page identically to the free function' do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, '')
    expect(result.status_code).to eq(200)
    expect(result.content_type).to eq('text/html')
    expect(result.metadata.title).to eq('Engine Test')
    expect(result.metadata.description).to include('Testing the engine')
    expect(result.links.length).to be >= 1
    expect(result.metadata.headings.length).to be >= 1
  end
end
