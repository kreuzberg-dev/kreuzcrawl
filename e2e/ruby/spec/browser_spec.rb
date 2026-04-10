# frozen_string_literal: true

require "kreuzcrawl"

RSpec.describe "browser" do
  it "browser_config_auto_no_feature: Browser mode 'auto' without browser feature enabled does not use browser" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.browser.js_render_hint).to eq(true)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_config_never_mode: Browser mode 'never' prevents browser fallback even for SPA shell content" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.browser.js_render_hint).to eq(true)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_detect_minimal_page: Does NOT flag a short but real content page as needing JS rendering" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.browser.js_render_hint).to eq(false)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_detect_next_empty: Detects Next.js page with __NEXT_DATA__ but no rendered content as needing JS rendering" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.browser.js_render_hint).to eq(true)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_detect_next_rendered: Does NOT flag Next.js page with full SSR content as needing JS rendering" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.html_not_empty).to eq(true)
    expect(result.browser.js_render_hint).to eq(false)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_detect_normal_page: Does NOT flag a normal server-rendered page as needing JS rendering" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.browser.js_render_hint).to eq(false)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_detect_nuxt_shell: Detects Nuxt SPA shell with empty #__nuxt div as needing JS rendering" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.browser.js_render_hint).to eq(true)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_detect_react_shell: Detects React SPA shell with empty #root div as needing JS rendering" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.html_not_empty).to eq(true)
    expect(result.browser.js_render_hint).to eq(true)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_detect_vue_shell: Detects Vue SPA shell with empty #app div as needing JS rendering" do
    result = Kreuzcrawl.scrape()
    expect(result.status_code).to eq(200)
    expect(result.browser.js_render_hint).to eq(true)
    expect(result.browser.browser_used).to eq(false)
  end

  it "browser_fallback_spa_render: Browser auto re-fetches SPA shell when JS rendering is detected" do
    result = Kreuzcrawl.scrape()
    expect(result.browser.js_render_hint).to eq(true)
    expect(result.browser.browser_used).to eq(true)
  end

  it "browser_fallback_waf_blocked: Browser fallback triggers when WAF blocks the HTTP request (Cloudflare 403)" do
    result = Kreuzcrawl.scrape()
    expect(result.browser.browser_used).to eq(true)
  end

  it "browser_mode_always: Browser mode 'always' uses browser even for normal server-rendered pages" do
    result = Kreuzcrawl.scrape()
    expect(result.browser.browser_used).to eq(true)
  end
end
