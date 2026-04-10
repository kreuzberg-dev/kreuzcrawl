import { describe, it, expect } from 'vitest';
import { scrape } from '@kreuzberg/kreuzcrawl';

describe('cookies', () => {
  it('cookies_per_domain: Isolates cookies per domain during crawl', async () => {
    const result = await scrape();
    expect(result.cookies.length).toBe(1);
    expect(result.cookies).toContain("domain_cookie");
  });

  it('cookies_persistence: Maintains cookies across multiple crawl requests', async () => {
    const result = await scrape();
    expect(result.cookies).toContain("session");
  });

  it('cookies_set_cookie_response: Respects Set-Cookie header from server responses', async () => {
    const result = await scrape();
    expect(result.cookies).toContain("tracking");
  });
});
