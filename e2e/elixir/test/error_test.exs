# E2e tests for category: error
defmodule E2e.ErrorTest do
  use ExUnit.Case, async: true

  describe "error_401_unauthorized" do
    test "Handles 401 Unauthorized response correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_401_unauthorized"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_403_forbidden" do
    test "Handles 403 Forbidden response correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_403_forbidden"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_404_page" do
    test "Handles 404 response correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_404_page"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_408_request_timeout" do
    test "Handles 408 Request Timeout response correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_408_request_timeout"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_410_gone" do
    test "Handles 410 Gone response correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_410_gone"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_500_server" do
    test "Handles 500 server error" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_500_server"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_502_bad_gateway" do
    test "Handles 502 Bad Gateway response correctly" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_502_bad_gateway"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_empty_response" do
    test "Handles 200 with completely empty body gracefully" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_empty_response"
      result = Kreuzcrawl.scrape!(engine, url)
      # skipped: field 'html_not_empty' not available on result type
      # skipped: field 'error.is_error' not available on result type
    end
  end

  describe "error_invalid_proxy" do
    test "Proxy pointing to unreachable address causes connection error during scrape" do
      engine_config = %{"proxy" => %{"url" => "http://127.0.0.1:1"}, "request_timeout" => 2000}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_invalid_proxy"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_partial_response" do
    test "Handles incomplete or truncated HTTP response" do
      engine_config = %{"respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_partial_response"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_rate_limited" do
    test "Handles 429 rate limiting with Retry-After" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_rate_limited"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_retry_503" do
    test "Retries request on 503 Service Unavailable response" do
      engine_config = %{"respect_robots_txt" => false, "retry_codes" => [503], "retry_count" => 2}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_retry_503"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_retry_backoff" do
    test "Implements exponential backoff when retrying failed requests" do
      engine_config = %{"respect_robots_txt" => false, "retry_codes" => [429], "retry_count" => 3}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_retry_backoff"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_timeout" do
    test "Handles request timeout" do
      engine_config = %{"request_timeout" => 1}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_timeout"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_waf_akamai" do
    test "Akamai WAF detection returns WafBlocked error" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_waf_akamai"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_waf_false_403" do
    test "Detects WAF/bot protection false 403 (Cloudflare challenge page)" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_waf_false_403"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "error_waf_imperva" do
    test "Imperva/Incapsula WAF detection" do
      engine = Kreuzcrawl.create_engine!(nil)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/error_waf_imperva"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end
end
