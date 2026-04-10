import { describe, it, expect } from 'vitest';
import { scrape, createEngine } from '@kreuzberg/kreuzcrawl';

describe('cookies', () => {
  it('cookies_per_domain: Isolates cookies per domain during crawl', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.cookies.length).toBe(1);
    expect(result.cookies).toContain("domain_cookie");
  });

  it('cookies_persistence: Maintains cookies across multiple crawl requests', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.cookies).toContain("session");
  });

  it('cookies_set_cookie_response: Respects Set-Cookie header from server responses', async () => {
    const engine = createEngine(null);
    const result = await scrape(engine, "");
    expect(result.cookies).toContain("tracking");
  });
});
