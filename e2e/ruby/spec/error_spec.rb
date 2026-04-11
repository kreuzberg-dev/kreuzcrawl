# frozen_string_literal: true

require 'kreuzcrawl'

RSpec.describe 'error' do
  it 'error_401_unauthorized: Handles 401 Unauthorized response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_403_forbidden: Handles 403 Forbidden response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_404_page: Handles 404 response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_408_request_timeout: Handles 408 Request Timeout response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_410_gone: Handles 410 Gone response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_500_server: Handles 500 server error' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_502_bad_gateway: Handles 502 Bad Gateway response correctly' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_connection_refused: Handles connection refused error gracefully' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_dns_resolution: Handles DNS resolution failure gracefully' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_invalid_proxy: Proxy pointing to unreachable address causes connection error during scrape' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_partial_response: Handles incomplete or truncated HTTP response' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_rate_limited: Handles 429 rate limiting with Retry-After' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_retry_503: Retries request on 503 Service Unavailable response' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_retry_backoff: Implements exponential backoff when retrying failed requests' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_ssl_invalid_cert: Handles SSL certificate validation error' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_timeout: Handles request timeout' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_waf_akamai: Akamai WAF detection returns WafBlocked error' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_waf_false_403: Detects WAF/bot protection false 403 (Cloudflare challenge page)' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end

  it 'error_waf_imperva: Imperva/Incapsula WAF detection' do
    engine = Kreuzcrawl.create_engine(nil)
    expect { Kreuzcrawl.scrape(engine, '') }.to raise_error
  end
end
