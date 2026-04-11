# E2e tests for category: auth
defmodule E2e.AuthTest do
  use ExUnit.Case, async: true

  describe "auth_basic_http" do
    test "Sends HTTP Basic authentication header" do
      engine_config = %{"auth" => %{"password" => "testpass", "type" => "basic", "username" => "testuser"}, "respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/auth_basic_http"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.auth_header_sent) == true
      assert String.trim(result.status_code) == 200
    end
  end

  describe "auth_bearer_token" do
    test "Sends Bearer token in Authorization header" do
      engine_config = %{"auth" => %{"token" => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.test", "type" => "bearer"}, "respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/auth_bearer_token"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.auth_header_sent) == true
      assert String.trim(result.status_code) == 200
    end
  end

  describe "auth_custom_header" do
    test "Sends authentication via custom header (X-API-Key)" do
      engine_config = %{"auth" => %{"name" => "X-API-Key", "type" => "header", "value" => "sk-test-key-12345"}, "respect_robots_txt" => false}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/auth_custom_header"
      result = Kreuzcrawl.scrape!(engine, url)
      assert String.trim(result.auth_header_sent) == true
      assert String.trim(result.status_code) == 200
    end
  end
end
