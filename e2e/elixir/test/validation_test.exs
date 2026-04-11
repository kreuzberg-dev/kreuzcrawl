# E2e tests for category: validation
defmodule E2e.ValidationTest do
  use ExUnit.Case, async: true

  describe "validation_invalid_exclude_regex" do
    test "Invalid regex in exclude_paths is rejected" do
      engine_config = %{"exclude_paths" => ["(unclosed"]}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/validation_invalid_exclude_regex"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "validation_invalid_include_regex" do
    test "Invalid regex in include_paths is rejected" do
      engine_config = %{"include_paths" => ["[invalid"]}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/validation_invalid_include_regex"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "validation_invalid_retry_code" do
    test "Retry code outside 100-599 is rejected" do
      engine_config = %{"retry_codes" => [999]}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/validation_invalid_retry_code"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "validation_max_pages_zero" do
    test "max_pages=0 is rejected as invalid config" do
      engine_config = %{"max_pages" => 0}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/validation_max_pages_zero"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "validation_max_redirects_too_high" do
    test "max_redirects > 100 is rejected as invalid config" do
      engine_config = %{"max_redirects" => 200}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/validation_max_redirects_too_high"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end

  describe "validation_timeout_zero" do
    test "Zero request timeout is rejected as invalid config" do
      engine_config = %{"request_timeout" => 0}
      engine = Kreuzcrawl.create_engine!(engine_config)
      url = System.get_env("MOCK_SERVER_URL") <> "/fixtures/validation_timeout_zero"
      assert_raise RuntimeError, fn ->
        Kreuzcrawl.scrape!(engine, url)
      end
    end
  end
end
