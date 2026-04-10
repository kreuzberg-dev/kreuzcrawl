import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('browser', () => {
  it('browser_config_auto_no_feature: Browser mode \'auto\' without browser feature enabled does not use browser', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.browser.jsRenderHint).toBe(true);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_config_never_mode: Browser mode \'never\' prevents browser fallback even for SPA shell content', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.browser.jsRenderHint).toBe(true);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_detect_minimal_page: Does NOT flag a short but real content page as needing JS rendering', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.browser.jsRenderHint).toBe(false);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_detect_next_empty: Detects Next.js page with __NEXT_DATA__ but no rendered content as needing JS rendering', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.browser.jsRenderHint).toBe(true);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_detect_next_rendered: Does NOT flag Next.js page with full SSR content as needing JS rendering', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.htmlNotEmpty).toBe(true);
    expect(result.browser.jsRenderHint).toBe(false);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_detect_normal_page: Does NOT flag a normal server-rendered page as needing JS rendering', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.browser.jsRenderHint).toBe(false);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_detect_nuxt_shell: Detects Nuxt SPA shell with empty #__nuxt div as needing JS rendering', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.browser.jsRenderHint).toBe(true);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_detect_react_shell: Detects React SPA shell with empty #root div as needing JS rendering', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.htmlNotEmpty).toBe(true);
    expect(result.browser.jsRenderHint).toBe(true);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_detect_vue_shell: Detects Vue SPA shell with empty #app div as needing JS rendering', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.statusCode).toBe(200);
    expect(result.browser.jsRenderHint).toBe(true);
    expect(result.browser.browserUsed).toBe(false);
  });

  it('browser_fallback_spa_render: Browser auto re-fetches SPA shell when JS rendering is detected', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.browser.jsRenderHint).toBe(true);
    expect(result.browser.browserUsed).toBe(true);
  });

  it('browser_fallback_waf_blocked: Browser fallback triggers when WAF blocks the HTTP request (Cloudflare 403)', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.browser.browserUsed).toBe(true);
  });

  it('browser_mode_always: Browser mode \'always\' uses browser even for normal server-rendered pages', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.browser.browserUsed).toBe(true);
  });
});
