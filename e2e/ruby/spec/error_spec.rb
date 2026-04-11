# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'error' do
  it 'error_401_unauthorized: Handles 401 Unauthorized response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_401_unauthorized"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_403_forbidden: Handles 403 Forbidden response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_403_forbidden"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_404_page: Handles 404 response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_404_page"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_408_request_timeout: Handles 408 Request Timeout response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_408_request_timeout"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_410_gone: Handles 410 Gone response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_410_gone"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_500_server: Handles 500 server error' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_500_server"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_502_bad_gateway: Handles 502 Bad Gateway response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_502_bad_gateway"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_invalid_proxy: Proxy pointing to unreachable address causes connection error during scrape' do
    engine_config = { 'proxy' => { 'url' => 'http://127.0.0.1:1' }, 'request_timeout' => 2000 }
    engine = Kreuzcrawl.create_engine(engine_config)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_invalid_proxy"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_partial_response: Handles incomplete or truncated HTTP response' do
    engine_config = { 'respect_robots_txt' => false }
    engine = Kreuzcrawl.create_engine(engine_config)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_partial_response"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_rate_limited: Handles 429 rate limiting with Retry-After' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_rate_limited"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_retry_503: Retries request on 503 Service Unavailable response' do
    engine_config = { 'respect_robots_txt' => false, 'retry_codes' => [503], 'retry_count' => 2 }
    engine = Kreuzcrawl.create_engine(engine_config)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_retry_503"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_retry_backoff: Implements exponential backoff when retrying failed requests' do
    engine_config = { 'respect_robots_txt' => false, 'retry_codes' => [429], 'retry_count' => 3 }
    engine = Kreuzcrawl.create_engine(engine_config)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_retry_backoff"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_timeout: Handles request timeout' do
    engine_config = { 'request_timeout' => 1 }
    engine = Kreuzcrawl.create_engine(engine_config)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_timeout"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_waf_akamai: Akamai WAF detection returns WafBlocked error' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_waf_akamai"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_waf_false_403: Detects WAF/bot protection false 403 (Cloudflare challenge page)' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_waf_false_403"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end

  it 'error_waf_imperva: Imperva/Incapsula WAF detection' do
    engine = Kreuzcrawl.create_engine(nil)
    url = "#{ENV.fetch('MOCK_SERVER_URL')}/fixtures/error_waf_imperva"
    expect { Kreuzcrawl.scrape(engine, url) }.to raise_error
  end
end
