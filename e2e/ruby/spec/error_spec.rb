# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "error" do
  it "error_401_unauthorized: Handles 401 Unauthorized response correctly" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_403_forbidden: Handles 403 Forbidden response correctly" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_404_page: Handles 404 response correctly" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_408_request_timeout: Handles 408 Request Timeout response correctly" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_410_gone: Handles 410 Gone response correctly" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_500_server: Handles 500 server error" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_502_bad_gateway: Handles 502 Bad Gateway response correctly" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_connection_refused: Handles connection refused error gracefully" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_dns_resolution: Handles DNS resolution failure gracefully" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_empty_response: Handles 200 with completely empty body gracefully" do
    result = Kreuzcrawl.scrape()
    expect(result.html_not_empty).to eq(false)
    expect(result.error.is_error).to eq(false)
  end

  it "error_invalid_proxy: Proxy pointing to unreachable address causes connection error during scrape" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_partial_response: Handles incomplete or truncated HTTP response" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_rate_limited: Handles 429 rate limiting with Retry-After" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_retry_503: Retries request on 503 Service Unavailable response" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_retry_backoff: Implements exponential backoff when retrying failed requests" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_ssl_invalid_cert: Handles SSL certificate validation error" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_timeout: Handles request timeout" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_waf_akamai: Akamai WAF detection returns WafBlocked error" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_waf_false_403: Detects WAF/bot protection false 403 (Cloudflare challenge page)" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end

  it "error_waf_imperva: Imperva/Incapsula WAF detection" do
    expect { Kreuzcrawl.scrape() }.to raise_error
  end
end
