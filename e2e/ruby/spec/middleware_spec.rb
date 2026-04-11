# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'middleware' do
  it 'middleware_noop_no_effect: Default middleware chain does not affect normal scraping' do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, '')
    expect(result.status_code).to eq(200)
    expect(result.metadata.title).to eq('Middleware Test')
  end
end
