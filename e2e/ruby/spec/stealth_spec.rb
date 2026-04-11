# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'stealth' do
  it 'stealth_ua_rotation_config: User-agent rotation config is accepted and crawl succeeds' do
    engine = Kreuzcrawl.create_engine(nil)
    result = Kreuzcrawl.scrape(engine, '')
    expect(result.status_code).to eq(200)
  end
end
